/// Separated new cold test databases are created during the tests,
/// and removed after test is performed, so the test can run in parallel
use crate::{produce_output, Action, StubNav};
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types","payload":{"types_hash":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035d49444154789cedd8bd8d144110c5712e84732e0d4c086163c1048b08b0c024960d014cd238674358fa6955a7b74d4d7d74578f5662fec69eb1a3ee373fe9a483a7ebf5faee7f6f1784cbe5327cc9f3f3f353fbb1b42508332fedb502a51461e5cbf7556294208cbcfcd71fa7f679dfb7cfe7f699ab02630a61e4e59106208d40a0198c61841500d2de104308a3006825021a814821ccbcbcb41a41ca6084112a00d05e08280a1142c800fcfafea97ddef7f1cbcff6792b8be09de7158170116601241e6e417800129fe7e54194215883251eae414401243ecf6a0a210a802a47a3eaf32c884d840c00aa1e5d7d1eda825011b200a87a74f57992061146a8fe1d3ebffe6e9ff79d5e3eb4cf5bd9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e8638105a6f081a001abd64ab47414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e76d058803e1404822787fb96547ff39bfb6cffbde9f5edae7adec79debeadc208da05125f640de7c11a801485e0f3a2fbb44208d605125fa40de7c11680e441f079d97d7d4b10bcb2085eb3fb0e84d681d05a82e0fd0e6711bcf3b2fbfa4208c8ba882fd0064b3cdc82f000243e2fba4f2b8c80b48bf8026bb0c4c335882880c4e779fbb64a217865477b559fb7d581d03a105a6f08c883f0febd9e1dad3def7ddfc7cf7bfbb400d07edcfe3f015908da05125f640de7c1d5cf45f7f58511ac0b24be481bce83b5effbbce7f9fbec3e6e198297f6527dfc925e33fbfe41401ac4cc255a8f822000e84068dd21a01e227b89f7975b16c13b2fbb0f310072119075115fa00d9678b805e101487c5e749fe422a028045f600d9678b806110590f83c6f9fd403a030825776b457f5795218016521aa47579f873400b489803210d5a3abcfdb024026028a4264477bbfc3d9f3ac2c00548680ace13c580390a2107c9ed734029a85e0c11680e441f0795e1e000a21a00c84551661a608000a23a00a88bd10a200288520cd60ac46c8bcbc348480462156228c00a06104b402626f003485208d6068102300332f2f9520482318a355bcbc548a20adc4a87c79690942df0cca8a97eedb05e1d1fb0b150fbfaccbc005380000000049454e44ae426082"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r#""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types","payload":{"types_hash":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035d49444154789cedd8bd8d144110c5712e84732e0d4c086163c1048b08b0c024960d014cd238674358fa6955a7b74d4d7d74578f5662fec69eb1a3ee373fe9a483a7ebf5faee7f6f1784cbe5327cc9f3f3f353fbb1b42508332fedb502a51461e5cbf7556294208cbcfcd71fa7f679dfb7cfe7f699ab02630a61e4e59106208d40a0198c61841500d2de104308a3006825021a814821ccbcbcb41a41ca6084112a00d05e08280a1142c800fcfafea97ddef7f1cbcff6792b8be09de7158170116601241e6e417800129fe7e54194215883251eae414401243ecf6a0a210a802a47a3eaf32c884d840c00aa1e5d7d1eda825011b200a87a74f57992061146a8fe1d3ebffe6e9ff79d5e3eb4cf5bd9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e8638105a6f081a001abd64ab47414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e76d058803e1404822787fb96547ff39bfb6cffbde9f5edae7adec79debeadc208da05125f640de7c11a801485e0f3a2fbb44208d605125fa40de7c11680e441f079d97d7d4b10bcb2085eb3fb0e84d681d05a82e0fd0e6711bcf3b2fbfa4208c8ba882fd0064b3cdc82f000243e2fba4f2b8c80b48bf8026bb0c4c335882880c4e779fbb64a217865477b559fb7d581d03a105a6f08c883f0febd9e1dad3def7ddfc7cf7bfbb400d07edcfe3f015908da05125f640de7c1d5cf45f7f58511ac0b24be481bce83b5effbbce7f9fbec3e6e198297f6527dfc925e33fbfe41401ac4cc255a8f822000e84068dd21a01e227b89f7975b16c13b2fbb0f310072119075115fa00d9678b805e101487c5e749fe422a028045f600d9678b806110590f83c6f9fd403a030825776b457f5795218016521aa47579f873400b489803210d5a3abcfdb024026028a4264477bbfc3d9f3ac2c00548680ace13c580390a2107c9ed734029a85e0c11680e441f0795e1e000a21a00c84551661a608000a23a00a88bd10a200288520cd60ac46c8bcbc348480462156228c00a06104b402626f003485208d6068102300332f2f9520482318a355bcbc548a20adc4a87c79690942df0cca8a97eedb05e1d1fb0b150fbfaccbc005380000000049454e44ae426082"}}]"#;
    let stub_nav_known = StubNav::LoadTypes;
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
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
        assert!(
            content == content_known,
            "Expected: {}\nReceived: {}",
            content_known,
            content
        );
        assert!(
            author_info == author_info_known,
            "Expected: {}\nReceived: {}",
            author_info_known,
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Expected: {}\nReceived: {}",
            network_info_known,
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
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"2053656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a205468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a204d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a202d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e0a0a204966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a20627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a2023203c7765696768743e0a202d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a2023203c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"calls"},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"2054616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a20626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a206076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a20456d6974732060426f6e646564602e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a202d204f2831292e0a202d20546872656520657874726120444220656e74726965732e0a0a204e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a20756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a202d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c65646765722c205b4f726967696e204163636f756e745d2c2043757272656e74204572612c20486973746f72792044657074682c204c6f636b730a202d2057726974653a20426f6e6465642c2050617965652c205b4f726967696e204163636f756e745d2c204c6f636b732c204c65646765720a2023203c2f7765696768743e"}},{"index":5,"indent":5,"type":"varname","payload":"controller"},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082"}},{"index":8,"indent":5,"type":"varname","payload":"value"},{"index":9,"indent":6,"type":"balance","payload":{"amount":"300.000000000","units":"mWND"}},{"index":10,"indent":5,"type":"varname","payload":"payee"},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"204465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e20546869732063616e206f6e6c792062652063616c6c6564207768656e0a205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a20416e642c2069742063616e206265206f6e6c792063616c6c6564207768656e205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a2023203c7765696768743e0a202d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a2077686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a202d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a202d2d2d2d2d2d2d2d2d0a205765696768743a204f284e290a207768657265204e20697320746865206e756d626572206f6620746172676574730a204442205765696768743a0a202d2052656164733a2045726120456c656374696f6e205374617475732c204c65646765722c2043757272656e74204572610a202d205772697465733a2056616c696461746f72732c204e6f6d696e61746f72730a2023203c2f7765696768743e"}},{"index":14,"indent":5,"type":"varname","payload":"targets"},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034749444154789cedd8bf6a145114c771439e20422a6111acac7c013b11d2d82c089217f1197c912008dbd804c4ce17b0b21264c12ae03e4158ef8fe12433933bf7f73bf7cfb049e65b9c2dee85ddf32986658ef6fbfd93c7de2c08bbdd2efb4b4e4e4e8ec247d39a20942ccd6a815215a1e5f2e36a625441c8597ebd7a1de6b0cdf64798be6a601421e42c8f6200560e042ac1c846680160cd0d9185900b805a22a01c081742c9f2566b04cb832123d40040732120154242f0007cfcfc2acc619f3efc0cb3cb8bb07eb10a73d8e6f7364c2d0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6212c103800e1d014d414411bc00e83e20a018848cb07e7bf7476dbeddfe082fc2fbc8fd2fbd732f027bc65812820a60a9100cc052211880a54050841480c52054008b41a800d61822893006405e04961781958380fa100b42e8062106801e2a023288ea086fd677ef7fdfdc9e7b1162f7fbe7b320a0140403b05488fe82eabd14c414007221a018840a603188fe62b1f371fdfb318814001a203000252f02cb8b901b20168405c18970f5e72ccc61a7cf2fc3ecf222fc3dbb0a73d8b3cbd330bbbc08ec9fea5432420cc052211880a54230008b41480829008b41a80016835001ac14441304961781b520841684d04122ac572fc31cb6d9fe0ab3cb8bc09e31b320a0140403b054080660a91029002423a018840a60310815c062100c00b910585e04961721b70521b420846e101083b8fe7711e6b0e3a7e76176791162f7fbe75e04f6be231600c247f73e01a5106200960ad15f50bd97826000560a42464801580ca2bf58ec7c5cff7e0c4205b0a6209a21b0bc08acaa082806f150110c002d08a101021a4378112eaebf8639ecfcf85d985d5e04fa4f3503a10f8028024a4130004b856000960a3106401401a9102a80c52054008b4128004846607911585e043519017921ee03420c004d22200fc4a1234c01a024025221bc08ec195313210580aa21a0140403b0540815001523a0520815c0621035019084803c10a9bc082529004846403520e6425001900bc12ac1688de059deca4240b9102d1172005036026a013137002a42b072306210390025cb5b5510ac1c8cdc6a2c6f5545b05a62d45cde6a8230ae04a5c5d2e3664138f4fe038c73bfacab7b7c0d0000000049454e44ae426082"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000036249444154789cedd8c16d14411085611c82e320073871742e48201f0880830512b9f8c80972200e8760fa695556755353afaabb7a6589f90febc38ebadf7c9225c3cdf3f3f39bffbdab203c3d3d4d5f727b7b7bd37e6c6d0bc2ca4bb376a09422ec7cf9b14a8c128499977fb8fbd23efbee1fbfb6cf5c15184b08332f8f2c00690602ad604c23ec0090ae0d3185300b807622a0198814c2cacb4bbb11a40c4618a102005d0b014521420819804fbf3fb6cfbeefef7eb4cf4b5904761e2b0241115601243ddc836000923e8fc520ca10bcc1921e6e414401247d9ed712421400558e46d5e7791087081900543dbafa3c74046122640150f5e8eaf3240b228c50fd3bfcf0f9ae7df6dd7f7b6c9f97d2e7917d5208210a20e98bbce1dd6003408a4274e705f749230445f02e90f445d6f06eb003203188eebce43ee4228c0068e612af2c026b769f8638115a2f0816009abde4a8d7828004a21cc17a49fd52d6f763ecf9eefbe43e5d18017917e90bacc15237bcfab9e0beb11402b22ed2177883a56eb8f13cfb7eac7b9eecb3ea101840a4ec6856f5794701e24438119208f42fb7e4e80fefdfb6cfbe9fbffeb4cf4bd9f3d8bea3c208d60592bec81bae075b005214429f17dd671542f02e90f445d6703dd8039018843e2fbb6f6c0b022b8bc05add7722b44e84d61604f63b9c4560e765f78d8510907791bec01a2ce9e11e040390f479d17d566104645da42ff0064b7ab805110590f4796cdf512904567634abfabca34e84d689d07a41400c82fd7b3d3bda7a9e7d3fd63d4ff65901a0fdb8fc7f02f210ac0b247d9137bc1b5cfd5c70df5818c1bb40d21759c3bbc1c6f763ecf9eefbe43edd360496f55263fa25592bfbfe414016c4ca2556af054100d089d0ea10d00891bd84fde59645a0e725f7210d802802f22ed2175883a56eb803c100a4eebce03e8922a02884bec01b2c75c30d882880d49d47f64923000a23b0b2a359d5e74961049485a81e5d7d1eb200d02102ca40548fae3eef0800b908280a911dcd7e87b3e7797900a80c0179c3f5600b408a42e8f358cb086815420ff6002406a1cf6331001442401908af2cc24a11001446401510d7428802a01482b482b11b21f3f2d214029a85d889300380a611d00e886b03a025046906c3829801587979a904419ac198ade2e5a55204692746e5cb4b5b10c6565076bcf4d855105e7b7f014ae9bfacbb93aaab0000000049454e44ae426082"}},{"index":19,"indent":3,"type":"pallet","payload":"Staking"},{"index":20,"indent":4,"type":"method","payload":{"method_name":"set_controller","docs":"202852652d297365742074686520636f6e74726f6c6c6572206f6620612073746173682e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f206279207468652073746173682c206e6f742074686520636f6e74726f6c6c65722e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e20496e7369676e69666963616e7420636f6d706c65786974792e0a202d20436f6e7461696e732061206c696d69746564206e756d626572206f662072656164732e0a202d2057726974657320617265206c696d6974656420746f2074686520606f726967696e60206163636f756e74206b65792e0a202d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a202d2057726974653a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a2023203c2f7765696768743e"}},{"index":21,"indent":5,"type":"varname","payload":"controller"},{"index":22,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":23,"indent":7,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}}],"extensions":[{"index":24,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":25,"indent":0,"type":"nonce","payload":"45"},{"index":26,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":27,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":28,"indent":0,"type":"tx_version","payload":"5"},{"index":29,"indent":0,"type":"block_hash","payload":"314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
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
        assert!(
            content == content_known,
            "Expected: {}\nReceived: {}",
            content_known,
            content
        );
        assert!(
            author_info == author_info_known,
            "Expected: {}\nReceived: {}",
            author_info_known,
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Expected: {}\nReceived: {}",
            network_info_known,
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
    let content_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":2,"indent":2,"type":"varname","payload":"dest"},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":5,"indent":2,"type":"varname","payload":"value"},{"index":6,"indent":3,"type":"balance","payload":{"amount":"300.000000000000","units":"WND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"55","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"89"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"300.000000000000","units":"WND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"}]"#;
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
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
        assert!(
            content == content_known,
            "Expected: {}\nReceived: {}",
            content_known,
            content
        );
        assert!(
            author_info == author_info_known,
            "Expected: {}\nReceived: {}",
            author_info_known,
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Expected: {}\nReceived: {}",
            network_info_known,
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
    let reply_known = r##""method":[{"index":0,"indent":0,"type":"pallet","payload":"test_pallet"},{"index":1,"indent":0,"type":"method","payload":{"method_name":"test_method","docs":"766572626f7365200a6465736372697074696f6e200a6f66200a746865200a6d6574686f64"}},{"index":2,"indent":0,"type":"varname","payload":"test_Varname"},{"index":3,"indent":0,"type":"default","payload":"12345"},{"index":4,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e0a557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e0a44756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e0a4578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"},{"index":5,"indent":0,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":6,"indent":0,"type":"none","payload":""},{"index":7,"indent":0,"type":"identity_field","payload":"Twitter"},{"index":8,"indent":0,"type":"bitvec","payload":"[00000100, 00100000, 11011001]"},{"index":9,"indent":0,"type":"balance","payload":{"amount":"300.000000","units":"KULU"}},{"index":10,"indent":0,"type":"field_name","payload":{"name":"test_FieldName","docs_field_name":"612076657279207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"7479706520697320646966666963756c7420746f206465736372696265"}},{"index":11,"indent":0,"type":"field_number","payload":{"number":"1","docs_field_number":"6c657373207370656369616c206669656c64","path_type":"field >> path >> TypePath","docs_type":"74797065206973206a75737420617320646966666963756c7420746f206465736372696265"}},{"index":12,"indent":0,"type":"enum_variant_name","payload":{"name":"test_EnumVariantName","docs_enum_variant":""}},{"index":13,"indent":0,"type":"era","payload":{"era":"Immortal","phase":"","period":""}},{"index":14,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"31","period":"64"}},{"index":15,"indent":0,"type":"nonce","payload":"15"},{"index":16,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":17,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":18,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9110"}},{"index":19,"indent":0,"type":"tx_version","payload":"5"},{"index":20,"indent":0,"type":"author","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false}},{"index":21,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}},{"index":22,"indent":0,"type":"author_public_key","payload":{"public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082","encryption":"sr25519"}},{"index":23,"indent":0,"type":"verifier","payload":{"public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082","encryption":"sr25519"}},{"index":24,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9100","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000039f49444154789cedd83d6e14411086619c12312b4bbe049903c8380101e2063ec31e63cfe01b20024e400681332e61c9da2122355d1a7d76cd6c4dfd7457af2c79dea01dd8eafee6097df1f8f8f8e6b5771684711cab1f1986e1a2fce85a1784968fb6ea81928ad0f3e3976562a420d47cfc707728e7bcf17a5fce5819184d08351f4f4900a806826ac1a846e80180ce0d5185500b40f544a06a2042082d1f8f7a23a008861b2103803a1702e5857021440086b757e59c37febb2fe75414c1bacfca036122b402203e5c83b00010bfcfca824843d006233e5c82f002207e9f5613821780ca1c4da5dfa740ac224400a8f4d1c9f7516b1022421480ca1e9d7d1f9220dc08bbaf37e59c77fc765bcea9e8e81fc3e9df7f1e9f7f1fbdcfda875c085e00c41fd286f3c11200f242f0fbbcfbd012c244d01e40fc2169381fac01200b82df17dd47a9084b00aae611ad288255ed3e0eb121949e102400aaf691b55e0a0205887484c3f0ab9cf3f6e3c7724e4511acfba2fb786e044a7b883f200d467cb8066101207e9f77dfb21002253dc41fd006233e5c82f002207e9fb54f6a866001788a8eb6cabe6f2d82d810368420c2cdee4339e7dd1e7f97732a3afa30fc29e7bcfdf8be9c53d1fbac7d6bb911a407107f481bce074b00c80bc1eff3ee937221680f20fe90349c0fd6009005c1ef8bee5bd605c12a8a60d5ba6f43286d08a52e08c3e15d39e78dfbbfe59c8a2258f745f72d732150da43fc016930e2c335080b00f1fbbcfba4dc0894f4107f401b8cf87009c20b80f87dd6beb5420856d1d156d9f7adb521943684d213026541dcedbe9773def5f14b39a7a2a3af1e8672cebbbf1ccb3915bdcfda274500e5c7f4ff044a43901e40fc216d381f2c01202f04bfcfbb6f991b417b00f187a4e17cb006802c087e5f741faf1b825514c1aa65df09022541b43c22f5521000406d08a51902b584883ef26977fa5fdf9fc7e7fffa4611acfba2fb280e40990894f6107f401a8cf8700dc20240fc3eef3e6422505e08fe803618f1e112841700f1fbac7d680940b911aca2a3adb2ef436e042a0a913d3afb3e4a02a05611a80844f6e8ecfbd600281581f2424447ef2e4ffffef8f0fcfbe87d5a1a0095864069c3f96009007921f87d56cd08542b041fac01200b82df676501502e042a02a1154568c90340b911a80c8873217801a810026ac1e88d10f978548540d542f444a801a0aa11a81e10e706a09a10500d86045103d0f2f1280501d560d496f1f1281501f5c4c8fc78d40561590b4a8f8f5e76168497de7fa22fbfac7f3444800000000049454e44ae426082"}},{"index":25,"indent":0,"type":"types","payload":{"types_hash":"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c945000003a649444154789cedd83f8a544110c771f708cbe0190c1613cf30a032b1998960244c249e428c068c041333e341853983896ce0196498238c553c6aad7ed6ab3fddd58f817ddfa037ebfebd0fec067b753e9f1fdcf76641389d4ed58f5c5f5f5fc18fae754168f968ab1e28a9083d3f7e5c26460a42cdc7ef3eace12cdbbe3dc0192b03a309a1e6e3310980aa81c05a30aa117a005073435421d402603d11b01a881042cbc753bd11a808861b2103009b0b01f342b8102200c7fd2d9c65abcd0d9c435104eb3e2b0f8489d00a40f1e11a840540f1fbac2c8834046d30c5874b105e008adfa7d584e005c0324763d9f76910930811002c7b74f67dd81484881005c0b24767df4749106e843ffb239c650f372b3887a2a377af84bf099febff2658fb2817821780e20f69c3f9600980f242f0fbbcfba8318489a03d40f187a4e17cb006405910fcbee83e4c45180360358f684511ac6af771880501ba439000b0da47a6ba14048c20d2115eac7770967d3d6ce11c8a2258f745f7f1dc0898f6107f401a4cf1e11a840540f1fbbcfbc6851030e921fe803698e2c325082f00c5efb3f64915081680a7e868abecfba642880561410822ec7f1ce12cdb3c5dc139141dbd5bbf81b36c7bf808e750f43e6bdf546e04e9018a3fa40de7832500ca0bc1eff3ee937221680f50fc2169381fac01501604bf2fba6f5c1704ab288255ebbe05015a10a02e08378f9ec35976fbfb1b9c435104ebbee8be712e044c7b883f200da6f8700dc202a0f87dde7d526e044c7a883fa00da6f87009c20b40f1fbac7d538510aca2a3adb2ef9b6a41801604e80e01b320f6bf84dfb9c7ff7ee7a2a3d7bb6770961db6dfe11c8ade67ed934200f831fc3f01d310a40728fe90369c0f9600282f04bfcfbb6f9c1b417b80e20f49c3f9600d80b220f87dd17dbc6e08565104ab967dff21601244cb235297824000d882001508d81822fac8979fafe12c7bf9e4139c435104ebbee83e8c03602602a63dc41f9006537cb806610150fc3eef3eca44c0bc10fc016d30c5874b105e008adf67eda3c600981bc12a3ada2afb3eca8d804521b24767df874900d824021681c81e9d7ddf1400a622605e88e8e8f5ee3d9c6587ed3b3887a2f7696900581a02a60de7832500ca0bc1efb36a46c05a21f8600d80b220f87d561600e642c022105a5184963c00981b01cb80980bc10b808510a8168cde08918fa7aa10b05a889e083500583502d603626e00ac0981aac190206a005a3e9e4a41a06a306acbf8782a1581ea8991f9f1541784712d283d3e7adc2c0897de5fe32ebfac15a8b9430000000049454e44ae426082"}},{"index":26,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}},{"index":27,"indent":0,"type":"network_info","payload":{"network_title":"Westend","network_logo":"westend"}},{"index":28,"indent":0,"type":"network_genesis_hash","payload":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"},{"index":29,"indent":0,"type":"derivations","payload":["//Alice","//Alice/2/1","//secret//westend"]},{"index":30,"indent":0,"type":"warning","payload":"Transaction author public key not found."},{"index":31,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 50. Latest known available version is 9010."},{"index":32,"indent":0,"type":"warning","payload":"Public key is on record, but not associated with the network used."},{"index":33,"indent":0,"type":"warning","payload":"Received network information is not verified."},{"index":34,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."},{"index":35,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":36,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: none; affected metadata entries: none. Types information is purged."},{"index":37,"indent":0,"type":"warning","payload":"Received message is verified by the general verifier. Current verifier for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":38,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":39,"indent":0,"type":"warning","payload":"Received message is verified. Currently no verifier is set for network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: none; affected metadata entries: none."},{"index":40,"indent":0,"type":"warning","payload":"Received types information is identical to the one that was in the database."},{"index":41,"indent":0,"type":"warning","payload":"Received network specs information for Westend is same as the one already in the database."},{"index":42,"indent":0,"type":"warning","payload":"Received metadata has incomplete set of signed extensions. As a result, Signer may be unable to parse signable transactions using this metadata."},{"index":43,"indent":0,"type":"error","payload":"Error on the interface. Network specs key 0xabracadabra is not in hexadecimal format."},{"index":44,"indent":0,"type":"error","payload":"Error on the interface. Input content is not in hexadecimal format."},{"index":45,"indent":0,"type":"error","payload":"Error on the interface. Address key 0xabracadabra is not in hexadecimal format."},{"index":46,"indent":0,"type":"error","payload":"Error on the interface. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 passed through the interface."},{"index":47,"indent":0,"type":"error","payload":"Error on the interface. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e passed through the interface."},{"index":48,"indent":0,"type":"error","payload":"Error on the interface. Public key length does not match the encryption."},{"index":49,"indent":0,"type":"error","payload":"Error on the interface. Requested history page 14 does not exist. Total number of pages 10."},{"index":50,"indent":0,"type":"error","payload":"Database error. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."},{"index":51,"indent":0,"type":"error","payload":"Database error. Unable to parse history entry order 640455 from the database."},{"index":52,"indent":0,"type":"error","payload":"Database error. Unable to parse meta key 1c77657374656e64a2230000 from the database."},{"index":53,"indent":0,"type":"error","payload":"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from the database."},{"index":54,"indent":0,"type":"error","payload":"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from network id set of address book entry with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."},{"index":55,"indent":0,"type":"error","payload":"Database error. Internal error. Collection [1] does not exist"},{"index":56,"indent":0,"type":"error","payload":"Database error. Internal error. Unsupported: Something Unsupported."},{"index":57,"indent":0,"type":"error","payload":"Database error. Internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":58,"indent":0,"type":"error","payload":"Database error. Internal error. IO error: oh no!"},{"index":59,"indent":0,"type":"error","payload":"Database error. Internal error. Read corrupted data at file offset None backtrace ()"},{"index":60,"indent":0,"type":"error","payload":"Database error. Transaction error. Collection [1] does not exist"},{"index":61,"indent":0,"type":"error","payload":"Database error. Transaction error. Unsupported: Something Unsupported."},{"index":62,"indent":0,"type":"error","payload":"Database error. Transaction error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":63,"indent":0,"type":"error","payload":"Database error. Transaction error. IO error: oh no!"},{"index":64,"indent":0,"type":"error","payload":"Database error. Transaction error. Read corrupted data at file offset None backtrace ()"},{"index":65,"indent":0,"type":"error","payload":"Database error. Checksum mismatch."},{"index":66,"indent":0,"type":"error","payload":"Database error. Unable to decode address details entry for key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":67,"indent":0,"type":"error","payload":"Database error. Unable to decode current verifier entry for key 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."},{"index":68,"indent":0,"type":"error","payload":"Database error. Unable to decode danger status entry."},{"index":69,"indent":0,"type":"error","payload":"Database error. Unable to decode general verifier entry."},{"index":70,"indent":0,"type":"error","payload":"Database error. Unable to decode history entry for order 135."},{"index":71,"indent":0,"type":"error","payload":"Database error. Unable to decode network specs (NetworkSpecs) entry for key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":72,"indent":0,"type":"error","payload":"Database error. Unable to decode temporary entry with information needed for signing approved transaction."},{"index":73,"indent":0,"type":"error","payload":"Database error. Unable to decode temporary entry with information needed for accepting approved information."},{"index":74,"indent":0,"type":"error","payload":"Database error. Unable to decode types information."},{"index":75,"indent":0,"type":"error","payload":"Database error. Mismatch found. Meta key corresponds to westend1922. Stored metadata is westend9122."},{"index":76,"indent":0,"type":"error","payload":"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":77,"indent":0,"type":"error","payload":"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching encryption ecdsa."},{"index":78,"indent":0,"type":"error","payload":"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has not matching encryption ecdsa."},{"index":79,"indent":0,"type":"error","payload":"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has associated network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e with wrong encryption."},{"index":80,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."},{"index":81,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. No system pallet in runtime metadata."},{"index":82,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. No runtime version in system pallet constants."},{"index":83,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime version from system pallet constants could not be decoded."},{"index":84,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Metadata vector does not start with 0x6d657461."},{"index":85,"indent":0,"type":"error","payload":"Database error. Bad metadata for westend9000. Runtime metadata could not be decoded."},{"index":86,"indent":0,"type":"error","payload":"Database error. No verifier information found for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, however genesis hash is encountered in network specs entry with key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":87,"indent":0,"type":"error","payload":"Database error. More than one entry for network specs with name westend and encryption sr25519."},{"index":88,"indent":0,"type":"error","payload":"Database error. Different network names (westend, WeStEnD) in database for same genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":89,"indent":0,"type":"error","payload":"Database error. Entry with order 135 contains more than one transaction-related event. This should not be possible in current Signer and likely indicates database corruption."},{"index":90,"indent":0,"type":"error","payload":"Database error. Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption."},{"index":91,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `add_specs`."},{"index":92,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `load_meta`."},{"index":93,"indent":0,"type":"error","payload":"Bad input data. Payload could not be decoded as `load_types`."},{"index":94,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."},{"index":95,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. No system pallet in runtime metadata."},{"index":96,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. No runtime version in system pallet constants."},{"index":97,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime version from system pallet constants could not be decoded."},{"index":98,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Metadata vector does not start with 0x6d657461."},{"index":99,"indent":0,"type":"error","payload":"Bad input data. Received metadata is unsuitable. Runtime metadata could not be decoded."},{"index":100,"indent":0,"type":"error","payload":"Bad input data. Input is too short."},{"index":101,"indent":0,"type":"error","payload":"Bad input data. Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x35."},{"index":102,"indent":0,"type":"error","payload":"Bad input data. Payload type with code 0x0f is not supported."},{"index":103,"indent":0,"type":"error","payload":"Bad input data. Metadata for kusama9110 is already in the database and is different from the one in received payload."},{"index":104,"indent":0,"type":"error","payload":"Bad input data. Metadata for westend9122 is already in the database."},{"index":105,"indent":0,"type":"error","payload":"Bad input data. Similar network specs are already stored in the database under key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit)."},{"index":106,"indent":0,"type":"error","payload":"Bad input data. Payload with encryption 0x03 is not supported."},{"index":107,"indent":0,"type":"error","payload":"Bad input data. Received payload has bad signature."},{"index":108,"indent":0,"type":"error","payload":"Bad input data. Network kulupu is not in the database. Add network specs before loading the metadata."},{"index":109,"indent":0,"type":"error","payload":"Bad input data. Network westend was previously known to the database with verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata."},{"index":110,"indent":0,"type":"error","payload":"Bad input data. Saved network kulupu information was signed by verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Received information is not signed."},{"index":111,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier."},{"index":112,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier."},{"index":113,"indent":0,"type":"error","payload":"Bad input data. Network kulupu currently has no verifier set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. In order to accept verified metadata, first download properly verified network specs."},{"index":114,"indent":0,"type":"error","payload":"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing verifier for the network would require wipe and reset of Signer."},{"index":115,"indent":0,"type":"error","payload":"Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."},{"index":116,"indent":0,"type":"error","payload":"Bad input data. Network westend is verified by the general verifier which currently is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."},{"index":117,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received network westend specs could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":118,"indent":0,"type":"error","payload":"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received types information could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."},{"index":119,"indent":0,"type":"error","payload":"Bad input data. Exactly same types information is already in the database."},{"index":120,"indent":0,"type":"error","payload":"Bad input data. Received message could not be read."},{"index":121,"indent":0,"type":"error","payload":"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519."},{"index":122,"indent":0,"type":"error","payload":"Bad input data. Input transaction is generated in network westend. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata."},{"index":123,"indent":0,"type":"error","payload":"Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database."},{"index":124,"indent":0,"type":"error","payload":"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."},{"index":125,"indent":0,"type":"error","payload":"Could not find current verifier for network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."},{"index":126,"indent":0,"type":"error","payload":"Could not find general verifier."},{"index":127,"indent":0,"type":"error","payload":"Could not find types information."},{"index":128,"indent":0,"type":"error","payload":"Could not find network specs for network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."},{"index":129,"indent":0,"type":"error","payload":"Could not find network specs for westend."},{"index":130,"indent":0,"type":"error","payload":"Could not find network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e in address details with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":131,"indent":0,"type":"error","payload":"Could not find address details for address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."},{"index":132,"indent":0,"type":"error","payload":"Could not find metadata entry for westend9120."},{"index":133,"indent":0,"type":"error","payload":"Could not find danger status information."},{"index":134,"indent":0,"type":"error","payload":"Could not find database temporary entry with information needed for accepting approved information."},{"index":135,"indent":0,"type":"error","payload":"Could not find database temporary entry with information needed for signing approved transaction."},{"index":136,"indent":0,"type":"error","payload":"Could not find history entry with order 135."},{"index":137,"indent":0,"type":"error","payload":"Could not find network specs for westend with encryption ed25519 needed to decode historical transaction."},{"index":138,"indent":0,"type":"error","payload":"Entry with order 280 contains no transaction-related events."},{"index":139,"indent":0,"type":"error","payload":"Historical transaction was generated in network kulupu and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata."},{"index":140,"indent":0,"type":"error","payload":"Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd is disabled. It could be enabled again only after complete wipe and re-installation of Signer."},{"index":141,"indent":0,"type":"error","payload":"Error generating address. Network encryption sr25519 is different from seed object encryption ed25519."},{"index":142,"indent":0,"type":"error","payload":"Error generating address. Address key collision for seed name Alice super secret seed"},{"index":143,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid overall format."},{"index":144,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid bip39 phrase."},{"index":145,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid password."},{"index":146,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid seed."},{"index":147,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid seed length."},{"index":148,"indent":0,"type":"error","payload":"Error generating address. Bad secret string: invalid path."},{"index":149,"indent":0,"type":"error","payload":"Error generating address. Could not create random phrase. Mnemonic generator refuses to work with a valid excuse."},{"index":150,"indent":0,"type":"error","payload":"Error generating address. Invalid derivation format."},{"index":151,"indent":0,"type":"error","payload":"Error generating qr code. QR generator refuses to work with a valid excuse."},{"index":152,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected mortal transaction due to prelude format. Found immortal transaction."},{"index":153,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected immortal transaction due to prelude format. Found mortal transaction."},{"index":154,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Genesis hash values from decoded extensions and from used network specs do not match."},{"index":155,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Block hash for immortal transaction not matching genesis hash for the network."},{"index":156,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to decode extensions for V12/V13 metadata using standard extensions set."},{"index":157,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Method number 2 not found in pallet test_Pallet."},{"index":158,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Pallet with index 3 not found."},{"index":159,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Method number 5 too high for pallet number 3. Only 4 indices available."},{"index":160,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. No calls found in pallet test_pallet_v14."},{"index":161,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Referenced type could not be resolved in v14 metadata."},{"index":162,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Argument type error."},{"index":163,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Argument name error."},{"index":164,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected primitive type. Found Option<u8>."},{"index":165,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Expected compact. Not found it."},{"index":166,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Data too short for expected content."},{"index":167,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to decode part of data as u32."},{"index":168,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected Option<_> variant."},{"index":169,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. IdentityField description error."},{"index":170,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unable to decode part of data as an array."},{"index":171,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unexpected type encountered for Balance"},{"index":172,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected enum variant."},{"index":173,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Unexpected type inside compact."},{"index":174,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Type claimed inside compact is not compactable."},{"index":175,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. No description found for type T::SomeUnknownType."},{"index":176,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Declared type is not suitable BitStore type for BitVec."},{"index":177,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Declared type is not suitable BitOrder type for BitVec."},{"index":178,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Could not decode BitVec."},{"index":179,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Could not decode Era."},{"index":180,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the method some data remained unused."},{"index":181,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the extensions some data remained unused."},{"index":182,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is missing."},{"index":183,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash information is missing."},{"index":184,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version information is missing."},{"index":185,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is encountered mora than once."},{"index":186,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Genesis hash is encountered more than once."},{"index":187,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash is encountered more than once."},{"index":188,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version is encountered more than once."},{"index":189,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Network spec version decoded from extensions (9122) differs from the version in metadata (9010)."},{"index":190,"indent":0,"type":"error","payload":"Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9000)."},{"index":191,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid overall format."},{"index":192,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid bip39 phrase."},{"index":193,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid password."},{"index":194,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid seed."},{"index":195,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid seed length."},{"index":196,"indent":0,"type":"error","payload":"Error with secret string of existing address: invalid path."},{"index":197,"indent":0,"type":"error","payload":"Wrong password."}]"##;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let reply_known = r#""warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000031a49444154789cedda3d6e13611485e1448826928b4851b28534543474281b6005d981574085a85881779015b081888e868a265b4814c985a53408997b649dc00ce3ef9efbfd8c6c67dee2b8b165dda77061fb78bd5e1fbdf44641582e97d96f727a7a7a6c0f4d6b825072b4570b94aa082d8fef5713a30a42cef1b31f0bdb6eabb773db5835308a10728e4743002c070295606423b4006063436421e402a096082807228450723c6b8dc0221832420d003416025221248408c0e2d5ccb6dbfcf7ca765314e1ecfa836db7c79bafb65a0a848b500ac054080f80d584a8869002601e840ac05488220415004511bc6a22a014c456840800da7504b40d6210210a80f601010d41c8088bd97faf3d9aaffe3e2d8a70f9eec4b6dbddf727db4d5184b337d7b6dd1e7fded87693105400a64278004c85f0009802e122a4009807a102300f4205607d8824421f004511bca2085e3908e85f8809c17a46180240878a8008511de1e2f6936db7fbabcfb69ba208b3c52fdb6eabf96bdb4da320a0148407c054080f80a910db005008010d41a800cc83500198079102401d040f40298ae01545c80d1013c284104438b97c6fdbede9ee9beda628c2c78b5bdb6e5feeaf6c374511cecfcf6dbb3d3c3cd8a693118600980ae1013015c203601e84849002601e840ac03c081580a5209a20784511bc26046b42b07612e1203f13500ac203602a8407c0548814009211d010840ac03c08158079101e000a21784511bca208b94d08d684603d23200fe2e0bf4f4029842100a64278004c85f000580a42464801300f4205601e840ac0b6413443f08a22785545404310878a40003421581d04d4878822ecfd6f91a88f8052101e0053213c00a642f401908b805408158079102a00f32014002423784511bca2086a32028a42ec03c21000da8a802210bb8eb00d002511900a1145d89bff31221501a5203c00a642a800a818019542a800cc83a8098024041481481545284901403202aa013116820a804208ac04a33542e47896858072215a22e400a06c04d402626c005484c07230862072004a8e675510580e466e358e675511584b8c9ac7b32608fd4a505a1cdd6f14845def0f3887bfac02c861740000000049454e44ae426082"}}]"#;
    let stub_nav_known = StubNav::LoadMeta(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035649444154789cedd8bd6e134114c57152f000692251a54b6f5a1e812a5448aea810af804485c42b202a2a4b54a4e21168719f2e15529a3c008599a3d54d7636e339e7cec7ca49f65f5c173392757fd24a6b1fed76bb674fbd59106e6e6e8abfe4f8f8f8287c74ad0b42cdd2ac1e284d117a2e3fad2546138492e5cf5f9e861977f1e72a4c5f2d30aa104a96472900ab0402d5601423f400b0e68628422805403d115009840ba16679ab3782e5c190115a00a0b910900a21217800ce5f9d861977f1fb2acc212fc2d9f9fdfb971777e72c058222d402582a0403b05a423443c801580c4205b054882a0415007911582d11500e622f8207001d3a02da079144f002a0878080521032027b86bd089bd58730e3d6dbaf610e791136abef61c6adb7efc28c931054004b856000960ac1002c058222e4002c06a102580c4205b0a61059842900f222b0bc08ac12043486581042b7082900f4581190413447482d395e2a753e6d7c7ff5651366dcf6e33acca15910500e820158e3c5d47b29004b85d807805c082805a10258e30553f7c7e739008b41e4005084c00094524b4d1b2fc9f2229406880561417022f037371fc2eb9f7fc38cfbf5e64598435e8493939330e3aeafafc3cc2723a4002c158201582a0403b01884849003b018840a60310815c0ca417441607911580b426841081d24027b86bd089b4fab30e3d69fb7610ecd828072100cc052211880a542e400908c8052102a80c52054008b413000e442607911585e84d21684d08210ba45400c82fd5ef722a4961c2f953a9f16dd7f7ffffefadbdd792a00848fe1ff0494434801582a0403b0a2c5d47b09002b072123e4002c06a10258d18289fbd17906c0da07d10d81955a6ada7849565304948278ac0806801684508480a6105e04f6e6e645b8fcf723ccb8b3e76fc31c2a411803208a8072100cc052211880a5424c011045402a840a60310815c062100a009211585e049617414d46405e8887809002407b119007e2d011f601a02c025221bc08ec196e89900340cd10500e8201582a840a80aa11502d840a60318896004842401e885c5e849a14002423a016107321a800c88560d560f446f02c6f1521a052889e082500a81801f580981b005521582518298812809ae5ad2608560946692d96b79a22583d315a2e6f7541985683d263e969b3201c7aff01ee1bbfac2a71e1050000000049454e44ae426082","encryption":"ed25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"22","color":"#660D35","decimals":"6","encryption":"sr25519","genesis_hash":"6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae","logo":"dock-pos-main-runtime","name":"dock-pos-main-runtime","path_id":"//dock-pos-main-runtime","secondary_color":"#262626","title":"dock-pos-main-runtime-sr25519","unit":"DOCK"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
        &Encryption::Sr25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
    let reply_known = r#""error":[{"index":0,"indent":0,"type":"error","payload":"Bad input data. Network with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e already has entries in the database with base58 prefix 42. Received network specs have different base 58 prefix 115."}]"#;
    let output = produce_output(line.trim(), dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged."}],"new_specs":[{"index":2,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Ed25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let reply_known = r##""verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}],"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"ed25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"westend-ed25519","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs(NetworkSpecsKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
        &Encryption::Ed25519,
    ));
    let output = produce_output(line.trim(), dbname);
    if let Action::Stub(reply, _, stub_nav) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
        assert!(
            stub_nav == stub_nav_known,
            "Expected: {:?}\nReceived: {:?}",
            stub_nav_known,
            stub_nav
        );
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
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_4_unknown_author() {
    let dbname = "for_tests/parse_transaction_5_unknown_author";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "5301008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48a4040300d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034d49444154789cedd93d6e144110c5717c04673e01c9264848080287dea8258ee2cc2200022040ce7c142447ebd00108098964134ee0ac8f60aa342a6b7b34dbef557f8cd6f6fc83b7a155bfa8e539babfbf7ff1dc9b0521c658fc478e8f8f8fe4a76b5d106a8e46f540698ad0f3f8712d319a20941c1f2edfcba65d5ffc90f5d502a30a21161caf850900ab0442abc12846881d00acb9218a10622180163a226825102e845871bc153a23581e0c1a213600d0c24c081a0b41214407c0ddcd2fd9b493b3b7b243c189b05ebd924ddb6cffca72311010215602582c0402b05a423443c80158088205b058882a844802685e04544b042d07b117c103a01d3a82b60f6212c10ba03d06046d0a824678b35ec9a6fdde6c6587bc08ffee6e64d35e9e9cc90e7911bede9ecba67d3abd924da31058008b854000160b81002c060222e4002c04c10258088205b0c61059843180e645407911502508da2ec482203d204c01684f15413388e60857e7b7b269e757a7b2435e8470f94d36edfae2a3ecd02c085a0e0201582c0402b058887d009a0b419b8260012c04c10258082207a025080880c98b80f22294a6100bc282e044402f412fc2ebd55a36edcf76233be44508979f65d3ae2fbec8e6a311a6002c160201582c0402b01004859003b010040b60210816c0ca417441407911500b82b420480789805e825e84d5fa9d6cda76f353766816042d0781002c160201582c440e40a311b4290816c042102c8085201080e642407911505e84d216046941901e10340411c0b7432f027a097a11be4fbc543fecbc54a75200f919fe9fa0c50c429800b058080460b11008c0ca41d008210360210816c042102c80b50fa21b02ca8b806a8aa0c50988f044110c405b10a404418b2388e04440df0ebd08e8255882b00ba041042d64201080c54220008b8518036810418b24040b60210816c042100c804623a0bc08282f021b8da079211e03c21480b61741f3401c3ac23e002d8ba0b1105e04f4edb025420e406b86a0e5201080c542b0005a3582560bc1025808a225804621681e885c5e849a18008d46d05a40cc85c002682e04ab06a33782e778ab08412b85e8895002a01523683d20e606d0aa10ac128c298812809ae3ad2608560946692d8eb79a22583d315a1e6f7541185783d2e3e871b3201c7aff01c564bfac995018020000000049454e44ae426082"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Transaction author public key not found."}],"method":[{"index":2,"indent":0,"type":"pallet","payload":"Balances"},{"index":3,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e"}},{"index":4,"indent":2,"type":"varname","payload":"dest"},{"index":5,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":6,"indent":4,"type":"Id","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082"}},{"index":7,"indent":2,"type":"varname","payload":"value"},{"index":8,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":9,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":10,"indent":0,"type":"nonce","payload":"46"},{"index":11,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":12,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":13,"indent":0,"type":"tx_version","payload":"5"},{"index":14,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_5_unknown_network() {
    let dbname = "for_tests/parse_transaction_6_unknown_network";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530102761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62a8030300761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c620b00407a10f35aa707000b00a0724e1809140000000a000000f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769badc21d36b69bae1e8a41dedb34758567ba4efe711412f33d1461f795ffcd1de13f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author_public_key","payload":{"public_key":"761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035c49444154789cedd8bd8d144110c571ce20808d008b00488014101e123e41e123e12152200102c022820d0063e9a7559dde3435f5d15d3d5a89f91b7bc68ebadffca4930e9e6eb7db8bffbd4310aed7ebf02597cbe5a9fd58da12849997f65a81528ab0f2e5fb2a314a10465efec79fafed73dbdb971fda67ae0a8c29849197471a803402816630861156004847430c218c02a095086804228530f3f2d26a0429831146a800404721a02844082103f0e6cbe7f6b9ede7c74fedf35e16c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46d5e75910bb081900543dbafa3cb407a122640150f5e8eaf3240d228c50fd3bfceed5fbf6b9edfbef6fedf35ef63c6f9f14428802487c91359c076b00521482cf8bee937a0817c1ba40e28bb4e13cd802903c083e2fbb0f99083d001ab9c42a8be035ba8f214e84d633820680462fd9eb511090409423682fc92fa57ddfe73dcfdf67f7716104645dc4176883251e5efd5c745f5f0a016917f105d66089876bcf7bdff7f1f3de3ead0d820710293bdaabfabcbd0071229c084904ef2fb7ece8d7caf3bfe8fbec79debebdc208da05125f640de7c11a801485e0f3a2fbb44208d605125fa40de7c11680e441f079d97d7d4b10bcb2085eb3fb4e84d689d05a82e0fd0e6711bcf3b2fbfa4208c8ba882fd0064b3cdc82f000243e2fba4f2b8c80b48bf8026bb0c4c335882880c4e779fbf64a217865477b559fb7d789d03a115acf08c883f0febd9e1dad3def7ddfc7cf7bfbb400d07edcff3f015908da05125f640de7c1d5cf45f7f58511ac0b24be481bce83b5effbbce7f9fbec3e6e198297f6527dfc925e33fbfe41401ac4cc255a8f822000e844686d10500f91bdc4fbcb2d8be09d97dd871800b908c8ba882fd0064b3cdc82f000243e2fba4f72115014822fb0064b3c5c838802487c9eb74fea015018c12b3bdaabfa3c298c80b210d5a3abcf431a00da45401988ead1d5e7ed012013014521b2a3bddfe1ec795616002a4340d6701eac014851083ecf6b1a01cd42f0600b40f220f83c2f0f0085105006c22a8b3053040085115005c45108510094429066305623645e5e1a4240a3102b114600d030025a017134009a42904630348811809997974a10a4118cd12a5e5e2a4590566254bebcb404a16f0665c54bf71d82f0e8fd0547e7bfac61a5ec4f0000000049454e44ae426082","encryption":"sr25519"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba and encryption sr25519."}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_6_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_7_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403018eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. After decoding the method some data remained unused."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_7_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_8_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403068eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Encountered unexpected enum variant."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_8_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_9_error_on_parsing";
    populate_cold(dbname, Verifier(None)).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403028eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let reply_known = r#""author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false}}],"error":[{"index":1,"indent":0,"type":"error","payload":"Error parsing incoming transaction content. Data too short for expected content."}],"extensions":[{"index":2,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":3,"indent":0,"type":"nonce","payload":"46"},{"index":4,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":5,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9010"}},{"index":6,"indent":0,"type":"tx_version","payload":"5"},{"index":7,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]"#;
    let output = produce_output(line, dbname);
    if let Action::Read(reply) = output {
        assert!(
            reply == reply_known,
            "Expected: {}\nReceived: {}",
            reply_known,
            reply
        );
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
    let author_info_known = r#""base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed":"Alice","derivation_path":"//Alice","has_pwd":false"#;
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
        assert!(
            content == content_known,
            "Expected: {}\nReceived: {}",
            content_known,
            content
        );
        assert!(
            author_info == author_info_known,
            "Expected: {}\nReceived: {}",
            author_info_known,
            author_info
        );
        assert!(
            network_info == network_info_known,
            "Expected: {}\nReceived: {}",
            network_info_known,
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
        assert!(
            content == content_known,
            "Expected: {}\nReceived: {}",
            content_known,
            content
        );
        assert!(
            network_info == network_info_known,
            "Expected: {}\nReceived: {}",
            network_info_known,
            network_info
        );
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}
