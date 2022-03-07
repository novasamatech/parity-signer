#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    
    use bip39::{Language, Mnemonic};
    use sled::{Db, Tree, open, Batch};
    use sp_runtime::MultiSigner;
    use std::fs;
    
    use constants::{ADDRTREE, ALICE_SEED_PHRASE, METATREE, test_values::{EMPTY_PNG, REAL_PARITY_VERIFIER}};
    use defaults::get_default_chainspecs;
    use definitions::{crypto::Encryption, error::ErrorSource, error_active::{Active, IncomingMetadataSourceActiveStr}, error_signer::Signer, keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{ValidCurrentVerifier, Verifier}, users::AddressDetails};
    
    use crate::{cold_default::{populate_cold, populate_cold_no_metadata, populate_cold_release, signer_init_no_cert, signer_init_with_cert}, db_transactions::TrDbCold, helpers::{display_general_verifier, get_danger_status, open_db, open_tree, try_get_valid_current_verifier, upd_id_batch}, hot_default::reset_hot_database, identities::{check_derivation_set, create_address, create_increment_set, DerivationCheck, derivation_check, generate_random_phrase, get_addresses_by_seed_name, is_passworded, remove_key, try_create_address, try_create_seed}, interface_signer::addresses_set_seed_name_network, manage_history::{device_was_online, events_to_batch, print_history, reset_danger_status_to_safe}, metadata::transfer_metadata_to_cold};
    
    #[test]
    fn get_danger_status_properly () {
        let dbname = "for_tests/get_danger_status_properly";
        populate_cold_release(dbname).unwrap();
        signer_init_no_cert(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the database initiation.");
        device_was_online(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == true, "Expected danger status = true after the reported exposure.");
        reset_danger_status_to_safe(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the danger reset.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn display_general_verifier_properly() {
        let dbname = "for_tests/display_general_verifier_properly";
        populate_cold_release(dbname).unwrap();
        signer_init_no_cert(dbname).unwrap();
        let print = display_general_verifier(dbname).unwrap()
            .replace(EMPTY_PNG, r#"<empty>"#);
        assert!(print == r#""public_key":"","identicon":"<empty>","encryption":"none""#, "Got: {}", print);
        signer_init_with_cert(dbname).unwrap();
        let print = display_general_verifier(dbname).unwrap()
            .replace(REAL_PARITY_VERIFIER, r#"<real_verifier>"#);
        assert!(print == r#""public_key":"c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57","identicon":"<real_verifier>","encryption":"sr25519""#, "Got: {}", print);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn find_westend_verifier() {
        let dbname = "for_tests/find_westend_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap());
        let westend_verifier = try_get_valid_current_verifier(&verifier_key, dbname).unwrap();
        assert!(westend_verifier == Some(ValidCurrentVerifier::General));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn not_find_mock_verifier() {
        let dbname = "for_tests/not_find_mock_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap());
        match try_get_valid_current_verifier(&verifier_key, dbname) {
            Ok(Some(_)) => panic!("Found network key that should not be in database."),
            Ok(None) => (),
            Err(e) => panic!("Error looking for mock verifier: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn test_generate_random_seed_phrase() {
        let random_phrase = generate_random_phrase(24).unwrap();
        assert!(Mnemonic::validate(&random_phrase, Language::English).is_ok());
        assert!(generate_random_phrase(1).is_err());
        let random_phrase2 = generate_random_phrase(24).unwrap();
        assert!(Mnemonic::validate(&random_phrase2, Language::English).is_ok());
        assert!(random_phrase2 != random_phrase);
    }

    #[test]
    fn test_check_for_seed_validity() {
        assert!(Mnemonic::validate(ALICE_SEED_PHRASE, Language::English).is_ok());
        assert!(Mnemonic::validate("the fox is triangular", Language::English).is_err());
        assert!(Mnemonic::validate("", Language::English).is_err());
        assert!(Mnemonic::validate("низ ехать подчиняться озеро занавеска дым корзина держать гонка одинокий подходящий прогулка", Language::English).is_err());
    }

    #[test]
    fn test_generate_default_addresses_for_alice() {
        let dbname = "for_tests/test_generate_default_addresses_for_Alice";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        {
            let database = open_db::<Signer>(dbname).unwrap();
            let addresses = open_tree::<Signer>(&database, ADDRTREE).unwrap();
            assert!(addresses.len() == 4, "real addresses length: {}", addresses.len());
        }
        let chainspecs = get_default_chainspecs();
        let default_addresses = addresses_set_seed_name_network (dbname, "Alice", &NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519)).unwrap();
        assert!(default_addresses.len()>0);
        assert!("[(MultiSigner::Sr25519(46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a (5DfhGyQd...)), AddressDetails { seed_name: \"Alice\", path: \"\", has_pwd: false, network_id: [NetworkSpecsKey([1, 128, 145, 177, 113, 187, 21, 142, 45, 56, 72, 250, 35, 169, 241, 194, 81, 130, 251, 142, 32, 49, 59, 44, 30, 180, 146, 25, 218, 122, 112, 206, 144, 195]), NetworkSpecsKey([1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254]), NetworkSpecsKey([1, 128, 225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62])], encryption: Sr25519 }), (MultiSigner::Sr25519(64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05 (5ELf63sL...)), AddressDetails { seed_name: \"Alice\", path: \"//kusama\", has_pwd: false, network_id: [NetworkSpecsKey([1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254])], encryption: Sr25519 })]" == format!("{:?}", default_addresses), "Default addresses:\n{:?}", default_addresses);
        let database: Db = open(dbname).unwrap();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        let test_key = AddressKey::from_parts(&hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(), &Encryption::Sr25519).unwrap();
        assert!(identities.contains_key(test_key.key()).unwrap());
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn must_check_for_valid_derivation_phrase() {
        assert!(!is_passworded("").expect("valid empty path"));
        assert!(is_passworded("//").is_err());
        assert!(!is_passworded("//path1").expect("valid path1"));
        assert!(!is_passworded("//path/path").expect("soft derivation"));
        assert!(!is_passworded("//path//path").expect("hard derivation"));
        assert!(is_passworded("//path///password").expect("path with password"));
        assert!(is_passworded("///").is_err());
        assert!(!is_passworded("//$~").expect("weird symbols"));
        assert!(is_passworded("abraca dabre").is_err());
        assert!(is_passworded("////").expect("//// - password is /"));
        assert!(is_passworded("//path///password///password").expect("password///password is a password"));
        assert!(!is_passworded("//путь").expect("valid utf8 abomination"));
    }

    #[test]
    fn test_derive() { 
        let dbname = "for_tests/test_derive";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        println!("[0]: {:?}, [1]: {:?}", chainspecs[0].name, chainspecs[1].name);
        let seed_name = "Alice";
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        let network_id_1 = NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash, &Encryption::Sr25519);
        let both_networks = vec![network_id_0.to_owned(), network_id_1.to_owned()];
        let only_one_network = vec![network_id_0.to_owned()];

        try_create_seed(seed_name, ALICE_SEED_PHRASE, true, dbname).unwrap();
        let (adds1, events1) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice", &chainspecs[0], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds1)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events1).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let (adds2, events2) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice", &chainspecs[1], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds2)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events2).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let (adds3, events3) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice/1", &chainspecs[0], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds3)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events3).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let identities = get_addresses_by_seed_name (&dbname, seed_name).unwrap();
        println!("{:?}", identities);
        let mut flag0 = false;
        let mut flag1 = false;
        for (_, details) in identities {
            flag0 = flag0 || details.network_id == both_networks;
            flag1 = flag1 || details.network_id == only_one_network;
        }
        assert!(flag0, "Something is wrong with //Alice");
        assert!(flag1, "Something is wrong with //Alice/1");
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_identity_deletion() {
        let dbname = "for_tests/test_identity_deletion";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_specs_key_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        let network_specs_key_1 = NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash, &Encryption::Sr25519);
        let mut identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_0).expect("Alice should have some addresses by default");
        println!("{:?}", identities);
        let (key0, _) = identities.remove(0); //TODO: this should be root key
        let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
        remove_key(dbname, &key0, &network_specs_key_0).expect("delete an address");
        remove_key(dbname, &key1, &network_specs_key_0).expect("delete another address");
        let identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_0).expect("Alice still should have some addresses after deletion of two");
        for (address_key, _) in identities {
            assert_ne!(address_key, key0);
            assert_ne!(address_key, key1);
        }
        let identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_1).expect("Alice still should have some addresses after deletion of two");
        let mut flag_to_check_key0_remains = false;
        for (address_key, _) in identities {
            if address_key == key0 {
                flag_to_check_key0_remains = true;
            }
            assert_ne!(address_key, key1);
        }
        assert!(flag_to_check_key0_remains, "An address that should have only lost network was removed entirely");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn history_with_identities() {
        let dbname = "for_tests/history_with_identities";
        populate_cold_release(dbname).unwrap();
        signer_init_with_cert(dbname).unwrap();
        let history_printed = print_history(dbname).unwrap()
            .replace(REAL_PARITY_VERIFIER, r#"<real_verifier>"#);
        let element1 = r#"{"event":"database_initiated"}"#;
        let element2 = r#"{"event":"general_verifier_added","payload":{"public_key":"c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57","identicon":"<real_verifier>","encryption":"sr25519"}}"#;
        assert!(history_printed.contains(element1), "\nReal history check1:\n{}", history_printed);
        assert!(history_printed.contains(element2), "\nReal history check2:\n{}", history_printed);
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        let history_printed_after_create_seed = print_history(dbname).unwrap()
            .replace(REAL_PARITY_VERIFIER, r#"<real_verifier>"#);
        let element3 = r#""events":[{"event":"seed_created","payload":"Alice"},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","path":"//polkadot","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","path":"//kusama","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}]"#;
        assert!(history_printed_after_create_seed.contains(element1), "\nReal history check3:\n{}", history_printed_after_create_seed);
        assert!(history_printed_after_create_seed.contains(element2), "\nReal history check4:\n{}", history_printed_after_create_seed);
        assert!(history_printed_after_create_seed.contains(element3), "\nReal history check5:\n{}", history_printed_after_create_seed);
        fs::remove_dir_all(dbname).unwrap();
    }

    fn get_multisigner_path_set(dbname: &str) -> Vec<(MultiSigner, String)> {
        let db = open_db::<Signer>(dbname).unwrap();
        let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
        let mut multisigner_path_set: Vec<(MultiSigner, String)> = Vec::new();
        for x in identities.iter() {
            if let Ok(a) = x {
                let (multisigner, address_details) = AddressDetails::process_entry_checked::<Signer>(a).unwrap();
                multisigner_path_set.push((multisigner, address_details.path.to_string()))
            }
        }
        multisigner_path_set
    }
    
    #[test]
    fn increment_identities_1() {
        let dbname = "for_tests/increment_identities_1";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 1, "Wrong number of identities: {:?}", multisigner_path_set);
        println!("{}", multisigner_path_set[0].1);
        create_increment_set(4, &multisigner_path_set[0].0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//0")));
        assert!(path_set.contains(&String::from("//Alice//1")));
        assert!(path_set.contains(&String::from("//Alice//2")));
        assert!(path_set.contains(&String::from("//Alice//3")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn increment_identities_2() {
        let dbname = "for_tests/increment_identities_2";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        let alice_multisigner_path = multisigner_path_set.iter().find(|(_, path)| path == "//Alice").unwrap();
        assert!(multisigner_path_set.len() == 2, "Wrong number of identities: {:?}", multisigner_path_set);
        create_increment_set(3, &alice_multisigner_path.0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//2")));
        assert!(path_set.contains(&String::from("//Alice//3")));
        assert!(path_set.contains(&String::from("//Alice//4")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn increment_identities_3() {
        let dbname = "for_tests/increment_identities_3";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        let alice_multisigner_path = multisigner_path_set.iter().find(|(_, path)| path == "//Alice//1").unwrap();
        assert!(multisigner_path_set.len() == 2, "Wrong number of identities: {:?}", multisigner_path_set);
        create_increment_set(3, &alice_multisigner_path.0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//1//0")));
        assert!(path_set.contains(&String::from("//Alice//1//1")));
        assert!(path_set.contains(&String::from("//Alice//1//2")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn checking_derivation_set() {
        assert!(check_derivation_set(&["/0".to_string(), "//Alice/westend".to_string(), "//secret//westend".to_string()]).is_ok());
        assert!(check_derivation_set(&["/0".to_string(), "/0".to_string(), "//Alice/westend".to_string(), "//secret//westend".to_string()]).is_ok());
        assert!(check_derivation_set(&["//remarkably///ugly".to_string()]).is_err());
        assert!(check_derivation_set(&["no_path_at_all".to_string()]).is_err());
        assert!(check_derivation_set(&["///".to_string()]).is_err());
    }
    
    #[test]
    fn creating_derivation_1() {
        let dbname = "for_tests/creating_derivation_1";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        if let DerivationCheck::NoPassword(Some(_)) = derivation_check("Alice", "//Alice", &network_id_0, dbname).unwrap() {println!("Found existing");}
        else {panic!("Derivation should already exist.");}
        match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice derivation again."),
            Err(e) => assert!(<Signer>::show(&e) == "Error generating address. Seed Alice already has derivation //Alice for network specs key 0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d.", "Wrong error: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_2() {
        let dbname = "for_tests/creating_derivation_2";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice/// secret derivation.");
        if let DerivationCheck::NoPassword(None) = derivation_check("Alice", "//Alice", &network_id_0, dbname).unwrap() {println!("It did well.");}
        else {panic!("New derivation has no password, existing derivation has password and is diffenent.");}
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_3() {
        let dbname = "for_tests/creating_derivation_3";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        if let DerivationCheck::Password = derivation_check("Alice", "//Alice///secret", &network_id_0, dbname).unwrap() {println!("It did well.");}
        else {panic!("New derivation has password, existing derivation has no password and is diffenent.");}
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_4() {
        let dbname = "for_tests/creating_derivation_4";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret1", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret1 derivation.");
        if let DerivationCheck::Password = derivation_check("Alice", "//Alice///secret2", &network_id_0, dbname).unwrap() {println!("It did well.");}
        else {panic!("Existing derivation has different password.");}
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret2", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret2 derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_5() {
        let dbname = "for_tests/creating_derivation_5";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        if let DerivationCheck::Password = derivation_check("Alice", "//Alice///secret", &network_id_0, dbname).unwrap() {println!("It did well.");}
        else {panic!("Derivation exists, but has password.");}
        match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice///secret derivation again."),
            Err(e) => assert!(<Signer>::show(&e) == "Error generating address. Seed Alice already has derivation //Alice///<password> for network specs key 0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key 08a5e583f74f54f3811cb5f7d74e686d473e3a466fd0e95738707a80c3183b15.", "Wrong error: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    fn insert_metadata_from_file (database_name: &str, filename: &str) {
        let meta_str = std::fs::read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta_str.trim(), IncomingMetadataSourceActiveStr::Default{filename: filename.to_string()}).unwrap();
        let mut meta_batch = Batch::default();
        meta_batch.insert(MetaKey::from_parts(&meta_values.name, meta_values.version).key(), meta_values.meta);
        TrDbCold::new()
            .set_metadata(meta_batch)
            .apply::<Active>(database_name).unwrap();
    }
    
    fn metadata_len(database_name: &str) -> usize {
        let database = open_db::<Active>(database_name).unwrap();
        let metadata = open_tree::<Active>(&database, METATREE).unwrap();
        metadata.len()
    }
    fn metadata_contents(database_name: &str) -> Vec<(String, u32)> {
        let database = open_db::<Active>(database_name).unwrap();
        let metadata = open_tree::<Active>(&database, METATREE).unwrap();
        let mut out: Vec<(String, u32)> = Vec::new();
        for x in metadata.iter() {
            if let Ok((meta_key_vec, _)) = x {
                let new = MetaKey::from_ivec(&meta_key_vec).name_version::<Active>().unwrap();
                out.push(new);
            }
        }
        out
    }
    
    #[test]
    fn test_metadata_transfer() {
        let dbname_hot = "for_tests/test_metadata_transfer_mock_hot";
        reset_hot_database(dbname_hot).unwrap();
        let dbname_cold = "for_tests/test_metadata_transfer_mock_cold";
        populate_cold(dbname_cold, Verifier(None)).unwrap();
        
        insert_metadata_from_file(dbname_hot, "for_tests/westend9010");
        assert!(metadata_len(dbname_hot) == 1, "Fresh hot database, should have only the single network added.");
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        insert_metadata_from_file(dbname_hot, "for_tests/westend9090");
        assert!(metadata_len(dbname_hot) == 2, "Now 2 entries in hot db.");
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("westend", 9090), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        std::fs::remove_dir_all(dbname_hot).unwrap();
        std::fs::remove_dir_all(dbname_cold).unwrap();
    }
}

