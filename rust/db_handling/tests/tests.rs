use constants::test_values::alice_ethereum_polkadot;
use db_handling::cold_default::populate_all_network_specs;
use pretty_assertions::{assert_eq, assert_ne};
use sled::{Batch, Tree};
use sp_core::ecdsa::Public as EcdsaPublic;
use sp_core::sr25519::Public;
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::collections::HashMap;
use std::{convert::TryInto, str::FromStr};

use constants::{
    test_values::{alice_sr_alice, empty_png, types_known, westend_9000, westend_9010},
    ADDRTREE, ALICE_SEED_PHRASE, METATREE, SCHEMA_VERSION, SPECSTREE,
};
use db_handling::Error;
use defaults::default_chainspecs;
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
        DerivationPack, Identicon, MBackup, MDeriveKey, MKeyDetails, MMMNetwork, MMNetwork,
        MManageMetadata, MMetadataRecord, MNetworkDetails, MNetworkMenu, MRawKey, MSCNetworkInfo,
        MTypesInfo, MVerifier, Network, NetworkSpecs, SeedNameCard,
    },
    network_specs::{OrderedNetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue},
    users::AddressDetails,
};

use db_handling::identities::{
    create_key_set, dynamic_derivations_response, get_all_addresses,
    process_dynamic_derivations_v1, validate_key_password,
};
use db_handling::{
    cold_default::{
        populate_cold, populate_cold_no_metadata, signer_init_no_cert, signer_init_with_cert,
    },
    db_transactions::TrDbCold,
    default_cold_release, default_hot,
    helpers::{
        get_danger_status, get_general_verifier, open_tree, remove_metadata, remove_network,
        remove_types_info, transfer_metadata_to_cold, try_get_valid_current_verifier,
    },
    identities::{
        create_increment_set, derivation_check, export_secret_key, get_addresses_by_seed_name,
        remove_key, remove_seed, try_create_address, try_create_seed, DerivationCheck,
    },
    interface_signer::{
        addresses_set_seed_name_network, backup_prep, derive_prep, dynamic_path_check, export_key,
        first_network, get_all_seed_names_with_identicons, metadata_details,
        network_details_by_key, print_all_identities, show_all_networks,
        show_all_networks_with_flag, show_types_status,
    },
    manage_history::{
        device_was_online, enter_events, get_history, get_history_entry_by_order,
        reset_danger_status_to_safe,
    },
};
use definitions::dynamic_derivations::{
    DynamicDerivationRequestInfo, DynamicDerivationsAddressRequestV1,
    DynamicDerivationsAddressResponse, DynamicDerivationsRequestInfo,
};
use definitions::helpers::multisigner_to_public;
use definitions::navigation::MAddressCard;

use db_handling::helpers::assert_db_version;
use tempfile::tempdir;

fn westend_genesis() -> H256 {
    H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap()
}

fn polkadot_genesis() -> H256 {
    H256::from_str("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3").unwrap()
}

fn mythos_genesis() -> H256 {
    H256::from_str("f6ee56e9c5277df5b4ce6ae9983ee88f3cbed27d31beeb98f9f84f997a1ab0b9").unwrap()
}

fn export_secret_key_with_path(path: &str) -> Result<MKeyDetails, Error> {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let ordered_specs = default_chainspecs();
    let spec = ordered_specs
        .into_iter()
        .find(|spec| spec.specs.name == "westend")
        .unwrap()
        .specs;
    let network_id = NetworkSpecsKey::from_parts(&spec.genesis_hash, &spec.encryption);
    let seed_name = "Alice";

    try_create_address(&db, seed_name, ALICE_SEED_PHRASE, path, &network_id).unwrap();
    let identities: Vec<(MultiSigner, AddressDetails)> =
        get_addresses_by_seed_name(&db, seed_name).unwrap();

    let (derivation_multisigner, _) = identities.iter().find(|(_, a)| a.path == path).unwrap();

    export_secret_key(
        &db,
        hex::encode(multisigner_to_public(derivation_multisigner)).as_str(),
        seed_name,
        &hex::encode(network_id.key()),
        ALICE_SEED_PHRASE,
        None,
    )
}

#[test]
fn print_seed_names() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let cards = get_all_seed_names_with_identicons(&db, &[String::from("Alice")]).unwrap();
    let expected_cards = vec![SeedNameCard {
        seed_name: "Alice".to_string(),
        identicon: Identicon::Jdenticon {
            identity: "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5".to_string(),
        },
        used_in_networks: vec!["westend".to_string()],
        derived_keys_count: 1, // "//Alice"
    }];
    assert_eq!(cards, expected_cards);
}

#[test]
fn print_seed_names_with_orphan() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let cards =
        get_all_seed_names_with_identicons(&db, &[String::from("Alice"), String::from("BobGhost")])
            .unwrap();

    let expected_cards = vec![
        SeedNameCard {
            seed_name: "Alice".to_string(),
            identicon: Identicon::Jdenticon {
                identity: "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5".to_string(),
            },
            used_in_networks: vec!["westend".to_string()],
            derived_keys_count: 1,
        },
        SeedNameCard {
            seed_name: "BobGhost".to_string(),
            identicon: Identicon::Dots {
                identity: empty_png(),
            },
            used_in_networks: vec![],
            derived_keys_count: 0,
        },
    ];
    assert_eq!(cards, expected_cards);
}

