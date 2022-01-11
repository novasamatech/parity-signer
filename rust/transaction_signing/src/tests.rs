#[cfg(test)]
mod tests {
    use hex;
    use transaction_parsing::{Action, produce_output, print_history_entry_by_order_with_decoding, StubNav};
    use crate::{handle_stub, sign_transaction::create_signature};
    use db_handling::{cold_default::{populate_cold, populate_cold_no_networks}, identities::try_create_seed, manage_history::print_history, remove_network::remove_network};
    use definitions::{crypto::Encryption, error::{AddressKeySource, DatabaseSigner, ErrorSigner, ErrorSource, Signer}, keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, network_specs::{CurrentVerifier, NetworkSpecs, Verifier, VerifierValue}, users::AddressDetails};
    use constants::{ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, VERIFIERS};
    use parity_scale_codec::{Decode, Encode};
    use std::fs;
    use sled::{Db, open, Tree};
    use sp_core;
    use sp_runtime::MultiSigner;
    
    const SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    const PWD: &str = "jaskier";
    const USER_COMMENT: &str = "";
    const ALICE: [u8; 32] = [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
    fn verifier_alice_sr25519() -> Verifier {
        Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(ALICE)))))
    }
    
    fn sign_action_test (checksum: u32, seed_phrase: &str, pwd_entry: &str, user_comment: &str, dbname: &str) -> Result<String, ErrorSigner> {
        Ok(hex::encode(create_signature(seed_phrase, pwd_entry, user_comment, dbname, checksum)?.encode()))
    }
    
    fn print_db_content (dbname: &str) -> String {
        let database: Db = open(dbname).unwrap();
        
        let mut metadata_set: Vec<String> = Vec::new();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        for x in metadata.iter() {
            if let Ok((meta_key_vec, _)) = x {
                let meta_key = MetaKey::from_ivec(&meta_key_vec);
                let (name, version) = meta_key.name_version::<Signer>().unwrap();
                metadata_set.push(format!("{}{}", name, version));
            }
        }
        metadata_set.sort();
        let mut metadata_str = String::new();
        for x in metadata_set.iter() {metadata_str.push_str(&format!("\n\t{}", x))}
        
        let mut network_specs_set: Vec<(NetworkSpecsKey, NetworkSpecs)> = Vec::new();
        let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
        for x in chainspecs.iter() {
            if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
                let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
                let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, network_specs_encoded).unwrap();
                network_specs_set.push((network_specs_key, network_specs));
            }
        }
        network_specs_set.sort_by(|(_, a), (_, b)| a.title.cmp(&b.title));
        let mut network_specs_str = String::new();
        for (network_specs_key, network_specs) in network_specs_set.iter() {network_specs_str.push_str(&format!("\n\t{}: {} ({} with {})", hex::encode(network_specs_key.key()), network_specs.title, network_specs.name, network_specs.encryption.show()))}
        
        let settings: Tree = database.open_tree(SETTREE).unwrap();
        let general_verifier_encoded = settings.get(&GENERALVERIFIER).unwrap().unwrap();
        let general_verifier = Verifier::decode(&mut &general_verifier_encoded[..]).unwrap();
        
        let mut verifiers_set: Vec<String> = Vec::new();
        let verifiers: Tree = database.open_tree(VERIFIERS).unwrap();
        for x in verifiers.iter() {
            if let Ok((verifier_key_vec, current_verifier_encoded)) = x {
                let verifier_key = VerifierKey::from_ivec(&verifier_key_vec);
                let current_verifier = CurrentVerifier::decode(&mut &current_verifier_encoded[..]).unwrap();
                match current_verifier {
                    CurrentVerifier::Valid(a) => verifiers_set.push(format!("{}: {}", hex::encode(verifier_key.key()), a.show(&general_verifier))),
                    CurrentVerifier::Dead => verifiers_set.push(format!("{}: network inactivated", hex::encode(verifier_key.key()))),
                }
            }
        }
        verifiers_set.sort();
        let mut verifiers_str = String::new();
        for x in verifiers_set.iter() {verifiers_str.push_str(&format!("\n\t{}", x))}
        
        let mut identities_set: Vec<String> = Vec::new();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        for x in identities.iter() {
            if let Ok((address_key_vec, address_details_encoded)) = x {
                let address_key = AddressKey::from_ivec(&address_key_vec);
                let address_details = AddressDetails::decode(&mut &address_details_encoded[..]).unwrap();
                let (public_key, encryption) = address_key.public_key_encryption::<Signer>(AddressKeySource::AddrTree).unwrap();
                
                let mut networks_set: Vec<String> = Vec::new();
                for y in address_details.network_id.iter() {
                    networks_set.push(hex::encode(y.key()))
                }
                networks_set.sort();
                let mut networks_str = String::new();
                for y in networks_set.iter() {networks_str.push_str(&format!("\n\t\t{}", y))}
                
                identities_set.push(format!("public_key: {}, encryption: {}, path: {}, available_networks: {}", hex::encode(public_key), encryption.show(), address_details.path, networks_str));
            }
        }
        identities_set.sort();
        let mut identities_str = String::new();
        for x in identities_set.iter() {identities_str.push_str(&format!("\n\t{}", x))}
        
        format!("Database contents:\nMetadata:{}\nNetwork Specs:{}\nVerifiers:{}\nGeneral Verifier: {}\nIdentities: {}", metadata_str, network_specs_str, verifiers_str, general_verifier.show_error(), identities_str)
    }
    
// can sign a parsed transaction
    #[test]
    fn can_sign_transaction_1() {
        let dbname = "for_tests/can_sign_transaction_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        let output = produce_output(line, dbname);
        if let Action::Sign{content, checksum, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
            
            match sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
                Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
                Err(e) => panic!("Was unable to sign. {:?}", e),
            }

            let history_recorded = print_history(&dbname).unwrap();
            let my_event = r#""events":[{"event":"transaction_signed","payload":{"transaction":"a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"},"user_comment":""}}]"#;
            assert!(history_recorded.contains(my_event), "Recorded history is different: \n{}", history_recorded);
            
            let result = sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname);
            if let Err(e) = result {
                let expected_err = ErrorSigner::Database(DatabaseSigner::ChecksumMismatch);
                if <Signer>::show(&e) != <Signer>::show(&expected_err) {panic!("Expected wrong checksum. Got error: {:?}.", e)}
            }
            else {panic!("Checksum should have changed.")}
                
            let historic_reply = print_history_entry_by_order_with_decoding(2, dbname).unwrap();
            let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
            assert!(historic_reply.contains(historic_reply_known), "Received different historic reply: \n{}", historic_reply);
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }

