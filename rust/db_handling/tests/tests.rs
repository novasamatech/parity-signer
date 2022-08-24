#[cfg(feature = "test")]
use pretty_assertions::{assert_eq, assert_ne};
#[cfg(feature = "test")]
use sled::{open, Batch, Db, Tree};
#[cfg(feature = "test")]
use sp_core::sr25519::Public;
#[cfg(feature = "test")]
use sp_core::H256;
#[cfg(feature = "test")]
use sp_runtime::MultiSigner;
#[cfg(feature = "test")]
use std::{convert::TryInto, fs, path::PathBuf, str::FromStr};

#[cfg(feature = "test")]
use constants::{
    test_values::{
        alice_sr_alice, alice_sr_kusama, alice_sr_polkadot, alice_sr_root,
        alice_sr_secret_abracadabra, alice_sr_westend, alice_westend_root_qr,
        alice_westend_secret_qr, empty_png, types_known, westend_9000, westend_9010,
    },
    ADDRTREE, ALICE_SEED_PHRASE, METATREE, SPECSTREE,
};
#[cfg(feature = "test")]
use db_handling::Error;
#[cfg(feature = "test")]
use defaults::default_chainspecs;
#[cfg(feature = "test")]
use definitions::{
    crypto::Encryption,
    history::{
        all_events_preview, Entry, Event, IdentityHistory, MetaValuesDisplay, MetaValuesExport,
        NetworkSpecsDisplay, NetworkSpecsExport, NetworkVerifierDisplay, SignDisplay,
        SignMessageDisplay, TypesDisplay, TypesExport,
    },
    keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey},
    metadata::MetaValues,
    navigation::{
        Address, DerivationCheck as NavDerivationCheck, DerivationDestination, DerivationEntry,
        DerivationPack, MBackup, MDeriveKey, MKeyDetails, MKeysCard, MMMNetwork, MMNetwork,
        MManageMetadata, MMetadataRecord, MNetworkDetails, MNetworkMenu, MRawKey, MSCNetworkInfo,
        MSeedKeyCard, MTypesInfo, MVerifier, Network, NetworkSpecsToSend, SeedNameCard,
    },
    network_specs::{NetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue},
    users::AddressDetails,
};

#[cfg(feature = "test")]
use db_handling::{
    cold_default::{
        populate_cold, populate_cold_no_metadata, signer_init_no_cert, signer_init_with_cert,
    },
    db_transactions::TrDbCold,
    default_cold_release, default_hot,
    helpers::{
        get_danger_status, get_general_verifier, open_db, open_tree, remove_metadata,
        remove_network, remove_types_info, transfer_metadata_to_cold,
        try_get_valid_current_verifier,
    },
    identities::{
        create_increment_set, derivation_check, export_secret_key, get_addresses_by_seed_name,
        remove_key, remove_seed, try_create_address, try_create_seed, DerivationCheck,
    },
    interface_signer::{
        addresses_set_seed_name_network, backup_prep, derive_prep, dynamic_path_check, export_key,
        first_network, get_all_seed_names_with_identicons, metadata_details,
        network_details_by_key, print_all_identities, print_identities_for_seed_name_and_network,
        show_all_networks, show_all_networks_with_flag, show_types_status,
    },
    manage_history::{
        device_was_online, enter_events, get_history, get_history_entry_by_order,
        reset_danger_status_to_safe,
    },
};

#[cfg(feature = "test")]
#[test]
fn print_seed_names() {
    let dbname = "for_tests/print_seed_names";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let cards = get_all_seed_names_with_identicons(dbname, &[String::from("Alice")]).unwrap();
    let expected_cards = vec![SeedNameCard {
        seed_name: "Alice".to_string(),
        identicon: alice_sr_root().to_vec(),
        derived_keys_count: 4, // "//westend", "//kusama", "//polkadot", "//Alice"
    }];
    assert!(cards == expected_cards, "\nReceived: \n{:?}", cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
            derived_keys_count: 4,
        },
        SeedNameCard {
            seed_name: "BobGhost".to_string(),
            identicon: empty_png().to_vec(),
            derived_keys_count: 0,
        },
    ];
    assert!(cards == expected_cards, "\nReceived: \n{:?}", cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
            secret_exposed: false,
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
            secret_exposed: false,
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
            secret_exposed: false,
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
            secret_exposed: false,
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
            secret_exposed: false,
        },
    ];

    assert_eq!(keys, expected_keys);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
            secret_exposed: false,
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
                secret_exposed: false,
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
                secret_exposed: false,
            },
        ],
    );
    // TODO: "network":{"title":"Westend","logo":"westend"}"#;
    assert_eq!((cards.0, cards.1), expected_cards);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
