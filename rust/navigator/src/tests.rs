#[cfg(test)]
mod tests {

    use crate::{do_action, init_navigation};
    
    use db_handling::cold_default::{populate_cold_release, signer_init};
    use definitions::network_specs::{Verifier, VerifierValue};
    use lazy_static::lazy_static;
    use regex::Regex;
    use sp_core;
    use sp_runtime::MultiSigner;
    use std::fs;
    
    const ALICE: [u8; 32] = [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125];
    
    fn verifier_alice_sr25519() -> Verifier {
        Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(ALICE)))))
    }
/*    
    fn verifier_alice_ed25519() -> Verifier {
        Verifier(Some(VerifierValue::Standard(MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(ALICE)))))
    }
*/    
    
    lazy_static! {
        static ref NO_TIME: Regex = Regex::new(r#""timestamp":".*?""#).expect("checked construction");
        static ref NO_CHECKSUM: Regex = Regex::new(r#""checksum":"[0-9A-F]*?""#).expect("checked construction");
    }
    
    fn timeless(current_real_json: &str) -> String {
        NO_TIME.replace_all(&current_real_json, r#""timestamp":"**""#).to_string()
    }
    fn cut_checksum(current_real_json: &str) -> String {
        NO_CHECKSUM.replace_all(&current_real_json, r#""checksum":"**""#).to_string()
    }
    
    #[test]
    fn flow_test_1() {
        let dbname = "for_tests/flow_test_1";
        populate_cold_release(dbname).unwrap();
        signer_init(dbname, verifier_alice_sr25519()).unwrap();
        init_navigation(dbname, "");
        
        let real_json = do_action("Start","","");
        let expected_json = r#"{"screen":"SeedSelector","screenLabel":"Select seed","back":false,"footer":true,"footerButton":"Keys","rightButton":"NewSeed","screenNameType":"h1","modal":"NewSeedMenu","alert":"Empty","screenData":{"seedNameCards":[]},"modalData":{},"alertData":{}}"#;
        assert!(real_json == expected_json, "Initiated with no keys. Started. Expected SeedSelector screen with no modals, got:\n{}", real_json);
        
        let real_json = do_action("NavbarLog","","");
        let cut_real_json = timeless(&real_json);
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":0,"timestamp":"**","events":[{"event":"database_initiated"},{"event":"general_verifier_added","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}]}],"total_entries":1},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "Switched to Log from empty SeedSelector. Expected Log screen with no modals, got:\n{}", real_json);
        
        let current_log_json = real_json;
        
        let real_json = do_action("GoBack","","");
        assert!(current_log_json == real_json, "GoBack on Log screen with no modals. Expected to remain where was, got:\n{}", real_json);
        let real_json = do_action("GoForward","","");
        assert!(current_log_json == real_json, "GoForward on Log screen with no modals. Expected to remain where was, got:\n{}", real_json);
        
        let real_json = do_action("RightButton","","");
        let cut_real_json = cut_checksum(&timeless(&real_json));
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"LogRight","alert":"Empty","screenData":{"log":[{"order":0,"timestamp":"**","events":[{"event":"database_initiated"},{"event":"general_verifier_added","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}]}],"total_entries":1},"modalData":{"checksum":"**"},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "RightButton on Log screen with no modals. Expected same Log screen with LogRight modal, got:\n{}", cut_real_json);
        let real_json = do_action("GoBack","","");
        assert!(current_log_json == real_json, "GoBack on Log screen with LogRight modal. Expected to get Log screen with no modals, got:\n{}", real_json);
        
        do_action("RightButton","","");
        let real_json = do_action("CreateLogComment","","");
        let cut_real_json = timeless(&real_json);
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"LogComment","alert":"Empty","screenData":{"log":[{"order":0,"timestamp":"**","events":[{"event":"database_initiated"},{"event":"general_verifier_added","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}]}],"total_entries":1},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "CreateLogComment on Log screen with LogRight modal. Expected same Log screen with LogComment modal, got:\n{}", cut_real_json);
        let real_json = do_action("GoBack","","");
        assert!(current_log_json == real_json, "GoBack on Log screen with LogComment modal. Expected same Log screen with no modals, got:\n{}", real_json);
        
        do_action("RightButton","","");
        do_action("CreateLogComment","","");
        let real_json = do_action("GoForward","Remember this moment","");
        let cut_real_json = timeless(&real_json);
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":1,"timestamp":"**","events":[{"event":"user_entered_event","payload":"Remember this moment"}]},{"order":0,"timestamp":"**","events":[{"event":"database_initiated"},{"event":"general_verifier_added","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","encryption":"sr25519"}}]}],"total_entries":2},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "GoForward on Log screen with LogComment modal. Expected updated Log screen with no modals, got:\n{}", real_json);
/*        
        let current_log_json = real_json;
        
        let real_json = do_action("SelectSeed","","");
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"","identicon":"","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[],"network":{"title":"Polkadot","logo":"polkadot"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(real_json == expected_json, "Different navstate json:\n{}", real_json);
*/        
        fs::remove_dir_all(dbname).unwrap();
    }
    
}
