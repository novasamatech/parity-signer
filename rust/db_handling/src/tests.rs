use bip39::{Language, Mnemonic};
use definitions::history::{
    Entry, IdentityHistory, MetaValuesDisplay, MetaValuesExport, NetworkSpecsDisplay,
    NetworkSpecsExport, NetworkVerifierDisplay, SignDisplay, SignMessageDisplay, TypesExport,
};
use definitions::navigation::{Address, DerivationDestination, MSCNetworkInfo, NetworkSpecsToSend};
use definitions::network_specs::NetworkSpecs;
use pretty_assertions::{assert_eq, assert_ne};
use sled::{open, Batch, Db, Tree};
use sp_core::sr25519::Public;
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::convert::TryInto;
use std::fs;
use std::str::FromStr;

use constants::{
    test_values::{
        alice_sr_alice, alice_sr_kusama, alice_sr_polkadot, alice_sr_root, alice_sr_westend,
        alice_westend_root_qr, empty_png, types_known, westend_9000, westend_9010,
    },
    ADDRTREE, ALICE_SEED_PHRASE, METATREE, SPECSTREE,
};
use defaults::default_chainspecs;
use definitions::{
    crypto::Encryption,
    error::ErrorSource,
    error_active::{Active, IncomingMetadataSourceActiveStr},
    error_signer::Signer,
    history::{all_events_preview, Event, TypesDisplay},
    keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey},
    metadata::MetaValues,
    navigation::{
        DerivationCheck as NavDerivationCheck, DerivationEntry, DerivationPack, MBackup,
        MDeriveKey, MKeyDetails, MKeysCard, MMMNetwork, MMNetwork, MManageMetadata,
        MMetadataRecord, MNetworkDetails, MNetworkMenu, MRawKey, MSeedKeyCard, MTypesInfo,
        MVerifier, Network, SeedNameCard,
    },
    network_specs::{ValidCurrentVerifier, Verifier, VerifierValue},
    users::AddressDetails,
};

use crate::helpers::get_general_verifier;
use crate::manage_history::get_history;
use crate::{
    cold_default::{
        populate_cold, populate_cold_no_metadata, populate_cold_release, signer_init_no_cert,
        signer_init_with_cert,
    },
    db_transactions::TrDbCold,
    helpers::{
        get_danger_status, open_db, open_tree, remove_metadata, remove_network, remove_types_info,
        transfer_metadata_to_cold, try_get_valid_current_verifier, upd_id_batch,
    },
    hot_default::reset_hot_database,
    identities::{
        check_derivation_set, create_address, create_increment_set, derivation_check,
        generate_random_phrase, get_addresses_by_seed_name, is_passworded, remove_key,
        try_create_address, try_create_seed, DerivationCheck,
    },
    interface_signer::{
        addresses_set_seed_name_network, backup_prep, derive_prep, dynamic_path_check, export_key,
        first_network, get_all_seed_names_with_identicons, guess, metadata_details,
        network_details_by_key, print_all_identities, print_identities_for_seed_name_and_network,
        show_all_networks, show_all_networks_with_flag, show_types_status, SeedDraft,
    },
    manage_history::{
        device_was_online, enter_events, events_to_batch, reset_danger_status_to_safe,
    },
};