#[test]
fn print_all_ids() {
    use definitions::navigation::Identicon;
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let keys = print_all_identities(&db).unwrap();

    let expected_keys = vec![MRawKey {
        address_key: concat!(
            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
        .to_string(),
        public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string(),
        address: Address {
            seed_name: "Alice".to_string(),
            identicon: Identicon::Dots {
                identity: alice_sr_alice().to_vec(),
            },
            has_pwd: false,
            path: "//Alice".to_string(),
            secret_exposed: false,
        },
        network_logo: "westend".to_owned(),
    }];

    assert_eq!(keys, expected_keys);
}

#[test]
fn print_show_all_networks_flag_westend() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let menu = show_all_networks_with_flag(
        &db,
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
}

#[test]
fn show_all_networks_no_flag() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let networks = show_all_networks(&db).unwrap();
    let expected_networks = vec![
        MMNetwork {
            key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3".to_string(),
            title: "polkadot".to_string(),
            logo: "polkadot".to_string(),
            order: 0,
            path_id: "//polkadot".to_string(),
        },
        MMNetwork {
            key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe".to_string(),
            title: "kusama".to_string(),
            logo: "kusama".to_string(),
            order: 1,
            path_id: "//kusama".to_string(),
        },
        MMNetwork {
            key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e".to_string(),
            title: "westend".to_string(),
            logo: "westend".to_string(),
            order: 2,
            path_id: "//westend".to_string(),
        },
    ];
    assert_eq!(networks, expected_networks);
}

#[test]
fn first_standard_network() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let specs = first_network(&db).unwrap();
    assert_eq!(specs.unwrap().specs.name, "polkadot");
}

#[test]
fn export_alice_westend() {
    use definitions::navigation::Identicon;
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();

    let pubkey = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let public: [u8; 32] = hex::decode(pubkey).unwrap().try_into().unwrap();
    let key = export_key(
        &db,
        &MultiSigner::Sr25519(Public::from_raw(public)),
        "Alice",
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();

    let expected_addr = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let westend_genesis = hex::encode(westend_genesis());
    // `subkey inspect "ALICE_SEED_PHRASE"`
    let expected_key = MKeyDetails {
        qr: definitions::navigation::QrData::Regular {
            data: format!("substrate:{expected_addr}:0x{westend_genesis}")
                .as_bytes()
                .to_vec(),
        },
        pubkey: pubkey.to_string(),
        base58: expected_addr.to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice().to_vec(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
        network_info: MSCNetworkInfo {
            network_title: "westend".to_string(),
            network_logo: "westend".to_string(),
            network_specs_key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                .to_string(),
        },
    };
    assert_eq!(key, expected_key);
}

#[test]
fn export_alice_mythos() {
    use definitions::navigation::Identicon;
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_all_network_specs(&db).unwrap();

    try_create_seed(&db, "Alice", ALICE_SEED_PHRASE, true).unwrap();

    let derivation_path = "//polkadot";
    let genesis = mythos_genesis();

    try_create_address(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        derivation_path,
        &NetworkSpecsKey::from_parts(&genesis, &Encryption::Ethereum),
    )
    .unwrap();

    let pubkey = "02c08517b1ff9501d42ab480ea6fa1b9b92f0430fb07e4a9575dbb2d5ec6edb6d6";
    let public: [u8; 33] = hex::decode(pubkey).unwrap().try_into().unwrap();
    let key = export_key(
        &db,
        &MultiSigner::Ecdsa(EcdsaPublic::from_raw(public)),
        "Alice",
        &NetworkSpecsKey::from_parts(&genesis, &Encryption::Ethereum),
    )
    .unwrap();

    let expected_addr = "0xe9267b732a8e9c9444e46f3d04d4610a996d682d";
    let hex_genesis = hex::encode(genesis);

    let expected_key = MKeyDetails {
        qr: definitions::navigation::QrData::Regular {
            data: format!("ethereum:{expected_addr}:0x{hex_genesis}")
                .as_bytes()
                .to_vec(),
        },
        pubkey: pubkey.to_string(),
        base58: expected_addr.to_string(),
        address: Address {
            identicon: Identicon::Blockies {
                identity: alice_ethereum_polkadot(),
            },
            seed_name: "Alice".to_string(),
            path: derivation_path.to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
        network_info: MSCNetworkInfo {
            network_title: "mythos".to_string(),
            network_logo: "mythos".to_string(),
            network_specs_key: format!("03{hex_genesis}").to_string(),
        },
    };
    assert_eq!(key, expected_key);
}

#[test]
fn backup_prep_alice() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let backup = backup_prep(&db, "Alice").unwrap();
    let expected_backup = MBackup {
        seed_name: "Alice".to_string(),
        derivations: vec![DerivationPack {
            network_title: "westend".to_string(),
            network_logo: "westend".to_string(),
            network_order: 2.to_string(),
            id_set: vec![DerivationEntry {
                path: "//Alice".to_string(),
                has_pwd: false,
            }],
        }],
    };
    assert_eq!(backup, expected_backup);
}

#[test]
fn derive_prep_alice() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let key = derive_prep(&db, "Alice", None, "//secret//derive", false).unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
    };
    assert_eq!(key, expected_key);
}