// can sign a message
    #[test]
    fn can_sign_message_1() {
        let dbname = "for_tests/can_sign_message_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27df5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let output = produce_output(line, dbname);
        let content_known = r#""message":[{"index":0,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"}]"#;
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        if let Action::Sign{content, checksum, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
            
            match sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
                Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
                Err(e) => panic!("Was unable to sign. {:?}", e),
            }
            
            let history_recorded = print_history(&dbname).unwrap();
            let my_event = r#""events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"},"user_comment":""}}]"#;
            assert!(history_recorded.contains(my_event), "Recorded history is different: \n{}", history_recorded);
            
            let result = sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname);
            if let Err(e) = result {
                let expected_err = ErrorSigner::Database(DatabaseSigner::ChecksumMismatch);
                if <Signer>::show(&e) != <Signer>::show(&expected_err) {panic!("Expected wrong checksum. Got error: {:?}.", e)}
            }
            else {panic!("Checksum should have changed.")}
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn add_specs_westend_no_network_info_not_signed() {
        let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
        populate_cold_no_networks(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = "Database contents:\nMetadata:\nNetwork Specs:\nVerifiers:\nGeneral Verifier: none\nIdentities: ";
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: "#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }
      
    #[test]
    fn add_specs_westend_ed25519_not_signed() {
        let dbname = "for_tests/add_specs_westend_ed25519_not_signed";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Ed25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error in parsing. Received: \n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
            try_create_seed("Alice", SEED_PHRASE, true, dbname).unwrap();
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef, encryption: ed25519, path: , available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9070() {
        let dbname = "for_tests/load_westend9070";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000031a49444154789cedda3d6e13611485e1448826928b4851b28534543474281b6005d981574085a85881779015b081888e868a265b4814c985a53408997b649dc00ce3ef9efbfd8c6c67dee2b8b165dda77061fb78bd5e1fbdf44641582e97d96f727a7a7a6c0f4d6b825072b4570b94aa082d8fef5713a30a42cef1b31f0bdb6eabb773db5835308a10728e4743002c070295606423b4006063436421e402a096082807228450723c6b8dc0221832420d003416025221248408c0e2d5ccb6dbfcf7ca765314e1ecfa836db7c79bafb65a0a848b500ac054080f80d584a8869002601e840ac05488220415004511bc6a22a014c456840800da7504b40d6210210a80f601010d41c8088bd97faf3d9aaffe3e2d8a70f9eec4b6dbddf727db4d5184b337d7b6dd1e7fded87693105400a64278004c85f0009802e122a4009807a102300f4205607d8824421f004511bca2085e3908e85f8809c17a46180240878a8008511de1e2f6936db7fbabcfb69ba208b3c52fdb6eabf96bdb4da320a0148407c054080f80a910db005008010d41a800cc83500198079102401d040f40298ae01545c80d1013c284104438b97c6fdbede9ee9beda628c2c78b5bdb6e5feeaf6c374511cecfcf6dbb3d3c3cd8a693118600980ae1013015c203601e84849002601e840ac03c081580a5209a20784511bc26046b42b07612e1203f13500ac203602a8407c0548814009211d010840ac03c08158079101e000a21784511bca208b94d08d684603d23200fe2e0bf4f4029842100a64278004c85f000580a42464801300f4205601e840ac0b6413443f08a22785545404310878a40003421581d04d4878822ecfd6f91a88f8052101e0053213c00a642f401908b805408158079102a00f32014002423784511bca2086a32028a42ec03c21000da8a802210bb8eb00d002511900a1145d89bff31221501a5203c00a642a800a818019542a800cc83a8098024041481481545284901403202aa013116820a804208ac04a33542e47896858072215a22e400a06c04d402626c005484c07230862072004a8e675510580e466e358e675511584b8c9ac7b32608fd4a505a1cdd6f14845def0f3887bfac02c861740000000049454e44ae426082"}}]"##;
        let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error in parsing. Received: \n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
        
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
	westend9070
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_known_types_upd_general_verifier() {
        let dbname = "for_tests/load_known_types_upd_general_verifier";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082"}}]"#;
        let stub_nav_known = StubNav::LoadTypes;
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_new_types_verified() {
        let dbname = "for_tests/load_new_types_verified";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types","payload":{"types_hash":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035d49444154789cedd8bd8d144110c5712e84732e0d4c086163c1048b08b0c024960d014cd238674358fa6955a7b74d4d7d74578f5662fec69eb1a3ee373fe9a483a7ebf5faee7f6f1784cbe5327cc9f3f3f353fbb1b42508332fedb502a51461e5cbf7556294208cbcfcd71fa7f679dfb7cfe7f699ab02630a61e4e59106208d40a0198c61841500d2de104308a3006825021a814821ccbcbcb41a41ca6084112a00d05e08280a1142c800fcfafea97ddef7f1cbcff6792b8be09de7158170116601241e6e417800129fe7e54194215883251eae414401243ecf6a0a210a802a47a3eaf32c884d840c00aa1e5d7d1eda825011b200a87a74f57992061146a8fe1d3ebffe6e9ff79d5e3eb4cf5bd9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e8638105a6f081a001abd64ab47414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e76d058803e1404822787fb96547ff39bfb6cffbde9f5edae7adec79debeadc208da05125f640de7c11a801485e0f3a2fbb44208d605125fa40de7c11680e441f079d97d7d4b10bcb2085eb3fb0e84d681d05a82e0fd0e6711bcf3b2fbfa4208c8ba882fd0064b3cdc82f000243e2fba4f2b8c80b48bf8026bb0c4c335882880c4e779fbb64a217865477b559fb7d581d03a105a6f08c883f0febd9e1dad3def7ddfc7cf7bfbb400d07edcfe3f015908da05125f640de7c1d5cf45f7f58511ac0b24be481bce83b5effbbce7f9fbec3e6e198297f6527dfc925e33fbfe41401ac4cc255a8f822000e84068dd21a01e227b89f7975b16c13b2fbb0f310072119075115fa00d9678b805e101487c5e749fe422a028045f600d9678b806110590f83c6f9fd403a030825776b457f5795218016521aa47579f873400b489803210d5a3abcfdb024026028a4264477bbfc3d9f3ac2c00548680ace13c580390a2107c9ed734029a85e0c11680e441f0795e1e000a21a00c84551661a608000a23a00a88bd10a200288520cd60ac46c8bcbc348480462156228c00a06104b402626f003485208d6068102300332f2f9520482318a355bcbc548a20adc4a87c79690942df0cca8a97eedb05e1d1fb0b150fbfaccbc005380000000049454e44ae426082"}}]"#;
        let stub_nav_known = StubNav::LoadTypes;
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn dock_adventures_1() {
        let dbname = "for_tests/dock_adventures_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033449444154789cedd93d6e1361148561b284d42ee9900b1a3a042276c15e68b38cb4ec85c20e02d1d150587494aeb384708f4637788699ef9cfbfd8c9c64dee2b8b4ee23591a792eeeefef5f3cf76641b8bbbbcbfe92cbcbcb0bfb685a138492a3592d50aa22b43c7e584d8c2a0839c75f7fb9b5ed77f3f1ca36560d8c22849ce3d118809703814a30b2115a007873436421e402a09608280722845072bcd71ac18b60c8083500d05c08488590102200fbe36fdb7e9bd52bdbae28c2f6eddab6dfeec7c1564b81a008a5009e0ac100bc9a10d51052001e8350013c15a2084105405104564d04948298448800a07347405310a3085100f41810d018848cf061bdb5edf7f5b0b3ed8a22ec8f7f6cfb6d562f6dbba208fbe34fdb7e9bd51bdb7e12820ae0a9100cc053211880a74050841480c72054008f41a800de10228930044051045614819583804e211604eb01610c003d5504e410d5116ef747db7e579b956d5714617f7b6ddb6f737563db350b024a4130004f8560009e0a3105804208680c4205f018840ae031881400ea213000a528022b8a901b201684052188b0debeb7ed77d87db3ed8a226cdfad6dfbedbe1f6cbba208ecc9722a19610cc053211880a74230008f41480829008f41a8001e835001bc14441304561481b520580b82759608d79f47fe45fe74f22f7210813d59ce828052100cc053211880a742a400908c80c62054008f41a8001e8360002884c08a22b0a208b92d08d682603d202006f17afbff6feed7eedf6f2e8ac09e2ca308ecdde65800b08feeff0494421803f0540806e0a9100cc04b41c80829008f41a8001e835001bc29886608ac2802ab2a021a8378aa080e801604ab878086105104f664194560ef3673104e01104540290806e0a9100cc053218600882220154205f018840ae031080500c908ac28022b8aa02623a028c463401803409308280271ee085300288980548828027bb75913210580aa21a0140403f0540815001523a0520815c0631035019084802210a9a2082529004846403520e64250015008c12bc1688d1039decb4240b9102d1172005036026a013137002a42f07230c62072004a8ef7aa20783918b9d538deab8ae0b5c4a879bcd7046158094a8ba387cd8270eefd056dddbfaccc6dc61e0000000049454e44ae426082"}}]"##;
        let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	dock-pos-main-runtime31
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."},{"index":3,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":4,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}

        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn dock_adventures_2() {
        let dbname = "for_tests/dock_adventures_2";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033449444154789cedd93d6e1361148561b284d42ee9900b1a3a042276c15e68b38cb4ec85c20e02d1d150587494aeb384708f4637788699ef9cfbfd8c9c64dee2b8b4ee23591a792eeeefef5f3cf76641b8bbbbcbfe92cbcbcb0bfb685a138492a3592d50aa22b43c7e584d8c2a0839c75f7fb9b5ed77f3f1ca36560d8c22849ce3d118809703814a30b2115a007873436421e402a09608280722845072bcd71ac18b60c8083500d05c08488590102200fbe36fdb7e9bd52bdbae28c2f6eddab6dfeec7c1564b81a008a5009e0ac100bc9a10d51052001e8350013c15a2084105405104564d04948298448800a07347405310a3085100f41810d018848cf061bdb5edf7f5b0b3ed8a22ec8f7f6cfb6d562f6dbba208fbe34fdb7e9bd51bdb7e12820ae0a9100cc053211880a74050841480c72054008f41a800de10228930044051045614819583804e211604eb01610c003d5504e410d5116ef747db7e579b956d5714617f7b6ddb6f737563db350b024a4130004f8560009e0a3105804208680c4205f018840ae031881400ea213000a528022b8a901b201684052188b0debeb7ed77d87db3ed8a226cdfad6dfbedbe1f6cbba208ecc9722a19610cc053211880a74230008f41480829008f41a8001e835001bc14441304561481b520580b82759608d79f47fe45fe74f22f7210813d59ce828052100cc053211880a742a400908c80c62054008f41a8001e8360002884c08a22b0a208b92d08d682603d202006f17afbff6feed7eedf6f2e8ac09e2ca308ecdde65800b08feeff0494421803f0540806e0a9100cc04b41c80829008f41a8001e835001bc29886608ac2802ab2a021a8378aa080e801604ab878086105104f664194560ef3673104e01104540290806e0a9100cc053218600882220154205f018840ae031080500c908ac28022b8aa02623a028c463401803409308280271ee085300288980548828027bb75913210580aa21a0140403f0540815001523a0520815c0631035019084802210a9a2082529004846403520e64250015008c12bc1688d1039decb4240b9102d1172005036026a013137002a42f07230c62072004a8ef7aa20783918b9d538deab8ae0b5c4a879bcd7046158094a8ba387cd8270eefd056dddbfaccc6dc61e0000000049454e44ae426082"}}]"##;
        let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	dock-pos-main-runtime31
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
             assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035649444154789cedd8bd6e134114c57152f000692251a54b6f5a1e812a5448aea810af804485c42b202a2a4b54a4e21168719f2e15529a3c008599a3d54d7636e339e7cec7ca49f65f5c173392757fd24a6b1fed76bb674fbd59106e6e6e8abfe4f8f8f8287c74ad0b42cdd2ac1e284d117a2e3fad2546138492e5cf5f9e861977f1e72a4c5f2d30aa104a96472900ab0402d5601423f400b0e68628422805403d115009840ba16679ab3782e5c190115a00a0b910900a21217800ce5f9d861977f1fb2acc212fc2d9f9fdfb971777e72c058222d402582a0403b05a423443c801580c4205b054882a0415007911582d11500e622f8207001d3a02da079144f002a0878080521032027b86bd089bd58730e3d6dbaf610e791136abef61c6adb7efc28c931054004b856000960ac1002c058222e4002c06a102580c4205b0a61059842900f222b0bc08ac12043486581042b7082900f4581190413447482d395e2a753e6d7c7ff5651366dcf6e33acca15910500e820158e3c5d47b29004b85d807805c082805a10258e30553f7c7e739008b41e4005084c00094524b4d1b2fc9f2229406880561417022f037371fc2eb9f7fc38cfbf5e64598435e8493939330e3aeafafc3cc2723a4002c158201582a0403b01884849003b018840a60310815c0ca417441607911580b426841081d24027b86bd089b4fab30e3d69fb7610ecd828072100cc052211880a542e400908c8052102a80c52054008b413000e442607911585e84d21684d08210ba45400c82fd5ef722a4961c2f953a9f16dd7f7ffffefadbdd792a00848fe1ff0494434801582a0403b0a2c5d47b09002b072123e4002c06a10258d18289fbd17906c0da07d10d81955a6ada7849565304948278ac0806801684508480a6105e04f6e6e645b8fcf723ccb8b3e76fc31c2a411803208a8072100cc052211880a5424c011045402a840a60310815c062100a009211585e049617414d46405e8887809002407b119007e2d011f601a02c025221bc08ec196e89900340cd10500e8201582a840a80aa11502d840a60318896004842401e885c5e849a14002423a016107321a800c88560d560f446f02c6f1521a052889e082500a81801f580981b005521582518298812809ae5ad2608560946692d96b79a22583d315a2e6f7541985683d263e969b3201c7aff01ee1bbfac2a71e1050000000049454e44ae426082","encryption":"ed25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035649444154789cedd8bd6e134114c57152f000692251a54b6f5a1e812a5448aea810af804485c42b202a2a4b54a4e21168719f2e15529a3c008599a3d54d7636e339e7cec7ca49f65f5c173392757fd24a6b1fed76bb674fbd59106e6e6e8abfe4f8f8f8287c74ad0b42cdd2ac1e284d117a2e3fad2546138492e5cf5f9e861977f1e72a4c5f2d30aa104a96472900ab0402d5601423f400b0e68628422805403d115009840ba16679ab3782e5c190115a00a0b910900a21217800ce5f9d861977f1fb2acc212fc2d9f9fdfb971777e72c058222d402582a0403b05a423443c801580c4205b054882a0415007911582d11500e622f8207001d3a02da079144f002a0878080521032027b86bd089bd58730e3d6dbaf610e791136abef61c6adb7efc28c931054004b856000960ac1002c058222e4002c06a102580c4205b0a61059842900f222b0bc08ac12043486581042b7082900f4581190413447482d395e2a753e6d7c7ff5651366dcf6e33acca15910500e820158e3c5d47b29004b85d807805c082805a10258e30553f7c7e739008b41e4005084c00094524b4d1b2fc9f2229406880561417022f037371fc2eb9f7fc38cfbf5e64598435e8493939330e3aeafafc3cc2723a4002c158201582a0403b01884849003b018840a60310815c0ca417441607911580b426841081d24027b86bd089b4fab30e3d69fb7610ecd828072100cc052211880a542e400908c8052102a80c52054008b413000e442607911585e84d21684d08210ba45400c82fd5ef722a4961c2f953a9f16dd7f7ffffefadbdd792a00848fe1ff0494434801582a0403b0a2c5d47b09002b072123e4002c06a10258d18289fbd17906c0da07d10d81955a6ada7849565304948278ac0806801684508480a6105e04f6e6e645b8fcf723ccb8b3e76fc31c2a411803208a8072100cc052211880a5424c011045402a840a60310815c062100a009211585e049617414d46405e8887809002407b119007e2d011f601a02c025221bc08ec196e89900340cd10500e8201582a840a80aa11502d840a60318896004842401e885c5e849a14002423a016107321a800c88560d560f446f02c6f1521a052889e082500a81801f580981b005521582518298812809ae5ad2608560946692d96b79a22583d315a2e6f7541985683d263e969b3201c7aff01ee1bbfac2a71e1050000000049454e44ae426082","encryption":"ed25519"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: none."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn can_parse_westend_with_v14() {
        let dbname = "for_tests/can_parse_westend_with_v14";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/load_metadata_westendV9111_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9111","meta_hash":"207956815bc7b3234fa8827ef40df5fd2879e93f18a680e22bc6801bca27312d","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000032d49444154789cedd93d8ed35014c5f1a4729d02690a047b0036011d15a246a9590b75448da8e86013c01e40142351a47615de517427b1f3fcceb9efc3ca0cfe17374522dbf7573dc5ebc3e1b0fadf9b0561bfdf67df64b3d9acc347d39a20942ccd6a815215a1e5f2e36a625441c859befbb20b7358ff7a1ba6af1a18450839cba318809503814a30b2115a005873436421e402a096082807c28550b2bcd51ac1f260c8083500d05c08488590103c00ddefcb25fb27a7a5bc086f6ede8639ecf3eda730b514088a500a60a9100cc0aa09510d210560310815c052218a105400e44560d54440298849040f00ba760434051145f002a0fb8080621032c2bbc3f330877d5cff08f39817e15564c9af674b7911bad5e5fdfbd5e97e9684a002582a0403b054080660291014210560310815c062102a80358648228c01901781e54560e520a07388052174871003400f150119447584eee7e543f5cf4e0fe145b8797cf9fbdb3fa7ef67414029080660a9100cc05221a600900b01c52054008b41a80016834801a001020350f222b0bc08b901624158109c08fdae0b7358b7edc33ce645d87dbfbcdef6c5e97a5e0476b29c4a468801582a0403b054080660310809210560310815c062102a8095826882c0f222b01684d08210ba4a846f4f3f8439ece5aff7611ef322b093e52c082805c1002c158201582a440a00c9082806a102580c4205b0180403402e04961781e545c86d41082d08a13b04c420767fbb30876d1ff5611ef322b093a51781bddb8c0580f071fc3f01a5106200960ac1002c15820158290819210560310815c062102a803505d10c81e545605545403188878a60006841080d10d018c28bc04e965e04f66e3307e11c00510494826000960ac1002c15620c802802522154008b41a80016835000908cc0f222b0bc086a3202f242dc078418009a44401e886b479802404904a4427811e8bbcd8a082900540d01a5201880a542a800a818019542a8001683a809802404e48148e545284901403202aa013117820a805c085609466b04cff2561602ca856889900380b211500b88b90150118295831183c8012859deaa8260e560e4566379ab2a82d512a3e6f25613847125282d961e370bc2b5f70f5df1b6ac6ce24e820000000049454e44ae426082"}}]"#;
        let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_before == expected_print_before, "Received:\n{}", print_before);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
	westend9111
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d9c0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480284d717d5031504025a62029723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let output = produce_output(&line, dbname);
        let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000","units":"uWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"61","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"261"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"10.000000","units":"uWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":11,"indent":0,"type":"tx_version","payload":"7"},{"index":12,"indent":0,"type":"block_hash","payload":"98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"}]"#;
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        if let Action::Sign{content, checksum, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
            sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname).unwrap();
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ffe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let output = produce_output(&line, dbname);
        let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e"}},{"index":5,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082"}},{"index":8,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":9,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":10,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":14,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033549444154789cedd9216e1b4118c5f14696428a02a3a25ca12027280baf422b85f502450545bd4059a4d2a83cac2708e81582aac0a09248563a4fd64bbcce78defb6667564eb27ff019d8dad5f703ab9177effefefecd6b6f1284dbdbdbea9b1c1c1ceca58fae754118b3b4aa074a53849ecb6fd612a30942cdf2fbffced31c76f7f62ccd582d304621d42c8f7200ac06028dc1a846e801c0a686a842a805403d11500d440861ccf2ac37028b60d8082d00d05408c885b01022007fcff7d31cf6eeec2ecd5551843fc7cb3487bdbf5aa4e9e5404884b100cc855000ac25443384120053102e0073214621b800288aa06a89804a105b11220068d711d036882c4214003d07049483b0110e4f4fd31c76737191e6aa28c2a7cc923fd7968a22fccafcfee3daf7cc427001980ba100980ba100980321114a004c41b8004c41b8006c13a288b00980a208aa2882aa0601ad43cc08a907841c007aa9088810cd117e5c1ea539ecf3c9759aaba2081f0e9fdefff7cde3fd264140250805c05c0805c05c886d0028848072102e0053102e0053102500344050004e51045514a13640cc08334210419d04a308e7cba7d73b5b3c5e2f8ab03cfe96e6b0c5d5d734cbd9083900e6422800e6422800a6202c84120053102e0053102e002b4174415045115433426a4648ed24823a094611be1c5da639ecfbf5499aab264140250805c05c0805c05c881200b211500ec205600ac205600a4201a010822a8aa08a22d43623a46684d403025210eadd6114419d04a3088bcc336bb9f6ccca0580f4b1fa3f0195107200cc855000cc855000ac04612394009882700198827001d836886e08aa2882aa2902ca41bc540402a019213540409b105104f5ee308aa04e823508eb004822a0128402602e8402602ec426009208c88570019882700198827000908da08a22a8a2086e36028a423c07841c00da8a802210bb8eb00d001511900b114550ef0e5b22940050330454825000cc857001d068043416c205600aa22500b2105004a25414614c0e00b211500b88a9105c0014426063307a234496675508a816a227420d00aa46403d20a60640a310580d460ea20660ccf2ac0902abc1a8adc5f2ac2902eb89d17279d60561b331283d96de6c12845def3fee5cbfaca5dec1920000000049454e44ae426082"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034749444154789cedd8bf6a145114c771439e20422a6111acac7c013b11d2d82c089217f1197c912008dbd804c4ce17b0b21264c12ae03e4158ef8fe12433933bf7f73bf7cfb049e65b9c2dee85ddf32986658ef6fbfd93c7de2c08bbdd2efb4b4e4e4e8ec247d39a20942ccd6a815215a1e5f2e36a625441c8597ebd7a1de6b0cdf64798be6a601421e42c8f6200560e042ac1c846680160cd0d9185900b805a22a01c081742c9f2566b04cb832123d40040732120154242f0007cfcfc2acc619f3efc0cb3cb8bb07eb10a73d8e6f7364c2d0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6212c103800e1d014d414411bc00e83e20a018848cb07e7bf7476dbeddfe082fc2fbc8fd2fbd732f027bc65812820a60a9100cc052211880a54050841480c52054008b41a800d61822893006405e04961781958380fa100b42e8062106801e2a023288ea086fd677ef7fdfdc9e7b1162f7fbe7b320a0140403b05488fe82eabd14c414007221a018840a603188fe62b1f371fdfb318814001a203000252f02cb8b901b20168405c18970f5e72ccc61a7cf2fc3ecf222fc3dbb0a73d8b3cbd330bbbc08ec9fea5432420cc052211880a54230008b41480829008b41a80016835001ac14441304961781b520841684d04122ac572fc31cb6d9fe0ab3cb8bc09e31b320a0140403b054080660a91029002423a018840a60310815c062100c00b910585e04961721b70521b420846e101083b8fe7711e6b0e3a7e76176791162f7fbe75e04f6be231600c247f73e01a5106200960ad15f50bd97826000560a42464801580ca2bf58ec7c5cff7e0c4205b0a6209a21b0bc08acaa082806f150110c002d08a101021a4378112eaebf8639ecfcf85d985d5e04fa4f3503a10f8028024a4130004b856000960a3106401401a9102a80c52054008b4128004846607911585e043519017921ee03420c004d22200fc4a1234c01a024025221bc08ec195313210580aa21a0140403b0540815001523a0520815c0621035019084803c10a9bc082529004846403520e6425001900bc12ac1688de059deca4240b9102d1172005036026a013137002a42b072306210390025cb5b5510ac1c8cdc6a2c6f5545b05a62d45cde6a8230ae04a5c5d2e3664138f4fe038c73bfacab7b7c0d0000000049454e44ae426082"}},{"index":19,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":20,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000036249444154789cedd8c16d14411085611c82e320073871742e48201f0880830512b9f8c80972200e8760fa695556755353afaabb7a6589f90febc38ebadf7c9225c3cdf3f3f39bffbdab203c3d3d4d5f727b7b7bd37e6c6d0bc2ca4bb376a09422ec7cf9b14a8c128499977fb8fbd23efbee1fbfb6cf5c15184b08332f8f2c00690602ad604c23ec0090ae0d3185300b807622a0198814c2cacb4bbb11a40c4618a102005d0b014521420819804fbf3fb6cfbeefef7eb4cf4b5904761e2b0241115601243ddc836000923e8fc520ca10bcc1921e6e414401247d9ed712421400558e46d5e7791087081900543dbafa3c74046122640150f5e8eaf3240b228c50fd3bfcf0f9ae7df6dd7f7b6c9f97d2e7917d5208210a20e98bbce1dd6003408a4274e705f749230445f02e90f445d6f06eb003203188eebce43ee4228c0068e612af2c026b769f8638115a2f0816009abde4a8d7828004a21cc17a49fd52d6f763ecf9eefbe43e5d18017917e90bacc15237bcfab9e0beb11402b22ed2177883a56eb8f13cfb7eac7b9eecb3ea101840a4ec6856f5794701e24438119208f42fb7e4e80fefdfb6cfbe9fbffeb4cf4bd9f3d8bea3c208d60592bec81bae075b005214429f17dd671542f02e90f445d6703dd8039018843e2fbb6f6c0b022b8bc05add7722b44e84d61604f63b9c4560e765f78d8510907791bec01a2ce9e11e040390f479d17d566104645da42ff0064b7ab805110590f4796cdf512904567634abfabca34e84d689d07a41400c82fd7b3d3bda7a9e7d3fd63d4ff65901a0fdb8fc7f02f210ac0b247d9137bc1b5cfd5c70df5818c1bb40d21759c3bbc1c6f763ecf9eefbe43edd360496f55263fa25592bfbfe414016c4ca2556af054100d089d0ea10d00891bd84fde59645a0e725f7210d802802f22ed2175883a56eb803c100a4eebce03e8922a02884bec01b2c75c30d882880d49d47f64923000a23b0b2a359d5e74961049485a81e5d7d1eb200d02102ca40548fae3eef0800b908280a911dcd7e87b3e7797900a80c0179c3f5600b408a42e8f358cb086815420ff6002406a1cf6331001442401908af2cc24a11001446401510d7428802a01482b482b11b21f3f2d214029a85d889300380a611d00e886b03a025046906c3829801587979a904419ac198ade2e5a55204692746e5cb4b5b10c6565076bcf4d855105e7b7f014ae9bfacbb93aaab0000000049454e44ae426082"}}],"extensions":[{"index":21,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":22,"indent":0,"type":"nonce","payload":"2"},{"index":23,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":24,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":25,"indent":0,"type":"tx_version","payload":"7"},{"index":26,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}]"#;
        let author_info_known = r#""base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed":"Alice","derivation_path":"","has_pwd":false"#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        if let Action::Sign{content, checksum, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
            sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname).unwrap();
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let historic_reply = print_history_entry_by_order_with_decoding(3, dbname).unwrap();
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000","units":"uWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"61","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"261"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"10.000000","units":"uWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":11,"indent":0,"type":"tx_version","payload":"7"},{"index":12,"indent":0,"type":"block_hash","payload":"98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"}]"#;
        assert!(historic_reply.contains(historic_reply_known), "Received different historic reply for order 3: \n{}\n{}", historic_reply, print_history(dbname).unwrap());
        
        let historic_reply = print_history_entry_by_order_with_decoding(4, dbname).unwrap();
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e"}},{"index":5,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082"}},{"index":8,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":9,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":10,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":14,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033549444154789cedd9216e1b4118c5f14696428a02a3a25ca12027280baf422b85f502450545bd4059a4d2a83cac2708e81582aac0a09248563a4fd64bbcce78defb6667564eb27ff019d8dad5f703ab9177effefefecd6b6f1284dbdbdbea9b1c1c1ceca58fae754118b3b4aa074a53849ecb6fd612a30942cdf2fbffced31c76f7f62ccd582d304621d42c8f7200ac06028dc1a846e801c0a686a842a805403d11500d440861ccf2ac37028b60d8082d00d05408c885b01022007fcff7d31cf6eeec2ecd5551843fc7cb3487bdbf5aa4e9e5404884b100cc855000ac25443384120053102e0073214621b800288aa06a89804a105b11220068d711d036882c4214003d07049483b0110e4f4fd31c76737191e6aa28c2a7cc923fd7968a22fccafcfee3daf7cc427001980ba100980ba100980321114a004c41b8004c41b8006c13a288b00980a208aa2882aa0601ad43cc08a907841c007aa9088810cd117e5c1ea539ecf3c9759aaba2081f0e9fdefff7cde3fd264140250805c05c0805c05c886d0028848072102e0053102e0053102500344050004e51045514a13640cc08334210419d04a308e7cba7d73b5b3c5e2f8ab03cfe96e6b0c5d5d734cbd9083900e6422800e6422800a6202c84120053102e0053102e002b4174415045115433426a4648ed24823a094611be1c5da639ecfbf5499aab264140250805c05c0805c05c881200b211500ec205600ac205600a4201a010822a8aa08a22d43623a46684d403025210eadd6114419d04a3088bcc336bb9f6ccca0580f4b1fa3f0195107200cc855000cc855000ac04612394009882700198827001d836886e08aa2882aa2902ca41bc540402a019213540409b105104f5ee308aa04e823508eb004822a0128402602e8402602ec426009208c88570019882700198827000908da08a22a8a2086e36028a423c07841c00da8a802210bb8eb00d001511900b114550ef0e5b22940050330454825000cc857001d068043416c205600aa22500b2105004a25414614c0e00b211500b88a9105c0014426063307a234496675508a816a227420d00aa46403d20a60640a310580d460ea20660ccf2ac0902abc1a8adc5f2ac2902eb89d17279d60561b331283d96de6c12845def3fee5cbfaca5dec1920000000049454e44ae426082"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034749444154789cedd8bf6a145114c771439e20422a6111acac7c013b11d2d82c089217f1197c912008dbd804c4ce17b0b21264c12ae03e4158ef8fe12433933bf7f73bf7cfb049e65b9c2dee85ddf32986658ef6fbfd93c7de2c08bbdd2efb4b4e4e4e8ec247d39a20942ccd6a815215a1e5f2e36a625441c8597ebd7a1de6b0cdf64798be6a601421e42c8f6200560e042ac1c846680160cd0d9185900b805a22a01c081742c9f2566b04cb832123d40040732120154242f0007cfcfc2acc619f3efc0cb3cb8bb07eb10a73d8e6f7364c2d0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6212c103800e1d014d414411bc00e83e20a018848cb07e7bf7476dbeddfe082fc2fbc8fd2fbd732f027bc65812820a60a9100cc052211880a54050841480c52054008b41a800d61822893006405e04961781958380fa100b42e8062106801e2a023288ea086fd677ef7fdfdc9e7b1162f7fbe7b320a0140403b05488fe82eabd14c414007221a018840a603188fe62b1f371fdfb318814001a203000252f02cb8b901b20168405c18970f5e72ccc61a7cf2fc3ecf222fc3dbb0a73d8b3cbd330bbbc08ec9fea5432420cc052211880a54230008b41480829008b41a80016835001ac14441304961781b520841684d04122ac572fc31cb6d9fe0ab3cb8bc09e31b320a0140403b054080660a91029002423a018840a60310815c062100c00b910585e04961721b70521b420846e101083b8fe7711e6b0e3a7e76176791162f7fbe75e04f6be231600c247f73e01a5106200960ad15f50bd97826000560a42464801580ca2bf58ec7c5cff7e0c4205b0a6209a21b0bc08acaa082806f150110c002d08a101021a4378112eaebf8639ecfcf85d985d5e04fa4f3503a10f8028024a4130004b856000960a3106401401a9102a80c52054008b4128004846607911585e043519017921ee03420c004d22200fc4a1234c01a024025221bc08ec195313210580aa21a0140403b0540815001523a0520815c0621035019084803c10a9bc082529004846403520e6425001900bc12ac1688de059deca4240b9102d1172005036026a013137002a42b072306210390025cb5b5510ac1c8cdc6a2c6f5545b05a62d45cde6a8230ae04a5c5d2e3664138f4fe038c73bfacab7b7c0d0000000049454e44ae426082"}},{"index":19,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":20,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000036249444154789cedd8c16d14411085611c82e320073871742e48201f0880830512b9f8c80972200e8760fa695556755353afaabb7a6589f90febc38ebadf7c9225c3cdf3f3f39bffbdab203c3d3d4d5f727b7b7bd37e6c6d0bc2ca4bb376a09422ec7cf9b14a8c128499977fb8fbd23efbee1fbfb6cf5c15184b08332f8f2c00690602ad604c23ec0090ae0d3185300b807622a0198814c2cacb4bbb11a40c4618a102005d0b014521420819804fbf3fb6cfbeefef7eb4cf4b5904761e2b0241115601243ddc836000923e8fc520ca10bcc1921e6e414401247d9ed712421400558e46d5e7791087081900543dbafa3c74046122640150f5e8eaf3240b228c50fd3bfcf0f9ae7df6dd7f7b6c9f97d2e7917d5208210a20e98bbce1dd6003408a4274e705f749230445f02e90f445d6f06eb003203188eebce43ee4228c0068e612af2c026b769f8638115a2f0816009abde4a8d7828004a21cc17a49fd52d6f763ecf9eefbe43e5d18017917e90bacc15237bcfab9e0beb11402b22ed2177883a56eb8f13cfb7eac7b9eecb3ea101840a4ec6856f5794701e24438119208f42fb7e4e80fefdfb6cfbe9fbffeb4cf4bd9f3d8bea3c208d60592bec81bae075b005214429f17dd671542f02e90f445d6703dd8039018843e2fbb6f6c0b022b8bc05add7722b44e84d61604f63b9c4560e765f78d8510907791bec01a2ce9e11e040390f479d17d566104645da42ff0064b7ab805110590f4796cdf512904567634abfabca34e84d689d07a41400c82fd7b3d3bda7a9e7d3fd63d4ff65901a0fdb8fc7f02f210ac0b247d9137bc1b5cfd5c70df5818c1bb40d21759c3bbc1c6f763ecf9eefbe43edd360496f55263fa25592bfbfe414016c4ca2556af054100d089d0ea10d00891bd84fde59645a0e725f7210d802802f22ed2175883a56eb803c100a4eebce03e8922a02884bec01b2c75c30d882880d49d47f64923000a23b0b2a359d5e74961049485a81e5d7d1eb200d02102ca40548fae3eef0800b908280a911dcd7e87b3e7797900a80c0179c3f5600b408a42e8f358cb086815420ff6002406a1cf6331001442401908af2cc24a11001446401510d7428802a01482b482b11b21f3f2d214029a85d889300380a611d00e886b03a025046906c3829801587979a904419ac198ade2e5a55204692746e5cb4b5b10c6565076bcf4d855105e7b7f014ae9bfacbb93aaab0000000049454e44ae426082"}}],"extensions":[{"index":21,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":22,"indent":0,"type":"nonce","payload":"2"},{"index":23,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":24,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":25,"indent":0,"type":"tx_version","payload":"7"},{"index":26,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}]"#;
        assert!(historic_reply.contains(historic_reply_known), "Received different historic reply for order 4: \n{}", historic_reply);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_alice_remarks_westend9122() {
        let dbname = "for_tests/parse_transaction_alice_remarks_westend9122";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/load_metadata_westendV9122_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9122","meta_hash":"d656951f4c58c9fdbe029be33b02a7095abc3007586656be7ff68fd0550d6ced","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034249444154789cedd93152dc40108561730432df81d411659c7923ee616764241042424666df830867c645e4943b38db23acfb95aa419247f35e8f66540bd61ff40633555bfd05aaadd5c16eb77bf7bfb708c276bb2dfe92c3c3c303fb685a1384394bb35aa0544568b9fcb89a1855104a96fff1f38fcd619f3fbdb719ab06c62c8492e5510ac02b814073308a115a00784b4314219402a09608a8042284306779af358217c190116a00a0a510900a21214400be3f9cd91cf6e5e4d666571461f3f1c8e6b0fb5f4f36b514088a3017c0532118805713a21a420ec063102a80a742cc425001501481551301e5202611220068df11d0144412210a805e03024a41c8089ba3639bc3ee9f1e6d7645118e361f6c0e7bbaff6db32b8ac09e319e84a002782a0403f0540806e02910142107e0310815c063102a803786c8228c015014811545609520a03ec48a603d23a400d05b45400e511de1ece1d2e6b0db932b9b5d5184d4fdfef922082807c1003c15a2bfa07a2f0731058042082805a102780ca2bf58ea7c5cff7e0a22078006080c40298ac08a2294068815614508229c5edfd81c7677716eb32b8a7073716a73d8f9f59dcdae2802fba53a958c9002f0540806e0a9100cc0631012420ec063102a80c72054002f07d10481154560ad08d68a60ed25c2e9d77fefdf7d7b398f22b067cc22082807c1003c158201782a440e00c9082805a102780c4205f01804034021045614811545286d45b05604eb19013188b3ab079bc36e2f4f6c76451152f7fbe75104f67f472a00d847f77f02ca21a4003c15a2bfa07a2f07c100bc1c848c9003f018447fb1d4f9b8fefd14840ae04d413443604511585511500ae2ad2238005a11ac01021a43441136c78977878f2fef0ea308ec976a09421f005104948360009e0ac1003c15620c802802522154008f41a8001e835000908cc08a22b0a2086a32028a42bc068414009a444011887d479802405904a4424411d833a626420e00554340390806e0a9102a009a8d80e642a8001e83a8098024041481c81545989302806404540362290415008510bc3918ad1122cb7b4508a814a2254209002a46402d20960640b310bc128c144409c09ce5bd2a085e0946693596f7aa22782d316a2eef3541183707a5c5d2e31641d8f7fe02ad72bfac19ff0cc90000000049454e44ae426082"}}]"#;
        let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519));
        
        if let Action::Stub(reply, checksum, stub_nav) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
            handle_stub(checksum, dbname).unwrap();
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a2509000115094c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e2045022c00a223000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66ae143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let output = produce_output(line, dbname);
        let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"System"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"remark","docs":"4d616b6520736f6d65206f6e2d636861696e2072656d61726b2e0a0a23203c7765696768743e0a2d20604f283129600a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"remark","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e20"}],"extensions":[{"index":4,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"36","period":"64"}},{"index":5,"indent":0,"type":"nonce","payload":"11"},{"index":6,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":7,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9122"}},{"index":8,"indent":0,"type":"tx_version","payload":"7"},{"index":9,"indent":0,"type":"block_hash","payload":"1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a"}]"#;
        let author_info_known = r#""base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed":"Alice","derivation_path":"","has_pwd":false"#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        if let Action::Sign{content, checksum: _, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
        }
        else {panic!("Wrong action: {:?}", output)}

        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn proper_hold_display() {
        let dbname = "for_tests/proper_hold_display";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum, _) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend, westend-ed25519; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082"}}]"#;
        let stub_nav_known = StubNav::LoadTypes;
        
        if let Action::Stub(reply, _, stub_nav) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
            assert!(stub_nav == stub_nav_known, "Expected: {:?}\nReceived: {:?}", stub_nav_known, stub_nav);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn delete_westend_try_load_metadata() {
        let dbname = "for_tests/delete_westend_try_load_metadata";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        remove_network(&NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), dbname).unwrap();
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        
        let line = fs::read_to_string("for_tests/load_metadata_westendV9122_Alice-sr25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network westend was previously known to the database with verifier public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata."}]"#;
        
        if let Action::Read(reply) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }
        
    #[test]
    fn dock_adventures_3() {
        let dbname = "for_tests/dock_adventures_3";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum, _) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum, _) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	dock-pos-main-runtime34
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035649444154789cedd8bd6e134114c57152f000692251a54b6f5a1e812a5448aea810af804485c42b202a2a4b54a4e21168719f2e15529a3c008599a3d54d7636e339e7cec7ca49f65f5c173392757fd24a6b1fed76bb674fbd59106e6e6e8abfe4f8f8f8287c74ad0b42cdd2ac1e284d117a2e3fad2546138492e5cf5f9e861977f1e72a4c5f2d30aa104a96472900ab0402d5601423f400b0e68628422805403d115009840ba16679ab3782e5c190115a00a0b910900a21217800ce5f9d861977f1fb2acc212fc2d9f9fdfb971777e72c058222d402582a0403b05a423443c801580c4205b054882a0415007911582d11500e622f8207001d3a02da079144f002a0878080521032027b86bd089bd58730e3d6dbaf610e791136abef61c6adb7efc28c931054004b856000960ac1002c058222e4002c06a102580c4205b0a61059842900f222b0bc08ac12043486581042b7082900f4581190413447482d395e2a753e6d7c7ff5651366dcf6e33acca15910500e820158e3c5d47b29004b85d807805c082805a10258e30553f7c7e739008b41e4005084c00094524b4d1b2fc9f2229406880561417022f037371fc2eb9f7fc38cfbf5e64598435e8493939330e3aeafafc3cc2723a4002c158201582a0403b01884849003b018840a60310815c0ca417441607911580b426841081d24027b86bd089b4fab30e3d69fb7610ecd828072100cc052211880a542e400908c8052102a80c52054008b413000e442607911585e84d21684d08210ba45400c82fd5ef722a4961c2f953a9f16dd7f7ffffefadbdd792a00848fe1ff0494434801582a0403b0a2c5d47b09002b072123e4002c06a10258d18289fbd17906c0da07d10d81955a6ada7849565304948278ac0806801684508480a6105e04f6e6e645b8fcf723ccb8b3e76fc31c2a411803208a8072100cc052211880a5424c011045402a840a60310815c062100a009211585e049617414d46405e8887809002407b119007e2d011f601a02c025221bc08ec196e89900340cd10500e8201582a840a80aa11502d840a60318896004842401e885c5e849a14002423a016107321a800c88560d560f446f02c6f1521a052889e082500a81801f580981b005521582518298812809ae5ad2608560946692d96b79a22583d315a2e6f7541985683d263e969b3201c7aff01ee1bbfac2a71e1050000000049454e44ae426082","encryption":"ed25519"}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        
        remove_network(&NetworkSpecsKey::from_parts(&hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(), &Encryption::Sr25519), dbname).unwrap();
        
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	westend9000
	westend9010