#[test]
fn print_seed_names() {
    let dbname = "for_tests/print_seed_names";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let cards = get_all_seed_names_with_identicons(dbname, &[String::from("Alice")]).unwrap();
    let expected_cards = vec![SeedNameCard {
        seed_name: "Alice".to_string(),
        identicon: alice_sr_root().to_vec(),
    }];
    assert!(cards == expected_cards, "\nReceived: \n{:?}", cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn print_seed_names_with_orphan() {
    let dbname = "for_tests/print_seed_names_with_orphan";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let cards = get_all_seed_names_with_identicons(
        dbname,
        &[String::from("Alice"), String::from("BobGhost")],
    )
    .unwrap();

    let expected_cards = vec![
        SeedNameCard {
            seed_name: "Alice".to_string(),
            identicon: alice_sr_root().to_vec(),
        },
        SeedNameCard {
            seed_name: "BobGhost".to_string(),
            identicon: empty_png().to_vec(),
        },
    ];
    assert!(cards == expected_cards, "\nReceived: \n{:?}", cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn print_all_ids() {
    let dbname = "for_tests/print_all_ids";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let keys = print_all_identities(dbname).unwrap();

    let expected_keys = vec![
        MRawKey {
            seed_name: "Alice".to_string(),
            address_key: "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                .to_string(),
            public_key: "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                .to_string(),
            identicon: alice_sr_westend().to_vec(),
            has_pwd: false,
            path: "//westend".to_string(),
        },
        MRawKey {
            seed_name: "Alice".to_string(),
            address_key: "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                .to_string(),
            public_key: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                .to_string(),
            identicon: alice_sr_root().to_vec(),
            has_pwd: false,
            path: "".to_string(),
        },
        MRawKey {
            seed_name: "Alice".to_string(),
            address_key: "0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                .to_string(),
            public_key: "64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                .to_string(),
            identicon: alice_sr_kusama().to_vec(),
            has_pwd: false,
            path: "//kusama".to_string(),
        },
        MRawKey {
            seed_name: "Alice".to_string(),
            address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                .to_string(),
            public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                .to_string(),
            identicon: alice_sr_alice().to_vec(),
            has_pwd: false,
            path: "//Alice".to_string(),
        },
        MRawKey {
            seed_name: "Alice".to_string(),
            address_key: "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                .to_string(),
            public_key: "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                .to_string(),
            identicon: alice_sr_polkadot().to_vec(),
            has_pwd: false,
            path: "//polkadot".to_string(),
        },
    ];

    assert_eq!(keys, expected_keys);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn print_ids_seed_name_network() {
    let dbname = "for_tests/print_ids_seed_name_network";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let cards = print_identities_for_seed_name_and_network(
        dbname,
        "Alice",
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        None,
        Vec::new(),
    )
    .unwrap();
    let expected_cards = (
        MSeedKeyCard {
            seed_name: "Alice".to_string(),
            identicon: alice_sr_root().to_vec(),
            address_key: "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                .to_string(),
            base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
            swiped: false,
            multiselect: false,
        },
        vec![
            MKeysCard {
                address_key: "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                    .to_string(),
                base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                identicon: alice_sr_westend().to_vec(),
                has_pwd: false,
                path: "//westend".to_string(),
                swiped: false,
                multiselect: false,
            },
            MKeysCard {
                address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    .to_string(),
                base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                identicon: alice_sr_alice().to_vec(),
                has_pwd: false,
                path: "//Alice".to_string(),
                swiped: false,
                multiselect: false,
            },
        ],
    );
    // TODO: "network":{"title":"Westend","logo":"westend"}"#;
    assert_eq!((cards.0, cards.1), expected_cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn print_show_all_networks_flag_westend() {
    let dbname = "for_tests/show_all_networks_flag_westend";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let menu = show_all_networks_with_flag(
        dbname,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();
    let expected_menu = MNetworkMenu {
        networks: vec![
            Network {
                key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                    .to_string(),
                logo: "polkadot".to_string(),
                order: 0,
                selected: false,
                title: "Polkadot".to_string(),
            },
            Network {
                key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                    .to_string(),
                logo: "kusama".to_string(),
                order: 1,
                selected: false,
                title: "Kusama".to_string(),
            },
            Network {
                key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                    .to_string(),
                logo: "westend".to_string(),
                order: 2,
                selected: true,
                title: "Westend".to_string(),
            },
        ],
    };
    assert_eq!(menu, expected_menu);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn show_all_networks_no_flag() {
    let dbname = "for_tests/show_all_networks_no_flag";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let networks = show_all_networks(dbname).unwrap();
    let expected_networks = vec![
        MMNetwork {
            key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3".to_string(),
            title: "Polkadot".to_string(),
            logo: "polkadot".to_string(),
            order: 0,
        },
        MMNetwork {
            key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe".to_string(),
            title: "Kusama".to_string(),
            logo: "kusama".to_string(),
            order: 1,
        },
        MMNetwork {
            key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e".to_string(),
            title: "Westend".to_string(),
            logo: "westend".to_string(),
            order: 2,
        },
    ];
    assert_eq!(networks, expected_networks);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn first_standard_network() {
    let dbname = "for_tests/first_standard_network";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let specs = first_network(dbname).unwrap();
    assert_eq!(specs.name, "polkadot");
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn export_alice_westend() {
    let dbname = "for_tests/export_alice_westend";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let public: [u8; 32] =
        hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a")
            .unwrap()
            .try_into()
            .unwrap();
    let key = export_key(
        dbname,
        &MultiSigner::Sr25519(Public::from_raw(public)),
        "Alice",
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();
    let expected_key = MKeyDetails {
        qr: alice_westend_root_qr().to_vec(),
        pubkey: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a".to_string(),
        address: Address {
            base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
            identicon: alice_sr_root().to_vec(),
            seed_name: "Alice".to_string(),
            path: "".to_string(),
            has_pwd: false,
            multiselect: None,
        },
        network_info: MSCNetworkInfo {
            network_title: "Westend".to_string(),
            network_logo: "westend".to_string(),
        },
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn backup_prep_alice() {
    let dbname = "for_tests/backup_prep_alice";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let backup = backup_prep(dbname, "Alice").unwrap();
    let expected_backup = MBackup {
        seed_name: "Alice".to_string(),
        derivations: vec![
            DerivationPack {
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
                network_order: 0.to_string(),
                id_set: vec![
                    DerivationEntry {
                        path: String::new(),
                        has_pwd: false,
                    },
                    DerivationEntry {
                        path: "//polkadot".to_string(),
                        has_pwd: false,
                    },
                ],
            },
            DerivationPack {
                network_title: "Kusama".to_string(),
                network_logo: "kusama".to_string(),
                network_order: 1.to_string(),
                id_set: vec![
                    DerivationEntry {
                        path: String::new(),
                        has_pwd: false,
                    },
                    DerivationEntry {
                        path: "//kusama".to_string(),
                        has_pwd: false,
                    },
                ],
            },
            DerivationPack {
                network_title: "Westend".to_string(),
                network_logo: "westend".to_string(),
                network_order: 2.to_string(),
                id_set: vec![
                    DerivationEntry {
                        path: "//westend".to_string(),
                        has_pwd: false,
                    },
                    DerivationEntry {
                        path: String::new(),
                        has_pwd: false,
                    },
                    DerivationEntry {
                        path: "//Alice".to_string(),
                        has_pwd: false,
                    },
                ],
            },
        ],
    };
    assert_eq!(backup, expected_backup);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn derive_prep_alice() {
    let dbname = "for_tests/derive_prep_alice";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let key = derive_prep(
        dbname,
        "Alice",
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        None,
        "//secret//derive",
        false,
    )
    .unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
        network_title: "Westend".to_string(),
        network_logo: "westend".to_string(),
        network_specs_key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            .to_string(),
        suggested_derivation: "//secret//derive".to_string(),
        keyboard: false,
        derivation_check: None,
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn derive_prep_alice_collided() {
    let dbname = "for_tests/derive_prep_alice_collided";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let network_specs_key = NetworkSpecsKey::from_parts(
        &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .unwrap(),
        &Encryption::Sr25519,
    );
    let mut collision = None;
    for (multisigner, address_details) in
        addresses_set_seed_name_network(dbname, "Alice", &network_specs_key)
            .unwrap()
            .into_iter()
    {
        if address_details.path == "//Alice" {
            collision = Some((multisigner, address_details));
            break;
        }
    }
    let collision = match collision {
        Some(a) => a,
        None => panic!("Did not create address?"),
    };
    let key = derive_prep(
        dbname,
        "Alice",
        &network_specs_key,
        Some(collision),
        "//Alice",
        false,
    )
    .unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
        network_title: "Westend".to_string(),
        network_logo: "westend".to_string(),
        network_specs_key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            .to_string(),
        suggested_derivation: "//Alice".to_string(),
        keyboard: false,
        derivation_check: None,
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn derive_prep_alice_collided_with_password() {
    let dbname = "for_tests/derive_prep_alice_collided_with_password";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let network_specs_key = NetworkSpecsKey::from_parts(
        &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .unwrap(),
        &Encryption::Sr25519,
    );
    try_create_address(
        "Alice",
        ALICE_SEED_PHRASE,
        "//secret///abracadabra",
        &network_specs_key,
        dbname,
    )
    .unwrap();
    let mut collision = None;
    for (multisigner, address_details) in
        addresses_set_seed_name_network(dbname, "Alice", &network_specs_key)
            .unwrap()
            .into_iter()
    {
        if address_details.path == "//secret" {
            collision = Some((multisigner, address_details));
            break;
        }
    }
    let collision = match collision {
        Some(a) => a,
        None => panic!("Did not create address?"),
    };
    let key = derive_prep(
        dbname,
        "Alice",
        &network_specs_key,
        Some(collision),
        "//secret///abracadabra",
        false,
    )
    .unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
        network_title: "Westend".to_string(),
        network_logo: "westend".to_string(),
        network_specs_key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            .to_string(),
        suggested_derivation: "//secret///abracadabra".to_string(),
        keyboard: false,
        derivation_check: None, //TODO "collision":{"base58":"5EkMjdgyuHqnWA9oWXUoFRaMwMUgMJ1ik9KtMpPNuTuZTi2t","path":"//secret","has_pwd":true,"identicon":"<alice_sr25519_//secret///abracadabra>","seed_name":"Alice"}
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn westend_network_details() {
    let dbname = "for_tests/westend_network_details";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let details = network_details_by_key(
        dbname,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();
    let expected_details = MNetworkDetails {
        base58prefix: 42,
        color: "#660D35".to_string(),
        decimals: 12,
        encryption: Encryption::Sr25519,
        genesis_hash: "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            .to_string(),
        logo: "westend".to_string(),
        name: "westend".to_string(),
        order: "2".to_string(),
        path_id: "//westend".to_string(),
        secondary_color: "#262626".to_string(),
        title: "Westend".to_string(),
        unit: "WND".to_string(),
        current_verifier: MVerifier {
            ttype: "general".to_string(),
            details: definitions::navigation::MVerifierDetails {
                public_key: "".to_string(),
                identicon: empty_png().to_vec(),
                encryption: "".to_string(),
            },
        },
        meta: vec![
            MMetadataRecord {
                specname: "westend".to_string(),
                specs_version: "9000".to_string(),
                meta_hash: "e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"
                    .to_string(),
                meta_id_pic: westend_9000().to_vec(),
            },
            MMetadataRecord {
                specname: "westend".to_string(),
                specs_version: "9010".to_string(),
                meta_hash: "70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"
                    .to_string(),
                meta_id_pic: westend_9010().to_vec(),
            },
        ],
    };
    assert_eq!(details, expected_details);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn westend_9010_metadata_details() {
    let dbname = "for_tests/westend_9010_metadata_details";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let network = metadata_details(
        dbname,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        9010,
    )
    .unwrap();
    let expected_network = MManageMetadata {
        name: "westend".to_string(),
        version: "9010".to_string(),
        meta_hash: "70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf".to_string(),
        meta_id_pic: westend_9010().to_vec(),
        networks: vec![MMMNetwork {
            title: "Westend".to_string(),
            logo: "westend".to_string(),
            order: 2,
            current_on_screen: true,
        }],
    };
    assert_eq!(network, expected_network);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn types_status_and_history() {
    let dbname = "for_tests/types_status_and_history";
    populate_cold(dbname, Verifier { v: None }).unwrap();

    let types = show_types_status(dbname).unwrap();
    let mut expected_types = MTypesInfo {
        types_on_file: true,
        types_hash: Some(
            "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string(),
        ),
        types_id_pic: Some(types_known().to_vec()),
    };
    assert_eq!(types, expected_types);

    remove_types_info(dbname).unwrap();
    let types = show_types_status(dbname).unwrap();
    expected_types.types_on_file = false;
    expected_types.types_hash = None;
    expected_types.types_id_pic = None;

    assert_eq!(types, expected_types);

    let history_printed = get_history(dbname).unwrap();
    let expected_element = Event::TypesRemoved {
        types_display: TypesDisplay {
            types_hash: hex::decode(
                "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb",
            )
            .unwrap(),
            verifier: Verifier { v: None },
        },
    };
    assert!(history_printed
        .iter()
        .any(|h| h.1.events.contains(&expected_element)));

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn path_is_known() {
    let dbname = "for_tests/path_is_known";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        dbname,
        "Alice",
        "//Alice",
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    );
    let expected_check = NavDerivationCheck {
        button_good: false,
        where_to: None,
        collision: Some(Address {
            base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            identicon: alice_sr_alice().to_vec(),
            seed_name: "Alice".to_string(),
            multiselect: None,
        }),
        error: None,
    };
    assert_eq!(check, expected_check);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn path_is_unknown() {
    let dbname = "for_tests/path_is_unknown";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        dbname,
        "Alice",
        "//secret",
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    );
    let expected_check = NavDerivationCheck {
        button_good: true,
        where_to: Some(DerivationDestination::Pin),
        collision: None,
        error: None,
    };
    assert_eq!(check, expected_check);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn path_is_unknown_passworded() {
    let dbname = "for_tests/path_is_unknown_passworded";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        dbname,
        "Alice",
        "//secret///abracadabra",
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    );
    let expected_check = NavDerivationCheck {
        button_good: true,
        where_to: Some(DerivationDestination::Pwd),
        collision: None,
        error: None,
    };
    assert_eq!(check, expected_check);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn word_search_1() {
    let word_part = "dri";
    let out = guess(word_part);
    let out_expected = vec![
        "drift".to_string(),
        "drill".to_string(),
        "drink".to_string(),
        "drip".to_string(),
        "drive".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_2() {
    let word_part = "umbra";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{:?}", out);
}

#[test]
fn word_search_3() {
    let word_part = "котик";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{:?}", out);
}

#[test]
fn word_search_4() {
    let word_part = "";
    let out = guess(word_part);
    let out_expected = vec![
        "abandon".to_string(),
        "ability".to_string(),
        "able".to_string(),
        "about".to_string(),
        "above".to_string(),
        "absent".to_string(),
        "absorb".to_string(),
        "abstract".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_5() {
    let word_part = " ";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{:?}", out);
}

#[test]
fn word_search_6() {
    let word_part = "s";
    let out = guess(word_part);
    let out_expected = vec![
        "sad".to_string(),
        "saddle".to_string(),
        "sadness".to_string(),
        "safe".to_string(),
        "sail".to_string(),
        "salad".to_string(),
        "salmon".to_string(),
        "salon".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_7() {
    let word_part = "se";
    let out = guess(word_part);
    let out_expected = vec![
        "sea".to_string(),
        "search".to_string(),
        "season".to_string(),
        "seat".to_string(),
        "second".to_string(),
        "secret".to_string(),
        "section".to_string(),
        "security".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_8() {
    let word_part = "sen";
    let out = guess(word_part);
    let out_expected = vec![
        "senior".to_string(),
        "sense".to_string(),
        "sentence".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn alice_recalls_seed_phrase_1() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("bottom", None);
    seed_draft.added("lake", None);
    // oops, wrong place
    seed_draft.added("drive", Some(1));
    seed_draft.added("obey", Some(2));
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // adding invalid word - should be blocked through UI, expect no changes
    seed_draft.added("занавеска", None);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // removing invalid word - should be blocked through UI, expect no changes
    seed_draft.remove(5);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // removing word
    seed_draft.remove(1);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "obey", "lake"];
    assert_eq!(print, expected_print);
}

#[test]
fn alice_recalls_seed_phrase_2() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("fit", None);
    let print = seed_draft.draft();
    let expected_print = vec!["fit"];
    assert_eq!(print, expected_print);
}

#[test]
fn alice_recalls_seed_phrase_3() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("obe", None);
    let print = seed_draft.draft();
    let expected_print = vec!["obey"];
    assert_eq!(print, expected_print);
}

#[test]
fn get_danger_status_properly() {
    let dbname = "for_tests/get_danger_status_properly";
    populate_cold_release(dbname).unwrap();
    signer_init_no_cert(dbname).unwrap();
    assert!(
        !get_danger_status(dbname).unwrap(),
        "Expected danger status = false after the database initiation."
    );
    device_was_online(dbname).unwrap();
    assert!(
        get_danger_status(dbname).unwrap(),
        "Expected danger status = true after the reported exposure."
    );
    reset_danger_status_to_safe(dbname).unwrap();
    assert!(
        !get_danger_status(dbname).unwrap(),
        "Expected danger status = false after the danger reset."
    );
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn display_general_verifier_properly() {
    let dbname = "for_tests/display_general_verifier_properly";
    populate_cold_release(dbname).unwrap();
    signer_init_no_cert(dbname).unwrap();
    let verifier = get_general_verifier(dbname).unwrap();

    let expected_verifier = Verifier { v: None };
    assert_eq!(verifier, expected_verifier);

    signer_init_with_cert(dbname).unwrap();
    let verifier = get_general_verifier(dbname).unwrap();
    let expected_verifier = Verifier {
        v: Some(VerifierValue::Standard {
            m: MultiSigner::Sr25519(
                Public::try_from(
                    hex::decode("c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57")
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
            ),
        }),
    };
    assert_eq!(verifier, expected_verifier);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn find_westend_verifier() {
    let dbname = "for_tests/find_westend_verifier";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        &hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
    );
    let westend_verifier = try_get_valid_current_verifier(&verifier_key, dbname).unwrap();
    assert_eq!(westend_verifier, Some(ValidCurrentVerifier::General));
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn not_find_mock_verifier() {
    let dbname = "for_tests/not_find_mock_verifier";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        &hex::decode("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap(),
    );
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
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
    {
        let database = open_db::<Signer>(dbname).unwrap();
        let addresses = open_tree::<Signer>(&database, ADDRTREE).unwrap();
        assert_eq!(addresses.len(), 4);
    }
    let chainspecs = default_chainspecs();
    let default_addresses = addresses_set_seed_name_network(
        dbname,
        "Alice",
        &NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519),
    )
    .unwrap();

    let expected_default_addresses = vec![
        (
            MultiSigner::Sr25519(
                Public::try_from(
                    hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a")
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
            ),
            AddressDetails {
                seed_name: "Alice".to_string(),
                path: "".to_string(),
                has_pwd: false,
                network_id: vec![
                    NetworkSpecsKey::from_hex(&hex::encode(&[
                        1, 145, 177, 113, 187, 21, 142, 45, 56, 72, 250, 35, 169, 241, 194, 81,
                        130, 251, 142, 32, 49, 59, 44, 30, 180, 146, 25, 218, 122, 112, 206, 144,
                        195,
                    ]))
                    .unwrap(),
                    NetworkSpecsKey::from_hex(&hex::encode(&[
                        1, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135,
                        15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254,
                    ]))
                    .unwrap(),
                    NetworkSpecsKey::from_hex(&hex::encode(&[
                        1, 225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206,
                        158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62,
                    ]))
                    .unwrap(),
                ],
                encryption: Encryption::Sr25519,
            },
        ),
        (
            MultiSigner::Sr25519(
                Public::try_from(
                    hex::decode("64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05")
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
            ),
            AddressDetails {
                seed_name: "Alice".to_string(),
                path: "//kusama".to_string(),
                has_pwd: false,
                network_id: vec![NetworkSpecsKey::from_hex(&hex::encode(&[
                    1, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15,
                    23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254,
                ]))
                .unwrap()],
                encryption: Encryption::Sr25519,
            },
        ),
    ];

    assert_eq!(default_addresses, expected_default_addresses);

    let database: Db = open(dbname).unwrap();
    let identities: Tree = database.open_tree(ADDRTREE).unwrap();
    let test_key = AddressKey::from_parts(
        &hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(),
        &Encryption::Sr25519,
    )
    .unwrap();
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
    assert!(
        is_passworded("//path///password///password").expect("password///password is a password")
    );
    assert!(!is_passworded("//путь").expect("valid utf8 abomination"));
}

#[test]
fn test_derive() {
    let dbname = "for_tests/test_derive";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    println!(
        "[0]: {:?}, [1]: {:?}",
        chainspecs[0].name, chainspecs[1].name
    );
    let seed_name = "Alice";
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    let network_id_1 =
        NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash, &Encryption::Sr25519);
    let both_networks = vec![network_id_0.to_owned(), network_id_1];
    let only_one_network = vec![network_id_0];

    try_create_seed(seed_name, ALICE_SEED_PHRASE, true, dbname).unwrap();
    let prep_data_1 = {
        create_address::<Signer>(
            dbname,
            &Vec::new(),
            "//Alice",
            &chainspecs[0],
            seed_name,
            ALICE_SEED_PHRASE,
        )
        .unwrap()
    };
    TrDbCold::new()
        .set_addresses(upd_id_batch(Batch::default(), prep_data_1.address_prep)) // modify addresses
        .set_history(events_to_batch::<Signer>(dbname, prep_data_1.history_prep).unwrap()) // add corresponding history
        .apply::<Signer>(dbname)
        .unwrap();
    let prep_data_2 = {
        create_address::<Signer>(
            dbname,
            &Vec::new(),
            "//Alice",
            &chainspecs[1],
            seed_name,
            ALICE_SEED_PHRASE,
        )
        .unwrap()
    };
    TrDbCold::new()
        .set_addresses(upd_id_batch(Batch::default(), prep_data_2.address_prep)) // modify addresses
        .set_history(events_to_batch::<Signer>(dbname, prep_data_2.history_prep).unwrap()) // add corresponding history
        .apply::<Signer>(dbname)
        .unwrap();
    let prep_data_3 = {
        create_address::<Signer>(
            dbname,
            &Vec::new(),
            "//Alice/1",
            &chainspecs[0],
            seed_name,
            ALICE_SEED_PHRASE,
        )
        .unwrap()
    };
    TrDbCold::new()
        .set_addresses(upd_id_batch(Batch::default(), prep_data_3.address_prep)) // modify addresses
        .set_history(events_to_batch::<Signer>(dbname, prep_data_3.history_prep).unwrap()) // add corresponding history
        .apply::<Signer>(dbname)
        .unwrap();
    let identities = get_addresses_by_seed_name(dbname, seed_name).unwrap();
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
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
    let chainspecs = default_chainspecs();
    let network_specs_key_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    let network_specs_key_1 =
        NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash, &Encryption::Sr25519);
    let mut identities = addresses_set_seed_name_network(dbname, "Alice", &network_specs_key_0)
        .expect("Alice should have some addresses by default");
    println!("{:?}", identities);
    let (key0, _) = identities.remove(0); //TODO: this should be root key
    let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
    remove_key(dbname, &key0, &network_specs_key_0).expect("delete an address");
    remove_key(dbname, &key1, &network_specs_key_0).expect("delete another address");
    let identities = addresses_set_seed_name_network(dbname, "Alice", &network_specs_key_0)
        .expect("Alice still should have some addresses after deletion of two");
    for (address_key, _) in identities {
        assert_ne!(address_key, key0);
        assert_ne!(address_key, key1);
    }
    let identities = addresses_set_seed_name_network(dbname, "Alice", &network_specs_key_1)
        .expect("Alice still should have some addresses after deletion of two");
    let mut flag_to_check_key0_remains = false;
    for (address_key, _) in identities {
        if address_key == key0 {
            flag_to_check_key0_remains = true;
        }
        assert_ne!(address_key, key1);
    }
    assert!(
        flag_to_check_key0_remains,
        "An address that should have only lost network was removed entirely"
    );
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn history_with_identities() {
    let dbname = "for_tests/history_with_identities";
    populate_cold_release(dbname).unwrap();
    signer_init_with_cert(dbname).unwrap();
    let history_printed = get_history(dbname).unwrap();
    let element1 = Event::DatabaseInitiated;
    let element2 = Event::GeneralVerifierSet {
        verifier: Verifier {
            v: Some(VerifierValue::Standard {
                m: MultiSigner::Sr25519(
                    Public::try_from(
                        hex::decode(
                            "c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57",
                        )
                        .unwrap()
                        .as_ref(),
                    )
                    .unwrap(),
                ),
            }),
        },
    };
    assert!(history_printed
        .iter()
        .any(|e| e.1.events.contains(&element1)));
    assert!(history_printed
        .iter()
        .any(|e| e.1.events.contains(&element2)));
    try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
    let history_printed_after_create_seed: Vec<_> = get_history(dbname)
        .unwrap()
        .into_iter()
        .map(|e| e.1)
        .collect();

    let element3 = vec![
        Event::SeedCreated {
            seed_created: "Alice".to_string(),
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
                )
                .unwrap(),
                path: String::new(),
                network_genesis_hash: hex::decode(
                    "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
                )
                .unwrap(),
            },
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730",
                )
                .unwrap(),
                path: "//polkadot".to_string(),
                network_genesis_hash: hex::decode(
                    "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
                )
                .unwrap(),
            },
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
                )
                .unwrap(),
                path: "".to_string(),
                network_genesis_hash: hex::decode(
                    "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                )
                .unwrap(),
            },
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05",
                )
                .unwrap(),
                path: "//kusama".to_string(),
                network_genesis_hash: hex::decode(
                    "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                )
                .unwrap(),
            },
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
                )
                .unwrap(),
                path: String::new(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
                )
                .unwrap(),
            },
        },
        Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
                )
                .unwrap(),
                path: "//westend".to_string(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
                )
                .unwrap(),
            },
        },
    ];

    assert!(entries_contain_event(
        &history_printed_after_create_seed,
        &element1
    ));
    assert!(entries_contain_event(
        &history_printed_after_create_seed,
        &element2
    ));

    for (i, e) in element3.iter().enumerate() {
        assert!(
            entries_contain_event(&history_printed_after_create_seed, e),
            "{}-th missing",
            i
        );
    }

    fs::remove_dir_all(dbname).unwrap();
}

fn get_multisigner_path_set(dbname: &str) -> Vec<(MultiSigner, String)> {
    let db = open_db::<Signer>(dbname).unwrap();
    let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
    let mut multisigner_path_set: Vec<(MultiSigner, String)> = Vec::new();
    for a in identities.iter().flatten() {
        let (multisigner, address_details) =
            AddressDetails::process_entry_checked::<Signer>(a).unwrap();
        multisigner_path_set.push((multisigner, address_details.path.to_string()))
    }
    multisigner_path_set
}

#[test]
fn increment_identities_1() {
    let dbname = "for_tests/increment_identities_1";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db::<Signer>(dbname).unwrap();
        let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    assert!(
        multisigner_path_set.len() == 1,
        "Wrong number of identities: {:?}",
        multisigner_path_set
    );
    println!("{}", multisigner_path_set[0].1);
    create_increment_set(
        4,
        &multisigner_path_set[0].0,
        &network_id_0,
        ALICE_SEED_PHRASE,
        dbname,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {:?}",
        multisigner_path_set
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//0")));
    assert!(path_set.contains(&String::from("//Alice//1")));
    assert!(path_set.contains(&String::from("//Alice//2")));
    assert!(path_set.contains(&String::from("//Alice//3")));
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn increment_identities_2() {
    let dbname = "for_tests/increment_identities_2";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db::<Signer>(dbname).unwrap();
        let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
    try_create_address(
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice//1",
        &network_id_0,
        dbname,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    let alice_multisigner_path = multisigner_path_set
        .iter()
        .find(|(_, path)| path == "//Alice")
        .unwrap();
    assert!(
        multisigner_path_set.len() == 2,
        "Wrong number of identities: {:?}",
        multisigner_path_set
    );
    create_increment_set(
        3,
        &alice_multisigner_path.0,
        &network_id_0,
        ALICE_SEED_PHRASE,
        dbname,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {:?}",
        multisigner_path_set
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//2")));
    assert!(path_set.contains(&String::from("//Alice//3")));
    assert!(path_set.contains(&String::from("//Alice//4")));
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn increment_identities_3() {
    let dbname = "for_tests/increment_identities_3";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db::<Signer>(dbname).unwrap();
        let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
    try_create_address(
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice//1",
        &network_id_0,
        dbname,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    let alice_multisigner_path = multisigner_path_set
        .iter()
        .find(|(_, path)| path == "//Alice//1")
        .unwrap();
    assert!(
        multisigner_path_set.len() == 2,
        "Wrong number of identities: {:?}",
        multisigner_path_set
    );
    create_increment_set(
        3,
        &alice_multisigner_path.0,
        &network_id_0,
        ALICE_SEED_PHRASE,
        dbname,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(dbname);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {:?}",
        multisigner_path_set
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//1//0")));
    assert!(path_set.contains(&String::from("//Alice//1//1")));
    assert!(path_set.contains(&String::from("//Alice//1//2")));
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn checking_derivation_set() {
    assert!(check_derivation_set(&[
        "/0".to_string(),
        "//Alice/westend".to_string(),
        "//secret//westend".to_string()
    ])
    .is_ok());
    assert!(check_derivation_set(&[
        "/0".to_string(),
        "/0".to_string(),
        "//Alice/westend".to_string(),
        "//secret//westend".to_string()
    ])
    .is_ok());
    assert!(check_derivation_set(&["//remarkably///ugly".to_string()]).is_err());
    assert!(check_derivation_set(&["no_path_at_all".to_string()]).is_err());
    assert!(check_derivation_set(&["///".to_string()]).is_err());
}

#[test]
fn creating_derivation_1() {
    let dbname = "for_tests/creating_derivation_1";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::NoPassword(Some(_)) =
        derivation_check("Alice", "//Alice", &network_id_0, dbname).unwrap()
    {
        println!("Found existing");
    } else {
        panic!("Derivation should already exist.");
    }
    match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice derivation again."),
            Err(e) => assert_eq!(<Signer>::show(&e), "Error generating address. Seed Alice already has derivation //Alice for network specs key 01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d.".to_string()),
        }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn creating_derivation_2() {
    let dbname = "for_tests/creating_derivation_2";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
            dbname
        )
        .is_ok(),
        "Should be able to create //Alice/// secret derivation."
    );
    if let DerivationCheck::NoPassword(None) =
        derivation_check("Alice", "//Alice", &network_id_0, dbname).unwrap()
    {
        println!("It did well.");
    } else {
        panic!(
            "New derivation has no password, existing derivation has password and is diffenent."
        );
    }
    assert!(
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(),
        "Should be able to create //Alice derivation."
    );
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn creating_derivation_3() {
    let dbname = "for_tests/creating_derivation_3";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::Password =
        derivation_check("Alice", "//Alice///secret", &network_id_0, dbname).unwrap()
    {
        println!("It did well.");
    } else {
        panic!(
            "New derivation has password, existing derivation has no password and is diffenent."
        );
    }
    assert!(
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
            dbname
        )
        .is_ok(),
        "Should be able to create //Alice///secret derivation."
    );
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn creating_derivation_4() {
    let dbname = "for_tests/creating_derivation_4";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret1",
            &network_id_0,
            dbname
        )
        .is_ok(),
        "Should be able to create //Alice///secret1 derivation."
    );
    if let DerivationCheck::Password =
        derivation_check("Alice", "//Alice///secret2", &network_id_0, dbname).unwrap()
    {
        println!("It did well.");
    } else {
        panic!("Existing derivation has different password.");
    }
    assert!(
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret2",
            &network_id_0,
            dbname
        )
        .is_ok(),
        "Should be able to create //Alice///secret2 derivation."
    );
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn creating_derivation_5() {
    let dbname = "for_tests/creating_derivation_5";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
            dbname
        )
        .is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::Password =
        derivation_check("Alice", "//Alice///secret", &network_id_0, dbname).unwrap()
    {
        println!("It did well.");
    } else {
        panic!("Derivation exists, but has password.");
    }
    match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice///secret derivation again."),
            Err(e) => assert_eq!(<Signer>::show(&e), "Error generating address. Seed Alice already has derivation //Alice///<password> for network specs key 01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key 08a5e583f74f54f3811cb5f7d74e686d473e3a466fd0e95738707a80c3183b15.".to_string()),
        }
    fs::remove_dir_all(dbname).unwrap();
}

fn insert_metadata_from_file(database_name: &str, filename: &str) {
    let meta_str = std::fs::read_to_string(filename).unwrap();
    let meta_values = MetaValues::from_str_metadata(
        meta_str.trim(),
        IncomingMetadataSourceActiveStr::Default {
            filename: filename.to_string(),
        },
    )
    .unwrap();
    let mut meta_batch = Batch::default();
    meta_batch.insert(
        MetaKey::from_parts(&meta_values.name, meta_values.version).key(),
        meta_values.meta,
    );
    TrDbCold::new()
        .set_metadata(meta_batch)
        .apply::<Active>(database_name)
        .unwrap();
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
    for (meta_key_vec, _) in metadata.iter().flatten() {
        let new = MetaKey::from_ivec(&meta_key_vec)
            .name_version::<Active>()
            .unwrap();
        out.push(new);
    }
    out
}

#[test]
fn test_metadata_transfer() {
    let dbname_hot = "for_tests/test_metadata_transfer_mock_hot";
    reset_hot_database(dbname_hot).unwrap();
    let dbname_cold = "for_tests/test_metadata_transfer_mock_cold";
    populate_cold(dbname_cold, Verifier { v: None }).unwrap();

    insert_metadata_from_file(dbname_hot, "for_tests/westend9010");
    assert!(
        metadata_len(dbname_hot) == 1,
        "Fresh hot database, should have only the single network added."
    );
    assert!(
        format!("{:?}", metadata_contents(dbname_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(dbname_cold)
    );

    transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
    assert!(
        format!("{:?}", metadata_contents(dbname_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(dbname_cold)
    );

    insert_metadata_from_file(dbname_hot, "for_tests/westend9090");
    assert!(metadata_len(dbname_hot) == 2, "Now 2 entries in hot db.");
    transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
    assert!(
        format!("{:?}", metadata_contents(dbname_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("westend", 9090), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(dbname_cold)
    );

    std::fs::remove_dir_all(dbname_hot).unwrap();
    std::fs::remove_dir_all(dbname_cold).unwrap();
}

#[test]
fn test_all_events() {
    let dbname = "for_tests/test_all_events";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let events = all_events_preview();
    enter_events::<Signer>(dbname, events).unwrap();
    let entries: Vec<_> = get_history(dbname)
        .unwrap()
        .into_iter()
        .map(|(_, a)| a)
        .collect();

    assert!(entries_contain_event(
        &entries,
        &Event::MetadataAdded {
            meta_values_display: MetaValuesDisplay {
                name: "westend".to_string(),
                version: 9000,
                meta_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap()
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay {
                name: "westend".to_string(),
                version: 9000,
                meta_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap()
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::MetadataSigned {
            meta_values_export: MetaValuesExport {
                name: "westend".to_string(),
                version: 9000,
                meta_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::NetworkSpecsAdded {
            network_specs_display: NetworkSpecsDisplay {
                specs: NetworkSpecs {
                    base58prefix: 42,
                    color: "#660D35".to_string(),
                    decimals: 12,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                    )
                    .unwrap(),
                    logo: "westend".to_string(),
                    name: "westend".to_string(),
                    order: 3,
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "Westend".to_string(),
                    unit: "WND".to_string(),
                },
                valid_current_verifier: ValidCurrentVerifier::General,
                general_verifier: Verifier {
                    v: Some(
                           VerifierValue::Standard {
                                m: MultiSigner::Sr25519(
                                       Public::try_from(
                                           hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                       )
                                           .unwrap().as_ref()).unwrap())
                    })
                },
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::NetworkSpecsRemoved {
            network_specs_display: NetworkSpecsDisplay {
                specs: NetworkSpecs {
                    base58prefix: 42,
                    color: "#660D35".to_string(),
                    decimals: 12,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                    )
                    .unwrap(),
                    logo: "westend".to_string(),
                    name: "westend".to_string(),
                    order: 3,
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "Westend".to_string(),
                    unit: "WND".to_string(),
                },
                valid_current_verifier: ValidCurrentVerifier::General,
                general_verifier: Verifier {
                    v: Some(VerifierValue::Standard {
                        m: MultiSigner::Sr25519(Public::try_from(
                                   hex::decode(
"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                                   ).unwrap()
                                   .as_ref()
                           ).unwrap())
                    })
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::NetworkSpecsSigned {
            network_specs_export: NetworkSpecsExport {
                specs_to_send: NetworkSpecsToSend {
                    base58prefix: 42,
                    color: "#660D35".to_string(),
                    decimals: 12,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                    )
                    .unwrap(),
                    logo: "westend".to_string(),
                    name: "westend".to_string(),
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "Westend".to_string(),
                    unit: "WND".to_string(),
                },
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::NetworkVerifierSet {
            network_verifier_display: NetworkVerifierDisplay {
                genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap(),
                valid_current_verifier: ValidCurrentVerifier::General,
                general_verifier: Verifier {
                    v: Some(VerifierValue::Standard {
                        m: MultiSigner::Sr25519(
                               Public::try_from(
                                   hex::decode(
                                    "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                                   ).unwrap().as_ref()
                                   ).unwrap()
                               )
                        }
                    )
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::GeneralVerifierSet {
            verifier: Verifier {
                v: Some(VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                })
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::TypesAdded {
            types_display: TypesDisplay {
                types_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap(),
                verifier: Verifier {
                    v: Some(VerifierValue::Standard {
                        m: MultiSigner::Sr25519(
                            Public::try_from(hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            ).unwrap().as_ref()).unwrap()
                        )
                    })
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::TypesRemoved {
            types_display: TypesDisplay {
                types_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap(),
                verifier: Verifier {
                    v: Some(VerifierValue::Standard {
                        m: MultiSigner::Sr25519(
                            Public::try_from(hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            ).unwrap().as_ref()).unwrap()
                        )
                    })
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::TypesSigned {
            types_export: TypesExport {
                types_hash: hex::decode(
                    "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"
                )
                .unwrap(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                }
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::TransactionSigned {
            sign_display: SignDisplay {
                transaction: vec![],
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                },
                user_comment: "send to Alice".to_string(),
            }
        }
    ));

    // TODO: "error":"wrong_password_entered"
    assert!(entries_contain_event(
        &entries,
        &Event::TransactionSignError {
            sign_display: SignDisplay {
                transaction: vec![],
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                },
                user_comment: "send to Alice".to_string(),
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::MessageSigned {
            sign_message_display: SignMessageDisplay {
                message: "This is Alice\nRoger".to_string(),
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                },
                user_comment: "send to Alice".to_string(),
            }
        }
    ));

    // TODO: "error":"wrong_password_entered"
    assert!(entries_contain_event(
        &entries,
        &Event::MessageSignError {
            sign_message_display: SignMessageDisplay {
                message: "This is Alice\nRoger".to_string(),
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            )
                            .unwrap()
                            .as_ref()
                        )
                        .unwrap()
                    )
                },
                user_comment: "send to Alice".to_string(),
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::IdentityAdded {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                )
                .unwrap(),
                path: "//".to_string(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            }
        }
    ));

    assert!(entries_contain_event(
        &entries,
        &Event::IdentityRemoved {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                )
                .unwrap(),
                path: "//".to_string(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            }
        }
    ));

    assert!(entries_contain_event(&entries, &Event::IdentitiesWiped));
    assert!(entries_contain_event(&entries, &Event::DeviceWasOnline));
    assert!(entries_contain_event(&entries, &Event::ResetDangerRecord));
    assert!(entries_contain_event(
        &entries,
        &Event::SeedCreated {
            seed_created: "Alice".to_string()
        }
    ));
    assert!(entries_contain_event(
        &entries,
        &Event::SeedNameWasShown {
            seed_name_was_shown: "AliceSecretSeed".to_string()
        }
    ));
    assert!(entries_contain_event(
        &entries,
        &Event::Warning {
            warning: "Received network information is not verified.".to_string()
        }
    ));
    assert!(entries_contain_event(&entries, &Event::WrongPassword));
    assert!(entries_contain_event(
        &entries,
        &Event::UserEntry {
            user_entry: "Lalala!!!".to_string()
        }
    ));
    assert!(entries_contain_event(
        &entries,
        &Event::SystemEntry {
            system_entry: "Blip blop".to_string()
        }
    ));

    assert!(entries_contain_event(&entries, &Event::HistoryCleared));
    assert!(entries_contain_event(&entries, &Event::DatabaseInitiated));

    std::fs::remove_dir_all(dbname).unwrap();
}

/*
#[test]
fn print_single_event() {
    let dbname = "for_tests/print_single_event";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let entry = get_history_entry_by_order(0, dbname).unwrap();
    let expected_events = vec![
        Event::DatabaseInitiated,
        Event::GeneralVerifierSet {
            verifier: Verifier { v: None },
        },
    ];

    assert_eq!(entry.events, expected_events);

    std::fs::remove_dir_all(dbname).unwrap();
}
*/

fn check_for_network(name: &str, version: u32, dbname: &str) -> bool {
    let database: Db = open(dbname).unwrap();
    let metadata: Tree = database.open_tree(METATREE).unwrap();
    let meta_key = MetaKey::from_parts(name, version);
    metadata.contains_key(meta_key.key()).unwrap()
}

fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().any(|e| e.events.contains(event))
}

#[test]
fn remove_all_westend() {
    let dbname = "for_tests/remove_all_westend";
    populate_cold(dbname, Verifier { v: None }).unwrap();

    let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let network_specs_key =
        NetworkSpecsKey::from_parts(&H256::from_str(genesis_hash).unwrap(), &Encryption::Sr25519);
    remove_network(&network_specs_key, dbname).unwrap();

    {
        let database: Db = open(dbname).unwrap();
        let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
        assert!(
            chainspecs.get(&network_specs_key.key()).unwrap() == None,
            "Westend network specs were not deleted"
        );
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        let prefix_meta = MetaKeyPrefix::from_name("westend");
        assert!(
            metadata.scan_prefix(&prefix_meta.prefix()).next() == None,
            "Some westend metadata was not deleted"
        );
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        for a in identities.iter().flatten() {
            let (_, address_details) = AddressDetails::process_entry_checked::<Signer>(a).unwrap();
            assert!(
                !address_details.network_id.contains(&network_specs_key),
                "Some westend identities still remain."
            );
            assert!(
                !address_details.network_id.is_empty(),
                "Did not remove address key entried with no network keys associated"
            );
        }
    }
    let history: Vec<_> = get_history(dbname)
        .unwrap()
        .into_iter()
        .map(|e| e.1)
        .collect();

    assert!(entries_contain_event(&history, &Event::DatabaseInitiated));
    assert!(entries_contain_event(
        &history,
        &Event::NetworkSpecsRemoved {
            network_specs_display: NetworkSpecsDisplay {
                specs: NetworkSpecs {
                    base58prefix: 42,
                    color: "#660D35".to_string(),
                    decimals: 12,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                    )
                    .unwrap(),
                    logo: "westend".to_string(),
                    name: "westend".to_string(),
                    order: 2,
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "Westend".to_string(),
                    unit: "WND".to_string(),
                },
                valid_current_verifier: ValidCurrentVerifier::General,
                general_verifier: Verifier { v: None },
            }
        }
    ));

    assert!(entries_contain_event(
        &history,
        &Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay {
                name: "westend".to_string(),
                version: 9000,
                meta_hash: hex::decode(
                    "e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"
                )
                .unwrap(),
            }
        }
    ));

    assert!(entries_contain_event(
        &history,
        &Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay {
                name: "westend".to_string(),
                version: 9010,
                meta_hash: hex::decode(
                    "70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"
                )
                .unwrap(),
            }
        }
    ));
    assert!(entries_contain_event(
        &history,
        &Event::IdentityRemoved {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                )
                .unwrap(),
                path: "//westend".to_string(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            },
        }
    ));

    assert!(entries_contain_event(
        &history,
        &Event::IdentityRemoved {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
                )
                .unwrap(),
                path: String::new(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            },
        }
    ));

    assert!(entries_contain_event(
        &history,
        &Event::IdentityRemoved {
            identity_history: IdentityHistory {
                seed_name: "Alice".to_string(),
                encryption: Encryption::Sr25519,
                public_key: hex::decode(
                    "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                )
                .unwrap(),
                path: "//Alice".to_string(),
                network_genesis_hash: hex::decode(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            },
        }
    ));
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn remove_westend_9010() {
    let dbname = "for_tests/remove_westend_9010";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let network_specs_key =
        NetworkSpecsKey::from_parts(&H256::from_str(genesis_hash).unwrap(), &Encryption::Sr25519);
    let network_version = 9010;
    assert!(
        check_for_network("westend", network_version, dbname),
        "No westend 9010 to begin with."
    );
    remove_metadata(&network_specs_key, network_version, dbname).unwrap();
    assert!(
        !check_for_network("westend", network_version, dbname),
        "Westend 9010 not removed."
    );
    fs::remove_dir_all(dbname).unwrap();
}
