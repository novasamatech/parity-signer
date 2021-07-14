/// Separated new cold test databases are created during the tests,
/// and removed after test is performed, so the test can run in parallel

#[cfg(test)]
mod tests {
    use transaction_parsing::produce_output;
    use super::super::handle_action;
    use db_handling::populate_cold;
    use definitions::{constants::{METATREE, SPECSTREE}};
    use std::fs;
    use sled::{Db, open, Tree};
    
    const METADATA_FILE: &str = "for_tests/metadata_database.ts";
    const SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    const PWD: &str = "jaskier";
    
    fn meta_count(dbname: &str) -> Result<usize, Box<dyn std::error::Error>> {
         let database: Db = open(dbname)?;
         let metadata: Tree = database.open_tree(METATREE)?;
         Ok(metadata.len())
    }
    
    fn specs_count(dbname: &str) -> Result<usize, Box<dyn std::error::Error>> {
         let database: Db = open(dbname)?;
         let chainspecs: Tree = database.open_tree(SPECSTREE)?;
         Ok(chainspecs.len())
    }
    
// can sign a parsed transaction
    #[test]
    fn can_sign_transaction_1() {
        let dbname = "for_tests/can_sign_transaction_1";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":"false","name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"transfer_keep_alive","pallet":"Balances"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":"Id"},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":9,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign_transaction","payload":{"type":"sign_transaction","checksum":"3665731191"}}}"#;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"sign_transaction","checksum":"3665731191"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to sign. {}", e)}
        fs::remove_dir_all("for_tests/can_sign_transaction_1").unwrap();
    }

// add_network for dock_main without verifier, then add_network with same metadata and with verifier
    #[test]
    fn add_network_add_two_verifiers_later() {
        
        let dbname = "for_tests/add_network_add_two_verifiers_later";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let meta1 = meta_count(dbname).unwrap();
        let specs1 = specs_count(dbname).unwrap();
        
        let line = fs::read_to_string("for_tests/add_network_with_defaults_dock-main-runtimeV25_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        let reply_known = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received new network information is not verified."}],"new_network":[{"index":1,"indent":0,"type":"new_network","payload":{"specname":"dock-main-runtime","spec_version":"25","meta_hash":"8dcc1cb8dd2119054ff1570eec01193dbfe4fdf43cea9fab0dac5674184ae06e","base58prefix":"22","color":"#660D35","decimals":"6","genesis_hash":"f73467c6544aa68df2ee546b135f955c46b90fa627e9b5d7935f41061bb8a5a9","logo":"dock-main-runtime","name":"dock-main-runtime","path_id":"//dock-main-runtime","secondary_color":"#262626","title":"dock-main-runtime","unit":"DCK","verifier":"none"}}],"action":{"type":"add_network","payload":{"type":"add_network","checksum":"140388874"}}}"##;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"add_network","checksum":"140388874"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to add network. {}", e)}
        
        let meta2 = meta_count(dbname).unwrap();
        let specs2 = specs_count(dbname).unwrap();
        
        let line = fs::read_to_string("for_tests/add_network_with_defaults_dock-main-runtimeV25_Alice.txt").unwrap();
        let reply = produce_output(&line, dbname);
        let reply_known = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Add network message is received for network that already has some entries in the database."},{"index":2,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":3,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":4,"indent":0,"type":"warning","payload":"Received metadata is already in database, both general verifier and network verifier could be added."}],"action":{"type":"add_two_verifiers","payload":{"type":"add_two_verifiers","checksum":"2705592468"}}}"##;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"add_two_verifiers","checksum":"2705592468"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to update two verifiers. {}", e)}
        
        let meta3 = meta_count(dbname).unwrap();
        let specs3 = specs_count(dbname).unwrap();
        
        assert!(meta2 == meta1+1, "Did not add metadata to database on first step.");
        assert!(meta3 == meta2, "Number of meta entries somehow changed on second step.");
        assert!(specs2 == specs1+1, "Did not add specs to database on first step.");
        assert!(specs3 == specs2, "Number of specs entries somehow changed on second step.");
        
        fs::remove_dir_all("for_tests/add_network_add_two_verifiers_later").unwrap();
    }