#[test]
fn derive_prep_alice_collided() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let network_specs_key = NetworkSpecsKey::from_parts(
        &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .unwrap(),
        &Encryption::Sr25519,
    );
    let mut collision = None;
    for (multisigner, address_details) in
        addresses_set_seed_name_network(&db, "Alice", &network_specs_key)
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
    let key = derive_prep(&db, "Alice", Some(collision), "//Alice", false).unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
    };
    assert_eq!(key, expected_key);
}

#[test]
fn derive_prep_alice_collided_with_password() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let network_specs_key = NetworkSpecsKey::from_parts(
        &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .unwrap(),
        &Encryption::Sr25519,
    );
    try_create_address(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        "//secret///abracadabra",
        &network_specs_key,
    )
    .unwrap();
    let mut collision = None;
    for (multisigner, address_details) in
        addresses_set_seed_name_network(&db, "Alice", &network_specs_key)
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
        &db,
        "Alice",
        Some(collision),
        "//secret///abracadabra",
        false,
    )
    .unwrap();
    let expected_key = MDeriveKey {
        seed_name: "Alice".to_string(),
    };
    assert_eq!(key, expected_key);
}

#[test]
fn westend_network_details() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let details = network_details_by_key(
        &db,
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
                identicon: Identicon::Dots {
                    identity: empty_png().to_vec(),
                },
                encryption: "".to_string(),
            },
        },
        meta: vec![
            MMetadataRecord {
                specname: "westend".to_string(),
                specs_version: "9000".to_string(),
                meta_hash: "e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"
                    .to_string(),
                meta_id_pic: Identicon::Dots {
                    identity: westend_9000().to_vec(),
                },
            },
            MMetadataRecord {
                specname: "westend".to_string(),
                specs_version: "9010".to_string(),
                meta_hash: "70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"
                    .to_string(),
                meta_id_pic: Identicon::Dots {
                    identity: westend_9010().to_vec(),
                },
            },
        ],
    };
    assert_eq!(details, expected_details);
}

#[test]
fn westend_9010_metadata_details() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let network = metadata_details(
        &db,
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
        meta_id_pic: Identicon::Dots {
            identity: westend_9010().to_vec(),
        },
        networks: vec![MMMNetwork {
            title: "Westend".to_string(),
            logo: "westend".to_string(),
            order: 2,
            current_on_screen: true,
        }],
    };
    assert_eq!(network, expected_network);
}

#[test]
fn types_status_and_history() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();

    let types = show_types_status(&db).unwrap();
    let mut expected_types = MTypesInfo {
        types_on_file: true,
        types_hash: Some(
            "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string(),
        ),
        types_id_pic: Some(Identicon::Dots {
            identity: types_known().to_vec(),
        }),
    };
    assert_eq!(types, expected_types);

    remove_types_info(&db).unwrap();
    let types = show_types_status(&db).unwrap();
    expected_types.types_on_file = false;
    expected_types.types_hash = None;
    expected_types.types_id_pic = None;

    assert_eq!(types, expected_types);

    let history_printed = get_history(&db).unwrap();
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
}

#[test]
fn path_is_known() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        &db,
        "Alice",
        "//Alice",
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    );
    let expected_check = NavDerivationCheck {
        button_good: false,
        where_to: None,
        collision: Some(MAddressCard {
            address_key: concat!(
                "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
                "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            )
            .to_string(),
            base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            address: Address {
                path: "//Alice".to_string(),
                has_pwd: false,
                identicon: Identicon::Dots {
                    identity: alice_sr_alice().to_vec(),
                },
                seed_name: "Alice".to_string(),
                secret_exposed: false,
            },
        }),
        error: None,
    };
    assert_eq!(check, expected_check);
}

#[test]
fn path_is_unknown() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        &db,
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
}

#[test]
fn path_is_unknown_passworded() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let check = dynamic_path_check(
        &db,
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
}

#[test]
fn get_danger_status_properly() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    default_cold_release(Some(&db)).unwrap();
    signer_init_no_cert(&db).unwrap();
    assert!(
        !get_danger_status(&db).unwrap(),
        "Expected danger status = false after the database initiation."
    );
    device_was_online(&db).unwrap();
    assert!(
        get_danger_status(&db).unwrap(),
        "Expected danger status = true after the reported exposure."
    );
    reset_danger_status_to_safe(&db).unwrap();
    assert!(
        !get_danger_status(&db).unwrap(),
        "Expected danger status = false after the danger reset."
    );
}

#[test]
fn display_general_verifier_properly() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    default_cold_release(Some(&db)).unwrap();
    signer_init_no_cert(&db).unwrap();
    let verifier = get_general_verifier(&db).unwrap();

    let expected_verifier = Verifier { v: None };
    assert_eq!(verifier, expected_verifier);

    signer_init_with_cert(&db).unwrap();
    let verifier = get_general_verifier(&db).unwrap();
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
}

