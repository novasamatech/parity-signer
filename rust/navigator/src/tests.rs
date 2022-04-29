use blake2_rfc::blake2b::blake2b;
use image::{GenericImageView, GrayImage, ImageBuffer, Pixel};
use lazy_static::lazy_static;
use regex::Regex;
use sp_core::{Pair, H256};
use sp_runtime::MultiSigner;
use std::{convert::TryInto, str::FromStr};

use constants::{
    test_values::{
        alice_polkadot_qr, alice_sr_0, alice_sr_1, alice_sr_alice, alice_sr_alice_secret_secret,
        alice_sr_alice_westend, alice_sr_kusama, alice_sr_polkadot, alice_sr_root,
        alice_sr_secret_path_multipass, alice_sr_westend, alice_sr_westend_0, alice_sr_westend_1,
        alice_sr_westend_2, alice_westend_alice_qr, alice_westend_alice_secret_secret_qr,
        alice_westend_root_qr, alice_westend_westend_qr, bob, empty_png, kusama_9130, kusama_9151,
        types_known, westend_9150,
    },
    ALICE_SEED_PHRASE,
};
use db_handling::cold_default::{populate_cold_nav_test, signer_init};
use definitions::{
    crypto::Encryption,
    error_signer::Signer,
    history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay, TypesDisplay},
    navigation::{
        ActionResult, Card, DerivationEntry, DerivationPack, History, MBackup, MDeriveKey,
        MKeyDetails, MKeys, MKeysCard, MLog, MLogDetails, MLogRight, MMMNetwork, MMManageNetworks,
        MMNetwork, MManageNetworks, MMetadataRecord, MNetworkCard, MNetworkDetails, MNetworkMenu,
        MNewSeed, MNewSeedBackup, MPasswordConfirm, MRecoverSeedName, MRecoverSeedPhrase,
        MSCMetaSpecs, MSeedKeyCard, MSeedMenu, MSeeds, MSettings, MSignSufficientCrypto,
        MTransaction, MTypesInfo, MVerifier, MVerifierDetails, ModalData, Network,
        NetworkSpecsToSend, ScreenData, SeedNameCard, SeedWord, TransactionCard,
        TransactionCardSet, TransactionNetworkInfo, TransactionType,
    },
    network_specs::{NetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue},
};

use pretty_assertions::assert_eq;

use crate::{do_action, init_navigation, update_seed_names, Action};

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

fn verifier_alice_sr25519() -> Verifier {
    Verifier {
        v: Some(VerifierValue::Standard {
            m: MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(ALICE)),
        }),
    }
}

