/// Separated new cold test databases are created during the tests,
/// and removed after test is performed, so the test can run in parallel

#[cfg(test)]
mod tests {
    use crate::produce_output;
    use db_handling::{cold_default::{populate_cold, populate_cold_no_metadata, populate_cold_no_networks}, manage_history::print_history};
    use definitions::network_specs::Verifier;
    use std::fs;
    
    const ALICE: [u8; 32] = [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
    
    #[test]
    fn add_specs_westend_no_network_info_not_signed() {
        let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
        populate_cold_no_networks(dbname, Verifier::None).unwrap();
        let current_history = print_history(dbname).unwrap();
        assert!(current_history.contains(r#""events":[{"event":"database_initiated"}]"#), "Current history: \n{}", current_history);
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_not_signed() {
        let dbname = "for_tests/add_specs_westend_not_signed";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network specs from the message are already in the database."}]}"#;
        assert!(reply == reply_known, "Expected: {}...\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_not_signed_general_verifier_disappear() {
        let dbname = "for_tests/add_specs_westend_not_signed_general_verifier_disappear";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Expected: {}...\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_not_signed() {
        let dbname = "for_tests/load_types_known_not_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Types information already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_not_signed_general_verifier_disappear() {
        let dbname = "for_tests/load_types_known_not_signed_general_verifier_disappear";
        populate_cold_no_metadata(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_types_known_alice_signed() {
        let dbname = "for_tests/load_types_known_alice_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_known_general_verifier() {
        let dbname = "for_tests/load_types_known_alice_signed_known_general_verifier";
        populate_cold_no_metadata(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Types information already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_bad_general_verifier() {
        let dbname = "for_tests/load_types_known_alice_signed_bad_general_verifier";
        populate_cold_no_metadata(dbname, Verifier::Ed25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. Different general verifier was used previously. Previously used public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Current attempt public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_metadata_hold() {
        let dbname = "for_tests/load_types_known_alice_signed_metadata_hold";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: polkadot30, kusama2030, rococo9103, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_unknown_not_signed() {
        let dbname = "for_tests/load_types_unknown_not_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_types_unknown_alice_signed() {
        let dbname = "for_tests/load_types_unknown_alice_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_westend_50_not_in_db() {
        let dbname = "for_tests/parse_transaction_westend_50_not_in_db";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003200000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"All parsing attempts failed with following errors. Parsing with westend9010 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9000)."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_1() {
        let dbname = "for_tests/parse_transaction_1";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Balances"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":3,"indent":2,"type":"varname","payload":"dest"},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":4,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":2,"type":"varname","payload":"value"},{"index":7,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":8,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":9,"indent":0,"type":"nonce","payload":"46"},{"index":10,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":11,"indent":0,"type":"network","payload":"westend"},{"index":12,"indent":0,"type":"version","payload":"9010"},{"index":13,"indent":0,"type":"tx_version","payload":"5"},{"index":14,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_2() {
        let dbname = "for_tests/parse_transaction_2";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d550210020c060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700b864d9450006050800aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d0608008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f501b4003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Utility"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"2053656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a205468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a204d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a202d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e0a0a204966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a20627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a2023203c7765696768743e0a202d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a2023203c2f7765696768743e"}},{"index":3,"indent":2,"type":"varname","payload":"calls"},{"index":4,"indent":3,"type":"pallet","payload":"Staking"},{"index":5,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"2054616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a20626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a206076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a20456d6974732060426f6e646564602e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a202d204f2831292e0a202d20546872656520657874726120444220656e74726965732e0a0a204e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a20756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a202d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c65646765722c205b4f726967696e204163636f756e745d2c2043757272656e74204572612c20486973746f72792044657074682c204c6f636b730a202d2057726974653a20426f6e6465642c2050617965652c205b4f726967696e204163636f756e745d2c204c6f636b732c204c65646765720a2023203c2f7765696768743e"}},{"index":6,"indent":5,"type":"varname","payload":"controller"},{"index":7,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":8,"indent":7,"type":"Id","payload":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},{"index":9,"indent":5,"type":"varname","payload":"value"},{"index":10,"indent":6,"type":"balance","payload":{"amount":"300.000000000","units":"mWND"}},{"index":11,"indent":5,"type":"varname","payload":"payee"},{"index":12,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":13,"indent":3,"type":"pallet","payload":"Staking"},{"index":14,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"204465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e20546869732063616e206f6e6c792062652063616c6c6564207768656e0a205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a20416e642c2069742063616e206265206f6e6c792063616c6c6564207768656e205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a2023203c7765696768743e0a202d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a2077686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a202d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a202d2d2d2d2d2d2d2d2d0a205765696768743a204f284e290a207768657265204e20697320746865206e756d626572206f6620746172676574730a204442205765696768743a0a202d2052656164733a2045726120456c656374696f6e205374617475732c204c65646765722c2043757272656e74204572610a202d205772697465733a2056616c696461746f72732c204e6f6d696e61746f72730a2023203c2f7765696768743e"}},{"index":15,"indent":5,"type":"varname","payload":"targets"},{"index":16,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":17,"indent":7,"type":"Id","payload":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ"},{"index":18,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":19,"indent":7,"type":"Id","payload":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f"},{"index":20,"indent":3,"type":"pallet","payload":"Staking"},{"index":21,"indent":4,"type":"method","payload":{"method_name":"set_controller","docs":"202852652d297365742074686520636f6e74726f6c6c6572206f6620612073746173682e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f206279207468652073746173682c206e6f742074686520636f6e74726f6c6c65722e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e20496e7369676e69666963616e7420636f6d706c65786974792e0a202d20436f6e7461696e732061206c696d69746564206e756d626572206f662072656164732e0a202d2057726974657320617265206c696d6974656420746f2074686520606f726967696e60206163636f756e74206b65792e0a202d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a202d2057726974653a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a2023203c2f7765696768743e"}},{"index":22,"indent":5,"type":"varname","payload":"controller"},{"index":23,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":24,"indent":7,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}],"extensions":[{"index":25,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":26,"indent":0,"type":"nonce","payload":"45"},{"index":27,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":28,"indent":0,"type":"network","payload":"westend"},{"index":29,"indent":0,"type":"version","payload":"9010"},{"index":30,"indent":0,"type":"tx_version","payload":"5"},{"index":31,"indent":0,"type":"block_hash","payload":"314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_3() {
        let dbname = "for_tests/parse_transaction_3";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001750365010f00c06e31d910013223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ea8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cde143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Balances"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":3,"indent":2,"type":"varname","payload":"dest"},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":4,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":2,"type":"varname","payload":"value"},{"index":7,"indent":3,"type":"balance","payload":{"amount":"300.000000000000","units":"WND"}}],"extensions":[{"index":8,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"55","period":"64"}},{"index":9,"indent":0,"type":"nonce","payload":"89"},{"index":10,"indent":0,"type":"tip","payload":{"amount":"300.000000000000","units":"WND"}},{"index":11,"indent":0,"type":"network","payload":"westend"},{"index":12,"indent":0,"type":"version","payload":"9010"},{"index":13,"indent":0,"type":"tx_version","payload":"5"},{"index":14,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_4() {
        let dbname = "for_tests/parse_transaction_4";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = "530102c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2790040300c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e2700b50100008f23000000000000c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2fc196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR","seed":"Alice","derivation_path":"//alice","has_password":false,"name":"Alice_test_rococo"}}],"method":[{"index":1,"indent":0,"type":"pallet","payload":"Balances"},{"index":2,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":3,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":4,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":5,"indent":4,"type":"Id","payload":"5Gb6Zfe8K8NSKrkFLCgqs8LUdk7wKweXM5pN296jVqDpdziR"},{"index":6,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":7,"indent":3,"type":"balance","payload":{"amount":"0","units":"pROC"}}],"extensions":[{"index":8,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":9,"indent":0,"type":"nonce","payload":"0"},{"index":10,"indent":0,"type":"tip","payload":{"amount":"0","units":"pROC"}},{"index":11,"indent":0,"type":"version","payload":"9103"},{"index":12,"indent":0,"type":"tx_version","payload":"0"},{"index":13,"indent":0,"type":"network","payload":"rococo"},{"index":14,"indent":0,"type":"block_hash","payload":"829eea54e7190c8a23bffafe869f87428f3a1fe1c63cc1ec033c110e5a27eb2f"}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn print_all_cards() {
        let dbname = "for_tests/print_all_cards";
        populate_cold_no_networks(dbname, Verifier::None).unwrap();
        let line = "5300f0";
        let reply = produce_output(line, dbname);
        let reply_known = r##"{"method":[{"index":0,"indent":0,"type":"pallet","payload":"test_pallet"},{"index":1,"indent":0,"type":"method","payload":{"method_name":"test_method","docs":"766572626f7365200a6465736372697074696f6e200a6f66200a746865200a6d6574686f64"}},{"index":2,"indent":0,"type":"varname","payload":"test_Varname"},{"index":3,"indent":0,"type":"default","payload":"12345"},{"index":4,"indent":0,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":0,"type":"none","payload":""},{"index":6,"indent":0,"type":"identity_field","payload":"Twitter"},{"index":7,"indent":0,"type":"bitvec","payload":"[00000100, 00100000, 11011001]"},{"index":8,"indent":0,"type":"balance","payload":{"amount":"300.000000","units":"KULU"}},{"index":9,"indent":0,"type":"field_name","payload":{"name":"test_FieldName","docs_field_name":"612076657279207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"7479706520697320646966666963756c7420746f206465736372696265"}},{"index":10,"indent":0,"type":"field_number","payload":{"number":"1","docs_field_number":"6c657373207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"74797065206973206a75737420617320646966666963756c7420746f206465736372696265"}},{"index":11,"indent":0,"type":"enum_variant_name","payload":{"name":"test_EnumVariantName","docs_enum_variant":""}},{"index":12,"indent":0,"type":"era","payload":{"era":"Immortal","phase":"","period":""}},{"index":13,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":14,"indent":0,"type":"nonce","payload":"15"},{"index":15,"indent":0,"type":"network","payload":"westend"},{"index":16,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":17,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":18,"indent":0,"type":"version","payload":"9110"},{"index":19,"indent":0,"type":"tx_version","payload":"5"},{"index":20,"indent":0,"type":"author","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":""}},{"index":21,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}},{"index":22,"indent":0,"type":"author_public_key","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","crypto":"sr25519"}},{"index":23,"indent":0,"type":"verifier","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}},{"index":24,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9100","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"index":25,"indent":0,"type":"types_hash","payload":"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314"},{"index":26,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}},{"index":27,"indent":0,"type":"warning","payload":"Transaction author public key not found."},{"index":28,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 50. Latest known available version is 9010."},{"index":29,"indent":0,"type":"warning","payload":"Public key is on record, but not associated with the network used."},{"index":30,"indent":0,"type":"warning","payload":"Received network information is not verified."},{"index":31,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."},{"index":32,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":33,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: none; affected metadata entries: none. Types information is purged."},{"index":34,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":35,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":36,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":37,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."},{"index":38,"indent":0,"type":"warning","payload":"Received network specs information for Westend is same as the one already in the database."},{"index":39,"indent":0,"type":"error","payload":"All parsing attempts failed with following errors. Parsing with westend9120 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9120). Parsing with westend9010 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9010)."},{"index":40,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Expected mortal transaction due to prelude format. Found immortal transaction."},{"index":41,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Expected immortal transaction due to prelude format. Found mortal transaction."},{"index":42,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Genesis hash values from decoded extensions and from used network specs do not match."},{"index":43,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Block hash for immortal transaction not matching genesis hash for the network."},{"index":44,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Unable to decode extensions for V12/V13 metadata using standard extensions set."},{"index":45,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Method number 2 not found in pallet test_Pallet."},{"index":46,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Pallet with index 3 not found."},{"index":47,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Method number 5 too high for pallet number 3. Only 4 indices available."},{"index":48,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. No calls found in pallet test_pallet_v14."},{"index":49,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Referenced type could not be resolved in v14 metadata."},{"index":50,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Argument type error."},{"index":51,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Argument name error."},{"index":52,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Expected primitive type. Found Option<u8>."},{"index":53,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Expected compact. Not found it."},{"index":54,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Data too short for expected content."},{"index":55,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Unable to decode part of data as u32."},{"index":56,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Encountered unexpected Option<_> variant."},{"index":57,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. IdentityField description error."},{"index":58,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Unable to decode part of data as an array."},{"index":59,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Unexpected type encountered for Balance"},{"index":60,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Encountered unexpected enum variant."},{"index":61,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Unexpected type inside compact."},{"index":62,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Type claimed inside compact is not compactable."},{"index":63,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. No description found for type T::SomeUnknownType."},{"index":64,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Declared type is not suitable BitStore type for BitVec."},{"index":65,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Declared type is not suitable BitOrder type for BitVec."},{"index":66,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Could not decode BitVec."},{"index":67,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Could not decode Era."},{"index":68,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. After decoding the method some data remained unused."},{"index":69,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. After decoding the extensions some data remained unused."},{"index":70,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Era information is missing."},{"index":71,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Block hash information is missing."},{"index":72,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Metadata spec version information is missing."},{"index":73,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Era information is encountered mora than once."},{"index":74,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Genesis hash is encountered more than once."},{"index":75,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Block hash is encountered more than once."},{"index":76,"indent":0,"type":"error","payload":"Metadata spec version matches. Signed extensions are not compatible with Signer (v14 metadata). Metadata spec version is encountered more than once."},{"index":77,"indent":0,"type":"error","payload":"Metadata spec version matches. System error. Balance printing failed."},{"index":78,"indent":0,"type":"error","payload":"Metadata spec version matches. System error. Unexpected regular expressions error."},{"index":79,"indent":0,"type":"error","payload":"Network spec version decoded from extensions (50) differs from the version in metadata (9120)."},{"index":80,"indent":0,"type":"error","payload":"Bad input data. Data is too short."},{"index":81,"indent":0,"type":"error","payload":"Bad input data. Only Substrate transactions are supported. Transaction is expected to start with 0x53."},{"index":82,"indent":0,"type":"error","payload":"Bad input data. Input data not in hex format."},{"index":83,"indent":0,"type":"error","payload":"Bad input data. Crypto type not supported."},{"index":84,"indent":0,"type":"error","payload":"Bad input data. Wrong payload type, as announced by prelude."},{"index":85,"indent":0,"type":"error","payload":"Bad input data. Network westend is not in the database. Add network before loading the metadata."},{"index":86,"indent":0,"type":"error","payload":"Bad input data. First characters in metadata are expected to be 0x6d657461."},{"index":87,"indent":0,"type":"error","payload":"Bad input data. Received metadata could not be decoded. Runtime metadata version is below 12."},{"index":88,"indent":0,"type":"error","payload":"Bad input data. Metadata already in database."},{"index":89,"indent":0,"type":"error","payload":"Bad input data. Attempt to load different metadata for same name and version."},{"index":90,"indent":0,"type":"error","payload":"Bad input data. Received metadata version could not be decoded."},{"index":91,"indent":0,"type":"error","payload":"Bad input data. No version in received metadata."},{"index":92,"indent":0,"type":"error","payload":"Bad input data. Unable to decode received metadata."},{"index":93,"indent":0,"type":"error","payload":"Bad input data. Network specs from the message are already in the database."},{"index":94,"indent":0,"type":"error","payload":"Bad input data. Unable to decode received types information."},{"index":95,"indent":0,"type":"error","payload":"Bad input data. Types information already in database."},{"index":96,"indent":0,"type":"error","payload":"Bad input data. Unable to decode received add specs message."},{"index":97,"indent":0,"type":"error","payload":"Bad input data. Unable to decode received load metadata message."},{"index":98,"indent":0,"type":"error","payload":"Bad input data. Network already has entries. Important chainspecs in received add network message are different."},{"index":99,"indent":0,"type":"error","payload":"Database error. Internal error. Collection [1] does not exist"},{"index":100,"indent":0,"type":"error","payload":"Database error. Internal error. Unsupported: Something Unsupported."},{"index":101,"indent":0,"type":"error","payload":"Database error. Internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":102,"indent":0,"type":"error","payload":"Database error. Internal error. IO error: oh no!"},{"index":103,"indent":0,"type":"error","payload":"Database error. Internal error. Read corrupted data at file offset None backtrace ()"},{"index":104,"indent":0,"type":"error","payload":"Database error. ChainSpecs could not be decoded."},{"index":105,"indent":0,"type":"error","payload":"Database error. Network not found. Please add the network."},{"index":106,"indent":0,"type":"error","payload":"Database error. Address details could not be decoded."},{"index":107,"indent":0,"type":"error","payload":"Database error. Types information could not be decoded."},{"index":108,"indent":0,"type":"error","payload":"Database error. Types information not found."},{"index":109,"indent":0,"type":"error","payload":"Database error. Network versioned name could not be decoded."},{"index":110,"indent":0,"type":"error","payload":"Database error. No metadata on file for this network."},{"index":111,"indent":0,"type":"error","payload":"Database error. General verifier information could not be decoded."},{"index":112,"indent":0,"type":"error","payload":"Database error. No general verifier information in the database."},{"index":113,"indent":0,"type":"error","payload":"Database error. Network verifier is damaged and could not be decoded."},{"index":114,"indent":0,"type":"error","payload":"Database error. Network specs stored under key 0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e do not match it."},{"index":115,"indent":0,"type":"error","payload":"Database error. No verifier information corresponding to genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, however, genesis hash is encountered in network specs"},{"index":116,"indent":0,"type":"error","payload":"Database error. Different network names in database for same genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":117,"indent":0,"type":"error","payload":"Database error. Error setting stub into storage. This error should not be here."},{"index":118,"indent":0,"type":"error","payload":"Database error. Network westend is disabled. It could be enabled again only after complete wipe and re-installation of Signer."},{"index":119,"indent":0,"type":"error","payload":"Database error. Custom verifier for VerifierKey e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is same as general verifier."},{"index":120,"indent":0,"type":"error","payload":"Database error. Metadata entry does not start with 0x6d657461."},{"index":121,"indent":0,"type":"error","payload":"Database error. Metadata could not be decoded. Runtime metadata version is below 12."},{"index":122,"indent":0,"type":"error","payload":"Database error. Network metadata entry corrupted in database, name and/or version in meta_key do not match the ones in metadata itself. Please remove the entry and download the metadata for this network."},{"index":123,"indent":0,"type":"error","payload":"Database error. Metadata in storage has no version."},{"index":124,"indent":0,"type":"error","payload":"Database error. Version block of metadata in storage could not be decoded."},{"index":125,"indent":0,"type":"error","payload":"Database error. Metadata in storage could not be decoded."},{"index":126,"indent":0,"type":"error","payload":"Database error. Metadata runtime version in database is not v12, v13, or v14."},{"index":127,"indent":0,"type":"error","payload":"Safety-related error. Corrupted data. Bad signature."},{"index":128,"indent":0,"type":"error","payload":"Safety-related error. Network westend current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."},{"index":129,"indent":0,"type":"error","payload":"Safety-related error. Saved information for this network was signed by a verifier. Received information is not signed."},{"index":130,"indent":0,"type":"error","payload":"Safety-related error. Different general verifier was used previously. Previously used public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Current attempt public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":131,"indent":0,"type":"error","payload":"Safety-related error. General verifier information exists in the database. Received information could be accepted only from the same general verifier."},{"index":132,"indent":0,"type":"error","payload":"Safety-related error. Network westend currently has no verifier set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata, first download properly verified network specs."},{"index":133,"indent":0,"type":"error","payload":"Safety-related error. Network westend current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing verifier for the network would require wipe and reset of Signer."},{"index":134,"indent":0,"type":"error","payload":"Safety-related error. Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."},{"index":135,"indent":0,"type":"error","payload":"Safety-related error. Network westend is verified by the general verifier which currently is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."}]}"##;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9070_not_signed() {
        let dbname = "for_tests/load_westend9070_not_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9070_alice_signed() {
        let dbname = "for_tests/load_westend9070_alice_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_not_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_not_signed";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_alice_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9000_already_in_db_alice_signed_known_general_verifier() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_known_general_verifier";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9000_already_in_db_alice_signed_bad_general_verifier() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_bad_general_verifier";
        populate_cold(dbname, Verifier::Ed25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. Network westend is verified by the general verifier which currently is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_dock31_unknown_network() {
        let dbname = "for_tests/load_dock31_unknown_network";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network dock-pos-main-runtime is not in the database. Add network before loading the metadata."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_dock_not_verified_db_not_verified() {
        let dbname = "for_tests/add_specs_dock_not_verified_db_not_verified";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_dock_alice_verified_db_not_verified() {
        let dbname = "for_tests/add_specs_dock_alice_verified_db_not_verified";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: polkadot30, kusama2030, rococo9103, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_dock_not_verified_db_alice_verified() {
        let dbname = "for_tests/add_specs_dock_not_verified_db_alice_verified";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_dock_both_verified_same() {
        let dbname = "for_tests/add_specs_dock_both_verified_same";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_dock_both_verified_different() {
        let dbname = "for_tests/add_specs_dock_both_verified_different";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","encryption":"ed25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
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
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_ed25519_alice_signed_db_not_verified() {
        let dbname = "for_tests/add_specs_westend_ed25519_alice_signed_db_not_verified";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Rococo, Westend; affected metadata entries: polkadot30, kusama2030, rococo9103, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_ed25519_not_verified_db_alice_verified() {
        let dbname = "for_tests/add_specs_westend_ed25519_not_verified_db_alice_verified";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_ed25519_both_verified_same() {
        let dbname = "for_tests/add_specs_westend_ed25519_both_verified_same";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}],"action":{"type":"stub","payload":""##;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_ed25519_both_verified_different() {
        let dbname = "for_tests/add_specs_westend_ed25519_both_verified_different";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-ed25519.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Safety-related error. Different general verifier was used previously. Previously used public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Current attempt public key: 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee, encryption: ed25519."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_5_unknown_author() {
        let dbname = "for_tests/parse_transaction_5_unknown_author";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "5301008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48a4040300d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Transaction author public key not found."}],"method":[{"index":2,"indent":0,"type":"pallet","payload":"Balances"},{"index":3,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":4,"indent":2,"type":"varname","payload":"dest"},{"index":5,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":6,"indent":4,"type":"Id","payload":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},{"index":7,"indent":2,"type":"varname","payload":"value"},{"index":8,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":9,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":10,"indent":0,"type":"nonce","payload":"46"},{"index":11,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":12,"indent":0,"type":"network","payload":"westend"},{"index":13,"indent":0,"type":"version","payload":"9010"},{"index":14,"indent":0,"type":"tx_version","payload":"5"},{"index":15,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_6_unknown_network() {
        let dbname = "for_tests/parse_transaction_6_unknown_network";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530102761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62a8030300761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c620b00407a10f35aa707000b00a0724e1809140000000a000000f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769badc21d36b69bae1e8a41dedb34758567ba4efe711412f33d1461f795ffcd1de13f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba";
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author_public_key","payload":{"hex":"761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62","crypto":"sr25519"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Database error. Network not found. Please add the network."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_7_error_on_parsing() {
        let dbname = "for_tests/parse_transaction_7_error_on_parsing";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403018eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. After decoding the method some data remained unused."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"network","payload":"westend"},{"index":6,"indent":0,"type":"version","payload":"9010"},{"index":7,"indent":0,"type":"tx_version","payload":"5"},{"index":8,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_8_error_on_parsing() {
        let dbname = "for_tests/parse_transaction_8_error_on_parsing";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403068eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Encountered unexpected enum variant."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"network","payload":"westend"},{"index":6,"indent":0,"type":"version","payload":"9010"},{"index":7,"indent":0,"type":"tx_version","payload":"5"},{"index":8,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn parse_transaction_9_error_on_parsing() {
        let dbname = "for_tests/parse_transaction_9_error_on_parsing";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403028eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Metadata spec version matches. Error decoding transaction content. Data too short for expected content."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"network","payload":"westend"},{"index":6,"indent":0,"type":"version","payload":"9010"},{"index":7,"indent":0,"type":"tx_version","payload":"5"},{"index":8,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

}