#[test]
fn find_westend_verifier() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(),
    );
    let westend_verifier = try_get_valid_current_verifier(&db, &verifier_key).unwrap();
    assert_eq!(westend_verifier, Some(ValidCurrentVerifier::General));
}

#[test]
fn not_find_mock_verifier() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let verifier_key = VerifierKey::from_parts(
        H256::from_str("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap(),
    );
    match try_get_valid_current_verifier(&db, &verifier_key) {
        Ok(Some(_)) => panic!("Found network key that should not be in database."),
        Ok(None) => (),
        Err(e) => panic!("Error looking for mock verifier: {e}"),
    }
}

#[test]
fn test_generate_default_addresses_for_alice() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    try_create_seed(&db, "Alice", ALICE_SEED_PHRASE, true).unwrap();
    {
        let addresses = open_tree(&db, ADDRTREE).unwrap();
        assert_eq!(addresses.len(), 1);
    }
    let chainspecs = default_chainspecs();
    let default_addresses = addresses_set_seed_name_network(
        &db,
        "Alice",
        &NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519),
    )
    .unwrap();

    let expected_default_addresses = vec![];

    assert_eq!(default_addresses, expected_default_addresses);

    let identities: Tree = db.open_tree(ADDRTREE).unwrap();
    let test_key = AddressKey::from_parts(
        &hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(),
        &Encryption::Sr25519,
        None,
    )
    .unwrap();
    assert!(identities.contains_key(test_key.key()).unwrap());
}

#[test]
fn test_derive() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let specs = default_chainspecs();
    println!(
        "[0]: {:?}, [1]: {:?}",
        specs[0].specs.name, specs[1].specs.name
    );
    let seed_name = "Alice";
    let network_id_0 =
        NetworkSpecsKey::from_parts(&specs[0].specs.genesis_hash, &specs[0].specs.encryption);
    let network_id_1 =
        NetworkSpecsKey::from_parts(&specs[1].specs.genesis_hash, &specs[1].specs.encryption);

    try_create_seed(&db, seed_name, ALICE_SEED_PHRASE, true).unwrap();
    try_create_address(&db, seed_name, ALICE_SEED_PHRASE, "//Alice", &network_id_0).unwrap();
    try_create_address(&db, seed_name, ALICE_SEED_PHRASE, "//Alice", &network_id_1).unwrap();
    try_create_address(
        &db,
        seed_name,
        ALICE_SEED_PHRASE,
        "//Alice/1",
        &network_id_0,
    )
    .unwrap();

    /*
    Since now only one key in one network, this is not needed.
    let _both_networks = vec![network_id_0.to_owned(), network_id_1];
    let _only_one_network = vec![network_id_0];


    let identities = get_addresses_by_seed_name(&db, seed_name).unwrap();
    println!("{:?}", identities);
    let mut flag0 = false;
    let mut flag1 = false;
    for (_, details) in identities {
        flag0 = flag0 || details.network_id == both_networks;
        flag1 = flag1 || details.network_id == only_one_network;
    }
    assert!(flag0, "Something is wrong with //Alice");
    assert!(flag1, "Something is wrong with //Alice/1");
    */
}

#[test]
fn test_identity_deletion() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    try_create_seed(&db, "Alice", ALICE_SEED_PHRASE, true).unwrap();
    let chainspecs = default_chainspecs();
    let network_specs_key_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    try_create_address(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice",
        &network_specs_key_0,
    )
    .unwrap();
    // let network_specs_key_1 =
    //    NetworkSpecsKey::from_parts(&chainspecs[1].specs.genesis_hash, &Encryption::Sr25519);
    let mut identities = addresses_set_seed_name_network(&db, "Alice", &network_specs_key_0)
        .expect("Alice should have some addresses by default");
    println!("{identities:?}");
    //let (key0, _) = identities.remove(0); //TODO: this should be root key
    let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
                                          //remove_key(&db, &key0, &network_specs_key_0).expect("delete an address");
    remove_key(&db, &key1, &network_specs_key_0).expect("delete another address");
    let identities = addresses_set_seed_name_network(&db, "Alice", &network_specs_key_0)
        .expect("Alice still should have some addresses after deletion of two");
    for (address_key, _) in identities {
        //assert_ne!(address_key, key0);
        assert_ne!(address_key, key1);
    }
}

#[test]
fn history_with_identities() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    default_cold_release(Some(&db)).unwrap();
    signer_init_with_cert(&db).unwrap();
    let history_printed = get_history(&db).unwrap();
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
    try_create_seed(&db, "Alice", ALICE_SEED_PHRASE, true).unwrap();
    let history_printed_after_create_seed: Vec<_> =
        get_history(&db).unwrap().into_iter().map(|e| e.1).collect();

    let element3 = vec![Event::SeedCreated {
        seed_created: "Alice".to_string(),
    }];

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
            "{i}-th missing"
        );
    }
}

