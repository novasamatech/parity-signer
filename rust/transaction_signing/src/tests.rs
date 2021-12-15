#[cfg(test)]
mod tests {
    use hex;
    use transaction_parsing::{Action, produce_output, produce_historic_output};
    use crate::{handle_stub, sign_transaction::create_signature};
    use db_handling::{cold_default::{populate_cold, populate_cold_no_networks}, identities::try_create_seed_phrase_proposal, manage_history::print_history, remove_network::remove_network_by_hex};
    use definitions::{error::{AddressKeySource, DatabaseSigner, ErrorSigner, ErrorSource, Signer}, keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, network_specs::{CurrentVerifier, NetworkSpecs, Verifier, VerifierValue}, users::AddressDetails};
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
        
        let mut network_specs_set: Vec<String> = Vec::new();
        let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
        for x in chainspecs.iter() {
            if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
                let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
                let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, network_specs_encoded).unwrap();
                network_specs_set.push(format!("{}: {} ({} with {})", hex::encode(network_specs_key.key()), network_specs.title, network_specs.name, network_specs.encryption.show()));
            }
        }
        network_specs_set.sort();
        let mut network_specs_str = String::new();
        for x in network_specs_set.iter() {network_specs_str.push_str(&format!("\n\t{}", x))}
        
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
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice""#;
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
            let my_event = r#""events":[{"event":"transaction_signed","payload":{"transaction":"a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"},"user_comment":""}}]"#;
            assert!(history_recorded.contains(my_event), "Recorded history is different: \n{}", history_recorded);
            
            let result = sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname);
            if let Err(e) = result {
                let expected_err = ErrorSigner::Database(DatabaseSigner::ChecksumMismatch);
                if <Signer>::show(&e) != <Signer>::show(&expected_err) {panic!("Expected wrong checksum. Got error: {:?}.", e)}
            }
            else {panic!("Checksum should have changed.")}
                
            let historic_reply = produce_historic_output(2, dbname);
            let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
            assert!(historic_reply == historic_reply_known, "Received different historic reply: \n{}", historic_reply);
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
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice""#;
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
            let my_event = r#""events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"},"user_comment":""}}]"#;
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
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
            
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
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"custom","details":{"hex":"","encryption":"none"}}
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
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error in parsing. Received: \n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
Network Specs:
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
            try_create_seed_phrase_proposal("Alice", SEED_PHRASE, dbname).unwrap();
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef, encryption: ed25519, path: , available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error in parsing. Received: \n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
	westend9070
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Rococo, Westend; affected metadata entries: kusama2030, polkadot30, rococo9103, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}]"#;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}]"#;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	dock-pos-main-runtime31
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Rococo, Westend; affected metadata entries: kusama2030, polkadot30, rococo9103, westend9000, westend9010. Types information is purged."},{"index":3,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":4,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	dock-pos-main-runtime31
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
             assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
            assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: none."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            handle_stub(checksum, dbname).unwrap();
            
            let print_after = print_db_content(&dbname);
            let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9111","meta_hash":"207956815bc7b3234fa8827ef40df5fd2879e93f18a680e22bc6801bca27312d"}}]"#;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            
            let print_before = print_db_content(&dbname);
            let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
	rococo9103
	westend9000
	westend9010
	westend9111
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice""#;
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
        let author_info_known = r#""base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed":"Alice","derivation_path":"""#;
        let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
        
        if let Action::Sign{content, checksum, has_pwd, author_info, network_info} = output {
            assert!(content == content_known, "Expected: {}\nReceived: {}", content_known, content);
            assert!(author_info == author_info_known, "Expected: {}\nReceived: {}", author_info_known, author_info);
            assert!(network_info == network_info_known, "Expected: {}\nReceived: {}", network_info_known, network_info);
            assert!(!has_pwd, "Expected no password");
            sign_action_test(checksum, SEED_PHRASE, PWD, USER_COMMENT, dbname).unwrap();
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let historic_reply = produce_historic_output(3, dbname);
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000","units":"uWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"61","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"261"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"10.000000","units":"uWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":11,"indent":0,"type":"tx_version","payload":"7"},{"index":12,"indent":0,"type":"block_hash","payload":"98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"}]"#;
        assert!(historic_reply == historic_reply_known, "Received different historic reply for order 3: \n{}\n{}", historic_reply, print_history(dbname).unwrap());
        
        let historic_reply = produce_historic_output(4, dbname);
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e"}},{"index":5,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082"}},{"index":8,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":9,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":10,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":14,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033549444154789cedd9216e1b4118c5f14696428a02a3a25ca12027280baf422b85f502450545bd4059a4d2a83cac2708e81582aac0a09248563a4fd64bbcce78defb6667564eb27ff019d8dad5f703ab9177effefefecd6b6f1284dbdbdbea9b1c1c1ceca58fae754118b3b4aa074a53849ecb6fd612a30942cdf2fbffced31c76f7f62ccd582d304621d42c8f7200ac06028dc1a846e801c0a686a842a805403d11500d440861ccf2ac37028b60d8082d00d05408c885b01022007fcff7d31cf6eeec2ecd5551843fc7cb3487bdbf5aa4e9e5404884b100cc855000ac25443384120053102e0073214621b800288aa06a89804a105b11220068d711d036882c4214003d07049483b0110e4f4fd31c76737191e6aa28c2a7cc923fd7968a22fccafcfee3daf7cc427001980ba100980ba100980321114a004c41b8004c41b8006c13a288b00980a208aa2882aa0601ad43cc08a907841c007aa9088810cd117e5c1ea539ecf3c9759aaba2081f0e9fdefff7cde3fd264140250805c05c0805c05c886d0028848072102e0053102e0053102500344050004e51045514a13640cc08334210419d04a308e7cba7d73b5b3c5e2f8ab03cfe96e6b0c5d5d734cbd9083900e6422800e6422800a6202c84120053102e0053102e002b4174415045115433426a4648ed24823a094611be1c5da639ecfbf5499aab264140250805c05c0805c05c881200b211500ec205600ac205600a4201a010822a8aa08a22d43623a46684d403025210eadd6114419d04a3088bcc336bb9f6ccca0580f4b1fa3f0195107200cc855000cc855000ac04612394009882700198827001d836886e08aa2882aa2902ca41bc540402a019213540409b105104f5ee308aa04e823508eb004822a0128402602e8402602ec426009208c88570019882700198827000908da08a22a8a2086e36028a423c07841c00da8a802210bb8eb00d001511900b114550ef0e5b22940050330454825000cc857001d068043416c205600aa22500b2105004a25414614c0e00b211500b88a9105c0014426063307a234496675508a816a227420d00aa46403d20a60640a310580d460ea20660ccf2ac0902abc1a8adc5f2ac2902eb89d17279d60561b331283d96de6c12845def3fee5cbfaca5dec1920000000049454e44ae426082"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034749444154789cedd8bf6a145114c771439e20422a6111acac7c013b11d2d82c089217f1197c912008dbd804c4ce17b0b21264c12ae03e4158ef8fe12433933bf7f73bf7cfb049e65b9c2dee85ddf32986658ef6fbfd93c7de2c08bbdd2efb4b4e4e4e8ec247d39a20942ccd6a815215a1e5f2e36a625441c8597ebd7a1de6b0cdf64798be6a601421e42c8f6200560e042ac1c846680160cd0d9185900b805a22a01c081742c9f2566b04cb832123d40040732120154242f0007cfcfc2acc619f3efc0cb3cb8bb07eb10a73d8e6f7364c2d0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6212c103800e1d014d414411bc00e83e20a018848cb07e7bf7476dbeddfe082fc2fbc8fd2fbd732f027bc65812820a60a9100cc052211880a54050841480c52054008b41a800d61822893006405e04961781958380fa100b42e8062106801e2a023288ea086fd677ef7fdfdc9e7b1162f7fbe7b320a0140403b05488fe82eabd14c414007221a018840a603188fe62b1f371fdfb318814001a203000252f02cb8b901b20168405c18970f5e72ccc61a7cf2fc3ecf222fc3dbb0a73d8b3cbd330bbbc08ec9fea5432420cc052211880a54230008b41480829008b41a80016835001ac14441304961781b520841684d04122ac572fc31cb6d9fe0ab3cb8bc09e31b320a0140403b054080660a91029002423a018840a60310815c062100c00b910585e04961721b70521b420846e101083b8fe7711e6b0e3a7e76176791162f7fbe75e04f6be231600c247f73e01a5106200960ad15f50bd97826000560a42464801580ca2bf58ec7c5cff7e0c4205b0a6209a21b0bc08acaa082806f150110c002d08a101021a4378112eaebf8639ecfcf85d985d5e04fa4f3503a10f8028024a4130004b856000960a3106401401a9102a80c52054008b4128004846607911585e043519017921ee03420c004d22200fc4a1234c01a024025221bc08ec195313210580aa21a0140403b0540815001523a0520815c0621035019084803c10a9bc082529004846403520e6425001900bc12ac1688de059deca4240b9102d1172005036026a013137002a42b072306210390025cb5b5510ac1c8cdc6a2c6f5545b05a62d45cde6a8230ae04a5c5d2e3664138f4fe038c73bfacab7b7c0d0000000049454e44ae426082"}},{"index":19,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":20,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000036249444154789cedd8c16d14411085611c82e320073871742e48201f0880830512b9f8c80972200e8760fa695556755353afaabb7a6589f90febc38ebadf7c9225c3cdf3f3f39bffbdab203c3d3d4d5f727b7b7bd37e6c6d0bc2ca4bb376a09422ec7cf9b14a8c128499977fb8fbd23efbee1fbfb6cf5c15184b08332f8f2c00690602ad604c23ec0090ae0d3185300b807622a0198814c2cacb4bbb11a40c4618a102005d0b014521420819804fbf3fb6cfbeefef7eb4cf4b5904761e2b0241115601243ddc836000923e8fc520ca10bcc1921e6e414401247d9ed712421400558e46d5e7791087081900543dbafa3c74046122640150f5e8eaf3240b228c50fd3bfcf0f9ae7df6dd7f7b6c9f97d2e7917d5208210a20e98bbce1dd6003408a4274e705f749230445f02e90f445d6f06eb003203188eebce43ee4228c0068e612af2c026b769f8638115a2f0816009abde4a8d7828004a21cc17a49fd52d6f763ecf9eefbe43e5d18017917e90bacc15237bcfab9e0beb11402b22ed2177883a56eb8f13cfb7eac7b9eecb3ea101840a4ec6856f5794701e24438119208f42fb7e4e80fefdfb6cfbe9fbffeb4cf4bd9f3d8bea3c208d60592bec81bae075b005214429f17dd671542f02e90f445d6703dd8039018843e2fbb6f6c0b022b8bc05add7722b44e84d61604f63b9c4560e765f78d8510907791bec01a2ce9e11e040390f479d17d566104645da42ff0064b7ab805110590f4796cdf512904567634abfabca34e84d689d07a41400c82fd7b3d3bda7a9e7d3fd63d4ff65901a0fdb8fc7f02f210ac0b247d9137bc1b5cfd5c70df5818c1bb40d21759c3bbc1c6f763ecf9eefbe43edd360496f55263fa25592bfbfe414016c4ca2556af054100d089d0ea10d00891bd84fde59645a0e725f7210d802802f22ed2175883a56eb803c100a4eebce03e8922a02884bec01b2c75c30d882880d49d47f64923000a23b0b2a359d5e74961049485a81e5d7d1eb200d02102ca40548fae3eef0800b908280a911dcd7e87b3e7797900a80c0179c3f5600b408a42e8f358cb086815420ff6002406a1cf6331001442401908af2cc24a11001446401510d7428802a01482b482b11b21f3f2d214029a85d889300380a611d00e886b03a025046906c3829801587979a904419ac198ade2e5a55204692746e5cb4b5b10c6565076bcf4d855105e7b7f014ae9bfacbb93aaab0000000049454e44ae426082"}}],"extensions":[{"index":21,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":22,"indent":0,"type":"nonce","payload":"2"},{"index":23,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":24,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":25,"indent":0,"type":"tx_version","payload":"7"},{"index":26,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}]"#;
        assert!(historic_reply == historic_reply_known, "Received different historic reply for order 4: \n{}", historic_reply);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_alice_remarks_westend9122() {
        let dbname = "for_tests/parse_transaction_alice_remarks_westend9122";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/load_metadata_westendV9122_unverified.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9122","meta_hash":"d656951f4c58c9fdbe029be33b02a7095abc3007586656be7ff68fd0550d6ced"}}]"#;
        
        if let Action::Stub(reply, checksum) = output {
            assert!(reply == reply_known, "Error on parsing. Received:\n{}", reply);
            handle_stub(checksum, dbname).unwrap();
        }
        else {panic!("Wrong action: {:?}", output)}
        
        let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a2509000115094c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e2045022c00a223000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66ae143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let output = produce_output(line, dbname);
        let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"System"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"remark","docs":"4d616b6520736f6d65206f6e2d636861696e2072656d61726b2e0a0a23203c7765696768743e0a2d20604f283129600a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"remark","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e20"}],"extensions":[{"index":4,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"36","period":"64"}},{"index":5,"indent":0,"type":"nonce","payload":"11"},{"index":6,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":7,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9122"}},{"index":8,"indent":0,"type":"tx_version","payload":"7"},{"index":9,"indent":0,"type":"block_hash","payload":"1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a"}]"#;
        let author_info_known = r#""base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed":"Alice","derivation_path":"""#;
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
        
        if let Action::Stub(_, checksum) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Rococo, Westend, westend-ed25519; affected metadata entries: kusama2030, polkadot30, rococo9103, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}]"#;
        
        if let Action::Stub(reply, _) = output {
            assert!(reply == reply_known, "Received: \n{}", reply);
        }
        else {panic!("Wrong action: {:?}", output)}
        
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn delete_westend_try_load_metadata() {
        let dbname = "for_tests/delete_westend_try_load_metadata";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        remove_network_by_hex("0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", dbname).unwrap();
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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
        
        if let Action::Stub(_, checksum) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt").unwrap();
        let output = produce_output(&line.trim(), dbname);
        
        if let Action::Stub(_, checksum) = output {handle_stub(checksum, dbname).unwrap();}
        else {panic!("Wrong action: {:?}", output)}
        
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	dock-pos-main-runtime34
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        
        remove_network_by_hex("01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae", dbname).unwrap();
        
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	polkadot30
	rococo9103
	westend9000
	westend9010
Network Specs:
	0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: Rococo (rococo with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: network inactivated
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770
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

}
