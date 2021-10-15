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
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Network specs from the message are already in the database."}]}"#;
        assert!(reply == reply_known, "Expected: {}...\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_specs_westend_not_signed_general_verifier_disappear() {
        let dbname = "for_tests/add_specs_westend_not_signed_general_verifier_disappear";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Expected: {}...\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_not_signed() {
        let dbname = "for_tests/load_types_known_not_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Types information already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_not_signed_general_verifier_disappear() {
        let dbname = "for_tests/load_types_known_not_signed_general_verifier_disappear";
        populate_cold_no_metadata(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_types_known_alice_signed() {
        let dbname = "for_tests/load_types_known_alice_signed";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_known_general_verifier() {
        let dbname = "for_tests/load_types_known_alice_signed_known_general_verifier";
        populate_cold_no_metadata(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Types information already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_bad_general_verifier() {
        let dbname = "for_tests/load_types_known_alice_signed_bad_general_verifier";
        populate_cold_no_metadata(dbname, Verifier::Ed25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Different general verifier was used previously. Previously used public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Current attempt public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_alice_signed_metadata_hold() {
        let dbname = "for_tests/load_types_known_alice_signed_metadata_hold";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: polkadot30, kusama2030, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"}],"action":{"type":"stub","payload":""#;
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
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Polkadot, Kusama, Westend, Rococo; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"stub","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_westend_50_not_in_db() {
        let dbname = "for_tests/parse_transaction_westend_50_not_in_db";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003200000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"No metadata on file for this version."}],"extrinsics":[{"index":2,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":3,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":4,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":5,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"50","tx_version":"5"}}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_1() {
        let dbname = "for_tests/parse_transaction_1";
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
 #</weight>"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":9,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_2() {
        let dbname = "for_tests/parse_transaction_2";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d550210020c060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700b864d9450006050800aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d0608008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f501b4003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"batch_all","pallet":"Utility","docs":" Send a batch of dispatch calls and atomically execute them.
 The whole transaction will rollback and fail if any of the calls failed.

 May be called from any origin.

 - `calls`: The calls to be dispatched from the same origin.

 If origin is root then call are dispatch without checking origin filter. (This includes
 bypassing `frame_system::Config::BaseCallFilter`).

 # <weight>
 - Complexity: O(C) where C is the number of calls to be batched.
 # </weight>"}},{"index":2,"indent":1,"type":"varname","payload":"calls"},{"index":3,"indent":2,"type":"call","payload":{"method":"bond","pallet":"Staking","docs":" Take the origin account as a stash and lock up `value` of its balance. `controller` will
 be the account that controls it.

 `value` must be more than the `minimum_balance` specified by `T::Currency`.

 The dispatch origin for this call must be _Signed_ by the stash account.

 Emits `Bonded`.

 # <weight>
 - Independent of the arguments. Moderate complexity.
 - O(1).
 - Three extra DB entries.

 NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned
 unless the `origin` falls below _existential deposit_ and gets removed as dust.
 ------------------
 Weight: O(1)
 DB Weight:
 - Read: Bonded, Ledger, [Origin Account], Current Era, History Depth, Locks
 - Write: Bonded, Payee, [Origin Account], Locks, Ledger
 # </weight>"}},{"index":4,"indent":3,"type":"varname","payload":"controller"},{"index":5,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":6,"indent":5,"type":"Id","payload":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},{"index":7,"indent":3,"type":"varname","payload":"value"},{"index":8,"indent":4,"type":"balance","payload":{"amount":"300.000000000","units":"mWND"}},{"index":9,"indent":3,"type":"varname","payload":"payee"},{"index":10,"indent":4,"type":"enum_variant_name","payload":{"name":"Staked","docs":""}},{"index":11,"indent":2,"type":"call","payload":{"method":"nominate","pallet":"Staking","docs":" Declare the desire to nominate `targets` for the origin controller.

 Effects will be felt at the beginning of the next era. This can only be called when
 [`EraElectionStatus`] is `Closed`.

 The dispatch origin for this call must be _Signed_ by the controller, not the stash.
 And, it can be only called when [`EraElectionStatus`] is `Closed`.

 # <weight>
 - The transaction's complexity is proportional to the size of `targets` (N)
 which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS).
 - Both the reads and writes follow a similar pattern.
 ---------
 Weight: O(N)
 where N is the number of targets
 DB Weight:
 - Reads: Era Election Status, Ledger, Current Era
 - Writes: Validators, Nominators
 # </weight>"}},{"index":12,"indent":3,"type":"varname","payload":"targets"},{"index":13,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":14,"indent":5,"type":"Id","payload":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ"},{"index":15,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":16,"indent":5,"type":"Id","payload":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f"},{"index":17,"indent":2,"type":"call","payload":{"method":"set_controller","pallet":"Staking","docs":" (Re-)set the controller of a stash.

 Effects will be felt at the beginning of the next era.

 The dispatch origin for this call must be _Signed_ by the stash, not the controller.

 # <weight>
 - Independent of the arguments. Insignificant complexity.
 - Contains a limited number of reads.
 - Writes are limited to the `origin` account key.
 ----------
 Weight: O(1)
 DB Weight:
 - Read: Bonded, Ledger New Controller, Ledger Old Controller
 - Write: Bonded, Ledger New Controller, Ledger Old Controller
 # </weight>"}},{"index":18,"indent":3,"type":"varname","payload":"controller"},{"index":19,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":20,"indent":5,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}],"extrinsics":[{"index":21,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"31","period":"64","nonce":"45"}},{"index":22,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":23,"indent":0,"type":"block_hash","payload":"314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"},{"index":24,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_3() {
        let dbname = "for_tests/parse_transaction_3";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001750365010f00c06e31d910013223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ea8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cde143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"transfer_keep_alive","pallet":"Balances","docs":" Same as the [`transfer`] call, but with a check that the transfer will not kill the
 origin account.

 99% of the time you want [`transfer`] instead.

 [`transfer`]: struct.Pallet.html#method.transfer
 # <weight>
 - Cheaper than transfer because account cannot be killed.
 - Base Weight: 51.4 µs
 - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
 #</weight>"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"300.000000000000","units":"WND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"55","period":"64","nonce":"89"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"300.000000000000","units":"WND"}},{"index":9,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign","payload":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn print_all_cards() {
        let dbname = "for_tests/print_all_cards";
        populate_cold_no_networks(dbname, Verifier::None).unwrap();
        let line = "5300f0";
        let reply = produce_output(line, dbname);
        let reply_known = r##"{"method":[{"index":0,"indent":0,"type":"call","payload":{"method":"test_Method","pallet":"test_Pallet","docs":"test docs description"}},{"index":1,"indent":0,"type":"pallet","payload":"test_pallet_v14"},{"index":2,"indent":0,"type":"varname","payload":"test_Varname"},{"index":3,"indent":0,"type":"default","payload":"12345"},{"index":4,"indent":0,"type":"path_and_docs","payload":{"path":["frame_system","pallet","Call"],"docs":"test docs"}},{"index":5,"indent":0,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":0,"type":"none","payload":""},{"index":7,"indent":0,"type":"identity_field","payload":"Twitter"},{"index":8,"indent":0,"type":"bitvec","payload":"[00000100, 00100000, 11011001]"},{"index":9,"indent":0,"type":"balance","payload":{"amount":"300.000000","units":"KULU"}},{"index":10,"indent":0,"type":"field_name","payload":{"name":"test_FieldName","docs":""}},{"index":11,"indent":0,"type":"field_number","payload":{"number":"1","docs":""}},{"index":12,"indent":0,"type":"enum_variant_name","payload":{"name":"test_EnumVariantName","docs":""}},{"index":13,"indent":0,"type":"era_immortal_nonce","payload":{"era":"Immortal","nonce":"4980"}},{"index":14,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"55","period":"64","nonce":"89"}},{"index":15,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":16,"indent":0,"type":"tip_plain","payload":"8800"},{"index":17,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":18,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"50","tx_version":"5"}},{"index":19,"indent":0,"type":"tx_spec_plain","payload":{"network_genesis_hash":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd","version":"50","tx_version":"5"}},{"index":20,"indent":0,"type":"author","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":""}},{"index":21,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}},{"index":22,"indent":0,"type":"author_public_key","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","crypto":"sr25519"}},{"index":23,"indent":0,"type":"verifier","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}},{"index":24,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9100","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"index":25,"indent":0,"type":"types_hash","payload":"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314"},{"index":26,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}},{"index":27,"indent":0,"type":"warning","payload":"Transaction author public key not found."},{"index":28,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 50. Latest known available version is 9010."},{"index":29,"indent":0,"type":"warning","payload":"Public key is on record, but not associated with the network used."},{"index":30,"indent":0,"type":"warning","payload":"Received network information is not verified."},{"index":31,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."},{"index":32,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":33,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: none; affected metadata entries: none. Types information is purged."},{"index":34,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":35,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":36,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."},{"index":37,"indent":0,"type":"warning","payload":"Received network specs information for Westend is same as the one already in the database."},{"index":38,"indent":0,"type":"error","payload":"Data is too short."},{"index":39,"indent":0,"type":"error","payload":"Only Substrate transactions are supported. Transaction is expected to start with 0x53."},{"index":40,"indent":0,"type":"error","payload":"Input data not in hex format."},{"index":41,"indent":0,"type":"error","payload":"Crypto type not supported."},{"index":42,"indent":0,"type":"error","payload":"Wrong payload type, as announced by prelude."},{"index":43,"indent":0,"type":"error","payload":"Genesis hash from extrinsics not matching with genesis hash at the transaction end."},{"index":44,"indent":0,"type":"error","payload":"Block hash for immortal transaction not matching genesis hash for the network."},{"index":45,"indent":0,"type":"error","payload":"After decoding some data remained unused."},{"index":46,"indent":0,"type":"error","payload":"Network westend in not in the database. Add network before loading the metadata."},{"index":47,"indent":0,"type":"error","payload":"First characters in metadata are expected to be 0x6d657461."},{"index":48,"indent":0,"type":"error","payload":"Received metadata could not be decoded. Runtime metadata version is below 12."},{"index":49,"indent":0,"type":"error","payload":"Metadata already in database."},{"index":50,"indent":0,"type":"error","payload":"Attempt to load different metadata for same name and version."},{"index":51,"indent":0,"type":"error","payload":"Received metadata version could not be decoded."},{"index":52,"indent":0,"type":"error","payload":"No version in received metadata."},{"index":53,"indent":0,"type":"error","payload":"Unable to decode received metadata."},{"index":54,"indent":0,"type":"error","payload":"Network specs from the message are already in the database."},{"index":55,"indent":0,"type":"error","payload":"Unable to decode received types information."},{"index":56,"indent":0,"type":"error","payload":"Types information already in database."},{"index":57,"indent":0,"type":"error","payload":"Unable to decode received add specs message."},{"index":58,"indent":0,"type":"error","payload":"Unable to decode received load metadata message."},{"index":59,"indent":0,"type":"error","payload":"Network already has entries. Important chainspecs in received add network message are different."},{"index":60,"indent":0,"type":"error","payload":"Encryption used in message is not supported by the network."},{"index":61,"indent":0,"type":"error","payload":"Unable to separate transaction vector, extrinsics, and genesis hash."},{"index":62,"indent":0,"type":"error","payload":"Error on decoding. Expected method and pallet information. Found data is shorter."},{"index":63,"indent":0,"type":"error","payload":"Error on decoding. Expected pallet information. Found data is shorter."},{"index":64,"indent":0,"type":"error","payload":"Method number 2 not found in pallet test_Pallet."},{"index":65,"indent":0,"type":"error","payload":"Pallet with index 3 not found."},{"index":66,"indent":0,"type":"error","payload":"Method number 5 too high for pallet number 3. Only 4 indices available."},{"index":67,"indent":0,"type":"error","payload":"No calls found in pallet test_pallet_v14."},{"index":68,"indent":0,"type":"error","payload":"Error decoding with v14 metadata. Referenced type could not be resolved."},{"index":69,"indent":0,"type":"error","payload":"Argument type error."},{"index":70,"indent":0,"type":"error","payload":"Argument name error."},{"index":71,"indent":0,"type":"error","payload":"Error decoding call contents. Expected primitive type. Found Option<u8>."},{"index":72,"indent":0,"type":"error","payload":"Error decoding call contents. Expected compact. Not found it."},{"index":73,"indent":0,"type":"error","payload":"Error decoding call contents. Data too short for expected content."},{"index":74,"indent":0,"type":"error","payload":"Error decoding call content. Unable to decode part of data as u32."},{"index":75,"indent":0,"type":"error","payload":"Error decoding call content. Encountered unexpected Option<_> variant."},{"index":76,"indent":0,"type":"error","payload":"Error decoding call content. IdentityField description error."},{"index":77,"indent":0,"type":"error","payload":"Error decoding call content. Unable to decode part of data as an [u8; 32] array."},{"index":78,"indent":0,"type":"error","payload":"Error decoding call content. Unexpected type encountered for Balance"},{"index":79,"indent":0,"type":"error","payload":"Error decoding call content. Encountered unexpected enum variant."},{"index":80,"indent":0,"type":"error","payload":"Error decoding call content. Unexpected type inside compact."},{"index":81,"indent":0,"type":"error","payload":"Error decoding call content. Type inside compact cound not be transformed into primitive."},{"index":82,"indent":0,"type":"error","payload":"Error decoding call content. No description found for type T::SomeUnknownType."},{"index":83,"indent":0,"type":"error","payload":"Error decoding call content. Declared type is not suitable BitStore type for BitVec."},{"index":84,"indent":0,"type":"error","payload":"Error decoding call content. Declared type is not suitable BitOrder type for BitVec."},{"index":85,"indent":0,"type":"error","payload":"Error decoding call content. Could not decode BitVec."},{"index":86,"indent":0,"type":"error","payload":"Database internal error. Collection [1] does not exist"},{"index":87,"indent":0,"type":"error","payload":"Database internal error. Unsupported: Something Unsupported."},{"index":88,"indent":0,"type":"error","payload":"Database internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":89,"indent":0,"type":"error","payload":"Database internal error. IO error: oh no!"},{"index":90,"indent":0,"type":"error","payload":"Database internal error. Read corrupted data at file offset None backtrace ()"},{"index":91,"indent":0,"type":"error","payload":"ChainSpecs from database could not be decoded."},{"index":92,"indent":0,"type":"error","payload":"Network not found. Please add the network."},{"index":93,"indent":0,"type":"error","payload":"Address details from database could not be decoded."},{"index":94,"indent":0,"type":"error","payload":"Types database from database could not be decoded."},{"index":95,"indent":0,"type":"error","payload":"Types information not found in the database"},{"index":96,"indent":0,"type":"error","payload":"Network versioned name from metadata database could not be decoded."},{"index":97,"indent":0,"type":"error","payload":"No metadata on file for this version."},{"index":98,"indent":0,"type":"error","payload":"No metadata on file for this network."},{"index":99,"indent":0,"type":"error","payload":"General verifier information from database could not be decoded."},{"index":100,"indent":0,"type":"error","payload":"No general verifier information in the database."},{"index":101,"indent":0,"type":"error","payload":"Network verifier is damaged and could not be decoded."},{"index":102,"indent":0,"type":"error","payload":"Network specs stored under key 0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e do not match it."},{"index":103,"indent":0,"type":"error","payload":"No verifier information corresponding to genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, however, genesis hash is encountered in network specs"},{"index":104,"indent":0,"type":"error","payload":"Different network names in database for same genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":105,"indent":0,"type":"error","payload":"Error setting stub into storage. This error should not be here."},{"index":106,"indent":0,"type":"error","payload":"Network westend is disabled. It could be enabled again only after complete wipe and re-installation of Signer."},{"index":107,"indent":0,"type":"error","payload":"Custom verifier for VerifierKey e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is same as general verifier."},{"index":108,"indent":0,"type":"error","payload":"System error. Balance printing failed."},{"index":109,"indent":0,"type":"error","payload":"System error. First characters in metadata are expected to be 0x6d657461."},{"index":110,"indent":0,"type":"error","payload":"System error. Metadata could not be decoded. Runtime metadata version is below 12."},{"index":111,"indent":0,"type":"error","payload":"Network metadata entry corrupted in database. Please remove the entry and download the metadata for this network."},{"index":112,"indent":0,"type":"error","payload":"System error. No version in metadata."},{"index":113,"indent":0,"type":"error","payload":"System error. Retrieved from metadata version constant could not be decoded."},{"index":114,"indent":0,"type":"error","payload":"System error. Unable to decode metadata."},{"index":115,"indent":0,"type":"error","payload":"System error. Unexpected regular expressions error."},{"index":116,"indent":0,"type":"error","payload":"Corrupted data. Bad signature."},{"index":117,"indent":0,"type":"error","payload":"Network westend current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."},{"index":118,"indent":0,"type":"error","payload":"Saved information for this network was signed by a verifier. Received information is not signed."},{"index":119,"indent":0,"type":"error","payload":"Different general verifier was used previously. Previously used public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Current attempt public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":120,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."},{"index":121,"indent":0,"type":"error","payload":"Network westend currently has no verifier set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata, first download properly verified network specs."},{"index":122,"indent":0,"type":"error","payload":"Network westend current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing verifier for the network would require wipe and reset of Signer."},{"index":123,"indent":0,"type":"error","payload":"Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."},{"index":124,"indent":0,"type":"error","payload":"Network westend is verified by the general verifier which currently is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."}]}"##;
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
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_not_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_not_signed";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_alice_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed";
        populate_cold(dbname, Verifier::None).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Network westend is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9000_already_in_db_alice_signed_known_general_verifier() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_known_general_verifier";
        populate_cold(dbname, Verifier::Sr25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_westend9000_already_in_db_alice_signed_bad_general_verifier() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_bad_general_verifier";
        populate_cold(dbname, Verifier::Ed25519(ALICE)).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Network westend is verified by the general verifier which currently is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
}