// add_network for dock_main with verifier
    #[test]
    fn add_network_and_add_general_verifier() {
    
        let dbname = "for_tests/add_network_and_add_general_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        let meta1 = meta_count(dbname).unwrap();
        let specs1 = specs_count(dbname).unwrap();
        
        let line = fs::read_to_string("for_tests/add_network_with_defaults_dock-main-runtimeV25_Alice.txt").unwrap();
        let reply = produce_output(&line, dbname);
        let reply_known = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."}],"new_network":[{"index":2,"indent":0,"type":"new_network","payload":{"specname":"dock-main-runtime","spec_version":"25","meta_hash":"8dcc1cb8dd2119054ff1570eec01193dbfe4fdf43cea9fab0dac5674184ae06e","base58prefix":"22","color":"#660D35","decimals":"6","genesis_hash":"f73467c6544aa68df2ee546b135f955c46b90fa627e9b5d7935f41061bb8a5a9","logo":"dock-main-runtime","name":"dock-main-runtime","path_id":"//dock-main-runtime","secondary_color":"#262626","title":"dock-main-runtime","unit":"DCK","verifier":"{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}"}}],"action":{"type":"add_network_and_add_general_verifier","payload":{"type":"add_network_and_add_general_verifier","checksum":"2643991971"}}}"##;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"add_network_and_add_general_verifier","checksum":"2643991971"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to add network and update general verifier. {}", e)}
        
        let meta2 = meta_count(dbname).unwrap();
        let specs2 = specs_count(dbname).unwrap();
        
        assert!(meta2 == meta1+1, "Did not add metadata to database.");
        assert!(specs2 == specs1+1, "Did not add specs to database.");
        
        fs::remove_dir_all("for_tests/add_network_and_add_general_verifier").unwrap();
    }

    #[test]
    fn correct_checksum_no_transaction_to_sign() {
    
        let dbname = "for_tests/correct_checksum_no_transaction_to_sign";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: add_network
        let line = fs::read_to_string("for_tests/add_network_with_defaults_dock-main-runtimeV25_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        // wrong action: sign_transaction
        let mock_action_line = r#"{"type":"sign_transaction","checksum":"140388874"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved transaction found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_transaction_to_sign").unwrap();
    }

    #[test]
    fn correct_checksum_no_approved_metadata() {
    
        let dbname = "for_tests/correct_checksum_no_approved_metadata";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: load_metadata
        let mock_action_line = r#"{"type":"load_metadata","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved metadata found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_approved_metadata").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_metadata_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_metadata_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: add_metadata_verifier
        let mock_action_line = r#"{"type":"add_metadata_verifier","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved verifier found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_metadata_verifier").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_types_to_load() {
    
        let dbname = "for_tests/correct_checksum_no_types_to_load";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: load_types
        let mock_action_line = r#"{"type":"load_types","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved types information found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_types_to_load").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_general_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_general_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: add_general_verifier
        let mock_action_line = r#"{"type":"add_general_verifier","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved general verifier found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        fs::remove_dir_all("for_tests/correct_checksum_no_general_verifier").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_two_verifiers() {
    
        let dbname = "for_tests/correct_checksum_no_two_verifiers";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: add_two_verifiers
        let mock_action_line = r#"{"type":"add_two_verifiers","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved verifier found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        fs::remove_dir_all("for_tests/correct_checksum_no_two_verifiers").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_load_meta_and_upd_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_load_meta_and_upd_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: load_metadata_and_add_general_verifier
        let mock_action_line = r#"{"type":"load_metadata_and_add_general_verifier","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved metadata found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_load_meta_and_upd_verifier").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_add_network() {
    
        let dbname = "for_tests/correct_checksum_no_add_network";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: add_network
        let mock_action_line = r#"{"type":"add_network","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved network information found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_add_network").unwrap();
    }
    
    #[test]
    fn correct_checksum_no_add_network_and_general_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_add_network_and_general_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line, dbname);
        
        // wrong action: add_network_and_add_general_verifier
        let mock_action_line = r#"{"type":"add_network_and_add_general_verifier","checksum":"3665731191"}"#;
        
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("No approved network information found.");
            if err != expected_err {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        }
        else {panic!("Should have correct checksum and wrong action. Parser reply: {}\nMock actionline: {}", reply, mock_action_line)}
        
        fs::remove_dir_all("for_tests/correct_checksum_no_add_network_and_general_verifier").unwrap();
        
    }
    