#[test]
fn remove_seed_history() {
    let seed_name = "Alice";
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    default_cold_release(Some(&db)).unwrap();

    try_create_seed(&db, seed_name, ALICE_SEED_PHRASE, true).unwrap();
    assert!(remove_seed(&db, "Wrong seed name").is_err());
    remove_seed(&db, seed_name).unwrap();

    let history_printed: Vec<_> = get_history(&db).unwrap().into_iter().map(|e| e.1).collect();
    assert!(entries_contain_event(
        &history_printed,
        &Event::SeedRemoved {
            seed_name: seed_name.to_string(),
        }
    ));
}

fn get_multisigner_path_set(database: &sled::Db) -> Vec<(MultiSigner, String)> {
    let identities = open_tree(database, ADDRTREE).unwrap();
    let mut multisigner_path_set: Vec<(MultiSigner, String)> = Vec::new();
    for a in identities.iter().flatten() {
        let (multisigner, address_details) = AddressDetails::process_entry_checked(a).unwrap();
        multisigner_path_set.push((multisigner, address_details.path.to_string()))
    }
    multisigner_path_set
}

#[test]
fn increment_identities_1() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    {
        let identities = open_tree(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0).unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    assert!(
        multisigner_path_set.len() == 1,
        "Wrong number of identities: {multisigner_path_set:?}"
    );
    println!("{}", multisigner_path_set[0].1);
    create_increment_set(
        &db,
        4,
        &multisigner_path_set[0].0,
        &network_id_0,
        ALICE_SEED_PHRASE,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {multisigner_path_set:?}"
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//0")));
    assert!(path_set.contains(&String::from("//Alice//1")));
    assert!(path_set.contains(&String::from("//Alice//2")));
    assert!(path_set.contains(&String::from("//Alice//3")));
}

#[test]
fn increment_identities_2() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    {
        let identities = open_tree(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0).unwrap();
    try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0).unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    let alice_multisigner_path = multisigner_path_set
        .iter()
        .find(|(_, path)| path == "//Alice")
        .unwrap();
    assert!(
        multisigner_path_set.len() == 2,
        "Wrong number of identities: {multisigner_path_set:?}"
    );
    create_increment_set(
        &db,
        3,
        &alice_multisigner_path.0,
        &network_id_0,
        ALICE_SEED_PHRASE,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {multisigner_path_set:?}"
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//2")));
    assert!(path_set.contains(&String::from("//Alice//3")));
    assert!(path_set.contains(&String::from("//Alice//4")));
}

#[test]
fn increment_identities_3() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    {
        let identities = open_tree(&db, ADDRTREE).unwrap();
        assert!(identities.is_empty());
    }
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0).unwrap();
    try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0).unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    let alice_multisigner_path = multisigner_path_set
        .iter()
        .find(|(_, path)| path == "//Alice//1")
        .unwrap();
    assert!(
        multisigner_path_set.len() == 2,
        "Wrong number of identities: {multisigner_path_set:?}"
    );
    create_increment_set(
        &db,
        3,
        &alice_multisigner_path.0,
        &network_id_0,
        ALICE_SEED_PHRASE,
    )
    .unwrap();
    let multisigner_path_set = get_multisigner_path_set(&db);
    assert!(
        multisigner_path_set.len() == 5,
        "Wrong number of identities after increment: {multisigner_path_set:?}"
    );
    let path_set: Vec<String> = multisigner_path_set
        .iter()
        .map(|(_, path)| path.to_string())
        .collect();
    assert!(path_set.contains(&String::from("//Alice//1//0")));
    assert!(path_set.contains(&String::from("//Alice//1//1")));
    assert!(path_set.contains(&String::from("//Alice//1//2")));
}

#[test]
fn creating_derivation_1() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0,).is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::NoPassword(Some(_)) =
        derivation_check(&db, "Alice", "//Alice", &network_id_0).unwrap()
    {
        println!("Found existing");
    } else {
        panic!("Derivation should already exist.");
    }
    match try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0) {
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
                panic!("expected Error::DerivationExists, got {e:?}");
            }
        }
    }
}

#[test]
fn creating_derivation_2() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
        )
        .is_ok(),
        "Should be able to create //Alice/// secret derivation."
    );
    if let DerivationCheck::NoPassword(None) =
        derivation_check(&db, "Alice", "//Alice", &network_id_0).unwrap()
    {
        println!("It did well.");
    } else {
        panic!(
            "New derivation has no password, existing derivation has password and is diffenent."
        );
    }
    assert!(
        try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0).is_ok(),
        "Should be able to create //Alice derivation."
    );
}

#[test]
fn creating_derivation_3() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(&db, "Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0).is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::Password =
        derivation_check(&db, "Alice", "//Alice///secret", &network_id_0).unwrap()
    {
        println!("It did well.");
    } else {
        panic!(
            "New derivation has password, existing derivation has no password and is diffenent."
        );
    }
    assert!(
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
        )
        .is_ok(),
        "Should be able to create //Alice///secret derivation."
    );
}

#[test]
fn creating_derivation_4() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret1",
            &network_id_0,
        )
        .is_ok(),
        "Should be able to create //Alice///secret1 derivation."
    );
    if let DerivationCheck::Password =
        derivation_check(&db, "Alice", "//Alice///secret2", &network_id_0).unwrap()
    {
        println!("It did well.");
    } else {
        panic!("Existing derivation has different password.");
    }
    assert!(
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret2",
            &network_id_0,
        )
        .is_ok(),
        "Should be able to create //Alice///secret2 derivation."
    );
}

