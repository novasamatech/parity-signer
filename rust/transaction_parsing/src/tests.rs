use crate::{produce_output, Action, StubNav};
use constants::test_values::{
    alice_sr_alice, bob, ed, empty_vec_hash_pic, id_01, id_02, id_03, types_known, types_unknown,
    westend_9070,
};
use db_handling::{
    cold_default::{populate_cold, populate_cold_no_metadata, populate_cold_no_networks},
    manage_history::print_history,
};
use definitions::{
    crypto::Encryption,
    keyring::NetworkSpecsKey,
    network_specs::{Verifier, VerifierValue},
};
use sp_runtime::MultiSigner;
use std::fs;

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

fn verifier_alice_sr25519() -> Verifier {
    Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(
        sp_core::sr25519::Public::from_raw(ALICE),
    ))))
}

fn verifier_alice_ed25519() -> Verifier {
    Verifier(Some(VerifierValue::Standard(MultiSigner::Ed25519(
        sp_core::ed25519::Public::from_raw(ALICE),
    ))))
}

#[test]
fn add_specs_westend_no_network_info_not_signed() {
    let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
    populate_cold_no_networks(dbname, Verifier(None)).unwrap();
    let current_history = print_history(dbname).unwrap();
    assert!(
        current_history.contains(r#""events":[{"event":"database_initiated"}]"#),
        "Current history: \n{}",
        current_history
    );
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_not_signed() {
    let dbname = "for_tests/add_specs_westend_not_signed";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_not_signed_general_verifier_disappear() {
    let dbname = "for_tests/add_specs_westend_not_signed_general_verifier_disappear";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_not_signed() {
    let dbname = "for_tests/load_types_known_not_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Exactly same types information is already in the database."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_not_signed_general_verifier_disappear() {
    let dbname = "for_tests/load_types_known_not_signed_general_verifier_disappear";
    populate_cold_no_metadata(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed() {
    let dbname = "for_tests/load_types_known_alice_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&types_known(), r#"<types_known>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed_known_general_verifier() {
    let dbname = "for_tests/load_types_known_alice_signed_known_general_verifier";
    populate_cold_no_metadata(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Exactly same types information is already in the database."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed_bad_general_verifier() {
    let dbname = "for_tests/load_types_known_alice_signed_bad_general_verifier";
    populate_cold_no_metadata(dbname, verifier_alice_ed25519()).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received types information could be accepted only if verified by the same general verifier. Current message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed_metadata_hold() {
    let dbname = "for_tests/load_types_known_alice_signed_metadata_hold";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&types_known(), r#"<types_known>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_unknown_not_signed() {
    let dbname = "for_tests/load_types_unknown_not_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
    let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types","payload":{"types_hash":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574","types_id_pic":"<types_unknown>"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&types_unknown(), r#"<types_unknown>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_unknown_alice_signed() {
    let dbname = "for_tests/load_types_unknown_alice_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574","types_id_pic":"<types_unknown>"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&types_unknown(), r#"<types_unknown>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_westend_50_not_in_db() {
    let dbname = "for_tests/parse_transaction_westend_50_not_in_db";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003200000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9000)."}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_1() {
    let dbname = "for_tests/parse_transaction_1";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
    let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
    let output = produce_output(line, dbname);
    if let Action::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        let author_info_cut = author_info.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let content_cut = content.replace(&bob(), r#"<bob>"#);
        assert!(content_cut == content_known, "Received: \n{}", content);
        assert!(
            author_info_cut == author_info_known,
            "Received: \n{}",
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Received: \n{}",
            network_info
        );
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_2() {
    let dbname = "for_tests/parse_transaction_2";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d550210020c060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700b864d9450006050800aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d0608008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f501b4003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"2053656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a205468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a204d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a202d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e0a0a204966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a20627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a2023203c7765696768743e0a202d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a2023203c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"calls"},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"2054616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a20626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a206076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a20456d6974732060426f6e646564602e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a202d204f2831292e0a202d20546872656520657874726120444220656e74726965732e0a0a204e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a20756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a202d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c65646765722c205b4f726967696e204163636f756e745d2c2043757272656e74204572612c20486973746f72792044657074682c204c6f636b730a202d2057726974653a20426f6e6465642c2050617965652c205b4f726967696e204163636f756e745d2c204c6f636b732c204c65646765720a2023203c2f7765696768743e"}},{"index":5,"indent":5,"type":"varname","payload":"controller"},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>"}},{"index":8,"indent":5,"type":"varname","payload":"value"},{"index":9,"indent":6,"type":"balance","payload":{"amount":"300.000000000","units":"mWND"}},{"index":10,"indent":5,"type":"varname","payload":"payee"},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"204465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e20546869732063616e206f6e6c792062652063616c6c6564207768656e0a205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a20416e642c2069742063616e206265206f6e6c792063616c6c6564207768656e205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a2023203c7765696768743e0a202d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a2077686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a202d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a202d2d2d2d2d2d2d2d2d0a205765696768743a204f284e290a207768657265204e20697320746865206e756d626572206f6620746172676574730a204442205765696768743a0a202d2052656164733a2045726120456c656374696f6e205374617475732c204c65646765722c2043757272656e74204572610a202d205772697465733a2056616c696461746f72732c204e6f6d696e61746f72730a2023203c2f7765696768743e"}},{"index":14,"indent":5,"type":"varname","payload":"targets"},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"<id_01>"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"<id_02>"}},{"index":19,"indent":3,"type":"pallet","payload":"Staking"},{"index":20,"indent":4,"type":"method","payload":{"method_name":"set_controller","docs":"202852652d297365742074686520636f6e74726f6c6c6572206f6620612073746173682e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f206279207468652073746173682c206e6f742074686520636f6e74726f6c6c65722e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e20496e7369676e69666963616e7420636f6d706c65786974792e0a202d20436f6e7461696e732061206c696d69746564206e756d626572206f662072656164732e0a202d2057726974657320617265206c696d6974656420746f2074686520606f726967696e60206163636f756e74206b65792e0a202d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a202d2057726974653a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a2023203c2f7765696768743e"}},{"index":21,"indent":5,"type":"varname","payload":"controller"},{"index":22,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":23,"indent":7,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}}],"extensions":[{"index":24,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":25,"indent":0,"type":"nonce","payload":"45"},{"index":26,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":27,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":28,"indent":0,"type":"tx_version","payload":"5"},{"index":29,"indent":0,"type":"block_hash","payload":"314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
    let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
    let output = produce_output(line, dbname);
    if let Action::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        let author_info_cut = author_info.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let content_cut = content
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&bob(), r#"<bob>"#)
            .replace(&id_01(), r#"<id_01>"#)
            .replace(&id_02(), r#"<id_02>"#);
        assert!(content_cut == content_known, "Received: \n{}", content);
        assert!(
            author_info_cut == author_info_known,
            "Received: \n{}",
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Received: \n{}",
            network_info
        );
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_3() {
    let dbname = "for_tests/parse_transaction_3";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001750365010f00c06e31d910013223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ea8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cde143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"300.000000000000","units":"WND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"55","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"89"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"300.000000000000","units":"WND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
    let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
    let output = produce_output(line, dbname);
    if let Action::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        let author_info_cut = author_info.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let content_cut = content.replace(&bob(), r#"<bob>"#);
        assert!(content_cut == content_known, "Received: \n{}", content);
        assert!(
            author_info_cut == author_info_known,
            "Received: \n{}",
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Received: \n{}",
            network_info
        );
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn print_all_cards() {
    let dbname = "for_tests/print_all_cards";
    populate_cold_no_networks(dbname, Verifier(None)).unwrap();
    let line = "5300f0";
    let reply_known = r##""method":[{"index":0,"indent":0,"type":"pallet","payload":"test_pallet"},{"index":1,"indent":0,"type":"method","payload":{"method_name":"test_method","docs":"766572626f7365200a6465736372697074696f6e200a6f66200a746865200a6d6574686f64"}},{"index":2,"indent":0,"type":"varname","payload":"test_Varname"},{"index":3,"indent":0,"type":"default","payload":"12345"},{"index":4,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e0a557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e0a44756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e0a4578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"},{"index":5,"indent":0,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":6,"indent":0,"type":"none","payload":""},{"index":7,"indent":0,"type":"identity_field","payload":"Twitter"},{"index":8,"indent":0,"type":"bitvec","payload":"[0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1]"},{"index":9,"indent":0,"type":"balance","payload":{"amount":"300.000000","units":"KULU"}},{"index":10,"indent":0,"type":"field_name","payload":{"name":"test_FieldName","docs_field_name":"612076657279207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"7479706520697320646966666963756c7420746f206465736372696265"}},{"index":11,"indent":0,"type":"field_number","payload":{"number":"1","docs_field_number":"6c657373207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"74797065206973206a75737420617320646966666963756c7420746f206465736372696265"}},{"index":12,"indent":0,"type":"enum_variant_name","payload":{"name":"test_EnumVariantName","docs_enum_variant":""}},{"index":13,"indent":0,"type":"era","payload":{"era":"Immortal","phase":"","period":""}},{"index":14,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":15,"indent":0,"type":"nonce","payload":"15"},{"index":16,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":17,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":18,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9110"}},{"index":19,"indent":0,"type":"tx_version","payload":"5"},{"index":20,"indent":0,"type":"author","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>","seed":"Alice","derivation_path":"//Bob","has_pwd":false}},{"index":21,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":22,"indent":0,"type":"author_public_key","payload":{"public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","identicon":"<bob>","encryption":"sr25519"}},{"index":23,"indent":0,"type":"verifier","payload":{"public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","identicon":"<bob>","encryption":"sr25519"}},{"index":24,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9100","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","meta_id_pic":"<empty_vec_hash_pic>"}},{"index":25,"indent":0,"type":"types","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","types_id_pic":"<empty_vec_hash_pic>"}},{"index":26,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}},{"index":27,"indent":0,"type":"network_info","payload":{"network_title":"Westend","network_logo":"westend"}},{"index":28,"indent":0,"type":"network_genesis_hash","payload":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"},{"index":29,"indent":0,"type":"derivations","payload":["//Alice","//Alice/2/1","//secret//westend"]},{"index":30,"indent":0,"type":"warning","payload":"Transaction author public key not found."},{"index":31,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 50. Latest known available version is 9010."},{"index":32,"indent":0,"type":"warning","payload":"Public key is on record, but not associated with the network used."},{"index":33,"indent":0,"type":"warning","payload":"Received network information is not verified."},{"index":34,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."},{"index":35,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":36,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: none; affected metadata entries: none. Types information is purged."},{"index":37,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":38,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":39,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":40,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."},{"index":41,"indent":0,"type":"warning","payload":"Received network specs information for Westend is same as the one already in the database."},{"index":42,"indent":0,"type":"warning","payload":"Received metadata has incomplete set of signed extensions. As a result, Signer may be unable to parse signable transactions using this metadata."},{"index":43,"indent":0,"type":"error","payload":"Error on the interface. Network specs key 0xabracadabra is not in hexadecimal format."},{"index":44,"indent":0,"type":"error","payload":"Error on the interface. Input content is not in hexadecimal format."},{"index":45,"indent":0,"type":"error","payload":"Error on the interface. Address key 0xabracadabra is not in hexadecimal format."},{"index":46,"indent":0,"type":"error","payload":"Error on the interface. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 passed through the interface."},{"index":47,"indent":0,"type":"error","payload":"Error on the interface. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e passed through the interface."},{"index":48,"indent":0,"type":"error","payload":"Error on the interface. Public key length does not match the encryption."},{"index":49,"indent":0,"type":"error","payload":"Error on the interface. Requested history page 14 does not exist. Total number of pages 10."},{"index":50,"indent":0,"type":"error","payload":"Error on the interface. Expected seed name Alice for address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779. Address details in database have ALICE name."},{"index":51,"indent":0,"type":"error","payload":"Error on the interface. Derivation had password, then lost it."},{"index":52,"indent":0,"type":"error","payload":"Error on the interface. Version a505 could not be converted into u32."},{"index":53,"indent":0,"type":"error","payload":"Error on the interface. Increment a505 could not be converted into u32."},{"index":54,"indent":0,"type":"error","payload":"Error on the interface. Order a505 could not be converted into u32"},{"index":55,"indent":0,"type":"error","payload":"Error on the interface. Flag FALSE could not be converted into bool."},{"index":56,"indent":0,"type":"error","payload":"Database error. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."},{"index":57,"indent":0,"type":"error","payload":"Database error. Unable to parse history entry order 640455 from the database."},{"index":58,"indent":0,"type":"error","payload":"Database error. Unable to parse meta key 1c77657374656e64a2230000 from the database."},{"index":59,"indent":0,"type":"error","payload":"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from the database."},{"index":60,"indent":0,"type":"error","payload":"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from network id set of address book entry with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."},{"index":61,"indent":0,"type":"error","payload":"Database error. Internal error. Collection [1] does not exist"},{"index":62,"indent":0,"type":"error","payload":"Database error. Internal error. Unsupported: Something Unsupported."},{"index":63,"indent":0,"type":"error","payload":"Database error. Internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":64,"indent":0,"type":"error","payload":"Database error. Internal error. IO error: oh no!"},{"index":65,"indent":0,"type":"error","payload":"Database error. Internal error. Read corrupted data at file offset None backtrace ()"},{"index":66,"indent":0,"type":"error","payload":"Database error. Transaction error. Collection [1] does not exist"},{"index":67,"indent":0,"type":"error","payload":"Database error. Transaction error. Unsupported: Something Unsupported."},{"index":68,"indent":0,"type":"error","payload":"Database error. Transaction error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":69,"indent":0,"type":"error","payload":"Database error. Transaction error. IO error: oh no!"},{"index":70,"indent":0,"type":"error","payload":"Database error. Transaction error. Read corrupted data at file offset None backtrace ()"},{"index":71,"indent":0,"type":"error","payload":"Database error. Checksum mismatch."},{"index":72,"indent":0,"type":"error","payload":"Database error. Unable to decode address details entry for key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":73,"indent":0,"type":"error","payload":"Database error. Unable to decode current verifier entry for key 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."},{"index":74,"indent":0,"type":"error","payload":"Database error. Unable to decode danger status entry."},{"index":75,"indent":0,"type":"error","payload":"Database error. Unable to decode temporary entry with information needed to import derivations."},{"index":76,"indent":0,"type":"error","payload":"Database error. Unable to decode general verifier entry."},{"index":77,"indent":0,"type":"error","payload":"Database error. Unable to decode history entry for order 135."},{"index":78,"indent":0,"type":"error","payload":"Database error. Unable to decode network specs (NetworkSpecs) entry for key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":79,"indent":0,"type":"error","payload":"Database error. Unable to decode temporary entry with information needed for signing approved transaction."},{"index":80,"indent":0,"type":"error","payload":"Database error. Unable to decode temporary entry with information needed for accepting approved information."},{"index":81,"indent":0,"type":"error","payload":"Database error. Unable to decode types information."},{"index":82,"indent":0,"type":"error","payload":"Database error. Mismatch found. Meta key corresponds to westend1922. Stored metadata is westend9122."},{"index":83,"indent":0,"type":"error","payload":"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":84,"indent":0,"type":"error","payload":"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching encryption ecdsa."},{"index":85,"indent":0,"type":"error","payload":"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has not matching encryption ecdsa."},{"index":86,"indent":0,"type":"error","payload":"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has associated network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e with wrong encryption."},{"index":87,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."},{"index":88,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. No system pallet in runtime metadata."},{"index":89,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. No runtime version in system pallet constants."},{"index":90,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime version from system pallet constants could not be decoded."},{"index":91,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Base58 prefix is found in system pallet constants, but could not be decoded."},{"index":92,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Base58 prefix 104 from system pallet constants does not match the base58 prefix from network specs 42."},{"index":93,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Metadata vector does not start with 0x6d657461."},{"index":94,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime metadata could not be decoded."},{"index":95,"indent":0,"type":"error","payload":"Database error. No verifier information found for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, however genesis hash is encountered in network specs entry with key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":96,"indent":0,"type":"error","payload":"Database error. More than one entry for network specs with name westend and encryption sr25519."},{"index":97,"indent":0,"type":"error","payload":"Database error. Different network names (westend, WeStEnD) in database for same genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":98,"indent":0,"type":"error","payload":"Database error. Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption."},{"index":99,"indent":0,"type":"error","payload":"Database error. More than one seed key (i.e. with empty path and without password) found for seed name Alice and encryption sr25519."},{"index":100,"indent":0,"type":"error","payload":"Database error. More than one base58 prefix in network specs database entries for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: 42 and 104."},{"index":101,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `add_specs`."},{"index":102,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `load_meta`."},{"index":103,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `load_types`."},{"index":104,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as derivations transfer."},{"index":105,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."},{"index":106,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. No system pallet in runtime metadata."},{"index":107,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. No runtime version in system pallet constants."},{"index":108,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime version from system pallet constants could not be decoded."},{"index":109,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Base58 prefix is found in system pallet constants, but could not be decoded."},{"index":110,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Base58 prefix 104 from system pallet constants does not match the base58 prefix from network specs 42."},{"index":111,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Metadata vector does not start with 0x6d657461."},{"index":112,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime metadata could not be decoded."},{"index":113,"indent":0,"type":"error","payload":"Bad input data. Input is too short."},{"index":114,"indent":0,"type":"error","payload":"Bad input data. Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x35."},{"index":115,"indent":0,"type":"error","payload":"Bad input data. Payload type with code 0x0f is not supported."},{"index":116,"indent":0,"type":"error","payload":"Bad input data. Metadata for kusama9110 is already in the database and is different from the one in received payload."},{"index":117,"indent":0,"type":"error","payload":"Bad input data. Metadata for westend9122 is already in the database."},{"index":118,"indent":0,"type":"error","payload":"Bad input data. Similar network specs are already stored in the database under key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit)."},{"index":119,"indent":0,"type":"error","payload":"Bad input data. Network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e already has entries in the database with base58 prefix 42. Received network specs have different base58 prefix 104."},{"index":120,"indent":0,"type":"error","payload":"Bad input data. Payload with encryption 0x03 is not supported."},{"index":121,"indent":0,"type":"error","payload":"Bad input data. Received payload has bad signature."},{"index":122,"indent":0,"type":"error","payload":"Bad input data. Network kulupu is not in the database. Add network specs before loading the metadata."},{"index":123,"indent":0,"type":"error","payload":"Bad input data. Network westend was previously known to the database with verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata."},{"index":124,"indent":0,"type":"error","payload":"Bad input data. Saved network kulupu information was signed by verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Received information is not signed."},{"index":125,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier."},{"index":126,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier."},{"index":127,"indent":0,"type":"error","payload":"Bad input data. Network kulupu currently has no verifier set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. In order to accept verified metadata, first download properly verified network specs."},{"index":128,"indent":0,"type":"error","payload":"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing verifier for the network would require wipe and reset of Signer."},{"index":129,"indent":0,"type":"error","payload":"Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."},{"index":130,"indent":0,"type":"error","payload":"Bad input data. Network westend is verified by the general verifier which currently is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."},{"index":131,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received network westend specs could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":132,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received types information could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":133,"indent":0,"type":"error","payload":"Bad input data. Exactly same types information is already in the database."},{"index":134,"indent":0,"type":"error","payload":"Bad input data. Received message could not be read."},{"index":135,"indent":0,"type":"error","payload":"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519."},{"index":136,"indent":0,"type":"error","payload":"Bad input data. Input transaction is generated in network westend. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata."},{"index":137,"indent":0,"type":"error","payload":"Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database."},{"index":138,"indent":0,"type":"error","payload":"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."},{"index":139,"indent":0,"type":"error","payload":"Bad input data. Derivation // has invalid format."},{"index":140,"indent":0,"type":"error","payload":"Bad input data. Only derivations without passwords are allowed in bulk import."},{"index":141,"indent":0,"type":"error","payload":"Bad input data. Seed name Alice already exists."},{"index":142,"indent":0,"type":"error","payload":"Could not find current verifier for network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."},{"index":143,"indent":0,"type":"error","payload":"Could not find general verifier."},{"index":144,"indent":0,"type":"error","payload":"Could not find types information."},{"index":145,"indent":0,"type":"error","payload":"Could not find network specs for network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":146,"indent":0,"type":"error","payload":"Could not find network specs for westend."},{"index":147,"indent":0,"type":"error","payload":"Could not find network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e in address details with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":148,"indent":0,"type":"error","payload":"Could not find address details for address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":149,"indent":0,"type":"error","payload":"Could not find metadata entry for westend9120."},{"index":150,"indent":0,"type":"error","payload":"Could not find danger status information."},{"index":151,"indent":0,"type":"error","payload":"Could not find database temporary entry with information needed for accepting approved information."},{"index":152,"indent":0,"type":"error","payload":"Could not find database temporary entry with information needed for signing approved transaction."},{"index":153,"indent":0,"type":"error","payload":"Could not find database temporary entry with information needed for importing derivations set."},{"index":154,"indent":0,"type":"error","payload":"Could not find history entry with order 135."},{"index":155,"indent":0,"type":"error","payload":"Could not find network specs for westend with encryption ed25519 needed to decode historical transaction."},{"index":156,"indent":0,"type":"error","payload":"Historical transaction was generated in network kulupu and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata."},{"index":157,"indent":0,"type":"error","payload":"Unable to import derivations for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519. Network is unknown. Please add corresponding network specs."},{"index":158,"indent":0,"type":"error","payload":"Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd is disabled. It could be enabled again only after complete wipe and re-installation of Signer."},{"index":159,"indent":0,"type":"error","payload":"Error generating address. Address key collision for seed name Alice"},{"index":160,"indent":0,"type":"error","payload":"Error generating address. Tried to create colliding addresses within same network. Address for seed name Alice and path //01 has same public key as address for seed name Alice and path //1."},{"index":161,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid overall format."},{"index":162,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid bip39 phrase."},{"index":163,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid password."},{"index":164,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid seed."},{"index":165,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid seed length."},{"index":166,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid path."},{"index":167,"indent":0,"type":"error","payload":"Error generating address. Seed Alice already has derivation //Alice for network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, public key 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48."},{"index":168,"indent":0,"type":"error","payload":"Error generating address. Could not create random phrase. Seed phrase has invalid length."},{"index":169,"indent":0,"type":"error","payload":"Error generating address. Invalid derivation format."},{"index":170,"indent":0,"type":"error","payload":"Error generating qr code. Qr generation failed."},{"index":171,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to separate transaction method and extensions."},{"index":172,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected mortal transaction due to prelude format. Found immortal transaction."},{"index":173,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected immortal transaction due to prelude format. Found mortal transaction."},{"index":174,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Genesis hash values from decoded extensions and from used network specs do not match."},{"index":175,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Block hash for immortal transaction not matching genesis hash for the network."},{"index":176,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to decode extensions for V12/V13 metadata using standard extensions set."},{"index":177,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Method number 2 not found in pallet test_Pallet."},{"index":178,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Pallet with index 3 not found."},{"index":179,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. No calls found in pallet test_pallet_v14."},{"index":180,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Referenced type could not be resolved in v14 metadata."},{"index":181,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Argument type error."},{"index":182,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Argument name error."},{"index":183,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected compact. Not found it."},{"index":184,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Data too short for expected content."},{"index":185,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to decode part of data as u32."},{"index":186,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected Option<_> variant."},{"index":187,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. IdentityField description error."},{"index":188,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unexpected type encountered for Balance"},{"index":189,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected enum variant."},{"index":190,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unexpected type inside compact."},{"index":191,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. No description found for type SomeUnknownType."},{"index":192,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Declared type is not suitable BitStore type for BitVec."},{"index":193,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Declared type is not suitable BitOrder type for BitVec."},{"index":194,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Could not decode BitVec."},{"index":195,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Could not decode Era."},{"index":196,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the method some data remained unused."},{"index":197,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the extensions some data remained unused."},{"index":198,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is missing."},{"index":199,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash information is missing."},{"index":200,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version information is missing."},{"index":201,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is encountered mora than once."},{"index":202,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Genesis hash is encountered more than once."},{"index":203,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash is encountered more than once."},{"index":204,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version is encountered more than once."},{"index":205,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Network spec version decoded from extensions (9122) differs from the version in metadata (9010)."},{"index":206,"indent":0,"type":"error","payload":"Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9000)."},{"index":207,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid overall format."},{"index":208,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid bip39 phrase."},{"index":209,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid password."},{"index":210,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid seed."},{"index":211,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid seed length."},{"index":212,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid path."},{"index":213,"indent":0,"type":"error","payload":"Wrong password."},{"index":214,"indent":0,"type":"error","payload":"Wrong password."},{"index":215,"indent":0,"type":"error","payload":"No networks available. Please load networks information to proceed."}]"##;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply
            .replace(&bob(), r#"<bob>"#)
            .replace(&empty_vec_hash_pic(), r#"<empty_vec_hash_pic>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070_not_signed() {
    let dbname = "for_tests/load_westend9070_not_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
    let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec","meta_id_pic":"<meta_pic_westend9070>"}}]"#;
    let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&westend_9070(), r#"<meta_pic_westend9070>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070_alice_signed() {
    let dbname = "for_tests/load_westend9070_alice_signed";
    populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_not_signed() {
    let dbname = "for_tests/load_westend9000_already_in_db_not_signed";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_None.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Metadata for westend9000 is already in the database."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_alice_signed() {
    let dbname = "for_tests/load_westend9000_already_in_db_alice_signed";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_alice_signed_known_general_verifier() {
    let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_known_general_verifier";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Metadata for westend9000 is already in the database."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_alice_signed_bad_general_verifier() {
    let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_bad_general_verifier";
    populate_cold(dbname, verifier_alice_ed25519()).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network westend is verified by the general verifier which currently is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_dock31_unknown_network() {
    let dbname = "for_tests/load_dock31_unknown_network";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network dock-pos-main-runtime is not in the database. Add network specs before loading the metadata."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_not_verified_db_not_verified() {
    let dbname = "for_tests/add_specs_dock_not_verified_db_not_verified";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_alice_verified_db_not_verified() {
    let dbname = "for_tests/add_specs_dock_alice_verified_db_not_verified";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_not_verified_db_alice_verified() {
    let dbname = "for_tests/add_specs_dock_not_verified_db_alice_verified";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_both_verified_same() {
    let dbname = "for_tests/add_specs_dock_both_verified_same";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_both_verified_different() {
    let dbname = "for_tests/add_specs_dock_both_verified_different";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed25519>","encryption":"ed25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&ed(), r#"<ed25519>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_signed() {
    let dbname = "for_tests/add_specs_westend_ed25519_not_signed";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let reply_known = r##""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Ed25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_bad_westend_ed25519_not_signed() {
    let dbname = "for_tests/add_specs_bad_westend_ed25519_not_signed";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified_bad_ones.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e already has entries in the database with base58 prefix 42. Received network specs have different base58 prefix 115."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_alice_signed_db_not_verified() {
    let dbname = "for_tests/add_specs_westend_ed25519_alice_signed_db_not_verified";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Ed25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_verified_db_alice_verified() {
    let dbname = "for_tests/add_specs_westend_ed25519_not_verified_db_alice_verified";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_both_verified_same() {
    let dbname = "for_tests/add_specs_westend_ed25519_both_verified_same";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Ed25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
        assert!(stub_nav == stub_nav_known, "Received: \n{:?}", stub_nav);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_both_verified_different() {
    let dbname = "for_tests/add_specs_westend_ed25519_both_verified_different";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-ed25519.txt").unwrap();
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received network westend specs could be accepted only if verified by the same general verifier. Current message is verified by public key: 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee, encryption: ed25519."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_4_unknown_author() {
    let dbname = "for_tests/parse_transaction_4_unknown_author";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "5301008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48a4040300d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Transaction author public key not found."}],"method":[{"index":2,"indent":0,"type":"pallet","payload":"Balances"},{"index":3,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":4,"indent":2,"type":"varname","payload":"dest"},{"index":5,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":6,"indent":4,"type":"Id","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>"}},{"index":7,"indent":2,"type":"varname","payload":"value"},{"index":8,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":9,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":10,"indent":0,"type":"nonce","payload":"46"},{"index":11,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":12,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":13,"indent":0,"type":"tx_version","payload":"5"},{"index":14,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&bob(), r#"<bob>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_5_unknown_network() {
    let dbname = "for_tests/parse_transaction_5_unknown_network";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530102761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62a8030300761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c620b00407a10f35aa707000b00a0724e1809140000000a000000f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769badc21d36b69bae1e8a41dedb34758567ba4efe711412f33d1461f795ffcd1de13f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author_public_key","payload":{"public_key":"761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62","identicon":"<id_03>","encryption":"sr25519"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba and encryption sr25519."}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply.replace(&id_03(), r#"<id_03>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_6_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_6_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403018eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the method some data remained unused."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_7_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_7_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403068eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected enum variant."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_8_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_8_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403028eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Data too short for expected content."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        let reply_cut = reply.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(reply_cut == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_1() {
    let dbname = "for_tests/parse_msg_1";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27df5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let content_known = r#""message":[{"index":0,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
    let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
    let output = produce_output(line, dbname);
    if let Action::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        let author_info_cut = author_info.replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        assert!(content == content_known, "Received: \n{}", content);
        assert!(
            author_info_cut == author_info_known,
            "Received: \n{}",
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Received: \n{}",
            network_info
        );
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_2() {
    let dbname = "for_tests/parse_msg_2";
    populate_cold(dbname, Verifier(None)).unwrap();
    // sneaking one extra byte in the text body
    let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27df5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c6c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Received message could not be read."}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(reply == reply_known, "Received: \n{}", reply);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn import_derivations() {
    let dbname = "for_tests/import_derivations";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "53ffde01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e141c2f2f416c6963653c2f2f416c6963652f77657374656e64582f2f416c6963652f7365637265742f2f7365637265740c2f2f300c2f2f31";
    let content_known = r#""importing_derivations":[{"index":0,"indent":0,"type":"derivations","payload":["//Alice","//Alice/westend","//Alice/secret//secret","//0","//1"]}]"#;
    let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;
    let output = produce_output(line, dbname);
    if let Action::Derivations {
        content,
        network_info,
        checksum: _,
        network_specs_key: _,
    } = output
    {
        assert!(content == content_known, "Received: \n{}", content);
        assert!(
            network_info == network_info_known,
            "Received: \n{}",
            network_info
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}