// load_metadata for westend9070 not verified, then load same metadata, but with verifier
    #[test]
    fn load_network_unsigned_add_verifier_later() {
        
        let dbname = "for_tests/load_network_unsigned_add_verifier_later";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        let meta1 = meta_count(dbname).unwrap();
        let specs1 = specs_count(dbname).unwrap();
        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":"2214182072"}}}"##;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"load_metadata","checksum":"2214182072"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to load metadata for westend 9070 network. {}", e)}
        
        let meta2 = meta_count(dbname).unwrap();
        let specs2 = specs_count(dbname).unwrap();
        
        assert!(meta2 == meta1+1, "Did not add metadata to database.");
        assert!(specs2 == specs1, "Number of specs entries somehow changed.");
        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":"3177445589"}}}"##;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"add_metadata_verifier","checksum":"3177445589"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to add verifier for westend. {}", e)}
        
        let meta3 = meta_count(dbname).unwrap();
        let specs3 = specs_count(dbname).unwrap();
        
        assert!(meta3 == meta2, "Number of meta entries somehow changed.");
        assert!(specs3 == specs2, "Number of specs entries somehow changed.");
        
        fs::remove_dir_all("for_tests/load_network_unsigned_add_verifier_later").unwrap();
    }
    
// load_types not verified, then load same types message, but with verifier
    #[test]
    fn load_types_unsigned_add_verifier_later() {
        
        let dbname = "for_tests/load_types_unsigned_add_verifier_later";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":"2058294086"}}}"#;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"load_types","checksum":"2058294086"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to load types. {}", e)}
        
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is already in database, only verifier could be added."}],"action":{"type":"add_general_verifier","payload":{"type":"add_general_verifier","checksum":"1604111942"}}}"#;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"add_general_verifier","checksum":"1604111942"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to add general verifier. {}", e)}
        
        fs::remove_dir_all("for_tests/load_types_unsigned_add_verifier_later").unwrap();
    }
    
// load_types with verifier, general verifier appears, still can load metadata without verifier, but cannot add networks unverified
    #[test]
    fn load_types_verified_then_test_unverified_load_metadata_and_unverified_add_network() {
        
        let dbname = "for_tests/load_types_verified_then_test_unverified_load_metadata_and_unverified_add_network";
        populate_cold(dbname, METADATA_FILE, true).unwrap();

        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":"2398490910"}}}"#;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"load_types","checksum":"2398490910"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to load types with adding general verifier. {}", e)}

    // loading metadata without verifier - should work        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":"2405833550"}}}"#;
        assert!(reply == reply_known, "Error in action.\nReceived: {}", reply);
        let mock_action_line = r#"{"type":"load_metadata","checksum":"2405833550"}"#;
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, dbname);
        if let Err(e) = result {panic!("Was unable to load metadata without signature after general verifier appeared. {}", e)}
        
    // adding network without verifier - should not work
        let line = fs::read_to_string("for_tests/add_network_with_defaults_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line, dbname);
        
        let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]"#;
        assert!(reply == reply_known, "Error in parsing outcome.\nReceived: {}", reply);
        
        fs::remove_dir_all("for_tests/load_types_verified_then_test_unverified_load_metadata_and_unverified_add_network").unwrap();
    }

}