lazy_static! {
    static ref NO_TIME: Regex = Regex::new(r#""timestamp":".*?""#).expect("checked construction");
    static ref NO_CHECKSUM: Regex = Regex::new(r#""checksum":"[0-9A-F]*?""#).expect("checked construction");
    static ref SEED: Regex = Regex::new(r#""seed_phrase":"(?P<seed_phrase>.*?)""#).expect("checked construction");
    static ref IDENTICON: Regex = Regex::new(r#""identicon":"[^"(<.+?>)]+?""#).expect("checked construction");
    static ref BASE: Regex = Regex::new(r#""base58":"[^"]+?""#).expect("checked construction");
    static ref ADDRESS_KEY: Regex = Regex::new(r#""address_key":"[^"]+?""#).expect("checked construction");
    static ref PUBLIC_KEY: Regex = Regex::new(r#""public_key":".*?""#).expect("checked construction");
    static ref QR: Regex = Regex::new(r#""qr":"(?P<qr>[0-9a-f]*?)""#).expect("checked construction");
    static ref SUFFICIENT: Regex = Regex::new(r#""sufficient":"(?P<sufficient>[0-9a-f]*?)""#).expect("checked construction");
    static ref SIGNATURE: Regex = Regex::new(r#""signature":"(?P<signature>[0-9a-f]*?)""#).expect("checked construction");
    static ref TEXT: Regex = Regex::new(r#""type":"text","payload":"(?P<text>[0-9a-f]*?)""#).expect("checked construction");
    static ref SET: Regex = Regex::new(r#""set":\[\{"address_key":"01(?P<public>[0-9a-f]*?)","base58":"(?P<base>.*?)","identicon":"(?P<identicon>[0-9a-f]*?)".*?\}\]"#).expect("checked construction");
    static ref KEY0: Regex = Regex::new(r#"\{"address_key":"01(?P<public>[0-9a-f]*?)","base58":"(?P<base>.*?)","identicon":"(?P<identicon>[0-9a-f]*?)","has_pwd":true.*?,"path":"//0".*?\}"#).expect("checked construction");
    static ref OS_MSG: Regex = Regex::new(r#"Os \{[^}]*\}"#).expect("checked_construction");
}

fn timeless(current_real_json: &str) -> String {
    NO_TIME
        .replace_all(current_real_json, r#""timestamp":"**""#)
        .to_string()
}
fn cut_checksum(current_real_json: &str) -> String {
    NO_CHECKSUM
        .replace_all(current_real_json, r#""checksum":"**""#)
        .to_string()
}
fn cut_seed(current_real_json: &str) -> (String, String) {
    let seed_phrase = SEED
        .captures(current_real_json)
        .unwrap()
        .name("seed_phrase")
        .unwrap()
        .as_str()
        .to_string();
    let seedless_json = SEED
        .replace_all(current_real_json, r#""seed_phrase":"**""#)
        .to_string();
    (seedless_json, seed_phrase)
}
fn cut_identicon(current_real_json: &str) -> String {
    IDENTICON
        .replace_all(current_real_json, r#""identicon":"**""#)
        .to_string()
}
fn cut_base58(current_real_json: &str) -> String {
    BASE.replace_all(current_real_json, r#""base58":"**""#)
        .to_string()
}
fn cut_address_key(current_real_json: &str) -> String {
    ADDRESS_KEY
        .replace_all(current_real_json, r#""address_key":"**""#)
        .to_string()
}
fn cut_public_key(current_real_json: &str) -> String {
    PUBLIC_KEY
        .replace_all(current_real_json, r#""public_key":"**""#)
        .to_string()
}
fn cut_os_msg(current_real_json: &str) -> String {
    OS_MSG
        .replace_all(current_real_json, r#"Os {**}"#)
        .to_string()
}

fn qr_payload(qr_content_hex: &str) -> Vec<u8> {
    let qr_content = hex::decode(qr_content_hex).unwrap();
    let image = image::load_from_memory(&qr_content).unwrap();
    let mut gray_img: GrayImage = ImageBuffer::new(image.width(), image.height());
    for y in 0..image.height() {
        for x in 0..image.width() {
            let new_pixel = image.get_pixel(x, y).to_luma();
            gray_img.put_pixel(x, y, new_pixel);
        }
    }
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(image.width() as usize, image.height() as usize, &gray_img);
    codes.last().unwrap().unwrap().decode().unwrap().payload
}

fn process_sufficient(current_real_json: &str) -> (String, String) {
    let sufficient = SUFFICIENT
        .captures(current_real_json)
        .unwrap()
        .name("sufficient")
        .unwrap()
        .as_str()
        .to_string();
    let sufficient_free_json = SUFFICIENT
        .replace_all(current_real_json, r#""sufficient":"**""#)
        .to_string();
    (sufficient_free_json, hex::encode(qr_payload(&sufficient)))
}

fn process_signature(current_real_json: &str) -> (String, String) {
    let signature = SIGNATURE
        .captures(current_real_json)
        .unwrap()
        .name("signature")
        .unwrap()
        .as_str()
        .to_string();
    let signature_free_json = SIGNATURE
        .replace_all(current_real_json, r#""signature":"**""#)
        .to_string();
    (
        signature_free_json,
        String::from_utf8(qr_payload(&signature)).unwrap(),
    )
}

fn get_qr_info_read(current_real_json: &str) -> String {
    let qr_content_hex = QR
        .captures(current_real_json)
        .unwrap()
        .name("qr")
        .unwrap()
        .as_str()
        .to_string();
    String::from_utf8(qr_payload(&qr_content_hex)).unwrap()
}

fn signature_is_good(transaction_hex: &str, signature_hex: &str) -> bool {
    match &transaction_hex[..4] {
        "5300" => {
            assert!(
                signature_hex.starts_with("00"),
                "Signature in ed25519 should start with `00`."
            );
            let into_signature: [u8; 64] = hex::decode(&signature_hex[2..])
                .unwrap()
                .try_into()
                .unwrap();
            let signature = sp_core::ed25519::Signature::from_raw(into_signature);
            let into_public: [u8; 32] = hex::decode(&transaction_hex[6..70])
                .unwrap()
                .try_into()
                .unwrap();
            let public = sp_core::ed25519::Public::from_raw(into_public);
            let message = {
                let to_cut = hex::decode(&transaction_hex[70..transaction_hex.len() - 64]).unwrap();
                if (&transaction_hex[4..6] == "00") || (&transaction_hex[4..6] == "02") {
                    let (method, extensions) = parser::cut_method_extensions(&to_cut).unwrap();
                    [method, extensions].concat()
                } else {
                    to_cut
                }
            };
            let message = {
                if message.len() > 257 {
                    blake2b(32, &[], &message).as_bytes().to_vec()
                } else {
                    message
                }
            };
            sp_core::ed25519::Pair::verify(&signature, &message, &public)
        }
        "5301" => {
            assert!(
                signature_hex.starts_with("01"),
                "Signature in sr25519 should start with `01`."
            );
            let into_signature: [u8; 64] = hex::decode(&signature_hex[2..])
                .unwrap()
                .try_into()
                .unwrap();
            let signature = sp_core::sr25519::Signature::from_raw(into_signature);
            let into_public: [u8; 32] = hex::decode(&transaction_hex[6..70])
                .unwrap()
                .try_into()
                .unwrap();
            let public = sp_core::sr25519::Public::from_raw(into_public);
            let message = {
                let to_cut = hex::decode(&transaction_hex[70..transaction_hex.len() - 64]).unwrap();
                if (&transaction_hex[4..6] == "00") || (&transaction_hex[4..6] == "02") {
                    let (method, extensions) = parser::cut_method_extensions(&to_cut).unwrap();
                    [method, extensions].concat()
                } else {
                    to_cut
                }
            };
            let message = {
                if message.len() > 257 {
                    blake2b(32, &[], &message).as_bytes().to_vec()
                } else {
                    message
                }
            };
            sp_core::sr25519::Pair::verify(&signature, &message, &public)
        }
        "5302" => {
            assert!(
                signature_hex.starts_with("02"),
                "Signature in ecdsa should start with `02`."
            );
            let into_signature: [u8; 65] = hex::decode(&signature_hex[2..])
                .unwrap()
                .try_into()
                .unwrap();
            let signature = sp_core::ecdsa::Signature::from_raw(into_signature);
            let into_public: [u8; 33] = hex::decode(&transaction_hex[6..72])
                .unwrap()
                .try_into()
                .unwrap();
            let public = sp_core::ecdsa::Public::from_raw(into_public);
            let message = {
                let to_cut = hex::decode(&transaction_hex[72..transaction_hex.len() - 64]).unwrap();
                if (&transaction_hex[4..6] == "00") || (&transaction_hex[4..6] == "02") {
                    let (method, extensions) = parser::cut_method_extensions(&to_cut).unwrap();
                    [method, extensions].concat()
                } else {
                    to_cut
                }
            };
            let message = {
                if message.len() > 257 {
                    blake2b(32, &[], &message).as_bytes().to_vec()
                } else {
                    message
                }
            };
            sp_core::ecdsa::Pair::verify(&signature, &message, &public)
        }
        _ => panic!("Transaction has bad format"),
    }
}

fn sr_multisigner_from_hex(hex: &str) -> MultiSigner {
    MultiSigner::Sr25519(
        sp_core::sr25519::Public::try_from(hex::decode(hex).unwrap().as_ref()).unwrap(),
    )
}

fn erase_modal_data_checksum(m: &mut ModalData) {
    if let ModalData::LogRight { f } = m {
        f.checksum = String::new();
    } else {
        panic!("Expected ModalData::LogRight, got {:?}", m);
    }
}

fn erase_log_timestamps(log: &mut ScreenData) {
    match log {
        ScreenData::Log { f } => {
            for entry in f.log.iter_mut() {
                entry.timestamp = String::new();
            }
        }
        ScreenData::LogDetails { f } => {
            f.timestamp = String::new();
        }
        _ => {
            panic!(
                "expected SreenData::Log or ScreenData::LogDetails got {:?}",
                log
            );
        }
    }
}

fn erase_identicon(m: &mut ScreenData) {
    if let ScreenData::SeedSelector { f } = m {
        for seed_name_card in f.seed_name_cards.iter_mut() {
            seed_name_card.identicon = String::new();
        }
    } else {
        panic!("expected ScreenData::SeedSelector, got {:?}", m);
    }
}

fn erase_modal_seed_phrase_and_identicon(m: &mut ModalData) -> String {
    if let ModalData::NewSeedBackup { f } = m {
        let res = f.seed_phrase.clone();
        f.seed_phrase = String::new();
        f.identicon = String::new();
        res
    } else {
        panic!("expected ModalData::NewSeedBackup got {:?}", m);
    }
}

fn erase_base58_address_identicon(m: &mut ScreenData) {
    if let ScreenData::Keys { f } = m {
        for key in f.set.iter_mut() {
            key.identicon = String::new();
            key.base58 = String::new();
            key.address_key = String::new();
        }
        f.root.identicon = String::new();
        f.root.base58 = String::new();
        f.root.address_key = String::new();
    } else {
        panic!("expected ScreenData::Keys got {:?}", m);
    }
}

fn erase_public_keys(m: &mut ScreenData) {
    if let ScreenData::Log { f } = m {
        for entry in f.log.iter_mut() {
            for event in entry.events.iter_mut() {
                if let Event::IdentityAdded { identity_history } = event {
                    identity_history.public_key = vec![];
                }
            }
        }
    }
}

#[test]
fn flow_test_1() {
    let dbname = "for_tests/flow_test_1";
    populate_cold_nav_test(dbname).unwrap();
    signer_init(dbname, verifier_alice_sr25519()).unwrap();
    init_navigation(dbname, "");

    let action = do_action(Action::Start, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "NewSeedMenu".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: ModalData::NewSeedMenu,
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action);

    let mut seed_selector_action = action;

    let mut action = do_action(Action::NavbarLog, "", "").unwrap();

    erase_log_timestamps(&mut action.screen_data);

    let hex = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![
                        Event::DatabaseInitiated,
                        Event::GeneralVerifierSet {
                            verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex),
                                }),
                            },
                        },
                    ],
                }],
                total_entries: 1,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(action, expected_action);

    let mut current_log_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap();

    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with no modals. Expected to remain where was.",
    );

    let mut action = do_action(Action::GoForward, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoForward on Log screen with no modals. Expected to remain where was.",
    );

    let mut action = do_action(Action::RightButton, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    erase_modal_data_checksum(&mut action.modal_data);

    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "LogRight".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![
                        Event::DatabaseInitiated,
                        Event::GeneralVerifierSet {
                            verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex),
                                }),
                            },
                        },
                    ],
                }],
                total_entries: 1,
            },
        },
        modal_data: ModalData::LogRight {
            f: MLogRight {
                checksum: String::new(),
            },
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "RightButton on Log screen with no modals. Expected same Log screen with LogRight modal"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with LogRight modal. Expected to get Log screen with no modals"
    );

    do_action(Action::RightButton, "", "").unwrap();

    let mut action = do_action(Action::CreateLogComment, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "LogComment".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![
                        Event::DatabaseInitiated,
                        Event::GeneralVerifierSet {
                            verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex),
                                }),
                            },
                        },
                    ],
                }],
                total_entries: 1,
            },
        },
        modal_data: ModalData::LogComment,
        alert_data: "{}".to_string(),
    };

    assert_eq!(action, expected_action,
            "CreateLogComment on Log screen with LogRight modal. Expected same Log screen with LogComment modal");
    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with LogComment modal. Expected same Log screen with no modals"
    );

    do_action(Action::RightButton, "", "").unwrap();
    do_action(Action::CreateLogComment, "", "").unwrap();
    let mut action = do_action(Action::GoForward, "Remember this moment", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let mut expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![Event::UserEntry {
                            user_entry: "Remember this moment".to_string(),
                        }],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![
                            Event::DatabaseInitiated,
                            Event::GeneralVerifierSet {
                                verifier: Verifier {
                                    v: Some(VerifierValue::Standard {
                                        m: sr_multisigner_from_hex(hex),
                                    }),
                                },
                            },
                        ],
                    },
                ],
                total_entries: 2,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(action, expected_action, "GoForward on Log screen with LogComment modal. Expected updated Log screen with no modals.");

    let mut action = do_action(Action::Shield, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.alert = "Shield".to_string();
    expected_action.alert_data = "{\"shield_state\":\"unknown\"}".to_string();

    assert_eq!(
        action, expected_action,
        "Shield on Log screen with no modal. Expected same Log screen with Shield alert.",
    );

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![History {
                order: 0,
                timestamp: String::new(),
                events: vec![Event::HistoryCleared],
            }],
            total_entries: 1,
        },
    };
    expected_action.alert = "Empty".to_string();
    expected_action.alert_data = "{}".to_string();
    let log_action = expected_action.clone();
    let empty_log = expected_action.clone();

    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals."
    );

    let action = do_action(Action::NavbarSettings, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("Settings".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Settings".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Settings {
            f: MSettings {
                public_key: Some(
                    "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string(),
                ),
                identicon: Some(alice_sr_alice()),
                encryption: Some("sr25519".to_string()),
                error: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NavbarSettings on Log screen. Expected Settings screen with no modals",
    );

    let current_settings_action = action;

    let action = do_action(Action::BackupSeed, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("SelectSeedForBackup".to_string()),
        screen_label: "Select seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "BackupSeed on Settings screen. Expected SelectSeedForBackup screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on SelectSeedForBackup screen with no seeds available. Expected Settings screen with no modals."
    );

    let action = do_action(Action::ViewGeneralVerifier, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("Verifier".to_string()),
        screen_label: "VERIFIER CERTIFICATE".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::VVerifier {
            f: MVerifierDetails {
                public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    .to_string(),
                identicon: alice_sr_alice(),
                encryption: "sr25519".to_string(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "ViewGeneralVerifier on Settings screen. Expected Verifier screen with no modals.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on Verifier screen. Expected Settings screen with no modals.",
    );

    let action = do_action(Action::ShowDocuments, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("Documents".to_string()),
        screen_label: "ABOUT".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Documents,
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "ShowDocuments on Settings screen. Expected Documents screen with no modals.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on Documents screen. Expected Settings screen with no modals.",
    );

    let action = do_action(Action::ManageNetworks, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("ManageNetworks".to_string()),
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "TypesInfo".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                            .to_string(),
                        title: "Kusama".to_string(),
                        logo: "kusama".to_string(),
                        order: 1,
                    },
                    MMNetwork {
                        key: "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 2,
                    },
                ],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "ManageNetworks on Settings screen. Expected ManageNetworks screen with no modals."
    );

    let mut manage_networks_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on ManageNetworks screen. Expected Settings screen with no modals.",
    );

    do_action(Action::ManageNetworks, "", "").unwrap();
    let action = do_action(
        Action::GoForward,
        "0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
        "",
    )
    .unwrap();
    let expected_action = ActionResult {
        screen: Some("NetworkDetails".to_string()),
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "NDMenu".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
                base58prefix: 2,
                color: "#000".to_string(),
                decimals: 12,
                encryption: Encryption::Sr25519,
                genesis_hash: "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                    .to_string(),
                logo: "kusama".to_string(),
                name: "kusama".to_string(),
                order: "1".to_string(),
                path_id: "//kusama".to_string(),
                secondary_color: "#262626".to_string(),
                title: "Kusama".to_string(),
                unit: "KSM".to_string(),
                current_verifier: MVerifier {
                    ttype: "general".to_string(),
                    details: MVerifierDetails {
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![MMetadataRecord {
                    specs_version: "9130".to_string(),
                    meta_hash: "3e6bf025743e5cc550883170d91c8275fb238762b214922b41d64f9feba23987"
                        .to_string(),
                    meta_id_pic: kusama_9130(),
                }],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "GoForward on ManageNetworks screen with kusama sr25519 key. Expected NetworkDetails screen for kusama with no modals."
    );

    let mut kusama_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, manage_networks_action,
        "GoBack on NetworkDetails screen. Expected ManageNetworks screen with no modals.",
    );

    do_action(
        Action::GoForward,
        "0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
        "",
    )
    .unwrap();
    let action = do_action(Action::ManageMetadata, "9130", "").unwrap();

    let mut kusama_action_modal = kusama_action.clone();
    kusama_action_modal.modal = "ManageMetadata".to_string();
    kusama_action_modal.modal_data = ModalData::ManageNetworks {
        f: MMManageNetworks {
            name: "kusama".to_string(),
            version: "9130".to_string(),
            meta_hash: "3e6bf025743e5cc550883170d91c8275fb238762b214922b41d64f9feba23987"
                .to_string(),
            meta_id_pic: kusama_9130(),
            networks: vec![MMMNetwork {
                title: "Kusama".to_string(),
                logo: "kusama".to_string(),
                order: 1,
                current_on_screen: true,
            }],
        },
    };
    assert_eq!(action, kusama_action_modal, "ManageMetadata on NetworkDetails screen for kusama sr25519 key. Expected NetworkDetails screen for kusama with ManageMetadata modal");

    let action = do_action(Action::SignMetadata, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("SignSufficientCrypto".to_string()),
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto { identities: vec![] },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action, "SignMetadata on NetworkDetails screen for kusama sr25519 key with ManageMetadata modal for version 9130. Expected SignSufficientCrypto screen for kusama9130 metadata with no modals");
    let sign_sufficient_crypto_action = expected_action;

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, kusama_action,
        "GoBack on SignSufficientCrypto screen. Expected NetworkDetails screen with no modals."
    );

    do_action(Action::ManageMetadata, "9130", "").unwrap();
    let action = do_action(Action::RemoveMetadata, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("NetworkDetails".to_string()),
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "NDMenu".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
                base58prefix: 2,
                color: "#000".to_string(),
                decimals: 12,
                encryption: Encryption::Sr25519,
                genesis_hash: "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                    .to_string(),
                logo: "kusama".to_string(),
                name: "kusama".to_string(),
                order: "1".to_string(),
                path_id: "//kusama".to_string(),
                secondary_color: "#262626".to_string(),
                title: "Kusama".to_string(),
                unit: "KSM".to_string(),
                current_verifier: MVerifier {
                    ttype: "general".to_string(),
                    details: MVerifierDetails {
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action, "RemoveMetadata on ManageNetworks screen with kusama sr25519 key with ManageMetadata modal for version 9130. Expected updated NetworkDetails screen for kusama with no modals");

    let kusama_action = action;

    let action = do_action(Action::RightButton, "", "").unwrap();
    let mut expected_action = kusama_action.clone();
    expected_action.modal = "NetworkDetailsMenu".to_string();
    expected_action.modal_data = ModalData::NetworkDetailsMenu;
    assert_eq!(action, expected_action, "RightButton on NetworkDetails screen for kusama sr25519 key. Expected NetworkDetails screen for kusama with NetworkDetailsMenu modal");

    let action = do_action(Action::SignNetworkSpecs, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("SignSufficientCrypto".to_string()),
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto { identities: vec![] },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(action, expected_action, "SignNetworkSpecs on NetworkDetails screen for kusama sr25519 key with NetworkDetailsMenu modal. Expected SignSufficientCrypto screen for kusama specs with no modals.");

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, kusama_action,
        "GoBack on SignSufficientCrypto screen. Expected NetworkDetails screen with no modals."
    );

    do_action(Action::RightButton, "", "").unwrap();
    let action = do_action(Action::RemoveNetwork, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("ManageNetworks".to_string()),
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "TypesInfo".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 1,
                    },
                ],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "RemoveNetwork on NetworkDetails screen for kusama sr25519. Expected updated ManageNetworks screen with no modals"
    );

    let action = do_action(Action::RightButton, "", "").unwrap();
    let mut expected_action = expected_action;
    expected_action.right_button = "TypesInfo".to_string();
    expected_action.modal = "TypesInfo".to_string();
    expected_action.modal_data = ModalData::TypesInfo {
        f: MTypesInfo {
            types_on_file: true,
            types_hash: Some(
                "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string(),
            ),
            types_id_pic: Some(types_known()),
        },
    };

    assert_eq!(
        action, expected_action,
        "RightButton on ManageNetworks screen. Expected ManageNetworks screen with TypesInfo modal"
    );

    let action = do_action(Action::SignTypes, "", "").unwrap();

    let expected_action = sign_sufficient_crypto_action;
    assert_eq!(
        action, expected_action,
        "SignTypes on ManageNetworks screen with TypesInfo modal. Expected SignSufficientCrypto screen for types with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on SignSufficientCrypto screen. Expected Settings screen with no modals",
    );

    do_action(Action::ManageNetworks, "", "").unwrap();
    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::RemoveTypes, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let mut expected_action = log_action.clone();
    let hex_2 = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![
                History {
                    order: 3,
                    timestamp: String::new(),
                    events: vec![Event::TypesRemoved {
                        types_display: TypesDisplay {
                            types_hash: hex::decode(
                                "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb",
                            )
                            .unwrap(),
                            verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex_2),
                                }),
                            },
                        },
                    }],
                },
                History {
                    order: 2,
                    timestamp: String::new(),
                    events: vec![Event::NetworkSpecsRemoved {
                        network_specs_display: NetworkSpecsDisplay {
                            specs: NetworkSpecs {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(
                                    "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                                )
                                .unwrap(),
                                logo: "kusama".to_string(),
                                name: "kusama".to_string(),
                                order: 1,
                                path_id: "//kusama".to_string(),
                                secondary_color: "#262626".to_string(),
                                title: "Kusama".to_string(),
                                unit: "KSM".to_string(),
                            },
                            valid_current_verifier: ValidCurrentVerifier::General,
                            general_verifier: Verifier { v: Some(VerifierValue::Standard { m: sr_multisigner_from_hex(hex_2) })},
                        },
                    }],
                },
                History {
                    order: 1,
                    timestamp: String::new(),
                    events: vec![Event::MetadataRemoved { meta_values_display: MetaValuesDisplay {
                        name: "kusama".to_string(),
                        version: 9130,
                        meta_hash: hex::decode("3e6bf025743e5cc550883170d91c8275fb238762b214922b41d64f9feba23987").unwrap() } }],
                },
              History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                },
            ],
            total_entries: 4,
        },
    };

    assert_eq!(action, expected_action, "RemoveTypes on ManageNetworks screen with TypesInfo modal. Expected Log screen with no modals");

    current_log_action = action;

    let mut action = do_action(Action::ShowLogDetails, "2", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let mut expected_action = ActionResult {
        screen: Some("LogDetails".to_string()),
        screen_label: "Event details".to_string(),
        back: true,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::LogDetails {
            f: MLogDetails {
                timestamp: String::new(),
                events: vec![Event::NetworkSpecsRemoved {
                    network_specs_display: NetworkSpecsDisplay {
                        specs: NetworkSpecs {
                            base58prefix: 2,
                            color: "#000".to_string(),
                            decimals: 12,
                            encryption: Encryption::Sr25519,
                            genesis_hash: H256::from_str(
                                "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
                            )
                            .unwrap(),
                            logo: "kusama".to_string(),
                            name: "kusama".to_string(),
                            order: 1,
                            path_id: "//kusama".to_string(),
                            secondary_color: "#262626".to_string(),
                            title: "Kusama".to_string(),
                            unit: "KSM".to_string(),
                        },
                        valid_current_verifier: ValidCurrentVerifier::General,
                        general_verifier: Verifier {
                            v: Some(VerifierValue::Standard {
                                m: sr_multisigner_from_hex(hex_2),
                            }),
                        },
                    },
                }],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "ShowLogDetails on Log screen with order 2. Expected LogDetails screen with no modals"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, current_log_action,
        "GoBack on ShowLogDetails screen. Expected Log screen with no modals.",
    );

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.screen = Some("Log".to_string());
    expected_action.screen_label = "".to_string();
    expected_action.back = false;
    expected_action.right_button = "LogRight".to_string();
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![History {
                order: 0,
                timestamp: String::new(),
                events: vec![Event::HistoryCleared],
            }],
            total_entries: 1,
        },
    };
    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals."
    );

    let action = do_action(Action::NavbarScan, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("Scan".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Scan".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Scan,
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NavbarScan on Log screen. Expected Scan screen with no modals.",
    );

    let scan_action = action;

    let mut action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/add_specs_kusama-sr25519_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap();
    let aaa = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let genesis_hash = "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";
    let expected_action = ActionResult {
        screen: Some("Transaction".to_string()),
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: "Scan".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key: aaa,
                                identicon: alice_sr_alice(),
                                encryption: "sr25519".to_string(),
                            },
                        },
                    }]),
                    new_specs: Some(vec![TransactionCard {
                        index: 1,
                        indent: 0,
                        card: Card::NewSpecsCard {
                            f: NetworkSpecsToSend {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(genesis_hash).unwrap(),
                                logo: "kusama".to_string(),
                                name: "kusama".to_string(),
                                path_id: "//kusama".to_string(),
                                secondary_color: "#262626".to_string(),
                                title: "Kusama".to_string(),
                                unit: "KSM".to_string(),
                            },
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::Stub,
                author_info: None,
                network_info: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action, "TransactionFetched on Scan screen with add_specs info for kusama. Expected Transaction screen with no modals");

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, scan_action,
        "GoBack on Transaction screen. Expected Scan screen with no modals.",
    );

    do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/add_specs_kusama-sr25519_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap();
    let action = do_action(Action::GoForward, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("NetworkDetails".to_string()),
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "NDMenu".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
                base58prefix: 2,
                color: "#000".to_string(),
                decimals: 12,
                encryption: Encryption::Sr25519,
                genesis_hash: "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                    .to_string(),
                logo: "kusama".to_string(),
                name: "kusama".to_string(),
                order: "2".to_string(),
                path_id: "//kusama".to_string(),
                secondary_color: "#262626".to_string(),
                title: "Kusama".to_string(),
                unit: "KSM".to_string(),
                current_verifier: MVerifier {
                    ttype: "general".to_string(),
                    details: MVerifierDetails {
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "GoForward on Transaction screen with add specs stub. Expected NetworkDetails screen for kusama sr25519, with no modals"
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("ManageNetworks".to_string()),
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "TypesInfo".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 1,
                    },
                    MMNetwork {
                        key: "0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                            .to_string(),
                        title: "Kusama".to_string(),
                        logo: "kusama".to_string(),
                        order: 2,
                    },
                ],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action, "GoBack on NetworkDetails screen after adding kusama sr25519 specs. Expected ManageNetworks screen with no modals.");

    manage_networks_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(action, current_settings_action, "GoBack on ManageNetworks screen, to see footer. Expected known Settings screen with no modals.");

    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let mut expected_action = log_action.clone();
    let hhh = "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![
                History {
                    order: 1,
                    timestamp: String::new(),
                    events: vec![Event::NetworkSpecsAdded {
                        network_specs_display: NetworkSpecsDisplay {
                            specs: NetworkSpecs {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(hhh).unwrap(),
                                logo: "kusama".to_string(),
                                name: "kusama".to_string(),
                                order: 2,
                                path_id: "//kusama".to_string(),
                                secondary_color: "#262626".to_string(),
                                title: "Kusama".to_string(),
                                unit: "KSM".to_string(),
                            },
                            valid_current_verifier: ValidCurrentVerifier::General,
                            general_verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex_2),
                                }),
                            },
                        },
                    }],
                },
                History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                },
            ],
            total_entries: 2,
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::NavbarScan, "", "").unwrap();
    let action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/load_metadata_kusamaV9151_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap();

    let aaa_2 = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let meta_hash = "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46".to_string();
    let expected_action = ActionResult {
        screen: Some("Transaction".to_string()),
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: "Scan".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key: aaa_2,
                                identicon: alice_sr_alice(),
                                encryption: "sr25519".to_string(),
                            },
                        },
                    }]),
                    meta: Some(vec![TransactionCard {
                        index: 1,
                        indent: 0,
                        card: Card::MetaCard {
                            f: MSCMetaSpecs {
                                specname: "kusama".to_string(),
                                spec_version: "9151".to_string(),
                                meta_hash,
                                meta_id_pic: kusama_9151(),
                            },
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::Stub,
                author_info: None,
                network_info: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "TransactionFetched on Scan screen with load_metadata for kusama9151. Expected Transaction screen with no modals"
    );

    let action = do_action(Action::GoForward, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("NetworkDetails".to_string()),
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: "Settings".to_string(),
        right_button: "NDMenu".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
                base58prefix: 2,
                color: "#000".to_string(),
                decimals: 12,
                encryption: Encryption::Sr25519,
                genesis_hash: "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                    .to_string(),
                logo: "kusama".to_string(),
                name: "kusama".to_string(),
                order: "2".to_string(),
                path_id: "//kusama".to_string(),
                secondary_color: "#262626".to_string(),
                title: "Kusama".to_string(),
                unit: "KSM".to_string(),
                current_verifier: MVerifier {
                    ttype: "general".to_string(),
                    details: MVerifierDetails {
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![MMetadataRecord {
                    specs_version: "9151".to_string(),
                    meta_hash: "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46"
                        .to_string(),
                    meta_id_pic: kusama_9151(),
                }],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action,
        "GoForward on Transaction screen with load metadata stub. Expected NetworkDetails screen for kusama sr25519, updated with new metadata, with no modals"
    );

    do_action(Action::GoBack, "", "").unwrap();
    do_action(Action::GoBack, "", "").unwrap();
    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let mut expected_action = log_action.clone();
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![
                History {
                    order: 2,
                    timestamp: String::new(),
                    events: vec![Event::MetadataAdded {
                        meta_values_display: MetaValuesDisplay {
                            name: "kusama".to_string(),
                            version: 9151,
                            meta_hash: hex::decode(
                                "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46",
                            )
                            .unwrap(),
                        },
                    }],
                },
                History {
                    order: 1,
                    timestamp: String::new(),
                    events: vec![Event::NetworkSpecsAdded {
                        network_specs_display: NetworkSpecsDisplay {
                            specs: NetworkSpecs {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(hhh).unwrap(),
                                logo: "kusama".to_string(),
                                name: "kusama".to_string(),
                                order: 2,
                                path_id: "//kusama".to_string(),
                                secondary_color: "#262626".to_string(),
                                title: "Kusama".to_string(),
                                unit: "KSM".to_string(),
                            },
                            valid_current_verifier: ValidCurrentVerifier::General,
                            general_verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex_2),
                                }),
                            },
                        },
                    }],
                },
                History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                },
            ],
            total_entries: 3,
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::NavbarScan, "", "").unwrap();
    let action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/load_types_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap();
    let public_key = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let types_hash =
        Some("d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string());
    let expected_action = ActionResult {
        screen: Some("Transaction".to_string()),
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: "Scan".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key,
                                identicon: alice_sr_alice(),
                                encryption: "sr25519".to_string(),
                            },
                        },
                    }]),
                    warning: Some(vec![TransactionCard {
                        index: 1,
                        indent: 0,
                        card: Card::WarningCard {
                            f: "Updating types (really rare operation).".to_string(),
                        },
                    }]),
                    types_info: Some(vec![TransactionCard {
                        index: 2,
                        indent: 0,
                        card: Card::TypesInfoCard {
                            f: MTypesInfo {
                                types_on_file: false,
                                types_hash,
                                types_id_pic: Some(types_known()),
                            },
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::Stub,
                author_info: None,
                network_info: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "TransactionFetched on Scan screen with load_types. Not that we really need them anymore. Expected Transaction screen with no modals."
    );

    let action = do_action(Action::GoForward, "", "").unwrap();
    assert_eq!(
        action, manage_networks_action,
        "GoForward on Transaction screen with load types stub. Expected known ManageNetworks screen with no modals."
    );

    do_action(Action::GoBack, "", "").unwrap();
    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let mut expected_action = log_action;
    let hex_3 = "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb";
    let hex_4 = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![
                History {
                    order: 3,
                    timestamp: String::new(),
                    events: vec![
                        Event::Warning {
                            warning: "Updating types (really rare operation).".to_string(),
                        },
                        Event::TypesAdded {
                            types_display: TypesDisplay {
                                types_hash: hex::decode(hex_3).unwrap(),
                                verifier: Verifier {
                                    v: Some(VerifierValue::Standard {
                                        m: sr_multisigner_from_hex(hex_4),
                                    }),
                                },
                            },
                        },
                    ],
                },
                History {
                    order: 2,
                    timestamp: String::new(),
                    events: vec![Event::MetadataAdded {
                        meta_values_display: MetaValuesDisplay {
                            name: "kusama".to_string(),
                            version: 9151,
                            meta_hash: hex::decode(
                                "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46",
                            )
                            .unwrap(),
                        },
                    }],
                },
                History {
                    order: 1,
                    timestamp: String::new(),
                    events: vec![Event::NetworkSpecsAdded {
                        network_specs_display: NetworkSpecsDisplay {
                            specs: NetworkSpecs {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(hhh).unwrap(),
                                logo: "kusama".to_string(),
                                name: "kusama".to_string(),
                                order: 2,
                                path_id: "//kusama".to_string(),
                                secondary_color: "#262626".to_string(),
                                title: "Kusama".to_string(),
                                unit: "KSM".to_string(),
                            },
                            valid_current_verifier: ValidCurrentVerifier::General,
                            general_verifier: Verifier {
                                v: Some(VerifierValue::Standard {
                                    m: sr_multisigner_from_hex(hex_2),
                                }),
                            },
                        },
                    }],
                },
                History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                },
            ],
            total_entries: 4,
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals"
    );

    current_log_action = action;

    let action = do_action(Action::NavbarKeys, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "NewSeedMenu".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: ModalData::NewSeedMenu,
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NavbarKeys on Log screen. Expected SeedSelector screen with NewSeedMenu modal",
    );

    let action = do_action(Action::NewSeed, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("NewSeed".to_string()),
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: true },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NewSeed on SeedSelector screen with NewSeedMenu modal. Expected NewSeed screen.",
    );

    let new_seed_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, seed_selector_action,
        "GoBack on NewSeed screen. Expected SeedSelector screen with no modals.",
    );

    do_action(Action::NewSeed, "", "").unwrap();
    let mut action = do_action(Action::GoForward, "Portia", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("NewSeed".to_string()),
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "NewSeedBackup".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: false },
        },
        modal_data: ModalData::NewSeedBackup {
            f: MNewSeedBackup {
                seed: "Portia".to_string(),
                seed_phrase: String::new(),
                identicon: String::new(),
            },
        },
        alert_data: "{}".to_string(),
    };
    erase_modal_seed_phrase_and_identicon(&mut action.modal_data);
    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with non-empty seed name. Expected NewSeed screen with NewSeedBackup modal."
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, new_seed_action,
        "GoBack on NewSeed screen with generated seed. Expected NewSeed screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(action, seed_selector_action, "GoBack on NewSeed screen with no modals, to see footer. Expected known SeedSelector screen with no modals");

    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, current_log_action,
        "Switched to Log from SeedSelector after cancelling seed creation. Expected known Log screen with no modals.",
    );

    do_action(Action::NavbarKeys, "", "").unwrap();
    do_action(Action::NewSeed, "", "").unwrap();
    let mut action = do_action(Action::GoForward, "Portia", "").unwrap();
    let seed_phrase_portia = erase_modal_seed_phrase_and_identicon(&mut action.modal_data);
    let expected_json = r#"{"screen":"NewSeed","screenLabel":"New Seed","back":true,"footer":false,"footerButton":"Keys","rightButton":"None","screenNameType":"h1","modal":"NewSeedBackup","alert":"Empty","screenData":{"keyboard":false},"modalData":{"seed":"Portia","seed_phrase":"**","identicon":"**"},"alertData":{}}"#;
    let expected_action = ActionResult {
        screen: Some("NewSeed".to_string()),
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "NewSeedBackup".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: false },
        },
        modal_data: ModalData::NewSeedBackup {
            f: MNewSeedBackup {
                seed: "Portia".to_string(),
                seed_phrase: String::new(),
                identicon: String::new(),
            },
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with non-empty seed name. Expected NewSeed screen with NewSeedBackup modal."
    );

    let mut action = do_action(Action::GoForward, "true", &seed_phrase_portia).unwrap();
    erase_base58_address_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key: String::new(),
                    base58: String::new(),
                    identicon: String::new(),
                    has_pwd: false,
                    path: "//polkadot".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Portia".to_string(),
                    identicon: String::new(),
                    address_key: String::new(),
                    base58: String::new(),
                    swiped: false,
                    multiselect: false,
                },
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with NewSeedBackup modal active. Expected Keys screen with no modals."
    );

    update_seed_names(r#"Portia"#);

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_identicon(&mut action.screen_data);

    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![SeedNameCard {
                    seed_name: "Portia".to_string(),
                    identicon: String::new(),
                }],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "GoBack on Keys screen. Expected updated SeedSelector screen with no modals.",
    );

    seed_selector_action = action;

    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_public_keys(&mut action.screen_data);
    erase_log_timestamps(&mut action.screen_data);

    let network_genesis_hash_polkadot =
        "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";

    let network_genesis_hash_kusama =
        "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";

    let network_genesis_hash_westend =
        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![
                            Event::SeedCreated {
                                seed_created: "Portia".to_string(),
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: String::new(),
                                    network_genesis_hash: hex::decode(
                                        network_genesis_hash_polkadot,
                                    )
                                    .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: "//polkadot".to_string(),
                                    network_genesis_hash: hex::decode(
                                        network_genesis_hash_polkadot,
                                    )
                                    .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: String::new(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_kusama)
                                        .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: "//kusama".to_string(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_kusama)
                                        .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: String::new(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_westend)
                                        .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Portia".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: vec![],
                                    path: "//westend".to_string(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_westend)
                                        .unwrap(),
                                },
                            },
                        ],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
                total_entries: 2,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(action, expected_action);

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                }],
                total_entries: 1,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals"
    );

    do_action(Action::NavbarKeys, "", "").unwrap();
    do_action(Action::RightButton, "", "").unwrap();
    let action = do_action(Action::RecoverSeed, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedName".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: true,
                seed_name: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "RecoverSeed on SeedSelector screen with NewSeedMenu modal. Expected RecoverSeedName screen with no modals"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_identicon(&mut action.screen_data);
    assert_eq!(
        action, seed_selector_action,
        "GoBack on RecoverSeedName screen with no modals. Expected known SeedSelector screen"
    );

    do_action(Action::RightButton, "", "").unwrap();
    do_action(Action::RecoverSeed, "", "").unwrap();
    let action = do_action(Action::GoForward, "Portia", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedName".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Error".to_string(),
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: false,
                seed_name: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{\"error\":\"Bad input data. Seed name Portia already exists.\"}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "GoForward on RecoverSeedName screen using existing name. Expected RecoverSeedName screen with error."
    );

    do_action(Action::GoBack, "", "").unwrap();
    let action = do_action(Action::GoForward, "Alys", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alys".to_string(),
                user_input: String::new(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "GoForward on RecoverSeedName screen using new name. Expected RecoverSeedPhrase screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedName".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: true,
                seed_name: "Alys".to_string(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "GoBack on RecoverSeedPhrase screen. Expected RecoverSeedName screen with no modals and with retained name."
    );

    let action = do_action(Action::GoForward, "Alice", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: String::new(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on RecoverSeedName screen using new name. ",
            "Expected RecoverSeedPhrase screen with no modals."
        )
    );

    // Alice painstakingly recalls her seed phrase
    let action = do_action(Action::TextEntry, " botto", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "botto".to_string(),
                guess_set: vec!["bottom".to_string()],
                draft: vec![],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word. ",
            "Expected RecoverSeedPhrase screen with no modals."
        )
    );

    let action = do_action(Action::TextEntry, " botto ", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: String::new(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![SeedWord {
                    order: 0,
                    content: "bottom".to_string(),
                }],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word and space. ",
            "Expected word to be added"
        )
    );

    let action = do_action(Action::TextEntry, " abstract ", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: String::new(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![
                    SeedWord {
                        order: 0,
                        content: "bottom".to_string(),
                    },
                    SeedWord {
                        order: 1,
                        content: "abstract".to_string(),
                    },
                ],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with complete long word. ",
            " Wrong one. Expected word to be added"
        )
    );

    let action = do_action(Action::TextEntry, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: String::new(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![SeedWord {
                    order: 0,
                    content: "bottom".to_string(),
                }],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with empty text. ",
            "Expected last draft word to be deleted"
        )
    );

    do_action(Action::TextEntry, " d", "").unwrap();

    // a cat interfered
    let action = do_action(Action::TextEntry, " ddddddddddddddd", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "d".to_string(),
                guess_set: vec![
                    "dad".to_string(),
                    "damage".to_string(),
                    "damp".to_string(),
                    "dance".to_string(),
                    "danger".to_string(),
                    "daring".to_string(),
                    "dash".to_string(),
                    "daughter".to_string(),
                ],
                draft: vec![SeedWord {
                    order: 0,
                    content: "bottom".to_string(),
                }],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with no guesses. ",
            "Expected to keep previous good user entry",
        )
    );

    let action = do_action(Action::TextEntry, " dddddddd ", "").unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with erroneous entry, ",
            "attempted to add to the draft using whitespace. Expected nothing to happen."
        )
    );

    do_action(Action::TextEntry, " driv ", "").unwrap();
    do_action(Action::TextEntry, " obe ", "").unwrap();
    do_action(Action::TextEntry, " lake ", "").unwrap();
    do_action(Action::TextEntry, " curt ", "").unwrap();
    let action = do_action(Action::TextEntry, " som", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "som".to_string(),
                guess_set: vec!["someone".to_string()],
                draft: vec![
                    SeedWord {
                        order: 0,
                        content: "bottom".to_string(),
                    },
                    SeedWord {
                        order: 1,
                        content: "drive".to_string(),
                    },
                    SeedWord {
                        order: 2,
                        content: "obey".to_string(),
                    },
                    SeedWord {
                        order: 3,
                        content: "lake".to_string(),
                    },
                    SeedWord {
                        order: 4,
                        content: "curtain".to_string(),
                    },
                ],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word with typo. ",
            "Expected correct draft and guesses for wrong entry"
        )
    );

    let action = do_action(Action::TextEntry, " smo", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "smo".to_string(),
                guess_set: vec!["smoke".to_string(), "smooth".to_string()],
                draft: vec![
                    SeedWord {
                        order: 0,
                        content: "bottom".to_string(),
                    },
                    SeedWord {
                        order: 1,
                        content: "drive".to_string(),
                    },
                    SeedWord {
                        order: 2,
                        content: "obey".to_string(),
                    },
                    SeedWord {
                        order: 3,
                        content: "lake".to_string(),
                    },
                    SeedWord {
                        order: 4,
                        content: "curtain".to_string(),
                    },
                ],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word. ",
            "Expected correct draft and a few guesses"
        )
    );

    let action = do_action(Action::TextEntry, " smo ", "").unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "Try to enter with whitespace a word with multiple possible endings. ",
            "Expected nothing to happen"
        )
    );

    let action = do_action(Action::PushWord, "smoke", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "".to_string(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![
                    SeedWord {
                        order: 0,
                        content: "bottom".to_string(),
                    },
                    SeedWord {
                        order: 1,
                        content: "drive".to_string(),
                    },
                    SeedWord {
                        order: 2,
                        content: "obey".to_string(),
                    },
                    SeedWord {
                        order: 3,
                        content: "lake".to_string(),
                    },
                    SeedWord {
                        order: 4,
                        content: "curtain".to_string(),
                    },
                    SeedWord {
                        order: 5,
                        content: "smoke".to_string(),
                    },
                ],
                ready_seed: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "PushWord on RecoverSeedPhrase screen. ",
            "Expected correct draft and empty user_input."
        )
    );

    do_action(Action::TextEntry, " bask ", "").unwrap();
    do_action(Action::TextEntry, " hold ", "").unwrap();
    do_action(Action::TextEntry, " race ", "").unwrap();
    do_action(Action::TextEntry, " lone ", "").unwrap();
    do_action(Action::TextEntry, " fit ", "").unwrap();
    let action = do_action(Action::TextEntry, " walk ", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("RecoverSeedPhrase".to_string()),
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "".to_string(),
                guess_set: vec![
                    "abandon".to_string(),
                    "ability".to_string(),
                    "able".to_string(),
                    "about".to_string(),
                    "above".to_string(),
                    "absent".to_string(),
                    "absorb".to_string(),
                    "abstract".to_string(),
                ],
                draft: vec![
                    SeedWord {
                        order: 0,
                        content: "bottom".to_string(),
                    },
                    SeedWord {
                        order: 1,
                        content: "drive".to_string(),
                    },
                    SeedWord {
                        order: 2,
                        content: "obey".to_string(),
                    },
                    SeedWord {
                        order: 3,
                        content: "lake".to_string(),
                    },
                    SeedWord {
                        order: 4,
                        content: "curtain".to_string(),
                    },
                    SeedWord {
                        order: 5,
                        content: "smoke".to_string(),
                    },
                    SeedWord {
                        order: 6,
                        content: "basket".to_string(),
                    },
                    SeedWord {
                        order: 7,
                        content: "hold".to_string(),
                    },
                    SeedWord {
                        order: 8,
                        content: "race".to_string(),
                    },
                    SeedWord {
                        order: 9,
                        content: "lonely".to_string(),
                    },
                    SeedWord {
                        order: 10,
                        content: "fit".to_string(),
                    },
                    SeedWord {
                        order: 11,
                        content: "walk".to_string(),
                    },
                ],
                ready_seed: Some(
                    "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
                        .to_string(),
                ),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen finalizing draft. ",
            "Expected correct full draft and allowed seed phrase to proceed"
        )
    );

    // Woohoo!
    // here the phone gets the finalized allowed seed, and needs to check it with strongbox, to see if the seed phrase already is known
    // can't model it here

    let action = do_action(Action::GoForward, "false", ALICE_SEED_PHRASE).unwrap();
    let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","base58":"16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot","swiped":false,"multiselect":false}],"network":{"title":"Polkadot","logo":"polkadot"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;

    let expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key:
                        "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                            .to_string(),
                    base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                    identicon: alice_sr_polkadot(),
                    has_pwd: false,
                    path: "//polkadot".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                // since root == 'false' in do_action above.
                // TODO: This has to be wrapped with Option<_>.
                root: MSeedKeyCard::default(),
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on RecoverSeedPhrase screen with ",
            "seed phrase (seed phrase vaidity was already checked ",
            "elsewhere - currently in crate Signer). Expected updated ",
            "Keys screen with no modals, with known stable content since ",
            "this is Alice"
        )
    );

    update_seed_names(r#"Portia,Alice"#);

    let mut alice_polkadot_keys_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![
                    SeedNameCard {
                        seed_name: "Alice".to_string(),
                        identicon: String::new(),
                    },
                    SeedNameCard {
                        seed_name: "Portia".to_string(),
                        identicon: String::new(),
                    },
                ],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "GoBack on Keys screen. Expected updated SeedSelector screen with no modals",
    );

    let action = do_action(Action::SelectSeed, "Alice", "").unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        concat!(
            "SelectSeed on SeedSelector screen. ",
            "Expected known Keys screen for Alice polkadot keys"
        )
    );

    let action = do_action(
        Action::SelectKey,
        "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730",
        "",
    )
    .unwrap();

    let expected_action = ActionResult {
        screen: Some("KeyDetails".to_string()),
        screen_label: "Derived Key".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "KeyMenu".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::KeyDetails {
            f: MKeyDetails {
                qr: alice_polkadot_qr(),
                pubkey: "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                    .to_string(),
                base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                identicon: alice_sr_polkadot(),
                seed_name: "Alice".to_string(),
                path: "//polkadot".to_string(),
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "SelectKey on Keys screen. Expected KeyDetails screen for Alice //polkadot key.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        "GoBack on KeyDetails screen. Expected known Keys screen for Alice polkadot keys.",
    );

    let action = do_action(Action::NewKey, "", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("DeriveKey".to_string()),
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Alice".to_string(),
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
                network_specs_key:
                    "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                        .to_string(),
                suggested_derivation: String::new(),
                keyboard: true,
                derivation_check: None,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NewKey on Keys screen. Expected DeriveKey screen",
    );

    let action = do_action(Action::GoBack, "", "").unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        "GoBack on DeriveKey screen. Expected known Keys screen for Alice polkadot keys",
    );

    do_action(Action::NewKey, "", "").unwrap();
    let action = do_action(Action::CheckPassword, "//secret//path///multipass", "").unwrap();
    let expected_action = ActionResult {
        screen: Some("DeriveKey".to_string()),
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: "Keys".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "PasswordConfirm".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Alice".to_string(),
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
                network_specs_key:
                    "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                        .to_string(),
                suggested_derivation: "//secret//path///multipass".to_string(),
                keyboard: false,
                derivation_check: None,
            },
        },
        modal_data: ModalData::PasswordConfirm {
            f: MPasswordConfirm {
                pwd: "multipass".to_string(),
                seed_name: "Alice".to_string(),
                cropped_path: "//secret//path".to_string(),
            },
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "CheckPassword on DeriveKey screen with password ",
            "(path validity and password existence is checked elsewhere). ",
            "Expected updated DeriveKey screen with PasswordConfirm modal"
        )
    );

    // Plaintext secrets in json?

    let action = do_action(
        Action::GoForward,
        "//secret//path///multipass",
        ALICE_SEED_PHRASE,
    )
    .unwrap();

    let expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass(),
                        has_pwd: true,
                        path: "//secret//path".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                        identicon: alice_sr_polkadot(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                // since root == 'false' in do_action above.
                // TODO: This has to be wrapped with Option<_>.
                root: MSeedKeyCard::default(),
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on DeriveKey screen with PasswordConfirm modal. ",
            "Expected updated Keys screen"
        )
    );

    do_action(Action::NewKey, "", "").unwrap();
    // trying to create the missing root
    let action = do_action(Action::GoForward, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass(),
                        has_pwd: true,
                        path: "//secret//path".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                        identicon: alice_sr_polkadot(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root(),
                    address_key:
                        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                            .to_string(),
                    base58: "12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU".to_string(),
                    swiped: false,
                    multiselect: false,
                },
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "GoForward on DeriveKey screen with no modals. Expected updated Keys screen.",
    );

    alice_polkadot_keys_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap();
    erase_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![
                    SeedNameCard {
                        seed_name: "Alice".to_string(),
                        identicon: String::new(),
                    },
                    SeedNameCard {
                        seed_name: "Portia".to_string(),
                        identicon: String::new(),
                    },
                ],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoBack on Keys screen, root Alice have appeared in polkadot. ",
            "Expected updated SeedSelector screen with no modals."
        )
    );

    do_action(Action::SelectSeed, "Alice", "").unwrap();
    let action = do_action(Action::RightButton, "", "").unwrap();
    let mut expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "SeedMenu".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass(),
                        has_pwd: true,
                        path: "//secret//path".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                        identicon: alice_sr_polkadot(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root(),
                    address_key:
                        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                            .to_string(),
                    base58: "12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU".to_string(),
                    swiped: false,
                    multiselect: false,
                },
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::SeedMenu {
            f: MSeedMenu {
                seed: "Alice".to_string(),
            },
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "RightButton on Keys screen. Expected SeedMenu modal to appear",
    );

    let action = do_action(Action::BackupSeed, "", "").unwrap();
    expected_action.modal = "Backup".to_string();
    expected_action.modal_data = ModalData::Backup {
        f: MBackup {
            seed_name: "Alice".to_string(),
            derivations: vec![
                DerivationPack {
                    network_title: "Polkadot".to_string(),
                    network_logo: "polkadot".to_string(),
                    network_order: "0".to_string(),
                    id_set: vec![
                        DerivationEntry {
                            path: "".to_string(),
                            has_pwd: false,
                        },
                        DerivationEntry {
                            path: "//secret//path".to_string(),
                            has_pwd: true,
                        },
                        DerivationEntry {
                            path: "//polkadot".to_string(),
                            has_pwd: false,
                        },
                    ],
                },
                DerivationPack {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                    network_order: "1".to_string(),
                    id_set: vec![DerivationEntry {
                        path: "//westend".to_string(),
                        has_pwd: false,
                    }],
                },
                DerivationPack {
                    network_title: "Kusama".to_string(),
                    network_logo: "kusama".to_string(),
                    network_order: "2".to_string(),
                    id_set: vec![DerivationEntry {
                        path: "//kusama".to_string(),
                        has_pwd: false,
                    }],
                },
            ],
        },
    };
    expected_action.right_button = "None".to_string();
    assert_eq!(
        action, expected_action,
        "BackupSeed on Keys screen with SeedMenu button. Expected Keys screen with Backup modal"
    );
    // mock signal from phone; elsewise untestable;
    db_handling::manage_history::seed_name_was_shown(dbname, String::from("Alice")).unwrap();

    let mut action = do_action(Action::NavbarLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen: Some("Log".to_string()),
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: "Log".to_string(),
        right_button: "LogRight".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 4,
                        timestamp: String::new(),
                        events: vec![Event::SeedNameWasShown { seed_name_was_shown: "Alice".to_string() }]
                    },
                    History {
                        order: 3,
                        timestamp: String::new(),
                        events: vec![Event::IdentityAdded {
                            identity_history: IdentityHistory {
                                seed_name: "Alice".to_string(),
                                encryption: Encryption::Sr25519,
                                public_key: hex::decode(
                                    "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                                ).unwrap(),
                                path: "".to_string(),
                                network_genesis_hash: hex::decode(network_genesis_hash_polkadot)
                                    .unwrap(),
                            },
                        }],
                    },
                    History {
                        order: 2,
                        timestamp: String::new(),
                        events: vec![Event::IdentityAdded {
                            identity_history: IdentityHistory {
                                seed_name: "Alice".to_string(),
                                encryption: Encryption::Sr25519,
                                public_key: hex::decode(
                                    "e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                ).unwrap(),
                                path: "//secret//path".to_string(),
                                network_genesis_hash: hex::decode(network_genesis_hash_polkadot)
                                    .unwrap(),
                            },
                        }],
                    },
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![
                            Event::SeedCreated {
                                seed_created: "Alice".to_string(),
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Alice".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: hex::decode(
                                        "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                    ).unwrap(),
                                    path: "//polkadot".to_string(),
                                    network_genesis_hash: hex::decode(
                                        network_genesis_hash_polkadot,
                                    )
                                    .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Alice".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: hex::decode(
                                        "64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                                    ) .unwrap(),
                                    path: "//kusama".to_string(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_kusama)
                                        .unwrap(),
                                },
                            },
                            Event::IdentityAdded {
                                identity_history: IdentityHistory {
                                    seed_name: "Alice".to_string(),
                                    encryption: Encryption::Sr25519,
                                    public_key: hex::decode(
                                        "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                    ).unwrap(),
                                    path: "//westend".to_string(),
                                    network_genesis_hash: hex::decode(network_genesis_hash_westend)
                                        .unwrap(),
                                },
                            },
                        ],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
                total_entries: 5,
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "Switched to Log from SeedSelector after backuping seed. ",
            "Expected updated Log screen with no modals."
        )
    );

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        concat!(
            "ClearLog on Log screen with LogRight modal. ",
            "Expected updated Log screen with no modals"
        )
    );

    do_action(Action::NavbarKeys, "", "").unwrap();
    do_action(Action::SelectSeed, "Portia", "").unwrap();
    do_action(Action::RightButton, "", "").unwrap();
    let action = do_action(Action::RemoveSeed, "", "").unwrap();
    /* TODO: this.
    let cut_real_json = cut_public_key(&timeless(&real_json));

    let removal1 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"//kusama","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}}"#;
    let removal2 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"//polkadot","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}}"#;
    let removal3 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#;
    let removal4 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}}"#;
    let removal5 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}}"#;
    let removal6 = r#"{"event":"identity_removed","payload":{"seed_name":"Portia","encryption":"sr25519","public_key":"**","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#;
    assert!(cut_real_json.contains(removal1) && cut_real_json.contains(removal2) && cut_real_json.contains(removal3) && cut_real_json.contains(removal4) && cut_real_json.contains(removal5) && cut_real_json.contains(removal6), "RemoveSeed on Keys screen with SeedMenu modal. Expected updated Log screen with no modals, got:\n{}", real_json);

    */
    // Switching to log. Maybe we want to switch here to updated SeedSelector?

    update_seed_names(r#"Alice"#);

    do_action(Action::RightButton, "", "").unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        concat!(
            "ClearLog on Log screen with LogRight modal. ",
            "Expected updated Log screen with no modals"
        )
    );

    let action = do_action(Action::NavbarKeys, "", "").unwrap();

    let expected_action = ActionResult {
        screen: Some("SeedSelector".to_string()),
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "NewSeed".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![SeedNameCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root(),
                }],
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        "NavbarKeys on Log screen. Expected updated SeedSelector screen with no modals",
    );

    do_action(Action::SelectSeed, "Alice", "").unwrap();
    let action = do_action(Action::NetworkSelector, "", "").unwrap();
    let mut expected_action = alice_polkadot_keys_action.clone();
    expected_action.modal = "NetworkSelector".to_string();
    expected_action.modal_data = ModalData::NetworkSelector {
        f: MNetworkMenu {
            networks: vec![
                Network {
                    key: "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                        .to_string(),
                    logo: "polkadot".to_string(),
                    order: 0,
                    selected: true,
                    title: "Polkadot".to_string(),
                },
                Network {
                    key: "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                        .to_string(),
                    logo: "westend".to_string(),
                    order: 1,
                    selected: false,
                    title: "Westend".to_string(),
                },
                Network {
                    key: "0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                        .to_string(),
                    logo: "kusama".to_string(),
                    order: 2,
                    selected: false,
                    title: "Kusama".to_string(),
                },
            ],
        },
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "NetworkSelector on Keys screen for Alice polkadot keys. ",
            "Expected modal NetworkSelector with polkadot selected"
        )
    );

    let action = do_action(Action::NetworkSelector, "", "").unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        concat!(
            "NetworkSelector on Keys screen with NetworkSelector modal. ",
            "Expected known Keys screen for Alice"
        )
    );

    do_action(Action::NetworkSelector, "", "").unwrap();
    let action = do_action(
        Action::ChangeNetwork,
        "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap();
    let expected_action = ActionResult {
        screen: Some("Keys".to_string()),
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: "Keys".to_string(),
        right_button: "Backup".to_string(),
        screen_name_type: "h4".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key:
                        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                            .to_string(),
                    base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                    identicon: alice_sr_westend(),
                    has_pwd: false,
                    path: "//westend".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard::default(),
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };

    assert_eq!(
        action, expected_action,
        "ChangeNetwork on Keys screen. Expected Keys screen for Alice westend keys.",
    );

    do_action(Action::NavbarScan, "", "").unwrap();
    let action = do_action(Action::TransactionFetched,"53ffde01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e141c2f2f416c6963653c2f2f416c6963652f77657374656e64582f2f416c6963652f7365637265742f2f7365637265740c2f2f300c2f2f31","").unwrap();
    let mut expected_action = ActionResult {
        screen: Some("Transaction".to_string()),
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: "Scan".to_string(),
        right_button: "None".to_string(),
        screen_name_type: "h1".to_string(),
        modal: "Empty".to_string(),
        alert: "Empty".to_string(),
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    importing_derivations: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::DerivationsCard {
                            f: vec![
                                "//Alice".to_string(),
                                "//Alice/westend".to_string(),
                                "//Alice/secret//secret".to_string(),
                                "//0".to_string(),
                                "//1".to_string(),
                            ],
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::ImportDerivations,
                author_info: None,
                network_info: Some(TransactionNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                }),
            },
        },
        modal_data: ModalData::Text {
            f: "Empty".to_string(),
        },
        alert_data: "{}".to_string(),
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen with import_derivations info for westend. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(Action::GoForward, "", "").unwrap();
    expected_action.modal = "SelectSeed".to_string();
    expected_action.modal_data = ModalData::SelectSeed {
        f: MSeeds {
            seed_name_cards: vec![SeedNameCard {
                seed_name: "Alice".to_string(),
                identicon: alice_sr_root(),
            }],
        },
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen with derivations import. ",
            "Expected Transaction screen with SelectSeed modal"
        )
    );

    let action = do_action(Action::GoForward, "Alice", ALICE_SEED_PHRASE).unwrap();
    /*
    let cut_real_json = real_json
        .replace(&empty_png(), r#"<empty>"#)
        .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
        .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
        .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
        .replace(
            &alice_sr_alice_secret_secret(),
            r#"<alice_sr25519_//Alice/secret//secret>"#,
        )
        .replace(
            &alice_sr_alice_westend(),
            r#"<alice_sr25519_//Alice/westend>"#,
        )
        .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);

    let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805","base58":"5FcKjDXS89U79cXvhksZ2pF5XBeafmSM8rqkDVoTHQcXd5Gq","identicon":"<alice_sr25519_//Alice/westend>","has_pwd":false,"path":"//Alice/westend","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen with derivations ",
            "import, with SelectSeed modal. ",
            "Expected updated Keys screen for Alice westend keys"
        )
    );

    let mut alice_westend_keys_action = action;

    do_action(Action::NetworkSelector, "", "").unwrap();
    let action = do_action(
        Action::ChangeNetwork,
        "018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
        "",
    )
    .unwrap(); // switching to polkadot, expect no changes
    assert_eq!(
        action, alice_polkadot_keys_action,
        concat!(
            "Switched network to polkadot. Expected no changes on Keys screen ",
            "for Alice polkadot keys"
        )
    );

        do_action(Action::NetworkSelector, "", "").unwrap();
        do_action(
            Action::ChangeNetwork,
            "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            "",
        );
        let real_json = do_action(
            Action::Swipe,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        );
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(
                &alice_sr_alice_westend(),
                r#"<alice_sr25519_//Alice/westend>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);

        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":true,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805","base58":"5FcKjDXS89U79cXvhksZ2pF5XBeafmSM8rqkDVoTHQcXd5Gq","identicon":"<alice_sr25519_//Alice/westend>","has_pwd":false,"path":"//Alice/westend","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "Swipe on Keys screen for Alice westend keys. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(
            Action::Swipe,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        ); // unswipe
        assert!(real_json == alice_westend_keys_json, "Unswipe on Keys screen for Alice westend keys. Expected known vanilla Keys screen for Alice westend keys, got:\n{}", real_json);

        do_action(
            Action::Swipe,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        );
        let real_json = do_action(
            Action::Swipe,
            "019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805",
            "",
        ); // swipe another
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(
                &alice_sr_alice_westend(),
                r#"<alice_sr25519_//Alice/westend>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805","base58":"5FcKjDXS89U79cXvhksZ2pF5XBeafmSM8rqkDVoTHQcXd5Gq","identicon":"<alice_sr25519_//Alice/westend>","has_pwd":false,"path":"//Alice/westend","swiped":true,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "Swipe on Keys screen on another key while first swiped key is still selected. Expected updated Keys screen, got:\n{}", real_json);

        let real_json = do_action(Action::RemoveKey, "", ""); // remove swiped
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "RemoveKey on Keys screen with swiped key. Expected updated Keys screen, got:\n{}",
            real_json
        );

        // Note: after removal, stay on the Keys screen (previously went to log).

        do_action(
            Action::Swipe,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        );
        let real_json = do_action(Action::Increment, "2", ""); // increment swiped `//westend`
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "Increment on Keys screen with swiped key. Expected updated Keys screen, got:\n{}",
            real_json
        );

        do_action(
            Action::Swipe,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        );
        let real_json = do_action(Action::Increment, "1", ""); // increment swiped `//westend` again
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#)
            .replace(&alice_sr_westend_2(), r#"<alice_sr25519_//westend//2>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08","base58":"5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx","identicon":"<alice_sr25519_//westend//2>","has_pwd":false,"path":"//westend//2","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "Increment on Keys screen with swiped key. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(
            Action::LongTap,
            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
            "",
        ); // enter multi regime with LongTap
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#)
            .replace(&alice_sr_westend_2(), r#"<alice_sr25519_//westend//2>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08","base58":"5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx","identicon":"<alice_sr25519_//westend//2>","has_pwd":false,"path":"//westend//2","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":true},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"1"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "LongTap on Keys screen. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(
            Action::SelectKey,
            "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
            "",
        ); // select by SelectKey in multi
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#)
            .replace(&alice_sr_westend_2(), r#"<alice_sr25519_//westend//2>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":true},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08","base58":"5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx","identicon":"<alice_sr25519_//westend//2>","has_pwd":false,"path":"//westend//2","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":true},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"2"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "SelectKey on Keys screen in multiselect mode. Expected updated Keys screen, got:\n{}",
            cut_real_json
        );

        let real_json = do_action(
            Action::SelectKey,
            "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
            "",
        ); // deselect by SelectKey in multi
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#)
            .replace(&alice_sr_westend_2(), r#"<alice_sr25519_//westend//2>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08","base58":"5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx","identicon":"<alice_sr25519_//westend//2>","has_pwd":false,"path":"//westend//2","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":true},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"1"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "SelectKey on Keys screen in multiselect mode. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(
            Action::LongTap,
            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
            "",
        ); // deselect by LongTap in multi
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_0(), r#"<alice_sr25519_//0>"#)
            .replace(&alice_sr_1(), r#"<alice_sr25519_//1>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(&alice_sr_westend_0(), r#"<alice_sr25519_//westend//0>"#)
            .replace(&alice_sr_westend_1(), r#"<alice_sr25519_//westend//1>"#)
            .replace(&alice_sr_westend_2(), r#"<alice_sr25519_//westend//2>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f","base58":"5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND","identicon":"<alice_sr25519_//westend//1>","has_pwd":false,"path":"//westend//1","swiped":false,"multiselect":false},{"address_key":"012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972","base58":"5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH","identicon":"<alice_sr25519_//0>","has_pwd":false,"path":"//0","swiped":false,"multiselect":false},{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08","base58":"5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx","identicon":"<alice_sr25519_//westend//2>","has_pwd":false,"path":"//westend//2","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48","base58":"5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o","identicon":"<alice_sr25519_//1>","has_pwd":false,"path":"//1","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false},{"address_key":"01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470","base58":"5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx","identicon":"<alice_sr25519_//westend//0>","has_pwd":false,"path":"//westend//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"0"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "LongTap on Keys screen. Expected updated Keys screen, got:\n{}",
            real_json
        );

        // Note: although multiselect count is 0, remain in multiselect mode

        do_action(
            Action::LongTap,
            "0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f",
            "",
        );
        do_action(
            Action::LongTap,
            "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
            "",
        );
        do_action(
            Action::LongTap,
            "014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08",
            "",
        );
        do_action(
            Action::LongTap,
            "01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48",
            "",
        );
        do_action(
            Action::LongTap,
            "01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470",
            "",
        );
        let real_json = do_action(Action::RemoveKey, "", ""); // remove keys in multiselect mode
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "RemoveKey on Keys screen with multiselect mode. Expected updated Keys screen, got:\n{}",
            real_json
        );

        do_action(
            Action::LongTap,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        ); // enter multiselect mode
        let real_json = do_action(Action::SelectAll, "", ""); // select all
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":true},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":true},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":true}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"3"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "SelectAll on Keys screen with multiselect mode. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::SelectAll, "", ""); // deselect all
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"0"},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "SelectAll on Keys screen with multiselect mode. Expected updated Keys screen, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::GoBack, "", ""); // exit multiselect mode
        let cut_real_json = real_json
            .replace(&empty_png(), r#"<empty>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "GoBack on Keys screen with multiselect mode. Expected updated Keys screen, got:\n{}",
            real_json
        );

        alice_westend_keys_json = real_json;

        do_action(
            Action::LongTap,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        ); // enter multiselect mode
        do_action(Action::SelectAll, "", ""); // select all
        let real_json = do_action(Action::ExportMultiSelect, "", "");
        let cut_real_json = real_json
            .replace(
                &alice_westend_westend_qr(),
                r#"<alice_westend_//westend_qr>"#,
            )
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"KeyDetailsMultiSelect","screenLabel":"Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"KeyMenu","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"qr":"<alice_westend_//westend_qr>","pubkey":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","seed_name":"Alice","path":"//westend","network_title":"Westend","network_logo":"westend","current_number":"1","out_of":"3"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let unit1 = real_json;

        let real_json = do_action(Action::NextUnit, "", "");
        let cut_real_json = real_json
            .replace(
                &alice_westend_alice_secret_secret_qr(),
                r#"<alice_westend_//Alice/secret//secret_qr>"#,
            )
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            );
        let expected_json = r#"{"screen":"KeyDetailsMultiSelect","screenLabel":"Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"KeyMenu","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"qr":"<alice_westend_//Alice/secret//secret_qr>","pubkey":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","seed_name":"Alice","path":"//Alice/secret//secret","network_title":"Westend","network_logo":"westend","current_number":"2","out_of":"3"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let unit2 = real_json;

        let real_json = do_action(Action::NextUnit, "", "");
        let cut_real_json = real_json
            .replace(&alice_westend_alice_qr(), r#"<alice_westend_//Alice_qr>"#)
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let expected_json = r#"{"screen":"KeyDetailsMultiSelect","screenLabel":"Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"KeyMenu","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"qr":"<alice_westend_//Alice_qr>","pubkey":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","seed_name":"Alice","path":"//Alice","network_title":"Westend","network_logo":"westend","current_number":"3","out_of":"3"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let unit3 = real_json;

        let real_json = do_action(Action::NextUnit, "", "");
        assert!(real_json == unit1, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let real_json = do_action(Action::PreviousUnit, "", "");
        assert!(real_json == unit3, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let real_json = do_action(Action::PreviousUnit, "", "");
        assert!(real_json == unit2, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let real_json = do_action(Action::PreviousUnit, "", "");
        assert!(real_json == unit1, "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen, got:\n{}", real_json);

        let real_json = do_action(Action::GoBack, "", "");
        assert!(
            real_json == alice_westend_keys_json,
            "GoBack on KeyDetailsMulti screen. Expected Keys screen in plain mode, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::NewKey, "", "");
        let expected_json = r#"{"screen":"DeriveKey","screenLabel":"Derive Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"seed_name":"Alice","network_title":"Westend","network_logo":"westend","network_specs_key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","suggested_derivation":"","keyboard":true},"modalData":{},"alertData":{}}"#;
        assert!(
            real_json == expected_json,
            "NewKey on Keys screen. Expected DeriveKey screen, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::GoBack, "", "");
        assert!(
            real_json == alice_westend_keys_json,
            "GoBack on DeriveKey screen. Expected Keys screen in plain mode, got:\n{}",
            real_json
        );

        do_action(Action::NewKey, "", "");
        let real_json = do_action(Action::GoForward, "", ALICE_SEED_PHRASE); // create root derivation
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<alice_sr25519_root>","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "GoForward on DeriveKey screen with empty derivation string. Expected updated Keys screen, got:\n{}", real_json);

        do_action(
            Action::LongTap,
            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            "",
        ); // enter multiselect mode
        let real_json = do_action(Action::SelectAll, "", ""); // select all
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"MultiSelect","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<alice_sr25519_root>","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","swiped":false,"multiselect":true},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":true},{"address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret","swiped":false,"multiselect":true},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":true}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":true,"multiselect_count":"4"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "SelectAll on Keys screen in multiselect mode, with existing root key. Expected updated Keys screen, got:\n{}", real_json);

        do_action(Action::GoBack, "", "");
        let real_json = do_action(
            Action::SelectKey,
            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
            "",
        );
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&alice_westend_root_qr(), r#"<alice_westend_root_qr>"#);
        let expected_json = r#"{"screen":"KeyDetails","screenLabel":"Seed Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"KeyMenu","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"qr":"<alice_westend_root_qr>","pubkey":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"<alice_sr25519_root>","seed_name":"Alice","path":"","network_title":"Westend","network_logo":"westend"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "SelectKey on Keys screen with root key. Expected KeyDetails screen with Seed Key label, got:\n{}", real_json);

        let real_qr_info = get_qr_info_read(&real_json);
        let expected_qr_info = r#"substrate:5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV:0xe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert!(
            real_qr_info == expected_qr_info,
            "Received unexpected qr payload:\n{}",
            real_qr_info
        );

        do_action(Action::GoBack, "", "");
        do_action(Action::NavbarSettings, "", "");
        let real_json = do_action(Action::BackupSeed, "", "");
        let cut_real_json = real_json.replace(&alice_sr_root(), r#"<alice_sr25519_root>"#);
        let expected_json = r#"{"screen":"SelectSeedForBackup","screenLabel":"Select seed","back":true,"footer":false,"footerButton":"Settings","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"seedNameCards":[{"identicon":"<alice_sr25519_root>","seed_name":"Alice"}]},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "BackupSeed on Settings screen. Expected SelectSeedForBackup screen with no modals, got:\n{}", real_json);

        let real_json = do_action(Action::BackupSeed, "Alice", "");
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(
                &alice_sr_secret_path_multipass(),
                r#"<alice_sr25519_//secret//path///multipass>"#,
            )
            .replace(&alice_sr_polkadot(), r#"<alice_sr25519_//polkadot>"#);
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"None","screenNameType":"h4","modal":"Backup","alert":"Empty","screenData":{"root":{"seed_name":"Alice","identicon":"<alice_sr25519_root>","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU","swiped":false,"multiselect":false},"set":[{"address_key":"01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","base58":"16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb","identicon":"<alice_sr25519_//secret//path///multipass>","has_pwd":true,"path":"//secret//path","swiped":false,"multiselect":false},{"address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","base58":"16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot","swiped":false,"multiselect":false}],"network":{"title":"Polkadot","logo":"polkadot"},"multiselect_mode":false,"multiselect_count":""},"modalData":{"seed_name":"Alice","derivations":[{"network_title":"Polkadot","network_logo":"polkadot","network_order":0,"id_set":[{"path":"","has_pwd":false},{"path":"//secret//path","has_pwd":true},{"path":"//polkadot","has_pwd":false}]},{"network_title":"Westend","network_logo":"westend","network_order":1,"id_set":[{"path":"//westend","has_pwd":false},{"path":"","has_pwd":false},{"path":"//Alice/secret//secret","has_pwd":false},{"path":"//Alice","has_pwd":false}]},{"network_title":"Kusama","network_logo":"kusama","network_order":2,"id_set":[{"path":"//kusama","has_pwd":false}]}]},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "BackupSeed on SelectSeedForBackup screen with Alice as an entry. Expected Keys screen with Backup modal, got:\n{}", real_json);

        db_handling::manage_history::seed_name_was_shown(dbname, String::from("Alice")).unwrap(); // mock signal from phone

        do_action(Action::NavbarSettings, "", "");
        do_action(Action::ManageNetworks, "", "");
        do_action(
            Action::GoForward,
            "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            "",
        );
        do_action(Action::RightButton, "", "");
        let real_json = do_action(Action::SignNetworkSpecs, "", "");
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(
                &alice_sr_secret_path_multipass(),
                r#"<alice_sr25519_//secret//path///multipass>"#,
            )
            .replace(&alice_sr_polkadot(), r#"<alice_sr25519_//polkadot>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&alice_sr_kusama(), r#"<alice_sr25519_//kusama>"#);
        let expected_json = r#"{"screen":"SignSufficientCrypto","screenLabel":"Sign SufficientCrypto","back":true,"footer":false,"footerButton":"Settings","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"identities":[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"<alice_sr25519_//kusama>","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","public_key":"e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","identicon":"<alice_sr25519_//secret//path///multipass>","has_pwd":true,"path":"//secret//path"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot"}]},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "SignNetworkSpecs on NetworkDetails screen for westend sr25519. Expected SignSufficientCrypto screen, got:\n{}", real_json);

        let real_json = do_action(
            Action::GoForward,
            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
            "",
        );
        let expected_json = r#"{"screen":"SignSufficientCrypto","screenLabel":"Sign SufficientCrypto","back":true,"footer":false,"footerButton":"Settings","rightButton":"None","screenNameType":"h1","modal":"SufficientCryptoReady","alert":"Empty","screenData":{"identities":[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"<alice_sr25519_//kusama>","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","public_key":"e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","identicon":"<alice_sr25519_//secret//path///multipass>","has_pwd":true,"path":"//secret//path"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot"}]},"modalData":{"author_info":{"base58":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","seed":"Alice","derivation_path":""},"sufficient":"**","content":{"type":"add_specs","network_title":"Westend","network_logo":"westend"}},"alertData":{}}"#;
        let (cut_real_json, sufficient_hex) = process_sufficient(&real_json);
        let cut_real_json = cut_real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(
                &alice_sr_secret_path_multipass(),
                r#"<alice_sr25519_//secret//path///multipass>"#,
            )
            .replace(&alice_sr_polkadot(), r#"<alice_sr25519_//polkadot>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&alice_sr_kusama(), r#"<alice_sr25519_//kusama>"#);
        assert!(cut_real_json == expected_json, "GoForward on SignSufficientCrypto screen with Alice root key as an entry. Expected modal SufficientCryptoReady, got:\n{}", real_json);

        {
            // testing the validity of the received sufficient crypto object
            std::env::set_current_dir("../generate_message").unwrap();
            let command = std::process::Command::new("cargo")
                .arg("run")
                .args([
                    "sign",
                    "-qr",
                    "-sufficient",
                    "-hex",
                    &sufficient_hex,
                    "-msgtype",
                    "add_specs",
                    "-payload",
                    "navigator_test_files/sign_me_add_specs_westend_sr25519",
                ])
                .output()
                .unwrap();
            assert!(
                command.status.success(),
                "Produced sufficient crypto did not work. {}.",
                String::from_utf8(command.stderr).unwrap()
            );
            std::env::set_current_dir("../files/signed").unwrap();
            std::fs::remove_file("add_specs_westend-sr25519").unwrap();
            std::env::set_current_dir("../../navigator").unwrap();
        }

        let real_json = do_action(Action::GoBack, "", "");
        assert!(real_json == current_settings_json, "GoBack on SignSufficientCrypto screen with SufficientCryptoReady modal. Expected Settings screen, got:\n{}", real_json);

        let real_json = do_action(Action::NavbarLog, "", "");
        let cut_real_json = real_json.replace(&alice_sr_root(), r#"<alice_sr25519_root>"#);
        let expected_json_piece = r##""events":[{"event":"add_specs_message_signed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","signed_by":{"public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","encryption":"sr25519"}}}]"##;
        assert!(
            cut_real_json.contains(expected_json_piece),
            "Expected the updated log to contain entry about generating sufficient crypto, got:\n{}",
            real_json
        );

        do_action(Action::RightButton, "", "");
        let real_json = do_action(Action::ClearLog, "", "");
        let cut_real_json = timeless(&real_json);
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":0,"timestamp":"**","events":[{"event":"history_cleared"}]}],"total_entries":1},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals, got:\n{}", real_json);

        do_action(Action::NavbarSettings, "", "");
        do_action(Action::ManageNetworks, "", "");
        do_action(
            Action::GoForward,
            "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            "",
        );
        do_action(Action::ManageMetadata, "9150", "");
        do_action(Action::SignMetadata, "", "");
        let real_json = do_action(
            Action::GoForward,
            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
            "",
        );
        let expected_json = r#"{"screen":"SignSufficientCrypto","screenLabel":"Sign SufficientCrypto","back":true,"footer":false,"footerButton":"Settings","rightButton":"None","screenNameType":"h1","modal":"SufficientCryptoReady","alert":"Empty","screenData":{"identities":[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"<alice_sr25519_//kusama>","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","public_key":"e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","identicon":"<alice_sr25519_//secret//path///multipass>","has_pwd":true,"path":"//secret//path"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot"}]},"modalData":{"author_info":{"base58":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","seed":"Alice","derivation_path":""},"sufficient":"**","content":{"type":"load_metadata","specname":"westend","spec_version":"9150","meta_hash":"b5d422b92f0183c192cbae5e63811bffcabbef22b6f9e05a85ba7b738e91d44a","meta_id_pic":"<meta_pic_westend9150>"}},"alertData":{}}"#;
        let (cut_real_json, sufficient_hex) = process_sufficient(&real_json);
        let cut_real_json = cut_real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(
                &alice_sr_secret_path_multipass(),
                r#"<alice_sr25519_//secret//path///multipass>"#,
            )
            .replace(&alice_sr_polkadot(), r#"<alice_sr25519_//polkadot>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&alice_sr_kusama(), r#"<alice_sr25519_//kusama>"#)
            .replace(&westend_9150(), r#"<meta_pic_westend9150>"#);
        assert!(cut_real_json == expected_json, "GoForward on SignSufficientCrypto screen with Alice root key as an entry. Expected modal SufficientCryptoReady, got:\n{}", real_json);

        {
            // testing the validity of the received sufficient crypto object
            std::env::set_current_dir("../generate_message").unwrap();
            let command = std::process::Command::new("cargo")
                .arg("run")
                .args([
                    "sign",
                    "-text",
                    "-sufficient",
                    "-hex",
                    &sufficient_hex,
                    "-msgtype",
                    "load_metadata",
                    "-payload",
                    "navigator_test_files/sign_me_load_metadata_westendV9150",
                ])
                .output()
                .unwrap();
            assert!(
                command.status.success(),
                "Produced sufficient crypto did not work. {}.",
                String::from_utf8(command.stderr).unwrap()
            );
            std::env::set_current_dir("../files/signed").unwrap();
            std::fs::remove_file("load_metadata_westendV9150.txt").unwrap();
            std::env::set_current_dir("../../navigator").unwrap();
        }

        do_action(Action::GoBack, "", "");
        let real_json = do_action(Action::NavbarLog, "", "");
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&westend_9150(), r#"<meta_pic_westend9150>"#);
        let expected_json_piece = r#""events":[{"event":"load_metadata_message_signed","payload":{"specname":"westend","spec_version":"9150","meta_hash":"b5d422b92f0183c192cbae5e63811bffcabbef22b6f9e05a85ba7b738e91d44a","meta_id_pic":"<meta_pic_westend9150>","signed_by":{"public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","encryption":"sr25519"}}}]"#;
        assert!(
            cut_real_json.contains(expected_json_piece),
            "Expected the updated log to contain entry about generating sufficient crypto, got:\n{}",
            real_json
        );
        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");

        do_action(Action::NavbarSettings, "", "");
        do_action(Action::ManageNetworks, "", "");
        do_action(Action::RightButton, "", "");
        do_action(Action::SignTypes, "", "");
        let real_json = do_action(
            Action::GoForward,
            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
            "",
        );
        let expected_json = r#"{"screen":"SignSufficientCrypto","screenLabel":"Sign SufficientCrypto","back":true,"footer":false,"footerButton":"Settings","rightButton":"None","screenNameType":"h1","modal":"SufficientCryptoReady","alert":"Empty","screenData":{"identities":[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"<alice_sr25519_//kusama>","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","has_pwd":false,"path":"//Alice/secret//secret"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","public_key":"e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c","identicon":"<alice_sr25519_//secret//path///multipass>","has_pwd":true,"path":"//secret//path"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot"}]},"modalData":{"author_info":{"base58":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","seed":"Alice","derivation_path":""},"sufficient":"**","content":{"type":"load_types","types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>"}},"alertData":{}}"#;
        let (cut_real_json, sufficient_hex) = process_sufficient(&real_json);
        let cut_real_json = cut_real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(
                &alice_sr_secret_path_multipass(),
                r#"<alice_sr25519_//secret//path///multipass>"#,
            )
            .replace(&alice_sr_polkadot(), r#"<alice_sr25519_//polkadot>"#)
            .replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&alice_sr_alice(), r#"<alice_sr25519_//Alice>"#)
            .replace(&alice_sr_kusama(), r#"<alice_sr25519_//kusama>"#)
            .replace(&types_known(), r#"<types_known>"#);
        assert!(cut_real_json == expected_json, "GoForward on SignSufficientCrypto screen with Alice root key as an entry. Expected modal SufficientCryptoReady, got:\n{}", real_json);

        {
            // testing the validity of the received sufficient crypto object
            std::env::set_current_dir("../generate_message").unwrap();
            let command = std::process::Command::new("cargo")
                .arg("run")
                .args([
                    "sign",
                    "-text",
                    "-sufficient",
                    "-hex",
                    &sufficient_hex,
                    "-msgtype",
                    "load_types",
                    "-payload",
                    "navigator_test_files/sign_me_load_types",
                ])
                .output()
                .unwrap();
            assert!(
                command.status.success(),
                "Produced sufficient crypto did not work. {}.",
                String::from_utf8(command.stderr).unwrap()
            );
            std::env::set_current_dir("../files/signed").unwrap();
            std::fs::remove_file("load_types.txt").unwrap();
            std::env::set_current_dir("../../navigator").unwrap();
        }

        do_action(Action::GoBack, "", "");
        let real_json = do_action(Action::NavbarLog, "", "");
        let cut_real_json = real_json
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&types_known(), r#"<types_known>"#);
        let expected_json_piece = r#""events":[{"event":"load_types_message_signed","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>","signed_by":{"public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","encryption":"sr25519"}}}]"#;
        assert!(
            cut_real_json.contains(expected_json_piece),
            "Expected the updated log to contain entry about generating sufficient crypto, got:\n{}",
            real_json
        );
        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");

        // let's scan something!!! oops wrong network version
        do_action(Action::NavbarScan, "", "");
        let real_json = do_action(Action::TransactionFetched,"530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","");
        let expected_json = r#"{"screen":"Transaction","screenLabel":"","back":true,"footer":false,"footerButton":"Scan","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"content":{"error":[{"index":0,"indent":0,"type":"error","payload":"Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9150 metadata: Network spec version decoded from extensions (9010) differs from the version in metadata (9150)."}]},"type":"read"},"modalData":{},"alertData":{}}"#;
        assert!(real_json == expected_json, "TransactionFetched on Scan screen containing transaction. Expected Transaction screen with no modals, got:\n{}", real_json);

        let real_json = do_action(Action::GoForward, "", "");
        assert!(real_json == expected_json, "GoForward on Transaction screen with transaction that could be only read. Expected to stay in same place, got:\n{}", real_json);

        // let's scan something real!!!
        do_action(Action::GoBack, "", "");
        let transaction_hex = "5301008266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235ea40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b800be23000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let real_json = do_action(Action::TransactionFetched, transaction_hex, "");
        let cut_real_json = real_json
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&bob(), r#"<bob>"#);
        let expected_json_transaction_sign = r#"{"screen":"Transaction","screenLabel":"","back":true,"footer":false,"footerButton":"Scan","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"content":{"method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9150"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]},"author_info":{"base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","seed":"Alice","derivation_path":"//Alice/secret//secret","has_pwd":false},"network_info":{"network_title":"Westend","network_logo":"westend"},"type":"sign"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json_transaction_sign, "TransactionFetched on Scan screen containing transaction. Expected Transaction screen with no modals, got:\n{}", real_json);

        let real_json = do_action(
            Action::GoForward,
            "Alice sends some cash",
            ALICE_SEED_PHRASE,
        );
        let (cut_real_json, signature_hex) = process_signature(&real_json);
        let cut_real_json = cut_real_json
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&bob(), r#"<bob>"#);
        let expected_json_transaction_ready = r#"{"screen":"Transaction","screenLabel":"","back":true,"footer":false,"footerButton":"Scan","rightButton":"None","screenNameType":"h1","modal":"SignatureReady","alert":"Empty","screenData":{"content":{"method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9150"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]},"author_info":{"base58":"5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK","identicon":"<alice_sr25519_//Alice/secret//secret>","seed":"Alice","derivation_path":"//Alice/secret//secret","has_pwd":false},"network_info":{"network_title":"Westend","network_logo":"westend"},"type":"done"},"modalData":{"signature":"**"},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json_transaction_ready,
            "GoForward on parsed transaction. Expected modal SignatureReady, got:\n{}",
            real_json
        );

        assert!(
            signature_is_good(transaction_hex, &signature_hex),
            "Produced bad signature: \n{}",
            signature_hex
        );

        let real_json = do_action(Action::GoBack, "", "");
        let cut_real_json = timeless(&real_json).replace(
            &alice_sr_alice_secret_secret(),
            r#"<alice_sr25519_//Alice/secret//secret>"#,
        );
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":1,"timestamp":"**","events":[{"event":"transaction_signed","payload":{"transaction":"a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b800be23000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33","network_name":"westend","signed_by":{"public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","encryption":"sr25519"},"user_comment":"Alice sends some cash"}}]},{"order":0,"timestamp":"**","events":[{"event":"history_cleared"}]}],"total_entries":2},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "GoBack from Transaction with SignatureReady modal. Expected Log, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::ShowLogDetails, "1", "");
        let cut_real_json = timeless(&real_json)
            .replace(
                &alice_sr_alice_secret_secret(),
                r#"<alice_sr25519_//Alice/secret//secret>"#,
            )
            .replace(&bob(), r#"<bob>"#);
        let expected_json = r#"{"screen":"LogDetails","screenLabel":"Event details","back":true,"footer":true,"footerButton":"Log","rightButton":"None","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"timestamp":"**","events":[{"event":"transaction_signed","payload":{"transaction":{"method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9150"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]},"network_name":"westend","signed_by":{"public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","encryption":"sr25519"},"user_comment":"Alice sends some cash"}}]},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ShowLogDetails on Log screen with order 1. Expected LogDetails screen with decoded transaction and no modals, got:\n{}", real_json);

        do_action(Action::GoBack, "", "");
        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");

        // let's scan a text message
        do_action(Action::NavbarScan, "", "");
        let message_hex = "5301033efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34f5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let real_json = do_action(Action::TransactionFetched, message_hex, "");
        let cut_real_json = real_json.replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json_message_sign = r#"{"screen":"Transaction","screenLabel":"","back":true,"footer":false,"footerButton":"Scan","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"content":{"message":[{"index":0,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"}]},"author_info":{"base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","seed":"Alice","derivation_path":"//westend","has_pwd":false},"network_info":{"network_title":"Westend","network_logo":"westend"},"type":"sign"},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json_message_sign, "TransactionFetched on Scan screen containing message transaction. Expected Transaction screen with no modals, got:\n{}", real_json);

        let text = String::from_utf8(
            hex::decode(
                TEXT.captures(&real_json)
                    .unwrap()
                    .name("text")
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
        )
        .unwrap(); // the way we extract text in ui atm
        let expected_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        assert!(text == expected_text, "Different text: \n{}", text);

        let real_json = do_action(Action::GoForward, "text test", ALICE_SEED_PHRASE);
        let (cut_real_json, signature_hex) = process_signature(&real_json);
        let cut_real_json = cut_real_json.replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json_message_ready = r#"{"screen":"Transaction","screenLabel":"","back":true,"footer":false,"footerButton":"Scan","rightButton":"None","screenNameType":"h1","modal":"SignatureReady","alert":"Empty","screenData":{"content":{"message":[{"index":0,"indent":0,"type":"text","payload":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e"}]},"author_info":{"base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","seed":"Alice","derivation_path":"//westend","has_pwd":false},"network_info":{"network_title":"Westend","network_logo":"westend"},"type":"done"},"modalData":{"signature":"**"},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json_message_ready,
            "GoForward on parsed transaction. Expected modal SignatureReady, got:\n{}",
            real_json
        );

        assert!(
            signature_is_good(message_hex, &signature_hex),
            "Produced bad signature: \n{}",
            signature_hex
        );

        let real_json = do_action(Action::GoBack, "", "");
        let cut_real_json =
            timeless(&real_json).replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":1,"timestamp":"**","events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","encryption":"sr25519"},"user_comment":"text test"}}]},{"order":0,"timestamp":"**","events":[{"event":"history_cleared"}]}],"total_entries":2},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "GoBack from Transaction with SignatureReady modal. Expected Log, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::ShowLogDetails, "1", "");
        let cut_real_json =
            timeless(&real_json).replace(&alice_sr_westend(), r#"<alice_sr25519_//westend>"#);
        let expected_json = r#"{"screen":"LogDetails","screenLabel":"Event details","back":true,"footer":true,"footerButton":"Log","rightButton":"None","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"timestamp":"**","events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","encryption":"sr25519"},"user_comment":"text test"}}]},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "ShowLogDetails on Log screen with order 1. Expected LogDetails screen with decoded message and no modals, got:\n{}", real_json);

        do_action(Action::GoBack, "", "");
        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");

        do_action(Action::NavbarKeys, "", "");
        do_action(Action::RightButton, "", "");
        do_action(Action::NewSeed, "", "");
        let real_json = do_action(Action::GoForward, "Pepper", "");
        let (cut_real_json, seed_phrase_pepper) = cut_seed(&cut_identicon(&real_json));
        let expected_json = r#"{"screen":"NewSeed","screenLabel":"New Seed","back":true,"footer":false,"footerButton":"Keys","rightButton":"None","screenNameType":"h1","modal":"NewSeedBackup","alert":"Empty","screenData":{"keyboard":false},"modalData":{"seed":"Pepper","seed_phrase":"**","identicon":"**"},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "GoForward on NewSeed screen with non-empty seed name. Expected NewSeed screen with NewSeedBackup modal, got:\n{}", real_json);

        let real_json = do_action(Action::GoForward, "false", &seed_phrase_pepper);
        let cut_real_json = cut_address_key(&cut_base58(&cut_identicon(
            &real_json.replace(&empty_png(), r#"<empty>"#),
        )));
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Pepper","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"**","base58":"**","identicon":"**","has_pwd":false,"path":"//polkadot","swiped":false,"multiselect":false}],"network":{"title":"Polkadot","logo":"polkadot"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "GoForward on NewSeed screen with NewSeedBackup modal active. Expected Keys screen with no modals, got:\n{}", real_json);

        update_seed_names(r#"Alice,Pepper"#);

        do_action(Action::NetworkSelector, "", "");
        let real_json = do_action(
            Action::ChangeNetwork,
            "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            "",
        );
        let cut_real_json = cut_address_key(&cut_base58(&cut_identicon(
            &real_json.replace(&empty_png(), r#"<empty>"#),
        )));
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Pepper","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"**","base58":"**","identicon":"**","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "Changed network to westend. Expected Keys screen with no modals, got:\n{}",
            real_json
        );

        let caps = SET.captures(&real_json).unwrap();
        let pepper_westend_public = caps.name("public").unwrap().as_str().to_string();
        let pepper_westend_base58 = caps.name("base").unwrap().as_str().to_string();
        let pepper_westend_identicon = caps.name("identicon").unwrap().as_str().to_string();

        do_action(Action::NavbarScan, "", "");
        let transaction_hex = transaction_hex.replace(
            "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
            &pepper_westend_public,
        );
        let real_json = do_action(Action::TransactionFetched, &transaction_hex, "");
        let cut_real_json = real_json.replace(&bob(), r#"<bob>"#);
        let expected_json_transaction_sign = expected_json_transaction_sign
            .replace(
                "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK",
                &pepper_westend_base58,
            )
            .replace(
                "<alice_sr25519_//Alice/secret//secret>",
                &pepper_westend_identicon,
            )
            .replace("//Alice/secret//secret", "//westend")
            .replace("Alice", "Pepper");
        assert!(cut_real_json == expected_json_transaction_sign, "TransactionFetched on Scan screen containing transaction. Expected Transaction screen with no modals, got:\n{}", real_json);

        let real_json = do_action(
            Action::GoForward,
            "Pepper also sends some cash",
            &seed_phrase_pepper,
        );
        let (cut_real_json, signature_hex) = process_signature(&real_json);
        let cut_real_json = cut_real_json.replace(&bob(), r#"<bob>"#);
        let expected_json_transaction_ready = expected_json_transaction_ready
            .replace(
                "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK",
                &pepper_westend_base58,
            )
            .replace(
                "<alice_sr25519_//Alice/secret//secret>",
                &pepper_westend_identicon,
            )
            .replace("//Alice/secret//secret", "//westend")
            .replace("Alice", "Pepper");
        assert!(
            cut_real_json == expected_json_transaction_ready,
            "GoForward on parsed transaction. Expected modal SignatureReady, got:\n{}",
            real_json
        );

        assert!(
            signature_is_good(&transaction_hex, &signature_hex),
            "Produced bad signature: \n{}",
            signature_hex
        );

        do_action(Action::GoBack, "", "");
        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");
        do_action(Action::NavbarKeys, "", "");
        do_action(Action::SelectSeed, "Pepper", "");
        do_action(Action::NetworkSelector, "", "");
        do_action(
            Action::ChangeNetwork,
            "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            "",
        );
        do_action(Action::Swipe, &format!("01{}", pepper_westend_public), "");
        do_action(Action::RemoveKey, "", "");

        let real_json = do_action(Action::NewKey, "", "");
        let expected_json = r#"{"screen":"DeriveKey","screenLabel":"Derive Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"None","screenNameType":"h1","modal":"Empty","alert":"Empty","screenData":{"seed_name":"Pepper","network_title":"Westend","network_logo":"westend","network_specs_key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","suggested_derivation":"","keyboard":true},"modalData":{},"alertData":{}}"#;
        assert!(
            real_json == expected_json,
            "NewKey on Keys screen. Expected DeriveKey screen, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::CheckPassword, "//0///secret", "");
        let expected_json = r#"{"screen":"DeriveKey","screenLabel":"Derive Key","back":true,"footer":false,"footerButton":"Keys","rightButton":"None","screenNameType":"h1","modal":"PasswordConfirm","alert":"Empty","screenData":{"seed_name":"Pepper","network_title":"Westend","network_logo":"westend","network_specs_key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","suggested_derivation":"//0///secret","keyboard":false},"modalData":{"seed_name":"Pepper","cropped_path":"//0","pwd":"secret"},"alertData":{}}"#;
        assert!(real_json == expected_json, "CheckPassword on DeriveKey screen with password (path validity and password existence is checked elsewhere). Expected updated DeriveKey screen with PasswordConfirm modal, got:\n{}", real_json);

        let real_json = do_action(Action::GoForward, "//0///secret", &seed_phrase_pepper);
        let cut_real_json = cut_address_key(&cut_base58(&cut_identicon(
            &real_json.replace(&empty_png(), r#"<empty>"#),
        )));
        let expected_json = r#"{"screen":"Keys","screenLabel":"","back":true,"footer":true,"footerButton":"Keys","rightButton":"Backup","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"root":{"seed_name":"Pepper","identicon":"<empty>","address_key":"","base58":"","swiped":false,"multiselect":false},"set":[{"address_key":"**","base58":"**","identicon":"**","has_pwd":true,"path":"//0","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"},"multiselect_mode":false,"multiselect_count":""},"modalData":{},"alertData":{}}"#;
        assert!(cut_real_json == expected_json, "GoForward on DeriveKey screen with PasswordConfirm modal. Expected updated Keys screen, got:\n{}", real_json);

        let caps = KEY0.captures(&real_json).unwrap();
        let pepper_key0_public = caps.name("public").unwrap().as_str().to_string();
        let pepper_key0_base58 = caps.name("base").unwrap().as_str().to_string();
        let pepper_key0_identicon = caps.name("identicon").unwrap().as_str().to_string();

        do_action(Action::NavbarScan, "", "");
        let message_hex = message_hex.replace(
            "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
            &pepper_key0_public,
        );
        let real_json = do_action(Action::TransactionFetched, &message_hex, "");
        let expected_json = format!("{{\"screen\":\"Transaction\",\"screenLabel\":\"\",\"back\":true,\"footer\":false,\"footerButton\":\"Scan\",\"rightButton\":\"None\",\"screenNameType\":\"h1\",\"modal\":\"Empty\",\"alert\":\"Empty\",\"screenData\":{{\"content\":{{\"message\":[{{\"index\":0,\"indent\":0,\"type\":\"text\",\"payload\":\"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e\"}}]}},\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"network_info\":{{\"network_title\":\"Westend\",\"network_logo\":\"westend\"}},\"type\":\"sign\"}},\"modalData\":{{}},\"alertData\":{{}}}}", pepper_key0_base58, pepper_key0_identicon);
        assert!(real_json == expected_json, "TransactionFetched on Scan screen containing message transaction. Expected Transaction screen with no modals, got:\n{}", real_json);

        let real_json = do_action(
            Action::GoForward,
            "Pepper tries sending text from passworded account",
            &seed_phrase_pepper,
        );
        let expected_json = format!("{{\"screen\":\"Transaction\",\"screenLabel\":\"\",\"back\":true,\"footer\":false,\"footerButton\":\"Scan\",\"rightButton\":\"None\",\"screenNameType\":\"h1\",\"modal\":\"EnterPassword\",\"alert\":\"Empty\",\"screenData\":{{\"content\":{{\"message\":[{{\"index\":0,\"indent\":0,\"type\":\"text\",\"payload\":\"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e\"}}]}},\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"network_info\":{{\"network_title\":\"Westend\",\"network_logo\":\"westend\"}},\"type\":\"sign\"}},\"modalData\":{{\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"counter\":1}},\"alertData\":{{}}}}", pepper_key0_base58, pepper_key0_identicon, pepper_key0_base58, pepper_key0_identicon);
        assert!(real_json == expected_json, "GoForward on Transaction screen for passworded address. Expected Transaction screen with EnterPassword modal, got:\n{}", real_json);

        let real_json = do_action(Action::GoForward, "wrong_one", "");
        let expected_json = format!("{{\"screen\":\"Transaction\",\"screenLabel\":\"\",\"back\":true,\"footer\":false,\"footerButton\":\"Scan\",\"rightButton\":\"None\",\"screenNameType\":\"h1\",\"modal\":\"EnterPassword\",\"alert\":\"Error\",\"screenData\":{{\"content\":{{\"message\":[{{\"index\":0,\"indent\":0,\"type\":\"text\",\"payload\":\"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e\"}}]}},\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"network_info\":{{\"network_title\":\"Westend\",\"network_logo\":\"westend\"}},\"type\":\"sign\"}},\"modalData\":{{\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"counter\":2}},\"alertData\":{{\"error\":\"Wrong password.\"}}}}", pepper_key0_base58, pepper_key0_identicon, pepper_key0_base58, pepper_key0_identicon);
        assert!(real_json == expected_json, "GoForward on Transaction screen for passworded address with wrong password. Expected Transaction screen with EnterPassword modal with counter at 2, got:\n{}", real_json);

        do_action(Action::GoBack, "", "");
        let real_json = do_action(Action::GoForward, "wrong_two", "");
        let expected_json = format!("{{\"screen\":\"Transaction\",\"screenLabel\":\"\",\"back\":true,\"footer\":false,\"footerButton\":\"Scan\",\"rightButton\":\"None\",\"screenNameType\":\"h1\",\"modal\":\"EnterPassword\",\"alert\":\"Error\",\"screenData\":{{\"content\":{{\"message\":[{{\"index\":0,\"indent\":0,\"type\":\"text\",\"payload\":\"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e\"}}]}},\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"network_info\":{{\"network_title\":\"Westend\",\"network_logo\":\"westend\"}},\"type\":\"sign\"}},\"modalData\":{{\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"counter\":3}},\"alertData\":{{\"error\":\"Wrong password.\"}}}}", pepper_key0_base58, pepper_key0_identicon, pepper_key0_base58, pepper_key0_identicon);
        assert!(real_json == expected_json, "GoForward on Transaction screen for passworded address with second wrong password. Expected Transaction screen with EnterPassword modal with counter at 3, got:\n{}", real_json);

        do_action(Action::GoBack, "", "");
        let real_json = do_action(Action::GoForward, "wrong_three", "");
        let cut_real_json = cut_public_key(&cut_base58(&cut_identicon(&timeless(&real_json))));
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Error","screenData":{"log":[{"order":5,"timestamp":"**","events":[{"event":"message_sign_error","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"**","identicon":"**","encryption":"sr25519"},"user_comment":"Pepper tries sending text from passworded account","error":"wrong_password_entered"}}]},{"order":4,"timestamp":"**","events":[{"event":"message_sign_error","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"**","identicon":"**","encryption":"sr25519"},"user_comment":"Pepper tries sending text from passworded account","error":"wrong_password_entered"}}]},{"order":3,"timestamp":"**","events":[{"event":"message_sign_error","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"**","identicon":"**","encryption":"sr25519"},"user_comment":"Pepper tries sending text from passworded account","error":"wrong_password_entered"}}]},{"order":2,"timestamp":"**","events":[{"event":"identity_added","payload":{"seed_name":"Pepper","encryption":"sr25519","public_key":"**","path":"//0","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}]},{"order":1,"timestamp":"**","events":[{"event":"identity_removed","payload":{"seed_name":"Pepper","encryption":"sr25519","public_key":"**","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}]},{"order":0,"timestamp":"**","events":[{"event":"history_cleared"}]}],"total_entries":6},"modalData":{},"alertData":{"error":"Wrong password."}}"#;
        assert!(cut_real_json == expected_json, "GoForward on Transaction screen for passworded address with third wrong password. Expected Log screen, got:\n{}", cut_real_json);

        do_action(Action::RightButton, "", "");
        do_action(Action::ClearLog, "", "");

        do_action(Action::NavbarScan, "", "");
        do_action(Action::TransactionFetched, &message_hex, "");
        do_action(
            Action::GoForward,
            "Pepper tries better",
            &seed_phrase_pepper,
        );
        let real_json = do_action(Action::GoForward, "secret", "");
        let (cut_real_json, signature_hex) = process_signature(&real_json);
        let expected_json = format!("{{\"screen\":\"Transaction\",\"screenLabel\":\"\",\"back\":true,\"footer\":false,\"footerButton\":\"Scan\",\"rightButton\":\"None\",\"screenNameType\":\"h1\",\"modal\":\"SignatureReady\",\"alert\":\"Empty\",\"screenData\":{{\"content\":{{\"message\":[{{\"index\":0,\"indent\":0,\"type\":\"text\",\"payload\":\"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e\"}}]}},\"author_info\":{{\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"Pepper\",\"derivation_path\":\"//0\",\"has_pwd\":true}},\"network_info\":{{\"network_title\":\"Westend\",\"network_logo\":\"westend\"}},\"type\":\"done\"}},\"modalData\":{{\"signature\":\"**\"}},\"alertData\":{{}}}}", pepper_key0_base58, pepper_key0_identicon);
        assert!(cut_real_json == expected_json, "GoForward on Transaction screen for passworded address with correct password. Expected Transaction screen with SignatureReady modal, got:\n{}", real_json);

        assert!(
            signature_is_good(&message_hex, &signature_hex),
            "Produced bad signature: \n{}",
            signature_hex
        );

        do_action(Action::GoBack, "", "");

        {
            let _database = db_handling::helpers::open_db::<Signer>(dbname).unwrap(); // database got unavailable for some reason

            let real_json = do_action(Action::NavbarKeys, "", "");
            let expected_json = r#"{"screen":"SeedSelector","screenLabel":"Select seed","back":false,"footer":true,"footerButton":"Keys","rightButton":"NewSeed","screenNameType":"h1","modal":"Empty","alert":"ErrorDisplay","screenData":{"seedNameCards":[]},"modalData":{},"alertData":{"error":"Database error. Internal error. IO error: could not acquire lock on "for_tests/flow_test_1/db": Os {**}"}}"#;
            let cut_real_json = cut_os_msg(&real_json);
            assert!(cut_real_json == expected_json, "Tried to switch from Log to Keys with unavailable database. Expected empty SeedSelector with ErrorDisplay alert, got:\n{}", real_json);

            let real_json = do_action(Action::GoBack, "", "");
            let expected_json = r#"{"screen":"Settings","screenLabel":"","back":false,"footer":true,"footerButton":"Settings","rightButton":"None","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"error":"Database error. Internal error. IO error: could not acquire lock on "for_tests/flow_test_1/db": Os {**}"},"modalData":{},"alertData":{}}"#;
            let cut_real_json = cut_os_msg(&real_json);
            assert!(cut_real_json == expected_json, "GoBack on SeedSelector with ErrorDisplay alert. Expected Settings screen with error displayed in screen details, got:\n{}", real_json);
        }

        // Aaand, we are back
        let real_json = do_action(Action::NavbarSettings, "", "");
        assert!(
            real_json == current_settings_json,
            "Reload Settings. Expected known Settings screen with no errors, got:\n{}",
            real_json
        );

        let real_json = do_action(Action::NavbarLog, "", "");
        let cut_real_json = cut_public_key(&cut_base58(&cut_identicon(&timeless(&real_json))));
        let expected_json = r#"{"screen":"Log","screenLabel":"","back":false,"footer":true,"footerButton":"Log","rightButton":"LogRight","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"log":[{"order":1,"timestamp":"**","events":[{"event":"message_signed","payload":{"message":"4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e","network_name":"westend","signed_by":{"public_key":"**","identicon":"**","encryption":"sr25519"},"user_comment":"Pepper tries better"}}]},{"order":0,"timestamp":"**","events":[{"event":"history_cleared"}]}],"total_entries":2},"modalData":{},"alertData":{}}"#;
        assert!(
            cut_real_json == expected_json,
            "Switched to Log from Settings. Expected Log screen, got:\n{}",
            real_json
        );

        // What if the database is not initiated properly? This should have been a separate test, but mutex.
        populate_cold_nav_test(dbname).unwrap(); // no init after population
        let real_json = do_action(Action::NavbarSettings, "", "");
        let expected_json = r#"{"screen":"Settings","screenLabel":"","back":false,"footer":true,"footerButton":"Settings","rightButton":"None","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"error":"Could not find general verifier."},"modalData":{},"alertData":{}}"#;
        assert!(real_json == expected_json, "Switched to Settings from Log with non-initiated database. Expected Settings screen with error on screen, and no alerts (we should still allow to reset Signer), got:\n{}", real_json);
    */

    std::fs::remove_dir_all(dbname).unwrap();
}
