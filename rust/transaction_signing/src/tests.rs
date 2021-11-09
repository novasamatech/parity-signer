#[cfg(test)]
mod tests {
    use hex;
    use transaction_parsing::produce_output;
    use crate::{checksum, handle_stub, sign_transaction::create_signature};
    use db_handling::{cold_default::{populate_cold, populate_cold_no_networks}, identities::try_create_seed, manage_history::print_history};
    use definitions::{keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, network_specs::{CurrentVerifier, ChainSpecs, Verifier, VerifierValue}, users::AddressDetails};
    use constants::{ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, VERIFIERS};
    use parity_scale_codec::Decode;
    use std::fs;
    use sled::{Db, open, Tree};
    use sp_core;
    use sp_runtime::MultiSigner;
    use regex::Regex;
    use lazy_static::lazy_static;
    
    const SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    const PWD: &str = "jaskier";
    const USER_COMMENT: &str = "";
    const ALICE: [u8; 32] = [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
    fn verifier_alice_sr25519() -> Verifier {
        Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(ALICE)))))
    }
    
    lazy_static! {
        static ref ACTION: Regex = Regex::new(r#"(?i)"action":\{"type":"(?P<type>sign|stub)","payload":"(?P<payload>[0-9]+)"\}"#).expect("constructed from checked static value");
    }
    
    fn get_type_and_checksum(reply: &str) -> (bool, String) {
        let caps = ACTION.captures(&reply).unwrap();
        let action_is_sign = match caps.name("type").unwrap().as_str() {
            "sign" => true,
            "stub" => false,
            _ => unreachable!(),
        };
        let checksum = caps.name("payload").unwrap().as_str().to_string();
        (action_is_sign, checksum)
    }
    
    fn sign_action_test (action_is_sign: bool, checksum_str: &str, seed_phrase: &str, pwd_entry: &str, user_comment: &str, dbname: &str) -> anyhow::Result<String> {
        if action_is_sign {
            let checksum = checksum(checksum_str)?;
            create_signature(seed_phrase, pwd_entry, user_comment, dbname, checksum)
        }
        else {panic!("Should have been action `sign`.")}
    }
    
    fn print_db_content (dbname: &str) -> String {
        let database: Db = open(dbname).unwrap();
        let mut metadata_str = String::new();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        for x in metadata.iter() {
            if let Ok((meta_key_vec, _)) = x {
                let meta_key = MetaKey::from_vec(&meta_key_vec.to_vec());
                let (name, version) = meta_key.name_version().unwrap();
                metadata_str.push_str(&format!("\n\t{}{}", name, version));
            }
        }
        let mut network_specs_str = String::new();
        let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
        for x in chainspecs.iter() {
            if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
                let network_specs = ChainSpecs::decode(&mut &network_specs_encoded[..]).unwrap();
                let network_specs_key = NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec());
                network_specs_str.push_str(&format!("\n\t{}: {} ({} with {})", hex::encode(network_specs_key.key()), network_specs.title, network_specs.name, network_specs.encryption.show()));
            }
        }
        let settings: Tree = database.open_tree(SETTREE).unwrap();
        let general_verifier_encoded = settings.get(&GENERALVERIFIER).unwrap().unwrap();
        let general_verifier = Verifier::decode(&mut &general_verifier_encoded[..]).unwrap();
        
        let mut verifiers_str = String::new();
        let verifiers: Tree = database.open_tree(VERIFIERS).unwrap();
        for x in verifiers.iter() {
            if let Ok((verifier_key_vec, current_verifier_encoded)) = x {
                let verifier_key = VerifierKey::from_vec(&verifier_key_vec.to_vec());
                let current_verifier = CurrentVerifier::decode(&mut &current_verifier_encoded[..]).unwrap();
                verifiers_str.push_str(&format!("\n\t{}: {}", hex::encode(verifier_key.key()), current_verifier.show(&general_verifier)));
            }
        }
        
        let mut identities_str = String::new();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        for x in identities.iter() {
            if let Ok((address_key_vec, address_details_encoded)) = x {
                let address_key = AddressKey::from_vec(&address_key_vec.to_vec());
                let address_details = AddressDetails::decode(&mut &address_details_encoded[..]).unwrap();
                let (public_key, encryption) = address_key.public_key_encryption().unwrap();
                let mut networks_str = String::new();
                for y in address_details.network_id.iter() {
                    networks_str.push_str(&format!("\n\t\t{}", hex::encode(y.key())))
                }
                identities_str.push_str(&format!("\n\tpublic_key: {}, encryption: {}, path: {}, available_networks: {}", hex::encode(public_key), encryption.show(), address_details.path, networks_str));
            }
        }
        
        format!("Database contents:\nMetadata:{}\nNetwork Specs:{}\nVerifiers:{}\nGeneral Verifier: {}\nIdentities: {}", metadata_str, network_specs_str, verifiers_str, general_verifier.show_error(), identities_str)
    }
    