Network Specs:
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: network inactivated
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is disabled. It could be enabled again only after complete wipe and re-installation of Signer."}]"#;
        
        if let Action::Read(reply) = output {assert!(reply==reply_known, "\nReceived: {}", reply);}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is disabled. It could be enabled again only after complete wipe and re-installation of Signer."}]"#;
        
        if let Action::Read(reply) = output {assert!(reply==reply_known, "\nReceived: {}", reply);}
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn acala_adventures() {
        let dbname = "for_tests/acala_adventures";
        populate_cold_no_networks(dbname, Verifier(None)).unwrap();
        
        let line = fs::read_to_string("for_tests/add_specs_acala-sr25519_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum, _) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
	0180fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c: Acala (acala with sr25519)
Verifiers:
	fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c: "type":"custom","details":{"hex":"","identicon":"","encryption":"none"}
General Verifier: none
Identities: "#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/load_metadata_acalaV2012_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum, _) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = "530102dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce97359a80a0000dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce973590b00407a10f35a24010000dc07000001000000fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c";
        let output = produce_output(&line, dbname);
        let content_known = r#""author":[{"index":0,"indent":0,"type":"author_plain","payload":{"base58":"25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035149444154789cedd9ad6e146114c6719a54e1d6a0b806eea2018345a0707049e050082c06d2bbe01a509875a826cbfb64f3b46766cf9c8ff763324de72f66c54ede73e6279ac9f6ea743a3d7beaad82703c1eab871c0e87abf231b421082d0fed3502a52bc2c8879fd713a30b42cdc3dfbe7b59aed36ebeff29d75c3d309a106a1e1e6900ac0602b56054238c00606b435421d402a09108a8062285d0f2f06c3402cb6084117a00a0b51050142284900178f1e67db94efbfbf35bb99ecb22fc78fea55ca7bdfdf7b15c6345205c8456001685f000584f886e081600f320a2002c0ad1841005405904af9e08c8825844c800a0ad23a0250815210b801e0302d220c208d7bf5e95ebb4bbd7bfcbf55c16e1c3f5e5795fef1ececb2278fbb110421480c941168407c0a2101e0093fbb139848b600d607290061105601e441480c9fd90893007403543acb2085eb5fb49881da1748fa001a0da214b6d050111a23b8276bffc3e8be09da77d3f4fde2f0b23206b901c10bdcf82f000983c2f7adfbc1402d206c901daf7f3e4fd1a441480c9f3b4fbe5f75a13040f2092b6c43c6f2959eff39602c48eb02324117abfb97dbebdfc3de1d3cdc3ef09d9f3bc37d5a5c2081a008b42c8853500168590e76900cc8308215800cc83900b5b00cc8390e75900cc821882e09545f0da114a3b42699308de9b6016c13b6f1504644178004c2e6e4178004c9e67415800288c8034882800938b6b10510026cfd3203c009442f0ca2eedd5fbbca57684d28e50ba47401e84f7bfc3ecd2de9b60f63c6f3f2d00948ff3ef09c842d0063039c85a5c2eac01b028843c2fbadfbc30823580c941dae272610b807910f2bcec7eb261085e5904af96fd2e109006d132446b2b0804403b42698280e610d921de9b5b16c17b53cdee87240072119035480ed0005814c203605108b91f73115014420eb00098071105601e84dc8fcd015018c12b8be09545881646405988c780a001a045049481d83ac2120032115014228bd0fb6f82950580ba21200bc20360518828006a4640ad105100e641f4044021049481b0ca22b41401406104d403622d8428004a21b0168cd1089987675508a8166224420d00aa46402320d606404d08ac064383a801687978d60581d560d4d6e3e15957043612a3e7c3b32108f35a50463cf4bc5510b6de7fd01ebfac0eac30e50000000049454e44ae426082"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Transaction author public key not found."}],"method":[{"index":2,"indent":0,"type":"pallet","payload":"Balances"},{"index":3,"indent":1,"type":"method","payload":{"method_name":"transfer","docs":"5472616e7366657220736f6d65206c697175696420667265652062616c616e636520746f20616e6f74686572206163636f756e742e0a0a607472616e73666572602077696c6c207365742074686520604672656542616c616e636560206f66207468652073656e64657220616e642072656365697665722e0a49742077696c6c2064656372656173652074686520746f74616c2069737375616e6365206f66207468652073797374656d2062792074686520605472616e73666572466565602e0a4966207468652073656e6465722773206163636f756e742069732062656c6f7720746865206578697374656e7469616c206465706f736974206173206120726573756c740a6f6620746865207472616e736665722c20746865206163636f756e742077696c6c206265207265617065642e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d75737420626520605369676e65646020627920746865207472616e736163746f722e0a0a23203c7765696768743e0a2d20446570656e64656e74206f6e20617267756d656e747320627574206e6f7420637269746963616c2c20676976656e2070726f70657220696d706c656d656e746174696f6e7320666f7220696e70757420636f6e6669670a202074797065732e205365652072656c617465642066756e6374696f6e732062656c6f772e0a2d20497420636f6e7461696e732061206c696d69746564206e756d626572206f6620726561647320616e642077726974657320696e7465726e616c6c7920616e64206e6f20636f6d706c65780a2020636f6d7075746174696f6e2e0a0a52656c617465642066756e6374696f6e733a0a0a20202d2060656e737572655f63616e5f77697468647261776020697320616c776179732063616c6c656420696e7465726e616c6c792062757420686173206120626f756e64656420636f6d706c65786974792e0a20202d205472616e7366657272696e672062616c616e63657320746f206163636f756e7473207468617420646964206e6f74206578697374206265666f72652077696c6c2063617573650a2020202060543a3a4f6e4e65774163636f756e743a3a6f6e5f6e65775f6163636f756e746020746f2062652063616c6c65642e0a20202d2052656d6f76696e6720656e6f7567682066756e64732066726f6d20616e206163636f756e742077696c6c20747269676765722060543a3a4475737452656d6f76616c3a3a6f6e5f756e62616c616e636564602e0a20202d20607472616e736665725f6b6565705f616c6976656020776f726b73207468652073616d652077617920617320607472616e73666572602c206275742068617320616e206164646974696f6e616c20636865636b0a202020207468617420746865207472616e736665722077696c6c206e6f74206b696c6c20746865206f726967696e206163636f756e742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a2d204f726967696e206163636f756e7420697320616c726561647920696e206d656d6f72792c20736f206e6f204442206f7065726174696f6e7320666f72207468656d2e0a23203c2f7765696768743e"}},{"index":4,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":5,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":6,"indent":4,"type":"Id","payload":{"base58":"25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035149444154789cedd9ad6e146114c6719a54e1d6a0b806eea2018345a0707049e050082c06d2bbe01a509875a826cbfb64f3b46766cf9c8ff763324de72f66c54ede73e6279ac9f6ea743a3d7beaad82703c1eab871c0e87abf231b421082d0fed3502a52bc2c8879fd713a30b42cdc3dfbe7b59aed36ebeff29d75c3d309a106a1e1e6900ac0602b56054238c00606b435421d402a09108a8062285d0f2f06c3402cb6084117a00a0b51050142284900178f1e67db94efbfbf35bb99ecb22fc78fea55ca7bdfdf7b15c6345205c8456001685f000584f886e081600f320a2002c0ad1841005405904af9e08c8825844c800a0ad23a0250815210b801e0302d220c208d7bf5e95ebb4bbd7bfcbf55c16e1c3f5e5795fef1ececb2278fbb110421480c941168407c0a2101e0093fbb139848b600d607290061105601e441480c9fd90893007403543acb2085eb5fb49881da1748fa001a0da214b6d050111a23b8276bffc3e8be09da77d3f4fde2f0b23206b901c10bdcf82f000983c2f7adfbc1402d206c901daf7f3e4fd1a441480c9f3b4fbe5f75a13040f2092b6c43c6f2959eff39602c48eb02324117abfb97dbebdfc3de1d3cdc3ef09d9f3bc37d5a5c2081a008b42c8853500168590e76900cc8308215800cc83900b5b00cc8390e75900cc821882e09545f0da114a3b42699308de9b6016c13b6f1504644178004c2e6e4178004c9e67415800288c8034882800938b6b10510026cfd3203c009442f0ca2eedd5fbbca57684d28e50ba47401e84f7bfc3ecd2de9b60f63c6f3f2d00948ff3ef09c842d0063039c85a5c2eac01b028843c2fbadfbc30823580c941dae272610b807910f2bcec7eb261085e5904af96fd2e109006d132446b2b0804403b42698280e610d921de9b5b16c17b53cdee87240072119035480ed0005814c203605108b91f73115014420eb00098071105601e84dc8fcd015018c12b8be09545881646405988c780a001a045049481d83ac2120032115014228bd0fb6f82950580ba21200bc20360518828006a4640ad105100e641f4044021049481b0ca22b41401406104d403622d8428004a21b0168cd1089987675508a8166224420d00aa46402320d606404d08ac064383a801687978d60581d560d4d6e3e15957043612a3e7c3b32108f35a50463cf4bc5510b6de7fd01ebfac0eac30e50000000049454e44ae426082"}},{"index":7,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":8,"indent":3,"type":"balance","payload":{"amount":"100.000000000000","units":"ACA"}}],"extensions":[{"index":9,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"18","period":"32"}},{"index":10,"indent":0,"type":"nonce","payload":"0"},{"index":11,"indent":0,"type":"tip","payload":{"amount":"0","units":"pACA"}},{"index":12,"indent":0,"type":"name_version","payload":{"name":"acala","version":"2012"}},{"index":13,"indent":0,"type":"tx_version","payload":"1"},{"index":14,"indent":0,"type":"block_hash","payload":"5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620"}]"#;
        
        if let Action::Read(content) = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }

}