#[test]
fn creating_derivation_5() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let chainspecs = default_chainspecs();
    let network_id_0 =
        NetworkSpecsKey::from_parts(&chainspecs[0].specs.genesis_hash, &Encryption::Sr25519);
    assert!(
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//Alice///secret",
            &network_id_0,
        )
        .is_ok(),
        "Should be able to create //Alice derivation."
    );
    if let DerivationCheck::Password =
        derivation_check(&db, "Alice", "//Alice///secret", &network_id_0).unwrap()
    {
        println!("It did well.");
    } else {
        panic!("Derivation exists, but has password.");
    }
    match try_create_address(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice///secret",
        &network_id_0,
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
                panic!("expected Error::DerivationExists, got {e:?}");
            }
        }
    }
}

fn insert_metadata_from_file(database: &sled::Db, filename: &str) {
    let meta_str = std::fs::read_to_string(filename).unwrap();
    let meta_values = MetaValues::from_str_metadata(meta_str.trim()).unwrap();
    let mut meta_batch = Batch::default();
    meta_batch.insert(
        MetaKey::from_parts(&meta_values.name, meta_values.version).key(),
        meta_values.meta,
    );
    TrDbCold::new()
        .set_metadata(meta_batch)
        .apply(database)
        .unwrap();
}

fn metadata_len(database: &sled::Db) -> usize {
    let metadata = open_tree(database, METATREE).unwrap();
    metadata.len()
}

fn metadata_contents(database: &sled::Db) -> Vec<(String, u32)> {
    let metadata = open_tree(database, METATREE).unwrap();
    let mut out: Vec<(String, u32)> = Vec::new();
    for (meta_key_vec, _) in metadata.iter().flatten() {
        let new = MetaKey::from_ivec(&meta_key_vec).name_version().unwrap();
        out.push(new);
    }
    out
}

