#[cfg(test)]
mod tests {
    use hex;
    use transaction_parsing::produce_output;
    use crate::{checksum, handle_stub, sign_transaction::create_signature};
    use db_handling::{cold_default::{populate_cold, populate_cold_no_networks}, identities::try_create_seed};
    use definitions::{keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, network_specs::{CurrentVerifier, ChainSpecs, Verifier}, users::AddressDetails};
    use constants::{ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, VERIFIERS};
    use parity_scale_codec::Decode;
    use std::fs;
    use sled::{Db, open, Tree};
    use regex::Regex;
    use lazy_static::lazy_static;
    
    const SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    const PWD: &str = "jaskier";
    const USER_COMMENT: &str = "";
    const ALICE: [u8; 32] = [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
    
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
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"transfer_keep_alive","pallet":"Balances","docs":" Same as the [`transfer`] call, but with a check that the transfer will not kill the
 origin account.

 99% of the time you want [`transfer`] instead.

 [`transfer`]: struct.Pallet.html#method.transfer
 # <weight>
 - Cheaper than transfer because account cannot be killed.
 - Base Weight: 51.4 µs
 - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
 #</weight>"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":9,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        let result = sign_action_test(action_is_sign, &checksum_str, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        match result {
            Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
            Err(e) => panic!("Was unable to sign. {}", e),
        }
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
        populate_cold_no_networks(dbname, Verifier::None).unwrap();
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
        populate_cold(dbname, Verifier::None).unwrap();
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
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
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
        populate_cold(dbname, Verifier::None).unwrap();
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: polkadot30, kusama2030, westend9000, westend9010, rococo9103. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
        populate_cold(dbname, Verifier::None).unwrap();
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31."},{"index":2,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: polkadot30, kusama2030, westend9000, westend9010, rococo9103. Types information is purged."},{"index":3,"indent":0,"type":"warning","payload":"Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database."}],"new_specs":[{"index":4,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
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
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"","encryption":"none"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"custom","details":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
		01806bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn can_sign_metadata_v14_with_no_types_in_db() {
        let dbname = "for_tests/can_sign_metadata_v14_with_no_types_in_db";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: polkadot30, kusama2030, westend9000, westend9010, rococo9103. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received network specs information for Rococo is same as the one already in the database."}],"new_specs":[{"index":3,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#6f36dc","decimals":"12","encryption":"sr25519","genesis_hash":"f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a","logo":"rococo","name":"rococo","path_id":"//rococo","secondary_color":"#262626","title":"Rococo","unit":"ROC"}}],"action":{"type":"stub","payload":""##;
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
	0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"","encryption":"none"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"","encryption":"none"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"","encryption":"none"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"","encryption":"none"}}
General Verifier: none
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
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
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = fs::read_to_string("for_tests/load_metadata_rococoV9103_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"rococo","spec_version":"9103","meta_hash":"bd96af1b9561b124e05980ca9d32707d158be55d8bed115ecea2f31bbba8d270"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	rococo9103
Network Specs:
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = "530102c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2790040300c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2700b50100008f23000000000000f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2ff6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a";
        let reply = produce_output(&line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR","seed":"Alice","derivation_path":"//alice","has_password":false,"name":"Alice_test_rococo"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":{"pallet_name":"Balances","path":"pallet_balances >> pallet >> Call","docs":"Contains one variant per dispatchable that can be called by an extrinsic."}},{"index":2,"indent":1,"type":"enum_variant_name","payload":{"name":"transfer_keep_alive","docs_enum_variant":"Same as the [`transfer`] call, but with a check that the transfer will not kill the
origin account.

99% of the time you want [`transfer`] instead.

[`transfer`]: struct.Pallet.html#method.transfer
# <weight>
- Cheaper than transfer because account cannot be killed.
- Base Weight: 51.4 µs
- DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
#</weight>"}},{"index":3,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":5,"type":"Id","payload":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR"},{"index":6,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":7,"indent":3,"type":"balance","payload":{"amount":"0","units":"pROC"}}],"extrinsics":[{"index":8,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"0"}},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pROC"}},{"index":10,"indent":0,"type":"block_hash","payload":"829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2f"},{"index":11,"indent":0,"type":"tx_spec","payload":{"network":"rococo","version":"9103","tx_version":"0"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        
        let line = fs::read_to_string("for_tests/load_metadata_rococoV9106_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"rococo","spec_version":"9106","meta_hash":"78151026915b5c2301a96289cfee19d860a207df1d6e3497da8ad660b19fedbf"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        let (action_is_sign, checksum_str) = get_type_and_checksum(&reply);
        if action_is_sign {panic!("Should have been action `stub`.")}
        handle_stub(&checksum_str, dbname).unwrap();
        let print_after = print_db_content(&dbname);
        let expected_print_after = r#"Database contents:
Metadata:
	rococo9103
	rococo9106
Network Specs:
	0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: Rococo (rococo with sr25519)
Verifiers:
	91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
	f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a: {"type":"general","details":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities: 
	public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks: 
		0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
	public_key: 96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04, encryption: sr25519, path: //rococo, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27, encryption: sr25519, path: //alice, available_networks: 
		0180f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a
	public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks: 
		0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
	public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks: 
		018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(print_after == expected_print_after, "Received:\n{}", print_after);
        
        let line = "530102c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2790040300c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2700b50100008f23000000000000f6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2ff6e9983c37baf68846fedafe21e56718790e39fb1c582abc408b81bc7b208f9a";
        let reply = produce_output(&line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR","seed":"Alice","derivation_path":"//alice","has_password":false,"name":"Alice_test_rococo"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 9103. Latest known available version is 9106."}],"method":[{"index":2,"indent":0,"type":"pallet","payload":{"pallet_name":"Balances","path":"pallet_balances >> pallet >> Call","docs":"Contains one variant per dispatchable that can be called by an extrinsic."}},{"index":3,"indent":1,"type":"enum_variant_name","payload":{"name":"transfer_keep_alive","docs_enum_variant":"Same as the [`transfer`] call, but with a check that the transfer will not kill the
origin account.

99% of the time you want [`transfer`] instead.

[`transfer`]: struct.Pallet.html#method.transfer
# <weight>
- Cheaper than transfer because account cannot be killed.
- Base Weight: 51.4 µs
- DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
#</weight>"}},{"index":4,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":5,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":6,"indent":5,"type":"Id","payload":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR"},{"index":7,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":8,"indent":3,"type":"balance","payload":{"amount":"0","units":"pROC"}}],"extrinsics":[{"index":9,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"0"}},{"index":10,"indent":0,"type":"tip","payload":{"amount":"0","units":"pROC"}},{"index":11,"indent":0,"type":"block_hash","payload":"829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2f"},{"index":12,"indent":0,"type":"tx_spec","payload":{"network":"rococo","version":"9103","tx_version":"0"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        
        fs::remove_dir_all(dbname).unwrap();
    }
}