// can sign a parsed transaction
    #[test]
    fn can_sign_transaction_1() {
        let dbname = "for_tests/can_sign_transaction_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Balances"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":3,"indent":2,"type":"varname","payload":"dest"},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":4,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":2,"type":"varname","payload":"value"},{"index":7,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":8,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":9,"indent":0,"type":"nonce","payload":"46"},{"index":10,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":11,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":12,"indent":0,"type":"tx_version","payload":"5"},{"index":13,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        let result = sign_action_test(action_is_sign, &checksum_str, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        match result {
            Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
            Err(e) => panic!("Was unable to sign. {}", e),
        }
        let history_recorded = print_history(&dbname).unwrap();
        let my_event = r#""events":[{"event":"transaction_signed","payload":{"transaction":"a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"},"user_comment":""}}]"#;
        assert!(history_recorded.contains(my_event), "Recorded history is different: \n{}", history_recorded);
        
        let result = sign_action_test(action_is_sign, &checksum_str, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("Database checksum mismatch.");
            if err != expected_err {panic!("Expected wrong checksum. Got error: {}.", err)}
        }
        else {panic!("Checksum should have changed.")}
        fs::remove_dir_all(dbname).unwrap();
    }

// can sign a message
    #[test]
    fn can_sign_message_1() {
        let dbname = "for_tests/can_sign_message_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27df5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"message":[{"index":1,"indent":0,"type":"message","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"},{"index":2,"indent":0,"type":"network_name","payload":"westend"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        let result = sign_action_test(action_is_sign, &checksum_str, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        match result {
            Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
            Err(e) => panic!("Was unable to sign. {}", e),
        }
        let history_recorded = print_history(&dbname).unwrap();
        let my_event = r#""events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"},"user_comment":""}}]"#;
        assert!(history_recorded.contains(my_event), "Recorded history is different: \n{}", history_recorded);
        
        let result = sign_action_test(action_is_sign, &checksum_str, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("Database checksum mismatch.");
            if err != expected_err {panic!("Expected wrong checksum. Got error: {}.", err)}
        }
        else {panic!("Checksum should have changed.")}
        fs::remove_dir_all(dbname).unwrap();
    }
  
    #[test]
    fn add_specs_westend_no_network_info_not_signed() {
        let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
        populate_cold_no_networks(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = "Database contents:\nMetadata:\nNetwork Specs:\nVerifiers:\nGeneral Verifier: none\nIdentities: ";
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
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
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_ed25519_not_signed() {
        let dbname = "for_tests/add_specs_westend_ed25519_not_signed";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        try_create_seed("Alice", SEED_PHRASE, 0, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef, encryption: ed25519, path: , available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks: 
		0080e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9070() {
        let dbname = "for_tests/load_westend9070";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	westend9070
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_known_types_upd_general_verifier() {
        let dbname = "for_tests/load_known_types_upd_general_verifier";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: polkadot30, kusama2030, rococo9103, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Received: \n{}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_new_types_verified() {
        let dbname = "for_tests/load_new_types_verified";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Received: \n{}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn dock_adventures_1() {
        let dbname = "for_tests/dock_adventures_1";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
	dock-pos-main-runtime31
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: polkadot30, kusama2030, rococo9103, westend9000, westend9010. Types information is purged."},{"index":3,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":4,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn dock_adventures_2() {
        let dbname = "for_tests/dock_adventures_2";
        populate_cold(dbname, verifier_alice_sr25519()).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"dock-pos-main-runtime","spec_version":"31","meta_hash":"28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
	dock-pos-main-runtime31
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: none."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn can_parse_westend_with_v14() {
        let dbname = "for_tests/can_parse_westend_with_v14";
        populate_cold(dbname, Verifier(None)).unwrap();
        let line = fs::read_to_string("for_tests/load_metadata_westendV9111_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9111","meta_hash":"207956815bc7b3234fa8827ef40df5fd2879e93f18a680e22bc6801bca27312d"}}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        let print_before = print_db_content(&dbname);
        let expected_print_before = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_before == expected_print_before, "Received:\n{}", print_before);
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	kusama2030
	rococo9103
	westend9000
	westend9010
	westend9111
	polkadot30
Network Specs:
	018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
	0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
	0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: Rococo (rococo with sr25519)
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d9c0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480284d717d5031504025a62029723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Balances"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":3,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":4,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":7,"indent":3,"type":"balance","payload":{"amount":"100.000000","units":"uWND"}}],"extensions":[{"index":8,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"61","period":"64"}},{"index":9,"indent":0,"type":"nonce","payload":"261"},{"index":10,"indent":0,"type":"tip","payload":{"amount":"10.000000","units":"uWND"}},{"index":11,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":12,"indent":0,"type":"tx_version","payload":"7"},{"index":13,"indent":0,"type":"block_hash","payload":"98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        
        let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ffe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","seed":"Alice","derivation_path":"","has_password":false,"name":""}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Utility"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":3,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":4,"indent":3,"type":"pallet","payload":"Staking"},{"index":5,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e"}},{"index":6,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":7,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":8,"indent":7,"type":"Id","payload":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV"},{"index":9,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":10,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":11,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":12,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":13,"indent":3,"type":"pallet","payload":"Staking"},{"index":14,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":15,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":16,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":17,"indent":7,"type":"Id","payload":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh"},{"index":18,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":19,"indent":7,"type":"Id","payload":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ"},{"index":20,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":21,"indent":7,"type":"Id","payload":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f"}],"extensions":[{"index":22,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":23,"indent":0,"type":"nonce","payload":"2"},{"index":24,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":25,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":26,"indent":0,"type":"tx_version","payload":"7"},{"index":27,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        
        fs::remove_dir_all(dbname).unwrap();
    }
}