#[test]
fn first_standard_network() {
    let dbname = "for_tests/first_standard_network";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let specs = first_network(dbname).unwrap();
    assert_eq!(specs.name, "polkadot");
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
            secret_exposed: false,
        },
        network_info: MSCNetworkInfo {
            network_title: "Westend".to_string(),
            network_logo: "westend".to_string(),
        },
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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
        derivation_check: NavDerivationCheck {
            button_good: true,
            where_to: Some(DerivationDestination::Pin),
            ..Default::default()
        },
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
        derivation_check: NavDerivationCheck {
            button_good: false,
            where_to: None,
            collision: Some(Address {
                base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                path: "//Alice".to_string(),
                has_pwd: false,
                identicon: alice_sr_alice().to_vec(),
                seed_name: "Alice".to_string(),
                multiselect: None,
                secret_exposed: false,
            }),
            error: None,
        },
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
        derivation_check: NavDerivationCheck {
            button_good: false,
            where_to: None,
            collision: Some(Address {
                base58: "5EkMjdgyuHqnWA9oWXUoFRaMwMUgMJ1ik9KtMpPNuTuZTi2t".to_string(),
                path: "//secret".to_string(),
                has_pwd: true,
                identicon: alice_sr_secret_abracadabra().to_vec(),
                seed_name: "Alice".to_string(),
                multiselect: None,
                secret_exposed: false,
            }),
            error: None,
        },
    };
    assert_eq!(key, expected_key);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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
        genesis_hash: H256::from_str(
            "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        )
        .unwrap(),
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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
            types_hash: H256::from_str(
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

#[cfg(feature = "test")]
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
            secret_exposed: false,
        }),
        error: None,
    };
    assert_eq!(check, expected_check);
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
#[test]
fn get_danger_status_properly() {
    let dbname = "for_tests/get_danger_status_properly";
    default_cold_release(Some(PathBuf::from(dbname))).unwrap();
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

#[cfg(feature = "test")]
#[test]
fn display_general_verifier_properly() {
    let dbname = "for_tests/display_general_verifier_properly";
    default_cold_release(Some(PathBuf::from(dbname))).unwrap();
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

#[cfg(feature = "test")]
#[test]
fn find_westend_verifier() {
    let dbname = "for_tests/find_westend_verifier";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
    );
    let westend_verifier = try_get_valid_current_verifier(&verifier_key, dbname).unwrap();
    assert_eq!(westend_verifier, Some(ValidCurrentVerifier::General));
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
#[test]
fn not_find_mock_verifier() {
    let dbname = "for_tests/not_find_mock_verifier";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        H256::from_str("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap(),
    );
    match try_get_valid_current_verifier(&verifier_key, dbname) {
        Ok(Some(_)) => panic!("Found network key that should not be in database."),
        Ok(None) => (),
        Err(e) => panic!("Error looking for mock verifier: {}", e),
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
#[test]
fn test_generate_default_addresses_for_alice() {
    let dbname = "for_tests/test_generate_default_addresses_for_Alice";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
    {
        let database = open_db(dbname).unwrap();
        let addresses = open_tree(&database, ADDRTREE).unwrap();
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
                secret_exposed: false,
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
                secret_exposed: false,
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

#[cfg(feature = "test")]
#[test]
fn test_derive() {
    let dbname = "for_tests/test_derive";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let specs = default_chainspecs();
    println!("[0]: {:?}, [1]: {:?}", specs[0].name, specs[1].name);
    let seed_name = "Alice";
    let network_id_0 = NetworkSpecsKey::from_parts(&specs[0].genesis_hash, &specs[0].encryption);
    let network_id_1 = NetworkSpecsKey::from_parts(&specs[1].genesis_hash, &specs[1].encryption);

    try_create_seed(seed_name, ALICE_SEED_PHRASE, true, dbname).unwrap();
    try_create_address(
        seed_name,
        ALICE_SEED_PHRASE,
        "//Alice",
        &network_id_0,
        dbname,
    )
    .unwrap();
    try_create_address(
        seed_name,
        ALICE_SEED_PHRASE,
        "//Alice",
        &network_id_1,
        dbname,
    )
    .unwrap();
    try_create_address(
        seed_name,
        ALICE_SEED_PHRASE,
        "//Alice/1",
        &network_id_0,
        dbname,
    )
    .unwrap();

    let both_networks = vec![network_id_0.to_owned(), network_id_1];
    let only_one_network = vec![network_id_0];

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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
#[test]
fn history_with_identities() {
    let dbname = "for_tests/history_with_identities";
    default_cold_release(Some(PathBuf::from(dbname))).unwrap();
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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

#[cfg(feature = "test")]
#[test]
fn remove_seed_history() {
    let dbname = "for_tests/remove_seed_history";
    let seed_name = "Alice";
    default_cold_release(Some(PathBuf::from(dbname))).unwrap();

    try_create_seed(seed_name, ALICE_SEED_PHRASE, true, dbname).unwrap();
    assert!(remove_seed(dbname, "Wrong seed name").is_err());
    remove_seed(dbname, seed_name).unwrap();

    let history_printed: Vec<_> = get_history(dbname)
        .unwrap()
        .into_iter()
        .map(|e| e.1)
        .collect();
    assert!(entries_contain_event(
        &history_printed,
        &Event::SeedRemoved {
            seed_name: seed_name.to_string(),
        }
    ));
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
fn get_multisigner_path_set(dbname: &str) -> Vec<(MultiSigner, String)> {
    let db = open_db(dbname).unwrap();
    let identities = open_tree(&db, ADDRTREE).unwrap();
    let mut multisigner_path_set: Vec<(MultiSigner, String)> = Vec::new();
    for a in identities.iter().flatten() {
        let (multisigner, address_details) = AddressDetails::process_entry_checked(a).unwrap();
        multisigner_path_set.push((multisigner, address_details.path.to_string()))
    }
    multisigner_path_set
}

#[cfg(feature = "test")]
#[test]
fn increment_identities_1() {
    let dbname = "for_tests/increment_identities_1";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db(dbname).unwrap();
        let identities = open_tree(&db, ADDRTREE).unwrap();
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

#[cfg(feature = "test")]
#[test]
fn increment_identities_2() {
    let dbname = "for_tests/increment_identities_2";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db(dbname).unwrap();
        let identities = open_tree(&db, ADDRTREE).unwrap();
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

#[cfg(feature = "test")]
#[test]
fn increment_identities_3() {
    let dbname = "for_tests/increment_identities_3";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    {
        let db = open_db(dbname).unwrap();
        let identities = open_tree(&db, ADDRTREE).unwrap();
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

#[cfg(feature = "test")]
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
        Err(e) => {
            if let Error::DerivationExists {
                ref multisigner,
                ref address_details,
                ref network_specs_key,
            } = e
            {
                assert_eq!(address_details.seed_name, "Alice".to_string());

                assert_eq!(
                    hex::encode(multisigner.as_ref()),
                    "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string()
                );

                assert_eq!(
                    hex::encode(network_specs_key.key()),
                    "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                        .to_string()
                );
            } else {
                panic!("expected Error::DerivationExists, got {:?}", e);
            }
        }
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
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
    match try_create_address(
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice///secret",
        &network_id_0,
        dbname,
    ) {
        Ok(()) => panic!("Should NOT be able to create //Alice///secret derivation again."),
        Err(e) => {
            if let Error::DerivationExists {
                ref multisigner,
                ref address_details,
                ref network_specs_key,
            } = e
            {
                assert_eq!(address_details.seed_name, "Alice".to_string());

                assert_eq!(
                    hex::encode(multisigner.as_ref()),
                    "08a5e583f74f54f3811cb5f7d74e686d473e3a466fd0e95738707a80c3183b15".to_string(),
                );

                assert_eq!(
                    hex::encode(network_specs_key.key()),
                    "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                        .to_string()
                );
            } else {
                panic!("expected Error::DerivationExists, got {:?}", e);
            }
        }
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
fn insert_metadata_from_file(database_name: &str, filename: &str) {
    let meta_str = std::fs::read_to_string(filename).unwrap();
    let meta_values = MetaValues::from_str_metadata(meta_str.trim()).unwrap();
    let mut meta_batch = Batch::default();
    meta_batch.insert(
        MetaKey::from_parts(&meta_values.name, meta_values.version).key(),
        meta_values.meta,
    );
    TrDbCold::new()
        .set_metadata(meta_batch)
        .apply(database_name)
        .unwrap();
}

#[cfg(feature = "test")]
fn metadata_len(database_name: &str) -> usize {
    let database = open_db(database_name).unwrap();
    let metadata = open_tree(&database, METATREE).unwrap();
    metadata.len()
}

#[cfg(feature = "test")]
fn metadata_contents(database_name: &str) -> Vec<(String, u32)> {
    let database = open_db(database_name).unwrap();
    let metadata = open_tree(&database, METATREE).unwrap();
    let mut out: Vec<(String, u32)> = Vec::new();
    for (meta_key_vec, _) in metadata.iter().flatten() {
        let new = MetaKey::from_ivec(&meta_key_vec).name_version().unwrap();
        out.push(new);
    }
    out
}

#[cfg(feature = "test")]
#[test]
fn test_metadata_transfer() {
    let dbname_hot = "for_tests/test_metadata_transfer_mock_hot";
    default_hot(Some(PathBuf::from(dbname_hot))).unwrap();
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

#[cfg(feature = "test")]
#[test]
fn test_all_events() {
    let dbname = "for_tests/test_all_events";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let events = all_events_preview();
    enter_events(dbname, events).unwrap();
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
                meta_hash: H256::from_str(
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
                meta_hash: H256::from_str(
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
                meta_hash: H256::from_str(
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
                genesis_hash: H256::from_str(
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
                types_hash: H256::from_str(
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
                types_hash: H256::from_str(
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
                types_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
fn check_for_network(name: &str, version: u32, dbname: &str) -> bool {
    let database: Db = open(dbname).unwrap();
    let metadata: Tree = database.open_tree(METATREE).unwrap();
    let meta_key = MetaKey::from_parts(name, version);
    metadata.contains_key(meta_key.key()).unwrap()
}

#[cfg(feature = "test")]
fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().any(|e| e.events.contains(event))
}

#[cfg(feature = "test")]
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
            let (_, address_details) = AddressDetails::process_entry_checked(a).unwrap();
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
                meta_hash: H256::from_str(
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
                meta_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
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
                network_genesis_hash: H256::from_str(
                    "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                )
                .unwrap()
            },
        }
    ));
    fs::remove_dir_all(dbname).unwrap();
}

#[cfg(feature = "test")]
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

#[cfg(feature = "test")]
#[test]
fn test_export_secret_key() {
    let dbname = "for_tests/export_alice_secret";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let specs = default_chainspecs();
    let spec = specs.iter().find(|spec| spec.name == "westend").unwrap();
    let network_id = NetworkSpecsKey::from_parts(&spec.genesis_hash, &spec.encryption);
    let seed_name = "Alice";

    let (derivation_path, child_path) = ("//Alice", "//Alice//1");
    try_create_address(
        seed_name,
        ALICE_SEED_PHRASE,
        child_path,
        &network_id,
        dbname,
    )
    .unwrap();
    let identities: Vec<(MultiSigner, AddressDetails)> =
        get_addresses_by_seed_name(dbname, seed_name).unwrap();

    let (derivation_multisigner, _) = identities
        .iter()
        .find(|(_, a)| a.path == derivation_path)
        .unwrap();
    let secret_key = export_secret_key(
        dbname,
        derivation_multisigner,
        seed_name,
        &network_id,
        ALICE_SEED_PHRASE,
        None,
    )
    .unwrap();

    assert_eq!(secret_key.qr, alice_westend_secret_qr().to_vec());
    assert!(secret_key.address.secret_exposed);

    let identities: Vec<(MultiSigner, AddressDetails)> =
        get_addresses_by_seed_name(dbname, seed_name).unwrap();
    let (_, child_address) = identities
        .iter()
        .find(|(_, a)| a.path == child_path)
        .unwrap();
    assert!(child_address.secret_exposed);

    fs::remove_dir_all(dbname).unwrap();
}