#[test]
fn test_metadata_transfer() {
    let dbname_hot = tempdir().unwrap();
    let dbname_cold = tempdir().unwrap();

    let database_hot = sled::open(&dbname_hot).unwrap();
    let database_cold = sled::open(&dbname_cold).unwrap();

    default_hot(Some(&database_hot)).unwrap();
    populate_cold(&database_cold, Verifier { v: None }).unwrap();

    insert_metadata_from_file(&database_hot, "for_tests/westend9010");
    assert!(
        metadata_len(&database_hot) == 1,
        "Fresh hot database, should have only the single network added."
    );
    assert!(
        format!("{:?}", metadata_contents(&database_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(&database_cold)
    );

    transfer_metadata_to_cold(&database_hot, &database_cold).unwrap();
    assert!(
        format!("{:?}", metadata_contents(&database_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(&database_cold)
    );

    insert_metadata_from_file(&database_hot, "for_tests/westend9090");
    assert!(metadata_len(&database_hot) == 2, "Now 2 entries in hot db.");
    transfer_metadata_to_cold(&database_hot, &database_cold).unwrap();
    assert!(
        format!("{:?}", metadata_contents(&database_cold))
            == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("westend", 9090), ("polkadot", 30)]"#,
        "expected: \n{:?}",
        metadata_contents(&database_cold)
    );

    std::fs::remove_dir_all(dbname_hot).unwrap();
    std::fs::remove_dir_all(dbname_cold).unwrap();
}

#[test]
fn test_all_events() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let events = all_events_preview();
    enter_events(&db, events).unwrap();
    let entries: Vec<_> = get_history(&db)
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
                network: OrderedNetworkSpecs {
                    specs: NetworkSpecs{
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
                    unit: "WND".to_string(),},
                order:3,
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
                network: OrderedNetworkSpecs {
                    specs: NetworkSpecs{
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
                    unit: "WND".to_string(),},
                    order:3,
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
                specs_to_send: NetworkSpecs {
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
}

#[test]
fn print_single_event() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let entry = get_history_entry_by_order(&db, 0).unwrap();
    let expected_events = vec![
        Event::DatabaseInitiated,
        Event::GeneralVerifierSet {
            verifier: Verifier { v: None },
        },
    ];

    assert_eq!(entry.events, expected_events);
}

fn check_for_network(database: &sled::Db, name: &str, version: u32) -> bool {
    let metadata: Tree = database.open_tree(METATREE).unwrap();
    let meta_key = MetaKey::from_parts(name, version);
    metadata.contains_key(meta_key.key()).unwrap()
}

fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().any(|e| e.events.contains(event))
}

#[test]
fn remove_all_westend() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();

    let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let network_specs_key =
        NetworkSpecsKey::from_parts(&H256::from_str(genesis_hash).unwrap(), &Encryption::Sr25519);
    remove_network(&db, &network_specs_key).unwrap();

    {
        let chainspecs: Tree = db.open_tree(SPECSTREE).unwrap();
        assert!(
            chainspecs.get(network_specs_key.key()).unwrap().is_none(),
            "Westend network specs were not deleted"
        );
        let metadata: Tree = db.open_tree(METATREE).unwrap();
        let prefix_meta = MetaKeyPrefix::from_name("westend");
        assert!(
            metadata.scan_prefix(prefix_meta.prefix()).next().is_none(),
            "Some westend metadata was not deleted"
        );
        let identities: Tree = db.open_tree(ADDRTREE).unwrap();
        for a in identities.iter().flatten() {
            let (_, address_details) = AddressDetails::process_entry_checked(a).unwrap();
            assert_ne!(
                address_details.network_id.as_ref(),
                Some(&network_specs_key)
            );
        }
    }
    let history: Vec<_> = get_history(&db).unwrap().into_iter().map(|e| e.1).collect();

    assert!(entries_contain_event(&history, &Event::DatabaseInitiated));
    assert!(entries_contain_event(
        &history,
        &Event::NetworkSpecsRemoved {
            network_specs_display: NetworkSpecsDisplay {
                network: OrderedNetworkSpecs {
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
                        path_id: "//westend".to_string(),
                        secondary_color: "#262626".to_string(),
                        title: "Westend".to_string(),
                        unit: "WND".to_string(),
                    },
                    order: 2,
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
}

#[test]
fn remove_westend_9010() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let network_specs_key =
        NetworkSpecsKey::from_parts(&H256::from_str(genesis_hash).unwrap(), &Encryption::Sr25519);
    let network_version = 9010;
    assert!(
        check_for_network(&db, "westend", network_version),
        "No westend 9010 to begin with."
    );
    remove_metadata(&db, &network_specs_key, network_version).unwrap();
    assert!(
        !check_for_network(&db, "westend", network_version),
        "Westend 9010 not removed."
    );
}

#[test]
fn test_export_secret_key() {
    let dbname = tempdir().unwrap();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let ordered_specs = default_chainspecs();
    let spec = ordered_specs
        .into_iter()
        .find(|spec| spec.specs.name == "westend")
        .unwrap()
        .specs;
    let network_id = NetworkSpecsKey::from_parts(&spec.genesis_hash, &spec.encryption);
    let seed_name = "Alice";

    let (derivation_path, child_path) = ("//Alice", "//Alice//1");
    try_create_address(&db, seed_name, ALICE_SEED_PHRASE, child_path, &network_id).unwrap();
    let identities: Vec<(MultiSigner, AddressDetails)> =
        get_addresses_by_seed_name(&db, seed_name).unwrap();

    let (derivation_multisigner, _) = identities
        .iter()
        .find(|(_, a)| a.path == derivation_path)
        .unwrap();
    let secret_key = export_secret_key(
        &db,
        hex::encode(multisigner_to_public(derivation_multisigner)).as_str(),
        seed_name,
        &hex::encode(network_id.key()),
        ALICE_SEED_PHRASE,
        None,
    )
    .unwrap();

    // `subkey inspect "ALICE_SEED_PHRASE//Alice"`
    assert_eq!(
        String::from_utf8(secret_key.qr.data().to_vec()).unwrap(),
        "secret:0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a:e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e".to_string()
    );
    assert!(secret_key.address.secret_exposed);

    let identities: Vec<(MultiSigner, AddressDetails)> =
        get_addresses_by_seed_name(&db, seed_name).unwrap();
    let (_, child_address) = identities
        .iter()
        .find(|(_, a)| a.path == child_path)
        .unwrap();
    assert!(child_address.secret_exposed);
}

#[test]
fn export_secret_key_hard_derivation() {
    assert!(export_secret_key_with_path("//hard//hardmore").is_ok())
}

#[test]
fn export_secret_key_soft_derivation() {
    assert!(export_secret_key_with_path("/soft").is_ok())
}

#[test]
fn export_secret_key_mixed_derivation() {
    assert!(export_secret_key_with_path("//hard/soft").is_ok())
}

#[test]
fn test_create_key_set_generate_default_addresses() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();

    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();
    let westend_specs_key = "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    create_key_set(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        vec![westend_specs_key.to_owned()],
    )
    .unwrap();
    {
        let addresses = open_tree(&db, ADDRTREE).unwrap();
        assert_eq!(addresses.len(), 2);
    }
    let default_addresses = addresses_set_seed_name_network(
        &db,
        "Alice",
        &NetworkSpecsKey::from_hex(westend_specs_key).unwrap(),
    )
    .unwrap();

    let expected_default_addresses = vec![(
        MultiSigner::Sr25519(
            Public::try_from(
                hex::decode("3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34")
                    .unwrap()
                    .as_ref(),
            )
            .unwrap(),
        ),
        AddressDetails {
            seed_name: "Alice".to_string(),
            path: "//westend".to_string(),
            has_pwd: false,
            network_id: Some(NetworkSpecsKey::from_parts(
                &westend_genesis(),
                &Encryption::Sr25519,
            )),
            encryption: Encryption::Sr25519,
            secret_exposed: false,
        },
    )];

    assert_eq!(default_addresses, expected_default_addresses);

    let identities: Tree = db.open_tree(ADDRTREE).unwrap();
    let test_key = AddressKey::from_parts(
        &hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(),
        &Encryption::Sr25519,
        None,
    )
    .unwrap();
    assert!(identities.contains_key(test_key.key()).unwrap());
}

#[test]
fn test_dynamic_derivations() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    create_key_set(&db, "Alice", ALICE_SEED_PHRASE, vec![]).unwrap();

    let multisigner_path_set = get_multisigner_path_set(&db);
    let (seed_multisigner, _) = multisigner_path_set
        .iter()
        .find(|(_q, path)| path.is_empty())
        .unwrap();
    let request = DynamicDerivationsAddressRequestV1 {
        addr: DynamicDerivationsRequestInfo {
            multisigner: seed_multisigner.clone(),
            dynamic_derivations: vec![
                DynamicDerivationRequestInfo {
                    derivation_path: "//dd".to_string(),
                    encryption: Encryption::Sr25519,
                    genesis_hash: polkadot_genesis(),
                },
                DynamicDerivationRequestInfo {
                    derivation_path: "//nonetwork".to_string(),
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::zero(),
                },
            ],
        },
    };
    let mut seeds = HashMap::new();
    seeds.insert("Alice".to_string(), ALICE_SEED_PHRASE.to_string());
    let result = process_dynamic_derivations_v1(&db, seeds.clone(), request.clone()).unwrap();
    assert_eq!(result.key_set.seed_name, "Alice");
    assert_eq!(result.key_set.derivations.len(), 1);
    let derivation = result
        .key_set
        .derivations
        .first()
        .expect("dynamic derivations is missing from result");
    assert_eq!(derivation.path, "//dd");
    assert_eq!(
        derivation.base58,
        "14fiUi4zizXAAWP3HkMuS3qoCSP51owPoGaYhUmwuJZKTHwS"
    );

    let response = dynamic_derivations_response(&request, ALICE_SEED_PHRASE).unwrap();
    match response {
        DynamicDerivationsAddressResponse::V1(r) => {
            let key_set = r.addr;
            assert_eq!(key_set.dynamic_derivations.len(), 2);
            let derivation_1 = key_set.dynamic_derivations.first().unwrap();
            assert_eq!(derivation_1.derivation_path, "//dd");
            assert_eq!(
                derivation_1.public_key,
                MultiSigner::Sr25519(
                    sp_core::sr25519::Public::try_from(
                        hex::decode(
                            "a23ba6f64806d989d494723a1178dc101407f03358270219b3e734d68ed2ab2f"
                        )
                        .unwrap()
                        .as_ref()
                    )
                    .unwrap()
                )
            );

            let derivation_2 = key_set.dynamic_derivations.get(1).unwrap();
            assert_eq!("//nonetwork", derivation_2.derivation_path);
            assert_eq!(
                MultiSigner::Sr25519(
                    sp_core::sr25519::Public::try_from(
                        hex::decode(
                            "d438e94ff924a54e473760f058f9329ebbe59abcf2b6cb92a4c9914cf3855571"
                        )
                        .unwrap()
                        .as_ref()
                    )
                    .unwrap()
                ),
                derivation_2.public_key,
            );
        }
    }
}

#[test]
fn test_assert_db_version() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();
    populate_cold(&db, Verifier { v: None }).unwrap();
    assert!(assert_db_version(&db).is_ok());
}

#[test]
fn test_assert_empty_db_version() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();
    assert!(matches!(
        assert_db_version(&db),
        Err(Error::DbSchemaMismatch { found: 0, .. })
    ));
}

#[test]
fn test_assert_wrong_db_version() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();
    let mut batch = Batch::default();
    batch.insert(SCHEMA_VERSION, u32::MAX.to_be_bytes().to_vec());
    TrDbCold::new().set_settings(batch).apply(&db).unwrap();
    assert!(matches!(
        assert_db_version(&db),
        Err(Error::DbSchemaMismatch {
            found: u32::MAX,
            ..
        })
    ));
}

#[test]
fn test_validate_key_password() {
    let dbname = tempdir().unwrap();
    let db = sled::open(&dbname).unwrap();
    populate_cold_no_metadata(&db, Verifier { v: None }).unwrap();

    let westend_hex = "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let westend_specs_key = NetworkSpecsKey::from_hex(westend_hex).unwrap();
    create_key_set(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        vec![westend_hex.to_string()],
    )
    .unwrap();
    let address_key = AddressKey::from_parts(
        &hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(),
        &Encryption::Sr25519,
        None,
    )
    .unwrap();
    assert!(validate_key_password(&db, &address_key, ALICE_SEED_PHRASE, "").unwrap());

    try_create_address(
        &db,
        "Alice",
        ALICE_SEED_PHRASE,
        "//Alice///password",
        &westend_specs_key,
    )
    .unwrap();

    let ms = get_all_addresses(&db)
        .unwrap()
        .into_iter()
        .find(|(_, a)| a.path == "//Alice")
        .unwrap()
        .0;
    let address_key = AddressKey::new(ms, Some(westend_genesis()));
    assert!(!validate_key_password(&db, &address_key, ALICE_SEED_PHRASE, "wrong_pass").unwrap());
    assert!(validate_key_password(&db, &address_key, ALICE_SEED_PHRASE, "password").unwrap());
}
