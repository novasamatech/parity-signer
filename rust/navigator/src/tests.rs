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
        types_known,
    },
    ALICE_SEED_PHRASE,
};
use db_handling::cold_default::{init_db, populate_cold_nav_test};
use definitions::{
    crypto::Encryption,
    error_signer::Signer,
    history::{
        Event, IdentityHistory, MetaValuesDisplay, MetaValuesExport, NetworkSpecsDisplay,
        NetworkSpecsExport, SignDisplay, SignMessageDisplay, TypesDisplay, TypesExport,
    },
    navigation::{
        ActionResult, Address, AlertData, Card, DerivationCheck, DerivationDestination,
        DerivationEntry, DerivationPack, FooterButton, History, MBackup, MDeriveKey,
        MEnterPassword, MEventMaybeDecoded, MKeyDetails, MKeyDetailsMulti, MKeys, MKeysCard, MLog,
        MLogDetails, MLogRight, MMMNetwork, MMNetwork, MManageMetadata, MManageNetworks,
        MMetadataRecord, MNetworkCard, MNetworkDetails, MNetworkMenu, MNewSeed, MNewSeedBackup,
        MPasswordConfirm, MRawKey, MRecoverSeedName, MRecoverSeedPhrase, MSCCall, MSCContent,
        MSCCurrency, MSCEnumVariantName, MSCEraMortal, MSCFieldName, MSCId, MSCNameVersion,
        MSCNetworkInfo, MSeedKeyCard, MSeedMenu, MSeeds, MSettings, MSignSufficientCrypto,
        MSignatureReady, MSufficientCryptoReady, MTransaction, MTypesInfo, MVerifier,
        MVerifierDetails, ModalData, Network, NetworkSpecsToSend, RightButton, ScreenData,
        ScreenNameType, SeedNameCard, TransactionCard, TransactionCardSet, TransactionType,
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

fn cut_os_msg(error: &str) -> String {
    OS_MSG.replace_all(error, r#"Os {**}"#).to_string()
}

fn cut_seed_remove_identicon(data: &mut Option<ModalData>) -> String {
    if let Some(ModalData::NewSeedBackup { f }) = data {
        let res = f.seed_phrase.clone();
        f.seed_phrase = String::new();
        f.identicon = vec![];
        res
    } else {
        panic!("Expected ModalData::NewSeedBackup, got {:?}", data);
    }
}
fn qr_payload(qr_content: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(qr_content).unwrap();
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
                if message.len() > 257 && &transaction_hex[4..6] != "03" {
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
                if message.len() > 257 && &transaction_hex[4..6] != "03" {
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
                if message.len() > 257 && &transaction_hex[4..6] != "03" {
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
            seed_name_card.identicon = vec![];
        }
    } else {
        panic!("expected ScreenData::SeedSelector, got {:?}", m);
    }
}

fn erase_modal_seed_phrase_and_identicon(m: &mut ModalData) -> String {
    if let ModalData::NewSeedBackup { f } = m {
        let res = f.seed_phrase.clone();
        f.seed_phrase = String::new();
        f.identicon = vec![];
        res
    } else {
        panic!("expected ModalData::NewSeedBackup got {:?}", m);
    }
}

fn erase_base58_address_identicon(m: &mut ScreenData) {
    if let ScreenData::Keys { f } = m {
        for key in f.set.iter_mut() {
            key.identicon = vec![];
            key.base58 = String::new();
            key.address_key = String::new();
        }
        f.root.identicon = vec![];
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
    init_db::<Signer>(dbname, verifier_alice_sr25519()).unwrap();
    init_navigation(dbname, Vec::new());

    let action = do_action(Action::Start, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: Some(ModalData::NewSeedMenu),
        alert_data: None,
    };
    assert_eq!(action, expected_action);

    let mut seed_selector_action = action;

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();

    erase_log_timestamps(&mut action.screen_data);

    let hex = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(action, expected_action);

    let mut current_log_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();

    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with no modals. Expected to remain where was.",
    );

    let mut action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoForward on Log screen with no modals. Expected to remain where was.",
    );

    let mut action = do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    erase_log_timestamps(&mut action.screen_data);
    if let Some(ref mut m) = action.modal_data {
        erase_modal_data_checksum(m);
    }

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: Some(ModalData::LogRight {
            f: MLogRight {
                checksum: String::new(),
            },
        }),
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "RightButton on Log screen with no modals. Expected same Log screen with LogRight modal"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with LogRight modal. Expected to get Log screen with no modals"
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();

    let mut action = do_action(Action::CreateLogComment, "", "")
        .unwrap()
        .unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: Some(ModalData::LogComment),
        alert_data: None,
    };

    assert_eq!(action, expected_action,
            "CreateLogComment on Log screen with LogRight modal. Expected same Log screen with LogComment modal");
    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    assert_eq!(
        current_log_action, action,
        "GoBack on Log screen with LogComment modal. Expected same Log screen with no modals"
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::CreateLogComment, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::GoForward, "Remember this moment", "")
        .unwrap()
        .unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(action, expected_action, "GoForward on Log screen with LogComment modal. Expected updated Log screen with no modals.");

    let mut action = do_action(Action::Shield, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.alert_data = Some(AlertData::Shield { f: None });

    assert_eq!(
        action, expected_action,
        "Shield on Log screen with no modal. Expected same Log screen with Shield alert.",
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![History {
                order: 0,
                timestamp: String::new(),
                events: vec![Event::HistoryCleared],
            }],
        },
    };
    expected_action.alert_data = None;
    let log_action = expected_action.clone();
    let empty_log = expected_action.clone();

    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals."
    );

    let action = do_action(Action::NavbarSettings, "", "").unwrap().unwrap();

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Settings {
            f: MSettings {
                public_key: Some(
                    "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string(),
                ),
                identicon: Some(alice_sr_alice().to_vec()),
                encryption: Some("sr25519".to_string()),
                error: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NavbarSettings on Log screen. Expected Settings screen with no modals",
    );

    let current_settings_action = action;

    let action = do_action(Action::BackupSeed, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::SelectSeedForBackup {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "BackupSeed on Settings screen. Expected SelectSeedForBackup screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on SelectSeedForBackup screen with no seeds available. Expected Settings screen with no modals."
    );

    let action = do_action(Action::ViewGeneralVerifier, "", "")
        .unwrap()
        .unwrap();

    let expected_action = ActionResult {
        screen_label: "VERIFIER CERTIFICATE".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::VVerifier {
            f: MVerifierDetails {
                public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    .to_string(),
                identicon: alice_sr_alice().to_vec(),
                encryption: "sr25519".to_string(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "ViewGeneralVerifier on Settings screen. Expected Verifier screen with no modals.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on Verifier screen. Expected Settings screen with no modals.",
    );

    let action = do_action(Action::ShowDocuments, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "ABOUT".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Documents,
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "ShowDocuments on Settings screen. Expected Documents screen with no modals.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on Documents screen. Expected Settings screen with no modals.",
    );

    let action = do_action(Action::ManageNetworks, "", "").unwrap().unwrap();

    let expected_action = ActionResult {
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::TypesInfo),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                            .to_string(),
                        title: "Kusama".to_string(),
                        logo: "kusama".to_string(),
                        order: 1,
                    },
                    MMNetwork {
                        key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 2,
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "ManageNetworks on Settings screen. Expected ManageNetworks screen with no modals."
    );

    let mut manage_networks_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on ManageNetworks screen. Expected Settings screen with no modals.",
    );

    do_action(Action::ManageNetworks, "", "").unwrap().unwrap();
    let action = do_action(
        Action::GoForward,
        "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
        "",
    )
    .unwrap()
    .unwrap();
    let expected_action = ActionResult {
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::NDMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
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
                        identicon: alice_sr_alice().to_vec(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![MMetadataRecord {
                    specname: "kusama".to_string(),
                    specs_version: "9130".to_string(),
                    meta_hash: "3e6bf025743e5cc550883170d91c8275fb238762b214922b41d64f9feba23987"
                        .to_string(),
                    meta_id_pic: kusama_9130().to_vec(),
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "GoForward on ManageNetworks screen with kusama sr25519 key. Expected NetworkDetails screen for kusama with no modals."
    );

    let kusama_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, manage_networks_action,
        "GoBack on NetworkDetails screen. Expected ManageNetworks screen with no modals.",
    );

    do_action(
        Action::GoForward,
        "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
        "",
    )
    .unwrap();
    let action = do_action(Action::ManageMetadata, "9130", "")
        .unwrap()
        .unwrap();

    let mut kusama_action_modal = kusama_action.clone();
    kusama_action_modal.modal_data = Some(ModalData::ManageMetadata {
        f: MManageMetadata {
            name: "kusama".to_string(),
            version: "9130".to_string(),
            meta_hash: "3e6bf025743e5cc550883170d91c8275fb238762b214922b41d64f9feba23987"
                .to_string(),
            meta_id_pic: kusama_9130().to_vec(),
            networks: vec![MMMNetwork {
                title: "Kusama".to_string(),
                logo: "kusama".to_string(),
                order: 1,
                current_on_screen: true,
            }],
        },
    });
    assert_eq!(action, kusama_action_modal, "ManageMetadata on NetworkDetails screen for kusama sr25519 key. Expected NetworkDetails screen for kusama with ManageMetadata modal");

    let action = do_action(Action::SignMetadata, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto { identities: vec![] },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action, "SignMetadata on NetworkDetails screen for kusama sr25519 key with ManageMetadata modal for version 9130. Expected SignSufficientCrypto screen for kusama9130 metadata with no modals");
    let sign_sufficient_crypto_action = expected_action;

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, kusama_action,
        "GoBack on SignSufficientCrypto screen. Expected NetworkDetails screen with no modals."
    );

    do_action(Action::ManageMetadata, "9130", "")
        .unwrap()
        .unwrap();
    let action = do_action(Action::RemoveMetadata, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::NDMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
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
                        identicon: alice_sr_alice().to_vec(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action, "RemoveMetadata on ManageNetworks screen with kusama sr25519 key with ManageMetadata modal for version 9130. Expected updated NetworkDetails screen for kusama with no modals");

    let kusama_action = action;

    let action = do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut expected_action = kusama_action.clone();
    expected_action.modal_data = Some(ModalData::NetworkDetailsMenu);
    assert_eq!(action, expected_action, "RightButton on NetworkDetails screen for kusama sr25519 key. Expected NetworkDetails screen for kusama with NetworkDetailsMenu modal");

    let action = do_action(Action::SignNetworkSpecs, "", "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto { identities: vec![] },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(action, expected_action, "SignNetworkSpecs on NetworkDetails screen for kusama sr25519 key with NetworkDetailsMenu modal. Expected SignSufficientCrypto screen for kusama specs with no modals.");

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, kusama_action,
        "GoBack on SignSufficientCrypto screen. Expected NetworkDetails screen with no modals."
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let action = do_action(Action::RemoveNetwork, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::TypesInfo),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 1,
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "RemoveNetwork on NetworkDetails screen for kusama sr25519. Expected updated ManageNetworks screen with no modals"
    );

    let action = do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut expected_action = expected_action;
    expected_action.right_button = Some(RightButton::TypesInfo);
    expected_action.modal_data = Some(ModalData::TypesInfo {
        f: MTypesInfo {
            types_on_file: true,
            types_hash: Some(
                "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string(),
            ),
            types_id_pic: Some(types_known().to_vec()),
        },
    });

    assert_eq!(
        action, expected_action,
        "RightButton on ManageNetworks screen. Expected ManageNetworks screen with TypesInfo modal"
    );

    let action = do_action(Action::SignTypes, "", "").unwrap().unwrap();

    let expected_action = sign_sufficient_crypto_action;
    assert_eq!(
        action, expected_action,
        "SignTypes on ManageNetworks screen with TypesInfo modal. Expected SignSufficientCrypto screen for types with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "GoBack on SignSufficientCrypto screen. Expected Settings screen with no modals",
    );

    do_action(Action::ManageNetworks, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::RemoveTypes, "", "").unwrap().unwrap();
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
        },
    };

    assert_eq!(action, expected_action, "RemoveTypes on ManageNetworks screen with TypesInfo modal. Expected Log screen with no modals");

    current_log_action = action;

    let mut action = do_action(Action::ShowLogDetails, "2", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let genesis_hash = "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";
    let mut expected_action = ActionResult {
        screen_label: "Event details".to_string(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::LogDetails {
            f: MLogDetails {
                timestamp: String::new(),
                events: vec![MEventMaybeDecoded {
                    event: Event::NetworkSpecsRemoved {
                        network_specs_display: NetworkSpecsDisplay {
                            specs: NetworkSpecs {
                                base58prefix: 2,
                                color: "#000".to_string(),
                                decimals: 12,
                                encryption: Encryption::Sr25519,
                                genesis_hash: H256::from_str(genesis_hash).unwrap(),
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
                    },
                    decoded: None,
                    signed_by: None,
                    verifier_details: None,
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "ShowLogDetails on Log screen with order 2. Expected LogDetails screen with no modals"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, current_log_action,
        "GoBack on ShowLogDetails screen. Expected Log screen with no modals.",
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    expected_action.screen_label = "".to_string();
    expected_action.back = false;
    expected_action.right_button = Some(RightButton::LogRight);
    expected_action.screen_data = ScreenData::Log {
        f: MLog {
            log: vec![History {
                order: 0,
                timestamp: String::new(),
                events: vec![Event::HistoryCleared],
            }],
        },
    };
    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals."
    );

    let action = do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Scan,
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NavbarScan on Log screen. Expected Scan screen with no modals.",
    );

    let scan_action = action;

    let action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/add_specs_kusama-sr25519_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap()
    .unwrap();
    let aaa = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let genesis_hash = "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key: aaa,
                                identicon: alice_sr_alice().to_vec(),
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
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action, "TransactionFetched on Scan screen with add_specs info for kusama. Expected Transaction screen with no modals");

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
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
    let action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::NDMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
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
                        identicon: alice_sr_alice().to_vec(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoForward on Transaction screen with add specs stub. Expected NetworkDetails screen for kusama sr25519, with no modals"
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "MANAGE NETWORKS".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::TypesInfo),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::ManageNetworks {
            f: MManageNetworks {
                networks: vec![
                    MMNetwork {
                        key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                            .to_string(),
                        title: "Polkadot".to_string(),
                        logo: "polkadot".to_string(),
                        order: 0,
                    },
                    MMNetwork {
                        key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                            .to_string(),
                        title: "Westend".to_string(),
                        logo: "westend".to_string(),
                        order: 1,
                    },
                    MMNetwork {
                        key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                            .to_string(),
                        title: "Kusama".to_string(),
                        logo: "kusama".to_string(),
                        order: 2,
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action, "GoBack on NetworkDetails screen after adding kusama sr25519 specs. Expected ManageNetworks screen with no modals.");

    manage_networks_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(action, current_settings_action, "GoBack on ManageNetworks screen, to see footer. Expected known Settings screen with no modals.");

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
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
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/load_metadata_kusamaV9151_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap()
    .unwrap();

    let aaa_2 = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let meta_hash = "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46".to_string();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key: aaa_2,
                                identicon: alice_sr_alice().to_vec(),
                                encryption: "sr25519".to_string(),
                            },
                        },
                    }]),
                    meta: Some(vec![TransactionCard {
                        index: 1,
                        indent: 0,
                        card: Card::MetaCard {
                            f: MMetadataRecord {
                                specname: "kusama".to_string(),
                                specs_version: "9151".to_string(),
                                meta_hash,
                                meta_id_pic: kusama_9151().to_vec(),
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
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "TransactionFetched on Scan screen with load_metadata for kusama9151. Expected Transaction screen with no modals"
    );

    let action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Network details".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::NDMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::NNetworkDetails {
            f: MNetworkDetails {
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
                        identicon: alice_sr_alice().to_vec(),
                        encryption: "sr25519".to_string(),
                    },
                },
                meta: vec![MMetadataRecord {
                    specname: "kusama".to_string(),
                    specs_version: "9151".to_string(),
                    meta_hash: "9a179da92949dd3ab3829177149ec83dc46fb009af10a45f955949b2a6693b46"
                        .to_string(),
                    meta_id_pic: kusama_9151().to_vec(),
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action,
        "GoForward on Transaction screen with load metadata stub. Expected NetworkDetails screen for kusama sr25519, updated with new metadata, with no modals"
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
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
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let action = do_action(
        Action::TransactionFetched,
        std::fs::read_to_string("for_tests/load_types_Alice-sr25519.txt")
            .unwrap()
            .trim(),
        "",
    )
    .unwrap()
    .unwrap();
    let public_key = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string();
    let types_hash =
        Some("d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string());
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    verifier: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::VerifierCard {
                            f: MVerifierDetails {
                                public_key,
                                identicon: alice_sr_alice().to_vec(),
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
                                types_id_pic: Some(types_known().to_vec()),
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
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "TransactionFetched on Scan screen with load_types. Not that we really need them anymore. Expected Transaction screen with no modals."
    );

    let action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    assert_eq!(
        action, manage_networks_action,
        "GoForward on Transaction screen with load types stub. Expected known ManageNetworks screen with no modals."
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
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
        },
    };

    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected updated Log screen with no modals.",
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals"
    );

    current_log_action = action;

    let action = do_action(Action::NavbarKeys, "", "").unwrap().unwrap();

    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![],
            },
        },
        modal_data: Some(ModalData::NewSeedMenu),
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NavbarKeys on Log screen. Expected SeedSelector screen with NewSeedMenu modal",
    );

    let action = do_action(Action::NewSeed, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: true },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NewSeed on SeedSelector screen with NewSeedMenu modal. Expected NewSeed screen.",
    );

    let new_seed_action = action;

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, seed_selector_action,
        "GoBack on NewSeed screen. Expected SeedSelector screen with no modals.",
    );

    do_action(Action::NewSeed, "", "").unwrap().unwrap();
    let mut action = do_action(Action::GoForward, "Portia", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: false },
        },
        modal_data: Some(ModalData::NewSeedBackup {
            f: MNewSeedBackup {
                seed: "Portia".to_string(),
                seed_phrase: String::new(),
                identicon: vec![],
            },
        }),
        alert_data: None,
    };
    if let Some(ref mut m) = action.modal_data {
        erase_modal_seed_phrase_and_identicon(m);
    }
    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with non-empty seed name. Expected NewSeed screen with NewSeedBackup modal."
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, new_seed_action,
        "GoBack on NewSeed screen with generated seed. Expected NewSeed screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(action, seed_selector_action, "GoBack on NewSeed screen with no modals, to see footer. Expected known SeedSelector screen with no modals");

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, current_log_action,
        "Switched to Log from SeedSelector after cancelling seed creation. Expected known Log screen with no modals.",
    );

    do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
    do_action(Action::NewSeed, "", "").unwrap().unwrap();
    let mut action = do_action(Action::GoForward, "Portia", "").unwrap().unwrap();
    let seed_phrase_portia = if let Some(ref mut m) = action.modal_data {
        erase_modal_seed_phrase_and_identicon(m)
    } else {
        String::new()
    };
    let expected_action = ActionResult {
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: false },
        },
        modal_data: Some(ModalData::NewSeedBackup {
            f: MNewSeedBackup {
                seed: "Portia".to_string(),
                seed_phrase: String::new(),
                identicon: vec![],
            },
        }),
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with non-empty seed name. Expected NewSeed screen with NewSeedBackup modal."
    );

    let mut action = do_action(Action::GoForward, "true", &seed_phrase_portia)
        .unwrap()
        .unwrap();
    erase_base58_address_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key: String::new(),
                    base58: String::new(),
                    identicon: vec![],
                    has_pwd: false,
                    path: "//polkadot".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Portia".to_string(),
                    identicon: vec![],
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
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoForward on NewSeed screen with NewSeedBackup modal active. Expected Keys screen with no modals."
    );

    update_seed_names(vec![String::from("Portia")]);

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_identicon(&mut action.screen_data);

    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![SeedNameCard {
                    seed_name: "Portia".to_string(),
                    identicon: vec![],
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "GoBack on Keys screen. Expected updated SeedSelector screen with no modals.",
    );

    seed_selector_action = action;

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    erase_public_keys(&mut action.screen_data);
    erase_log_timestamps(&mut action.screen_data);

    let network_genesis_hash_polkadot =
        "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";

    let network_genesis_hash_kusama =
        "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";

    let network_genesis_hash_westend =
        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(action, expected_action);

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![History {
                    order: 0,
                    timestamp: String::new(),
                    events: vec![Event::HistoryCleared],
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "ClearLog on Log screen with LogRight modal. Expected updated Log screen with no modals"
    );

    do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let action = do_action(Action::RecoverSeed, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: true,
                seed_name: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "RecoverSeed on SeedSelector screen with NewSeedMenu modal. Expected RecoverSeedName screen with no modals"
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_identicon(&mut action.screen_data);
    assert_eq!(
        action, seed_selector_action,
        "GoBack on RecoverSeedName screen with no modals. Expected known SeedSelector screen"
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::RecoverSeed, "", "").unwrap().unwrap();
    let action = do_action(Action::GoForward, "Portia", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: false,
                seed_name: String::new(),
            },
        },
        modal_data: None,
        alert_data: Some(AlertData::ErrorData {
            f: "Bad input data. Seed name Portia already exists.".to_string(),
        }),
    };
    assert_eq!(
        action, expected_action,
        "GoForward on RecoverSeedName screen using existing name. Expected RecoverSeedName screen with error."
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let action = do_action(Action::GoForward, "Alys", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "GoForward on RecoverSeedName screen using new name. Expected RecoverSeedPhrase screen with no modals."
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::RecoverSeedName {
            f: MRecoverSeedName {
                keyboard: true,
                seed_name: "Alys".to_string(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "GoBack on RecoverSeedPhrase screen. Expected RecoverSeedName screen with no modals and with retained name."
    );

    let action = do_action(Action::GoForward, "Alice", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on RecoverSeedName screen using new name. ",
            "Expected RecoverSeedPhrase screen with no modals."
        )
    );

    // Alice painstakingly recalls her seed phrase
    let action = do_action(Action::TextEntry, " botto", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word. ",
            "Expected RecoverSeedPhrase screen with no modals."
        )
    );

    let action = do_action(Action::TextEntry, " botto ", "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                draft: vec!["bottom".to_string()],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word and space. ",
            "Expected word to be added"
        )
    );

    let action = do_action(Action::TextEntry, " abstract ", "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                draft: vec!["bottom".to_string(), "abstract".to_string()],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with complete long word. ",
            " Wrong one. Expected word to be added"
        )
    );

    let action = do_action(Action::TextEntry, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                draft: vec!["bottom".to_string()],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with empty text. ",
            "Expected last draft word to be deleted"
        )
    );

    do_action(Action::TextEntry, " d", "").unwrap().unwrap();

    // a cat interfered
    let action = do_action(Action::TextEntry, " ddddddddddddddd", "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                draft: vec!["bottom".to_string()],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with no guesses. ",
            "Expected to keep previous good user entry",
        )
    );

    let action = do_action(Action::TextEntry, " dddddddd ", "")
        .unwrap()
        .unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with erroneous entry, ",
            "attempted to add to the draft using whitespace. Expected nothing to happen."
        )
    );

    do_action(Action::TextEntry, " driv ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " obe ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " lake ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " curt ", "").unwrap().unwrap();
    let action = do_action(Action::TextEntry, " som", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "som".to_string(),
                guess_set: vec!["someone".to_string()],
                draft: vec![
                    "bottom".to_string(),
                    "drive".to_string(),
                    "obey".to_string(),
                    "lake".to_string(),
                    "curtain".to_string(),
                ],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word with typo. ",
            "Expected correct draft and guesses for wrong entry"
        )
    );

    let action = do_action(Action::TextEntry, " smo", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::RecoverSeedPhrase {
            f: MRecoverSeedPhrase {
                keyboard: true,
                seed_name: "Alice".to_string(),
                user_input: "smo".to_string(),
                guess_set: vec!["smoke".to_string(), "smooth".to_string()],
                draft: vec![
                    "bottom".to_string(),
                    "drive".to_string(),
                    "obey".to_string(),
                    "lake".to_string(),
                    "curtain".to_string(),
                ],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TextEntry on RecoverSeedPhrase screen with incomplete word. ",
            "Expected correct draft and a few guesses"
        )
    );

    let action = do_action(Action::TextEntry, " smo ", "").unwrap().unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "Try to enter with whitespace a word with multiple possible endings. ",
            "Expected nothing to happen"
        )
    );

    let action = do_action(Action::PushWord, "smoke", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                    "bottom".to_string(),
                    "drive".to_string(),
                    "obey".to_string(),
                    "lake".to_string(),
                    "curtain".to_string(),
                    "smoke".to_string(),
                ],
                ready_seed: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "PushWord on RecoverSeedPhrase screen. ",
            "Expected correct draft and empty user_input."
        )
    );

    do_action(Action::TextEntry, " bask ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " hold ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " race ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " lone ", "").unwrap().unwrap();
    do_action(Action::TextEntry, " fit ", "").unwrap().unwrap();
    let action = do_action(Action::TextEntry, " walk ", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Recover Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                    "bottom".to_string(),
                    "drive".to_string(),
                    "obey".to_string(),
                    "lake".to_string(),
                    "curtain".to_string(),
                    "smoke".to_string(),
                    "basket".to_string(),
                    "hold".to_string(),
                    "race".to_string(),
                    "lonely".to_string(),
                    "fit".to_string(),
                    "walk".to_string(),
                ],
                ready_seed: Some(
                    "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
                        .to_string(),
                ),
            },
        },
        modal_data: None,
        alert_data: None,
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

    let action = do_action(Action::GoForward, "false", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key:
                        "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                            .to_string(),
                    base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                    identicon: alice_sr_polkadot().to_vec(),
                    has_pwd: false,
                    path: "//polkadot".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                // since root == 'false' in do_action above.
                // TODO: This has to be wrapped with Option<_>.
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: empty_png().to_vec(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
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

    update_seed_names(vec![String::from("Portia"), String::from("Alice")]);

    let mut alice_polkadot_keys_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![
                    SeedNameCard {
                        seed_name: "Alice".to_string(),
                        identicon: vec![],
                    },
                    SeedNameCard {
                        seed_name: "Portia".to_string(),
                        identicon: vec![],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoBack on Keys screen. Expected updated SeedSelector screen with no modals",
    );

    let action = do_action(Action::SelectSeed, "Alice", "").unwrap().unwrap();
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
    .unwrap()
    .unwrap();

    let expected_action = ActionResult {
        screen_label: "Derived Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::KeyMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::KeyDetails {
            f: MKeyDetails {
                qr: alice_polkadot_qr().to_vec(),
                pubkey: "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                    .to_string(),
                address: Address {
                    base58: "16Zaf6BT6xc6WeYCX6YNAf67RumWaEiumwawt7cTdKMU7HqW".to_string(),
                    identicon: alice_sr_polkadot().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//polkadot".to_string(),
                    has_pwd: false,
                    multiselect: None,
                },
                network_info: MSCNetworkInfo {
                    network_title: "Polkadot".to_string(),
                    network_logo: "polkadot".to_string(),
                },
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "SelectKey on Keys screen. Expected KeyDetails screen for Alice //polkadot key.",
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        "GoBack on KeyDetails screen. Expected known Keys screen for Alice polkadot keys.",
    );

    let action = do_action(Action::NewKey, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Alice".to_string(),
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
                network_specs_key:
                    "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3".to_string(),
                suggested_derivation: String::new(),
                keyboard: true,
                derivation_check: DerivationCheck {
                    button_good: true,
                    where_to: Some(DerivationDestination::Pin),
                    ..Default::default()
                },
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NewKey on Keys screen. Expected DeriveKey screen",
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        "GoBack on DeriveKey screen. Expected known Keys screen for Alice polkadot keys",
    );

    do_action(Action::NewKey, "", "").unwrap().unwrap();
    let action = do_action(Action::CheckPassword, "//secret//path///multipass", "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Alice".to_string(),
                network_title: "Polkadot".to_string(),
                network_logo: "polkadot".to_string(),
                network_specs_key:
                    "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3".to_string(),
                suggested_derivation: "//secret//path///multipass".to_string(),
                keyboard: false,
                derivation_check: DerivationCheck {
                    button_good: true,
                    where_to: Some(DerivationDestination::Pwd),
                    ..Default::default()
                },
            },
        },
        modal_data: Some(ModalData::PasswordConfirm {
            f: MPasswordConfirm {
                pwd: "multipass".to_string(),
                seed_name: "Alice".to_string(),
                cropped_path: "//secret//path".to_string(),
            },
        }),
        alert_data: None,
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
    .unwrap()
    .unwrap();

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
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
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                // since root == 'false' in do_action above.
                // TODO: This has to be wrapped with Option<_>.
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: empty_png().to_vec(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Polkadot".to_string(),
                    logo: "polkadot".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on DeriveKey screen with PasswordConfirm modal. ",
            "Expected updated Keys screen"
        )
    );

    do_action(Action::NewKey, "", "").unwrap().unwrap();
    // trying to create the missing root
    let action = do_action(Action::GoForward, "", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
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
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
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
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoForward on DeriveKey screen with no modals. Expected updated Keys screen.",
    );

    alice_polkadot_keys_action = action;

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![
                    SeedNameCard {
                        seed_name: "Alice".to_string(),
                        identicon: vec![],
                    },
                    SeedNameCard {
                        seed_name: "Portia".to_string(),
                        identicon: vec![],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoBack on Keys screen, root Alice have appeared in polkadot. ",
            "Expected updated SeedSelector screen with no modals."
        )
    );

    do_action(Action::SelectSeed, "Alice", "").unwrap().unwrap();
    let action = do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
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
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
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
        modal_data: Some(ModalData::SeedMenu {
            f: MSeedMenu {
                seed: "Alice".to_string(),
            },
        }),
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "RightButton on Keys screen. Expected SeedMenu modal to appear",
    );

    let action = do_action(Action::BackupSeed, "", "").unwrap().unwrap();
    expected_action.modal_data = Some(ModalData::Backup {
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
    });
    expected_action.right_button = None;
    assert_eq!(
        action, expected_action,
        "BackupSeed on Keys screen with SeedMenu button. Expected Keys screen with Backup modal"
    );
    // mock signal from phone; elsewise untestable;
    db_handling::manage_history::seed_name_was_shown(dbname, String::from("Alice")).unwrap();

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
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
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "Switched to Log from SeedSelector after backuping seed. ",
            "Expected updated Log screen with no modals."
        )
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        concat!(
            "ClearLog on Log screen with LogRight modal. ",
            "Expected updated Log screen with no modals"
        )
    );

    do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
    do_action(Action::SelectSeed, "Portia", "")
        .unwrap()
        .unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let _action = do_action(Action::RemoveSeed, "", "").unwrap().unwrap();
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

    update_seed_names(vec![String::from("Alice")]);

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, empty_log,
        concat!(
            "ClearLog on Log screen with LogRight modal. ",
            "Expected updated Log screen with no modals"
        )
    );

    let action = do_action(Action::NavbarKeys, "", "").unwrap().unwrap();

    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::NewSeed),
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SeedSelector {
            f: MSeeds {
                seed_name_cards: vec![SeedNameCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NavbarKeys on Log screen. Expected updated SeedSelector screen with no modals",
    );

    do_action(Action::SelectSeed, "Alice", "").unwrap().unwrap();
    let action = do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    let mut expected_action = alice_polkadot_keys_action.clone();
    expected_action.modal_data = Some(ModalData::NetworkSelector {
        f: MNetworkMenu {
            networks: vec![
                Network {
                    key: "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"
                        .to_string(),
                    logo: "polkadot".to_string(),
                    order: 0,
                    selected: true,
                    title: "Polkadot".to_string(),
                },
                Network {
                    key: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                        .to_string(),
                    logo: "westend".to_string(),
                    order: 1,
                    selected: false,
                    title: "Westend".to_string(),
                },
                Network {
                    key: "01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"
                        .to_string(),
                    logo: "kusama".to_string(),
                    order: 2,
                    selected: false,
                    title: "Kusama".to_string(),
                },
            ],
        },
    });

    assert_eq!(
        action, expected_action,
        concat!(
            "NetworkSelector on Keys screen for Alice polkadot keys. ",
            "Expected modal NetworkSelector with polkadot selected"
        )
    );

    let action = do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    assert_eq!(
        action, alice_polkadot_keys_action,
        concat!(
            "NetworkSelector on Keys screen with NetworkSelector modal. ",
            "Expected known Keys screen for Alice"
        )
    );

    do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    let action = do_action(
        Action::ChangeNetwork,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap()
    .unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key:
                        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                            .to_string(),
                    base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                    identicon: alice_sr_westend().to_vec(),
                    has_pwd: false,
                    path: "//westend".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: empty_png().to_vec(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "ChangeNetwork on Keys screen. Expected Keys screen for Alice westend keys.",
    );

    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let action = do_action(Action::TransactionFetched,"53ffde01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e141c2f2f416c6963653c2f2f416c6963652f77657374656e64582f2f416c6963652f7365637265742f2f7365637265740c2f2f300c2f2f31","").unwrap().unwrap();
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
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
                network_info: Some(MSCNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                }),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen with import_derivations info for westend. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    expected_action.modal_data = Some(ModalData::SelectSeed {
        f: MSeeds {
            seed_name_cards: vec![SeedNameCard {
                seed_name: "Alice".to_string(),
                identicon: alice_sr_root().to_vec(),
            }],
        },
    });

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen with derivations import. ",
            "Expected Transaction screen with SelectSeed modal"
        )
    );

    let action = do_action(Action::GoForward, "Alice", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972"
                                .to_string(),
                        base58: "5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH".to_string(),
                        identicon: alice_sr_0().to_vec(),
                        has_pwd: false,
                        path: "//0".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        has_pwd: false,
                        path: "//westend".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        base58: "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK".to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        has_pwd: false,
                        path: "//Alice/secret//secret".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805"
                                .to_string(),
                        base58: "5FcKjDXS89U79cXvhksZ2pF5XBeafmSM8rqkDVoTHQcXd5Gq".to_string(),
                        identicon: alice_sr_alice_westend().to_vec(),
                        has_pwd: false,
                        path: "//Alice/westend".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48"
                                .to_string(),
                        base58: "5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o".to_string(),
                        identicon: alice_sr_1().to_vec(),
                        has_pwd: false,
                        path: "//1".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        has_pwd: false,
                        path: "//Alice".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: empty_png().to_vec(),
                    address_key: String::new(),
                    base58: String::new(),
                    swiped: false,
                    multiselect: false,
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    let mut keys_westend = expected_action.clone();

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen with derivations ",
            "import, with SelectSeed modal. ",
            "Expected updated Keys screen for Alice westend keys"
        )
    );

    let mut alice_westend_keys_action = action;

    do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    let action = do_action(
        Action::ChangeNetwork,
        "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
        "",
    )
    .unwrap()
    .unwrap(); // switching to polkadot, expect no changes
    assert_eq!(
        action, alice_polkadot_keys_action,
        concat!(
            "Switched network to polkadot. Expected no changes on Keys screen ",
            "for Alice polkadot keys"
        )
    );

    do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    do_action(
        Action::ChangeNetwork,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap();
    let action = do_action(
        Action::Swipe,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap()
    .unwrap();

    if let ScreenData::Keys { ref mut f } = keys_westend.screen_data {
        f.set[1].swiped = true;
    }
    assert_eq!(
        action, keys_westend,
        "Swipe on Keys screen for Alice westend keys. Expected updated Keys screen.",
    );
    // unswipe
    let action = do_action(
        Action::Swipe,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        action, alice_westend_keys_action,
        concat!(
            "Unswipe on Keys screen for Alice westend keys. ",
            "Expected known vanilla Keys screen for Alice westend keys."
        )
    );

    do_action(
        Action::Swipe,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    // swipe another
    let action = do_action(
        Action::Swipe,
        "019cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805",
        "",
    )
    .unwrap()
    .unwrap();

    if let ScreenData::Keys { ref mut f } = keys_westend.screen_data {
        f.set[1].swiped = false;
        f.set[3].swiped = true;
    }
    assert_eq!(
        action, keys_westend,
        concat!(
            "Swipe on Keys screen on another key while first swiped key ",
            "is still selected. Expected updated Keys screen"
        )
    );

    if let ScreenData::Keys { ref mut f } = keys_westend.screen_data {
        f.set.retain(|x| !x.swiped)
    }

    // remove swiped
    let action = do_action(Action::RemoveKey, "", "").unwrap().unwrap();
    assert_eq!(
        action, keys_westend,
        "RemoveKey on Keys screen with swiped key. Expected updated Keys screen.",
    );

    // Note: after removal, stay on the Keys screen (previously went to log).

    do_action(
        Action::Swipe,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    // increment swiped `//westend`
    let action = do_action(Action::Increment, "2", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f"
                                .to_string(),
                        base58: "5CofVLAGjwvdGXvBiP6ddtZYMVbhT5Xke8ZrshUpj2ZXAnND".to_string(),
                        identicon: alice_sr_westend_1().to_vec(),
                        has_pwd: false,
                        path: "//westend//1".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972"
                                .to_string(),
                        base58: "5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH".to_string(),
                        identicon: alice_sr_0().to_vec(),
                        has_pwd: false,
                        path: "//0".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        has_pwd: false,
                        path: "//westend".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        base58: "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK".to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        has_pwd: false,
                        path: "//Alice/secret//secret".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48"
                                .to_string(),
                        base58: "5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o".to_string(),
                        identicon: alice_sr_1().to_vec(),
                        has_pwd: false,
                        path: "//1".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        has_pwd: false,
                        path: "//Alice".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470"
                                .to_string(),
                        base58: "5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx".to_string(),
                        identicon: alice_sr_westend_0().to_vec(),
                        has_pwd: false,
                        path: "//westend//0".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: empty_png().to_vec(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "Increment on Keys screen with swiped key. Expected updated Keys screen",
    );

    do_action(
        Action::Swipe,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    let action = do_action(Action::Increment, "1", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set.insert(
            3,
            MKeysCard {
                address_key: "014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08"
                    .to_string(),
                base58: "5DqGKX9v7uR92EvmNNmiETZ9PcDBrg2YRYukGhzXHEkKmpfx".to_string(),
                identicon: alice_sr_westend_2().to_vec(),
                has_pwd: false,
                path: "//westend//2".to_string(),
                swiped: false,
                multiselect: false,
            },
        );
    }

    assert_eq!(
        action, expected_action,
        "Increment on Keys screen with swiped key. Expected updated Keys screen.",
    );

    // enter multi regime with LongTap
    let action = do_action(
        Action::LongTap,
        "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
        "",
    )
    .unwrap()
    .unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set[4].multiselect = true;
        f.multiselect_mode = true;
        f.multiselect_count = "1".to_string();
    };
    expected_action.right_button = Some(RightButton::MultiSelect);

    assert_eq!(
        action, expected_action,
        "LongTap on Keys screen. Expected updated Keys screen.",
    );

    // select by SelectKey in multi
    let action = do_action(
        Action::SelectKey,
        "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
        "",
    )
    .unwrap()
    .unwrap();
    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set[1].multiselect = true;
        f.multiselect_mode = true;
        f.multiselect_count = "2".to_string();
    };

    assert_eq!(
        action, expected_action,
        "SelectKey on Keys screen in multiselect mode. Expected updated Keys screen",
    );

    // deselect by SelectKey in multi
    let action = do_action(
        Action::SelectKey,
        "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
        "",
    )
    .unwrap()
    .unwrap();
    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set[1].multiselect = false;
        f.multiselect_count = "1".to_string();
    };

    assert_eq!(
        action, expected_action,
        "SelectKey on Keys screen in multiselect mode. Expected updated Keys screen.",
    );

    // deselect by LongTap in multi
    let action = do_action(
        Action::LongTap,
        "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
        "",
    )
    .unwrap()
    .unwrap();
    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set[4].multiselect = false;
        f.multiselect_count = "0".to_string();
    }

    assert_eq!(
        action, expected_action,
        "LongTap on Keys screen. Expected updated Keys screen.",
    );

    // Note: although multiselect count is 0, remain in multiselect mode

    do_action(
        Action::LongTap,
        "0120c394d410893cac63d993fa71eb8247e6af9a29cda467e836efec678b9f6b7f",
        "",
    )
    .unwrap();
    do_action(
        Action::LongTap,
        "012afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972",
        "",
    )
    .unwrap();
    do_action(
        Action::LongTap,
        "014e384fb30994d520094dce42086dbdd4977c11fb2f2cf9ca1c80056684934b08",
        "",
    )
    .unwrap();
    do_action(
        Action::LongTap,
        "01b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48",
        "",
    )
    .unwrap();
    do_action(
        Action::LongTap,
        "01e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470",
        "",
    )
    .unwrap();
    // remove keys in multiselect mode
    let action = do_action(Action::RemoveKey, "", "").unwrap().unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.set.remove(0);
        f.set.remove(0);
        f.set.remove(1);
        f.set.remove(2);
        f.set.remove(3);
        f.multiselect_count = "".to_string();
        f.multiselect_mode = false;
    }
    expected_action.right_button = Some(RightButton::Backup);
    assert_eq!(
        action, expected_action,
        "RemoveKey on Keys screen with multiselect mode. Expected updated Keys screen.",
    );
    // enter multiselect mode
    do_action(
        Action::LongTap,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    // select all
    let action = do_action(Action::SelectAll, "", "").unwrap().unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        for entry in f.set.iter_mut() {
            entry.multiselect = true;
        }
        f.multiselect_count = "3".to_string();
        f.multiselect_mode = true;
    }
    expected_action.right_button = Some(RightButton::MultiSelect);

    assert_eq!(
        action, expected_action,
        "SelectAll on Keys screen with multiselect mode. Expected updated Keys screen.",
    );

    // deselect all
    let action = do_action(Action::SelectAll, "", "").unwrap().unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        for entry in f.set.iter_mut() {
            entry.multiselect = false;
        }
        f.multiselect_count = "0".to_string();
        f.multiselect_mode = true;
    }
    assert_eq!(
        action, expected_action,
        "SelectAll on Keys screen with multiselect mode. Expected updated Keys screen.",
    );
    // exit multiselect mode
    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        f.multiselect_count = "".to_string();
        f.multiselect_mode = false;
    }
    expected_action.right_button = Some(RightButton::Backup);
    assert_eq!(
        action, expected_action,
        "GoBack on Keys screen with multiselect mode. Expected updated Keys screen.",
    );

    alice_westend_keys_action = action;

    // enter multiselect mode
    do_action(
        Action::LongTap,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    // select all
    do_action(Action::SelectAll, "", "").unwrap().unwrap();
    let action = do_action(Action::ExportMultiSelect, "", "")
        .unwrap()
        .unwrap();

    let expected_action = ActionResult {
        screen_label: "Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::KeyMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::KeyDetailsMulti {
            f: MKeyDetailsMulti {
                key_details: MKeyDetails {
                    qr: alice_westend_westend_qr().to_vec(),
                    pubkey: "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                        .to_string(),
                    address: Address {
                        base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        seed_name: "Alice".to_string(),
                        path: "//westend".to_string(),
                        has_pwd: false,
                        multiselect: None,
                    },
                    network_info: MSCNetworkInfo {
                        network_title: "Westend".to_string(),
                        network_logo: "westend".to_string(),
                    },
                },
                current_number: "1".to_string(),
                out_of: "3".to_string(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "ExportMultiSelect on Keys screen with multiselect mode. ",
            "Expected KeyDetailsMulti screen."
        )
    );

    let unit1 = action;

    let action = do_action(Action::NextUnit, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::KeyMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::KeyDetailsMulti {
            f: MKeyDetailsMulti {
                key_details: MKeyDetails {
                    qr: alice_westend_alice_secret_secret_qr().to_vec(),
                    pubkey: "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                        .to_string(),
                    address: Address {
                        base58: "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK".to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        seed_name: "Alice".to_string(),
                        path: "//Alice/secret//secret".to_string(),
                        has_pwd: false,
                        multiselect: None,
                    },
                    network_info: MSCNetworkInfo {
                        network_title: "Westend".to_string(),
                        network_logo: "westend".to_string(),
                    },
                },
                current_number: "2".to_string(),
                out_of: "3".to_string(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "ExportMultiSelect on Keys screen with multiselect mode. ",
            "Expected KeyDetailsMulti screen"
        )
    );

    let unit2 = action;

    let action = do_action(Action::NextUnit, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::KeyMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::KeyDetailsMulti {
            f: MKeyDetailsMulti {
                key_details: MKeyDetails {
                    qr: alice_westend_alice_qr().to_vec(),
                    pubkey: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    address: Address {
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        seed_name: "Alice".to_string(),
                        path: "//Alice".to_string(),
                        has_pwd: false,
                        multiselect: None,
                    },
                    network_info: MSCNetworkInfo {
                        network_title: "Westend".to_string(),
                        network_logo: "westend".to_string(),
                    },
                },
                current_number: "3".to_string(),
                out_of: "3".to_string(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "ExportMultiSelect on Keys screen with multiselect mode. ",
            "Expected KeyDetailsMulti screen."
        )
    );

    let unit3 = action;

    let action = do_action(Action::NextUnit, "", "").unwrap().unwrap();
    assert_eq!(
        action, unit1,
        "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen"
    );

    let action = do_action(Action::PreviousUnit, "", "").unwrap().unwrap();
    assert_eq!(
        action, unit3,
        "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen"
    );

    let action = do_action(Action::PreviousUnit, "", "").unwrap().unwrap();
    assert_eq!(
        action, unit2,
        "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen"
    );

    let action = do_action(Action::PreviousUnit, "", "").unwrap().unwrap();
    assert_eq!(
        action, unit1,
        "ExportMultiSelect on Keys screen with multiselect mode. Expected KeyDetailsMulti screen"
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, alice_westend_keys_action,
        "GoBack on KeyDetailsMulti screen. Expected Keys screen in plain mode",
    );

    let action = do_action(Action::NewKey, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Alice".to_string(),
                network_title: "Westend".to_string(),
                network_logo: "westend".to_string(),
                network_specs_key:
                    "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e".to_string(),
                suggested_derivation: String::new(),
                keyboard: true,
                derivation_check: DerivationCheck {
                    button_good: true,
                    where_to: Some(DerivationDestination::Pin),
                    ..Default::default()
                },
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "NewKey on Keys screen. Expected DeriveKey screen",
    );

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, alice_westend_keys_action,
        "GoBack on DeriveKey screen. Expected Keys screen in plain mode.",
    );

    do_action(Action::NewKey, "", "").unwrap().unwrap();
    // create root derivation
    let action = do_action(Action::GoForward, "", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        has_pwd: false,
                        path: "//westend".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        base58: "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK".to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        has_pwd: false,
                        path: "//Alice/secret//secret".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                    MKeysCard {
                        address_key:
                            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        has_pwd: false,
                        path: "//Alice".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
                    address_key:
                        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                            .to_string(),
                    base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
                    swiped: false,
                    multiselect: false,
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoForward on DeriveKey screen with empty derivation string. Expected updated Keys screen"
    );

    // enter multiselect mode
    do_action(
        Action::LongTap,
        "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        "",
    )
    .unwrap();
    // select all
    let action = do_action(Action::SelectAll, "", "").unwrap().unwrap();

    if let ScreenData::Keys { ref mut f } = expected_action.screen_data {
        for entry in f.set.iter_mut() {
            entry.multiselect = true;
        }
        f.multiselect_count = "4".to_string();
        f.multiselect_mode = true;
        f.root.multiselect = true;
    }
    expected_action.right_button = Some(RightButton::MultiSelect);

    assert_eq!(
        action, expected_action,
        concat!(
            "SelectAll on Keys screen in multiselect mode, ",
            "with existing root key. Expected updated Keys screen"
        )
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let action = do_action(
        Action::SelectKey,
        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
        "",
    )
    .unwrap()
    .unwrap();
    let expected_action = ActionResult {
        screen_label: "Seed Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::KeyMenu),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::KeyDetails {
            f: MKeyDetails {
                qr: alice_westend_root_qr().to_vec(),
                pubkey: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                    .to_string(),
                address: Address {
                    base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
                    identicon: alice_sr_root().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: String::new(),
                    has_pwd: false,
                    multiselect: None,
                },
                network_info: MSCNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                },
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "SelectKey on Keys screen with root key. ",
            "Expected KeyDetails screen with Seed Key label."
        )
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    let action = do_action(Action::BackupSeed, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: "Select seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::SelectSeedForBackup {
            f: MSeeds {
                seed_name_cards: vec![SeedNameCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "BackupSeed on Settings screen. ",
            "Expected SelectSeedForBackup screen with no modals"
        )
    );

    let action = do_action(Action::BackupSeed, "Alice", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![
                    MKeysCard {
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        base58: "16FWrEaDSDRwfDmNKacTBRNmYPH8Yg6s9o618vX2iHQLuWfb".to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
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
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                        swiped: false,
                        multiselect: false,
                    },
                ],
                root: MSeedKeyCard {
                    seed_name: "Alice".to_string(),
                    identicon: alice_sr_root().to_vec(),
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
        modal_data: Some(ModalData::Backup {
            f: MBackup {
                seed_name: "Alice".to_string(),
                derivations: vec![
                    DerivationPack {
                        network_title: "Polkadot".to_string(),
                        network_logo: "polkadot".to_string(),
                        network_order: "0".to_string(),
                        id_set: vec![
                            DerivationEntry {
                                path: String::new(),
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
                                path: "//Alice/secret//secret".to_string(),
                                has_pwd: false,
                            },
                            DerivationEntry {
                                path: "//Alice".to_string(),
                                has_pwd: false,
                            },
                        ],
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
        }),
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "BackupSeed on SelectSeedForBackup screen with Alice as ",
            "an entry. Expected Keys screen with Backup modal"
        )
    );
    // mock signal from phone
    db_handling::manage_history::seed_name_was_shown(dbname, String::from("Alice")).unwrap();

    do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    do_action(Action::ManageNetworks, "", "").unwrap().unwrap();
    do_action(
        Action::GoForward,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let action = do_action(Action::SignNetworkSpecs, "", "")
        .unwrap()
        .unwrap();
    let mut expected_action = ActionResult {
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto {
                identities: vec![
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        public_key:
                            "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        has_pwd: false,
                        path: "//westend".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                                .to_string(),
                        public_key:
                            "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                                .to_string(),
                        identicon: alice_sr_root().to_vec(),
                        has_pwd: false,
                        path: "".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                                .to_string(),
                        public_key:
                            "64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                                .to_string(),
                        identicon: alice_sr_kusama().to_vec(),
                        has_pwd: false,
                        path: "//kusama".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        public_key:
                            "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        has_pwd: false,
                        path: "//Alice/secret//secret".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        has_pwd: false,
                        path: "//Alice".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        public_key:
                            "e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
                        has_pwd: true,
                        path: "//secret//path".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        public_key:
                            "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "SignNetworkSpecs on NetworkDetails screen for ",
            "westend sr25519. Expected SignSufficientCrypto screen"
        )
    );

    let mut action = do_action(
        Action::GoForward,
        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
        "",
    )
    .unwrap()
    .unwrap();
    expected_action.modal_data = Some(ModalData::SufficientCryptoReady {
        f: MSufficientCryptoReady {
            author_info: Address {
                base58: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                    .to_string(),
                identicon: alice_sr_root().to_vec(),
                seed_name: "Alice".to_string(),
                path: String::new(),
                has_pwd: false,
                multiselect: None,
            },
            sufficient: vec![],
            content: MSCContent::AddSpecs {
                f: MSCNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                },
            },
        },
    });
    let sufficient = if let Some(ModalData::SufficientCryptoReady { ref mut f }) = action.modal_data
    {
        std::mem::take(&mut f.sufficient)
    } else {
        panic!(
            "Expected Some(ModalData::SufficientCrypto), got {:?}",
            action.modal_data
        );
    };
    let sufficient_hex = hex::encode(qr_payload(&sufficient));

    let mut new_log_with_modal = expected_action.clone();
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on SignSufficientCrypto screen with Alice ",
            "root key as an entry. Expected modal SufficientCryptoReady."
        )
    );

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
        std::env::set_current_dir("../files/completed").unwrap();
        std::fs::remove_file("add_specs_westend-sr25519").unwrap();
        std::env::set_current_dir("../../navigator").unwrap();
    }

    let action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        concat!(
            "GoBack on SignSufficientCrypto screen with SufficientCryptoReady modal. ",
            "Expected Settings screen."
        )
    );

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let alice_public_hex = "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a";
    if let ScreenData::Log { ref f } = action.screen_data {
        assert_eq!(
            f.log[0].events[0],
            Event::NetworkSpecsSigned {
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
                            sp_core::sr25519::Public::try_from(
                                hex::decode(alice_public_hex).unwrap().as_ref()
                            )
                            .unwrap()
                        )
                    }
                }
            }
        );
    } else {
        panic!("Expected ScreenData::Log, got {:?}", action.screen_data);
    }

    let mut expected_action = action;
    if let ScreenData::Log { ref mut f } = expected_action.screen_data {
        f.log.clear();
        f.log.push(History {
            order: 0,
            timestamp: String::new(),
            events: vec![Event::HistoryCleared],
        });
    } else {
        panic!(
            "Expected ScreenData::Log, got {:?}",
            expected_action.screen_data
        );
    }
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    let mut action = do_action(Action::ClearLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    assert_eq!(
        action, expected_action,
        concat!(
            "ClearLog on Log screen with LogRight modal. ",
            "Expected updated Log screen with no modals"
        )
    );

    do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    do_action(Action::ManageNetworks, "", "").unwrap().unwrap();
    do_action(
        Action::GoForward,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap();
    do_action(Action::ManageMetadata, "9150", "")
        .unwrap()
        .unwrap();
    do_action(Action::SignMetadata, "", "").unwrap().unwrap();
    let mut action = do_action(
        Action::GoForward,
        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
        "",
    )
    .unwrap()
    .unwrap();
    let expected_action = ActionResult {
        screen_label: "Sign SufficientCrypto".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::SignSufficientCrypto {
            f: MSignSufficientCrypto {
                identities: vec![
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        public_key:
                            "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34"
                                .to_string(),
                        identicon: alice_sr_westend().to_vec(),
                        has_pwd: false,
                        path: "//westend".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                                .to_string(),
                        public_key:
                            "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                                .to_string(),
                        identicon: alice_sr_root().to_vec(),
                        has_pwd: false,
                        path: "".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                                .to_string(),
                        public_key:
                            "64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05"
                                .to_string(),
                        identicon: alice_sr_kusama().to_vec(),
                        has_pwd: false,
                        path: "//kusama".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "018266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        public_key:
                            "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e"
                                .to_string(),
                        identicon: alice_sr_alice_secret_secret().to_vec(),
                        has_pwd: false,
                        path: "//Alice/secret//secret".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        public_key:
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                                .to_string(),
                        identicon: alice_sr_alice().to_vec(),
                        has_pwd: false,
                        path: "//Alice".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        public_key:
                            "e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c"
                                .to_string(),
                        identicon: alice_sr_secret_path_multipass().to_vec(),
                        has_pwd: true,
                        path: "//secret//path".to_string(),
                    },
                    MRawKey {
                        seed_name: "Alice".to_string(),
                        address_key:
                            "01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        public_key:
                            "f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730"
                                .to_string(),
                        identicon: alice_sr_polkadot().to_vec(),
                        has_pwd: false,
                        path: "//polkadot".to_string(),
                    },
                ],
            },
        },
        modal_data: Some(ModalData::SufficientCryptoReady {
            f: MSufficientCryptoReady {
                author_info: Address {
                    base58: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                        .to_string(),
                    identicon: alice_sr_root().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: String::new(),
                    has_pwd: false,
                    multiselect: None,
                },
                sufficient: vec![],
                content: MSCContent::LoadMetadata {
                    name: "westend".to_string(),
                    version: 9150,
                },
            },
        }),
        alert_data: None,
    };
    let sufficient = if let Some(ModalData::SufficientCryptoReady { ref mut f }) = action.modal_data
    {
        std::mem::take(&mut f.sufficient)
    } else {
        panic!(
            "Expected Some(ModalData::SufficientCrypto), got {:?}",
            action.modal_data
        );
    };
    let sufficient_hex = hex::encode(qr_payload(&sufficient));

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on SignSufficientCrypto screen with Alice ",
            "root key as an entry. Expected modal SufficientCryptoReady."
        )
    );

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
        std::env::set_current_dir("../files/completed").unwrap();
        std::fs::remove_file("load_metadata_westendV9150.txt").unwrap();
        std::env::set_current_dir("../../navigator").unwrap();
    }

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    if let ScreenData::Log { ref f } = action.screen_data {
        assert_eq!(
            f.log[0].events[0],
            Event::MetadataSigned {
                meta_values_export: MetaValuesExport {
                    name: "westend".to_string(),
                    version: 9150,
                    meta_hash: hex::decode(
                        "b5d422b92f0183c192cbae5e63811bffcabbef22b6f9e05a85ba7b738e91d44a"
                    )
                    .unwrap(),
                    signed_by: VerifierValue::Standard {
                        m: MultiSigner::Sr25519(
                            sp_core::sr25519::Public::try_from(
                                hex::decode(alice_public_hex).unwrap().as_ref()
                            )
                            .unwrap()
                        )
                    }
                }
            },
            "Expected the updated log to contain entry about generating sufficient crypto",
        );
    } else {
        panic!("Expected ScreenData::Log, got {:?}", action.screen_data);
    }
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();

    do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    do_action(Action::ManageNetworks, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::SignTypes, "", "").unwrap().unwrap();
    let mut action = do_action(
        Action::GoForward,
        "0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
        "",
    )
    .unwrap()
    .unwrap();
    let sufficient = if let Some(ModalData::SufficientCryptoReady { ref mut f }) = action.modal_data
    {
        std::mem::take(&mut f.sufficient)
    } else {
        panic!(
            "Expected Some(ModalData::SufficientCrypto), got {:?}",
            action.modal_data
        );
    };

    let sufficient_hex = hex::encode(qr_payload(&sufficient));

    new_log_with_modal.modal_data = Some(ModalData::SufficientCryptoReady {
        f: MSufficientCryptoReady {
            author_info: Address {
                base58: "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
                    .to_string(),
                identicon: alice_sr_root().to_vec(),
                seed_name: "Alice".to_string(),
                path: String::new(),
                has_pwd: false,
                multiselect: None,
            },
            sufficient: vec![],
            content: MSCContent::LoadTypes {
                types: "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"
                    .to_string(),
                pic: types_known().to_vec(),
            },
        },
    });
    assert_eq!(
        action, new_log_with_modal,
        concat!(
            "GoForward on SignSufficientCrypto screen with Alice root ",
            "key as an entry. Expected modal SufficientCryptoReady"
        )
    );

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
        std::env::set_current_dir("../files/completed").unwrap();
        std::fs::remove_file("load_types.txt").unwrap();
        std::env::set_current_dir("../../navigator").unwrap();
    }

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    if let ScreenData::Log { ref f } = action.screen_data {
        assert_eq!(
            f.log[0].events[0],
            Event::TypesSigned {
                types_export: TypesExport {
                    types_hash: hex::decode(
                        "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"
                    )
                    .unwrap(),
                    signed_by: VerifierValue::Standard {
                        m: MultiSigner::Sr25519(
                            sp_core::sr25519::Public::try_from(
                                hex::decode(alice_public_hex).unwrap().as_ref()
                            )
                            .unwrap()
                        )
                    }
                }
            },
            "Expected the updated log to contain entry about generating sufficient crypto.",
        );
    } else {
        panic!("Expected ScreenData::Log, got {:?}", action.screen_data);
    }

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();

    // let's scan something!!! oops wrong network version
    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let action = do_action(Action::TransactionFetched,"530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    error: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::ErrorCard {
                            f: concat!(
                                "Failed to decode extensions. ",
                                "Please try updating metadata for westend network. ",
                                "Parsing with westend9150 metadata: Network spec version ",
                                "decoded from extensions (9010) differs from the version ",
                                "in metadata (9150)."
                            )
                            .to_string(),
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::Read,
                author_info: None,
                network_info: None,
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen containing transaction. ",
            "Expected Transaction screen with no modals."
        )
    );

    let action = do_action(Action::GoForward, "", "").unwrap().unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen with transaction ",
            "that could be only read. Expected to stay ",
            "in same place, got."
        )
    );

    // let's scan something real!!!
    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let transaction_hex = "5301008266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235ea40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b800be23000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let action = do_action(Action::TransactionFetched, transaction_hex, "")
        .unwrap()
        .unwrap();
    let docs = "53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572".to_string();

    let block_hash = "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33".to_string();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    method: Some(vec![
                        TransactionCard {
                            index: 0,
                            indent: 0,
                            card: Card::PalletCard {
                                f: "Balances".to_string(),
                            },
                        },
                        TransactionCard {
                            index: 1,
                            indent: 1,
                            card: Card::CallCard {
                                f: MSCCall {
                                    method_name: "transfer_keep_alive".to_string(),
                                    docs: docs.clone(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 2,
                            indent: 2,
                            card: Card::FieldNameCard {
                                f: MSCFieldName {
                                    name: "dest".to_string(),
                                    docs_field_name: String::new(),
                                    path_type: "sp_runtime >> multiaddress >> MultiAddress"
                                        .to_string(),
                                    docs_type: String::new(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 3,
                            indent: 3,
                            card: Card::EnumVariantNameCard {
                                f: MSCEnumVariantName {
                                    name: "Id".to_string(),
                                    docs_enum_variant: String::new(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 4,
                            indent: 4,
                            card: Card::IdCard {
                                f: MSCId {
                                    base58: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
                                        .to_string(),
                                    identicon: bob().to_vec(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 5,
                            indent: 2,
                            card: Card::FieldNameCard {
                                f: MSCFieldName {
                                    name: "value".to_string(),
                                    docs_field_name: String::new(),
                                    path_type: String::new(),
                                    docs_type: String::new(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 6,
                            indent: 3,
                            card: Card::BalanceCard {
                                f: MSCCurrency {
                                    amount: "100.000000000".to_string(),
                                    units: "mWND".to_string(),
                                },
                            },
                        },
                    ]),
                    extensions: Some(vec![
                        TransactionCard {
                            index: 7,
                            indent: 0,
                            card: Card::EraMortalCard {
                                f: MSCEraMortal {
                                    era: "Mortal".to_string(),
                                    phase: "27".to_string(),
                                    period: "64".to_string(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 8,
                            indent: 0,
                            card: Card::NonceCard {
                                f: "46".to_string(),
                            },
                        },
                        TransactionCard {
                            index: 9,
                            indent: 0,
                            card: Card::TipCard {
                                f: MSCCurrency {
                                    amount: "0".to_string(),
                                    units: "pWND".to_string(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 10,
                            indent: 0,
                            card: Card::NameVersionCard {
                                f: MSCNameVersion {
                                    name: "westend".to_string(),
                                    version: "9150".to_string(),
                                },
                            },
                        },
                        TransactionCard {
                            index: 11,
                            indent: 0,
                            card: Card::TxSpecCard { f: "5".to_string() },
                        },
                        TransactionCard {
                            index: 12,
                            indent: 0,
                            card: Card::BlockHashCard { f: block_hash },
                        },
                    ]),
                    ..Default::default()
                },
                ttype: TransactionType::Sign,
                author_info: Some(Address {
                    base58: "5F1gaMEdLTzoYFV6hYqX9AnZYg4bknuYE5HcVXmnKi1eSCXK".to_string(),
                    identicon: alice_sr_alice_secret_secret().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//Alice/secret//secret".to_string(),
                    has_pwd: false,
                    multiselect: None,
                }),
                network_info: Some(MSCNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                }),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen containing transaction. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(
        Action::GoForward,
        "Alice sends some cash",
        ALICE_SEED_PHRASE,
    )
    .unwrap()
    .unwrap();
    let signature_hex = if let ScreenData::SignatureReady {
        f:
            MSignatureReady {
                author_info: _,
                content: _,
                network_info: _,
                signature,
                user_comment: _,
            },
    } = action.screen_data
    {
        String::from_utf8(qr_payload(&signature)).unwrap()
    } else {
        panic!(
            "Expected ScreenData::SigantureReady, got {:?}",
            action.screen_data
        );
    };

    assert!(
        signature_is_good(transaction_hex, &signature_hex),
        "Produced bad signature: \n{}",
        signature_hex
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let transaction = "a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b800be23000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33".to_string();

    let public = "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e";
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![Event::TransactionSigned {
                            sign_display: SignDisplay {
                                transaction: hex::decode(transaction).unwrap(),
                                network_name: "westend".to_string(),
                                signed_by: VerifierValue::Standard {
                                    m: MultiSigner::Sr25519(
                                        sp_core::sr25519::Public::try_from(
                                            hex::decode(public).unwrap().as_ref(),
                                        )
                                        .unwrap(),
                                    ),
                                },
                                user_comment: "Alice sends some cash".to_string(),
                            },
                        }],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "GoBack from Transaction with SignatureReady modal. Expected Log.",
    );

    let mut action = do_action(Action::ShowLogDetails, "1", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);

    //r#"{"screen":"LogDetails","screenLabel":"Event details","back":true,"footer":true,"footerButton":"Log","rightButton":"None","screenNameType":"h4","modal":"Empty","alert":"Empty","screenData":{"timestamp":"**","events":[{"event":"transaction_signed","payload":{"transaction":{"method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"27","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"46"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9150"}},{"index":11,"indent":0,"type":"tx_version","payload":"5"},{"index":12,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"}]},"network_name":"westend","signed_by":{"public_key":"8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e","identicon":"<alice_sr25519_//Alice/secret//secret>","encryption":"sr25519"},"user_comment":"Alice sends some cash"}}]},"modalData":{},"alertData":{}}"#;
    /* TODO: here is transaction decoded but returned in encoded version.
    assert_eq!(
        action, expected_action,
        concat!(
            "ShowLogDetails on Log screen with order 1. ",
            "Expected LogDetails screen with decoded transaction and no modals."
        )
    );
    */

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();

    // let's scan a text message
    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let message_hex = "5301033efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f343c42797465733e4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e3c2f42797465733ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let card_text = "4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e".to_string();
    let action = do_action(Action::TransactionFetched, message_hex, "")
        .unwrap()
        .unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: TransactionCardSet {
                    message: Some(vec![TransactionCard {
                        index: 0,
                        indent: 0,
                        card: Card::TextCard {
                            f: card_text.clone(),
                        },
                    }]),
                    ..Default::default()
                },
                ttype: TransactionType::Sign,
                author_info: Some(Address {
                    base58: "5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N".to_string(),
                    identicon: alice_sr_westend().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//westend".to_string(),
                    has_pwd: false,
                    multiselect: None,
                }),
                network_info: Some(MSCNetworkInfo {
                    network_title: "Westend".to_string(),
                    network_logo: "westend".to_string(),
                }),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen containing message transaction. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(Action::GoForward, "text test", ALICE_SEED_PHRASE)
        .unwrap()
        .unwrap();
    let signature_hex = if let ScreenData::SignatureReady {
        f:
            MSignatureReady {
                author_info: _,
                content: _,
                network_info: _,
                signature,
                user_comment: _,
            },
    } = action.screen_data
    {
        String::from_utf8(qr_payload(&signature)).unwrap()
    } else {
        panic!(
            "Expected ScreenData::SigantureReady, got {:?}",
            action.screen_data
        );
    };

    /*
    assert_eq!(
        action, expected_action,
        "GoForward on parsed transaction. Expected modal SignatureReady",
    );
    */

    assert!(
        signature_is_good(message_hex, &signature_hex),
        "Produced bad signature: \n{}",
        signature_hex
    );

    let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let signed_by = "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34";
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![Event::MessageSigned {
                            sign_message_display: SignMessageDisplay {
                                message: String::from_utf8(hex::decode(&card_text).unwrap())
                                    .unwrap(),
                                network_name: "westend".to_string(),
                                signed_by: VerifierValue::Standard {
                                    m: MultiSigner::Sr25519(
                                        sp_core::sr25519::Public::try_from(
                                            hex::decode(signed_by).unwrap().as_ref(),
                                        )
                                        .unwrap(),
                                    ),
                                },
                                user_comment: "text test".to_string(),
                            },
                        }],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "GoBack from Transaction with SignatureReady modal. Expected Log.",
    );

    let mut action = do_action(Action::ShowLogDetails, "1", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: "Event details".to_string(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::LogDetails {
            f: MLogDetails {
                timestamp: String::new(),
                events: vec![MEventMaybeDecoded {
                    event: Event::MessageSigned {
                        sign_message_display: SignMessageDisplay {
                            message: String::from_utf8(hex::decode(&card_text).unwrap()).unwrap(),
                            network_name: "westend".to_string(),
                            signed_by: VerifierValue::Standard {
                                m: MultiSigner::Sr25519(
                                    sp_core::sr25519::Public::try_from(
                                        hex::decode(signed_by).unwrap().as_ref(),
                                    )
                                    .unwrap(),
                                ),
                            },
                            user_comment: "text test".to_string(),
                        },
                    },
                    decoded: None,
                    signed_by: None,
                    verifier_details: None,
                }],
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "ShowLogDetails on Log screen with order 1. ",
            "Expected LogDetails screen with decoded message and no modals."
        )
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();

    do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::NewSeed, "", "").unwrap().unwrap();
    let mut action = do_action(Action::GoForward, "Pepper", "").unwrap().unwrap();
    let seed_phrase_pepper = cut_seed_remove_identicon(&mut action.modal_data);
    let expected_action = ActionResult {
        screen_label: "New Seed".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::NewSeed {
            f: MNewSeed { keyboard: false },
        },
        modal_data: Some(ModalData::NewSeedBackup {
            f: MNewSeedBackup {
                seed: "Pepper".to_string(),
                seed_phrase: String::new(),
                identicon: vec![],
            },
        }),
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on NewSeed screen with non-empty seed name. ",
            "Expected NewSeed screen with NewSeedBackup modal."
        )
    );

    let mut action = do_action(Action::GoForward, "false", &seed_phrase_pepper)
        .unwrap()
        .unwrap();
    erase_base58_address_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key: String::new(),
                    base58: String::new(),
                    identicon: vec![],
                    has_pwd: false,
                    path: "//polkadot".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Pepper".to_string(),
                    identicon: vec![],
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
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on NewSeed screen with NewSeedBackup modal active. ",
            "Expected Keys screen with no modals."
        )
    );

    update_seed_names(vec![String::from("Alice"), String::from("Pepper")]);

    do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    let mut action = do_action(
        Action::ChangeNetwork,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap()
    .unwrap();

    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key: String::new(),
                    base58: String::new(),
                    identicon: vec![],
                    has_pwd: false,
                    path: "//westend".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Pepper".to_string(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    let (pepper_westend_public, pepper_westend_base58, pepper_westend_identicon) =
        if let ScreenData::Keys { ref f } = action.screen_data {
            (
                f.set[0].address_key.strip_prefix("01").unwrap().to_string(),
                f.set[0].base58.clone(),
                f.set[0].identicon.clone(),
            )
        } else {
            panic!();
        };

    erase_base58_address_identicon(&mut action.screen_data);
    assert_eq!(
        action, expected_action,
        "Changed network to westend. Expected Keys screen with no modals",
    );
    do_action(Action::NavbarScan, "", "").unwrap().unwrap();

    let transaction_hex_pepper = transaction_hex.replace(
        "8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e",
        &pepper_westend_public,
    );
    let action = do_action(Action::TransactionFetched, &transaction_hex_pepper, "")
        .unwrap()
        .unwrap();

    let block_hash = "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33".to_string();
    let expected_content = TransactionCardSet {
        method: Some(vec![
            TransactionCard {
                index: 0,
                indent: 0,
                card: Card::PalletCard {
                    f: "Balances".to_string(),
                },
            },
            TransactionCard {
                index: 1,
                indent: 1,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "transfer_keep_alive".to_string(),
                        docs,
                    },
                },
            },
            TransactionCard {
                index: 2,
                indent: 2,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "dest".to_string(),
                        docs_field_name: String::new(),
                        path_type: "sp_runtime >> multiaddress >> MultiAddress".to_string(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 3,
                indent: 3,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 4,
                indent: 4,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                        identicon: bob().to_vec(),
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 2,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "value".to_string(),
                        docs_field_name: String::new(),
                        path_type: String::new(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 3,
                card: Card::BalanceCard {
                    f: MSCCurrency {
                        amount: "100.000000000".to_string(),
                        units: "mWND".to_string(),
                    },
                },
            },
        ]),
        extensions: Some(vec![
            TransactionCard {
                index: 7,
                indent: 0,
                card: Card::EraMortalCard {
                    f: MSCEraMortal {
                        era: "Mortal".to_string(),
                        phase: "27".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 0,
                card: Card::NonceCard {
                    f: "46".to_string(),
                },
            },
            TransactionCard {
                index: 9,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 10,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9150".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 11,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 12,
                indent: 0,
                card: Card::BlockHashCard { f: block_hash },
            },
        ]),
        ..Default::default()
    };
    let expected_author_info = Address {
        base58: pepper_westend_base58,
        identicon: pepper_westend_identicon,
        seed_name: "Pepper".to_string(),
        path: "//westend".to_string(),
        has_pwd: false,
        multiselect: None,
    };
    let expected_network_info = MSCNetworkInfo {
        network_title: "Westend".to_string(),
        network_logo: "westend".to_string(),
    };
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: expected_content.clone(),
                ttype: TransactionType::Sign,
                author_info: Some(expected_author_info.clone()),
                network_info: Some(expected_network_info.clone()),
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen containing transaction. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(
        Action::GoForward,
        "Pepper also sends some cash",
        &seed_phrase_pepper,
    )
    .unwrap()
    .unwrap();
    let signature_hex = if let ScreenData::SignatureReady {
        f:
            MSignatureReady {
                author_info: _,
                content: _,
                network_info: _,
                ref signature,
                user_comment: _,
            },
    } = action.screen_data
    {
        expected_action.screen_data = ScreenData::SignatureReady {
            f: MSignatureReady {
                author_info: expected_author_info,
                content: expected_content,
                network_info: expected_network_info.clone(),
                signature: signature.clone(),
                user_comment: String::from("Pepper also sends some cash"),
            },
        };
        String::from_utf8(qr_payload(signature)).unwrap()
    } else {
        panic!(
            "Expected ScreenData::SignatureReady, got {:?}",
            action.screen_data
        );
    };

    assert_eq!(
        action, expected_action,
        "GoForward on parsed transaction. Expected screen SignatureReady",
    );

    assert!(
        signature_is_good(&transaction_hex_pepper, &signature_hex),
        "Produced bad signature: \n{}",
        signature_hex
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();
    do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
    do_action(Action::SelectSeed, "Pepper", "")
        .unwrap()
        .unwrap();
    do_action(Action::NetworkSelector, "", "").unwrap().unwrap();
    do_action(
        Action::ChangeNetwork,
        "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        "",
    )
    .unwrap();
    do_action(Action::Swipe, &format!("01{}", pepper_westend_public), "")
        .unwrap()
        .unwrap();
    do_action(Action::RemoveKey, "", "").unwrap().unwrap();

    let action = do_action(Action::NewKey, "", "").unwrap().unwrap();
    let mut expected_action = ActionResult {
        screen_label: "Derive Key".to_string(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Keys),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::DeriveKey {
            f: MDeriveKey {
                seed_name: "Pepper".to_string(),
                network_title: "Westend".to_string(),
                network_logo: "westend".to_string(),
                network_specs_key:
                    "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e".to_string(),
                suggested_derivation: String::new(),
                keyboard: true,
                derivation_check: DerivationCheck {
                    button_good: true,
                    where_to: Some(DerivationDestination::Pin),
                    ..Default::default()
                },
            },
        },
        modal_data: None,
        alert_data: None,
    };

    assert_eq!(
        action, expected_action,
        "NewKey on Keys screen. Expected DeriveKey scree.",
    );
    expected_action.modal_data = Some(ModalData::PasswordConfirm {
        f: MPasswordConfirm {
            pwd: "secret".to_string(),
            seed_name: "Pepper".to_string(),
            cropped_path: "//0".to_string(),
        },
    });
    if let ScreenData::DeriveKey { ref mut f } = expected_action.screen_data {
        f.suggested_derivation = "//0///secret".to_string();
        f.derivation_check.where_to = Some(DerivationDestination::Pwd);
        f.keyboard = false;
    } else {
        panic!("");
    }

    let action = do_action(Action::CheckPassword, "//0///secret", "")
        .unwrap()
        .unwrap();
    assert_eq!(
        action, expected_action,
        concat!(
            "CheckPassword on DeriveKey screen with password ",
            "(path validity and password existence is checked elsewhere). ",
            "Expected updated DeriveKey screen with PasswordConfirm modal"
        )
    );

    let mut action = do_action(Action::GoForward, "//0///secret", &seed_phrase_pepper)
        .unwrap()
        .unwrap();
    let (pepper_key0_public, pepper_key0_base58, pepper_key0_identicon) =
        if let ScreenData::Keys { ref f } = action.screen_data {
            (
                f.set[0].address_key.strip_prefix("01").unwrap().to_string(),
                f.set[0].base58.clone(),
                f.set[0].identicon.clone(),
            )
        } else {
            panic!();
        };

    erase_base58_address_identicon(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: true,
        footer_button: Some(FooterButton::Keys),
        right_button: Some(RightButton::Backup),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Keys {
            f: MKeys {
                set: vec![MKeysCard {
                    address_key: String::new(),
                    base58: String::new(),
                    identicon: vec![],
                    has_pwd: true,
                    path: "//0".to_string(),
                    swiped: false,
                    multiselect: false,
                }],
                root: MSeedKeyCard {
                    seed_name: "Pepper".to_string(),
                    ..Default::default()
                },
                network: MNetworkCard {
                    title: "Westend".to_string(),
                    logo: "westend".to_string(),
                },
                multiselect_mode: false,
                multiselect_count: String::new(),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on DeriveKey screen with PasswordConfirm modal. ",
            "Expected updated Keys screen."
        )
    );

    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    let message_hex = message_hex.replace(
        "3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34",
        &pepper_key0_public,
    );
    let action = do_action(Action::TransactionFetched, &message_hex, "")
        .unwrap()
        .unwrap();
    let expected_text_content = TransactionCardSet {
        message: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::TextCard {
                f: card_text.clone(),
            },
        }]),
        ..Default::default()
    };
    let expected_author_info = Address {
        base58: pepper_key0_base58.clone(),
        identicon: pepper_key0_identicon.clone(),
        seed_name: "Pepper".to_string(),
        path: "//0".to_string(),
        has_pwd: true,
        multiselect: None,
    };
    let mut expected_action = ActionResult {
        screen_label: String::new(),
        back: true,
        footer: false,
        footer_button: Some(FooterButton::Scan),
        right_button: None,
        screen_name_type: ScreenNameType::H1,
        screen_data: ScreenData::Transaction {
            f: MTransaction {
                content: expected_text_content.clone(),
                ttype: TransactionType::Sign,
                author_info: Some(expected_author_info.clone()),
                network_info: Some(expected_network_info.clone()),
            },
        },
        modal_data: None,
        alert_data: None,
    };
    let mut text_sign_action = expected_action.clone();
    assert_eq!(
        action, expected_action,
        concat!(
            "TransactionFetched on Scan screen containing message transaction. ",
            "Expected Transaction screen with no modals"
        )
    );

    let action = do_action(
        Action::GoForward,
        "Pepper tries sending text from passworded account",
        &seed_phrase_pepper,
    )
    .unwrap()
    .unwrap();
    expected_action.modal_data = Some(ModalData::EnterPassword {
        f: MEnterPassword {
            author_info: Address {
                base58: pepper_key0_base58.clone(),
                identicon: pepper_key0_identicon.clone(),
                seed_name: "Pepper".to_string(),
                path: "//0".to_string(),
                has_pwd: true,
                multiselect: None,
            },
            counter: 1,
        },
    });
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen for passworded address. ",
            "Expected Transaction screen with EnterPassword modal"
        )
    );

    let action = do_action(Action::GoForward, "wrong_one", "")
        .unwrap()
        .unwrap();
    expected_action.modal_data = Some(ModalData::EnterPassword {
        f: MEnterPassword {
            author_info: Address {
                base58: pepper_key0_base58.clone(),
                identicon: pepper_key0_identicon.clone(),
                seed_name: "Pepper".to_string(),
                path: "//0".to_string(),
                has_pwd: true,
                multiselect: None,
            },
            counter: 2,
        },
    });
    expected_action.alert_data = Some(AlertData::ErrorData {
        f: "Wrong password.".to_string(),
    });

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen for passworded address with wrong password. ",
            "Expected Transaction screen with EnterPassword modal with counter at 2."
        )
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let action = do_action(Action::GoForward, "wrong_two", "")
        .unwrap()
        .unwrap();
    expected_action.modal_data = Some(ModalData::EnterPassword {
        f: MEnterPassword {
            author_info: Address {
                base58: pepper_key0_base58,
                identicon: pepper_key0_identicon,
                seed_name: "Pepper".to_string(),
                path: "//0".to_string(),
                has_pwd: true,
                multiselect: None,
            },
            counter: 3,
        },
    });
    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen for passworded address with second wrong password. ",
            "Expected Transaction screen with EnterPassword modal with counter at 3"
        )
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();
    let mut action = do_action(Action::GoForward, "wrong_three", "")
        .unwrap()
        .unwrap();
    erase_log_timestamps(&mut action.screen_data);

    let verifier_value = VerifierValue::Standard {
        m: MultiSigner::Sr25519(
            sp_core::sr25519::Public::try_from(hex::decode(&pepper_key0_public).unwrap().as_ref())
                .unwrap(),
        ),
    };
    let network_genesis_hash =
        hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let message = String::from_utf8(hex::decode(&card_text).unwrap()).unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 5,
                        timestamp: String::new(),
                        events: vec![Event::MessageSignError {
                            sign_message_display: SignMessageDisplay {
                                message: message.clone(),
                                network_name: "westend".to_string(),
                                signed_by: verifier_value.clone(),
                                user_comment: "Pepper tries sending text from passworded account"
                                    .to_string(),
                            },
                        }],
                    },
                    History {
                        order: 4,
                        timestamp: String::new(),
                        events: vec![Event::MessageSignError {
                            sign_message_display: SignMessageDisplay {
                                message: message.clone(),
                                network_name: "westend".to_string(),
                                signed_by: verifier_value.clone(),
                                user_comment: "Pepper tries sending text from passworded account"
                                    .to_string(),
                            },
                        }],
                    },
                    History {
                        order: 3,
                        timestamp: String::new(),
                        events: vec![Event::MessageSignError {
                            sign_message_display: SignMessageDisplay {
                                message,
                                network_name: "westend".to_string(),
                                signed_by: verifier_value.clone(),
                                user_comment: "Pepper tries sending text from passworded account"
                                    .to_string(),
                            },
                        }],
                    },
                    History {
                        order: 2,
                        timestamp: String::new(),
                        events: vec![Event::IdentityAdded {
                            identity_history: IdentityHistory {
                                seed_name: "Pepper".to_string(),
                                encryption: Encryption::Sr25519,
                                public_key: hex::decode(&pepper_key0_public).unwrap(),
                                path: "//0".to_string(),
                                network_genesis_hash: network_genesis_hash.clone(),
                            },
                        }],
                    },
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![Event::IdentityRemoved {
                            identity_history: IdentityHistory {
                                seed_name: "Pepper".to_string(),
                                encryption: Encryption::Sr25519,
                                public_key: hex::decode(pepper_westend_public).unwrap(),
                                path: "//westend".to_string(),
                                network_genesis_hash,
                            },
                        }],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: Some(AlertData::ErrorData {
            f: "Wrong password.".to_string(),
        }),
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "GoForward on Transaction screen for passworded ",
            "address with third wrong password. Expected Log screen"
        )
    );

    do_action(Action::RightButtonAction, "", "")
        .unwrap()
        .unwrap();
    do_action(Action::ClearLog, "", "").unwrap().unwrap();

    do_action(Action::NavbarScan, "", "").unwrap().unwrap();
    do_action(Action::TransactionFetched, &message_hex, "")
        .unwrap()
        .unwrap();
    do_action(
        Action::GoForward,
        "Pepper tries better",
        &seed_phrase_pepper,
    )
    .unwrap();
    let action = do_action(Action::GoForward, "secret", "").unwrap().unwrap();
    let signature_hex = if let ScreenData::SignatureReady {
        f:
            MSignatureReady {
                author_info: _,
                content: _,
                network_info: _,
                ref signature,
                user_comment: _,
            },
    } = action.screen_data
    {
        text_sign_action.screen_data = ScreenData::SignatureReady {
            f: MSignatureReady {
                author_info: expected_author_info,
                content: expected_text_content,
                network_info: expected_network_info,
                signature: signature.clone(),
                user_comment: String::from("Pepper tries better"),
            },
        };
        String::from_utf8(qr_payload(signature)).unwrap()
    } else {
        panic!(
            "Expected ScreenData::SignatureReady, got {:?}",
            action.screen_data
        );
    };

    assert_eq!(
        action, text_sign_action,
        concat!(
            "GoForward on Transaction screen for passworded address with correct password. ",
            "Expected Transaction screen with SignatureReady modal."
        )
    );

    assert!(
        signature_is_good(&message_hex, &signature_hex),
        "Produced bad signature: \n{}",
        signature_hex
    );

    do_action(Action::GoBack, "", "").unwrap().unwrap();

    {
        // database got unavailable for some reason
        let _database = db_handling::helpers::open_db::<Signer>(dbname).unwrap();

        let mut action = do_action(Action::NavbarKeys, "", "").unwrap().unwrap();
        let expected_alert = "Database error. Internal error. IO error: could not acquire lock on \"for_tests/flow_test_1/db\": Os {**}".to_string();
        let expected_action = ActionResult {
            screen_label: "Select seed".to_string(),
            back: false,
            footer: true,
            footer_button: Some(FooterButton::Keys),
            right_button: Some(RightButton::NewSeed),
            screen_name_type: ScreenNameType::H1,
            modal_data: None,
            alert_data: Some(AlertData::ErrorData { f: expected_alert }),
            screen_data: ScreenData::Settings {
                f: MSettings {
                    ..Default::default()
                },
            },
        };
        if let Some(AlertData::ErrorData { ref mut f }) = action.alert_data {
            *f = cut_os_msg(f);
        } else {
            panic!("Expected AlertData::ErrorData");
        }

        assert_eq!(
            action, expected_action,
            "Tried to switch from Log to Keys with unavailable database."
        );

        let mut action = do_action(Action::GoBack, "", "").unwrap().unwrap();

        let expected_alert = "Database error. Internal error. IO error: could not acquire lock on \"for_tests/flow_test_1/db\": Os {**}".to_string();
        let expected_action = ActionResult {
            screen_label: "".to_string(),
            back: false,
            footer: true,
            footer_button: Some(FooterButton::Settings),
            right_button: None,
            screen_name_type: ScreenNameType::H4,
            screen_data: ScreenData::Settings {
                f: MSettings {
                    public_key: None,
                    identicon: None,
                    encryption: None,
                    error: Some(expected_alert),
                },
            },
            modal_data: None,
            alert_data: None,
        };

        if let ScreenData::Settings { ref mut f } = action.screen_data {
            if let Some(ref mut e) = f.error {
                *e = cut_os_msg(e);
            } else {
                panic!("Expected some error on Settings screen");
            }
        } else {
            panic!("Expected Screen::Settings");
        };

        //Important test! Please check that alert is `None` after going back or we app get stuck
        assert_eq!(
            action, expected_action,
            "GoBack on SeedSelector with ErrorDisplay alert."
        );
    }

    // Aaand, we are back
    let action = do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    assert_eq!(
        action, current_settings_action,
        "Reload Settings. Expected known Settings screen with no errors.",
    );

    let mut action = do_action(Action::NavbarLog, "", "").unwrap().unwrap();
    erase_log_timestamps(&mut action.screen_data);
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Log),
        right_button: Some(RightButton::LogRight),
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Log {
            f: MLog {
                log: vec![
                    History {
                        order: 1,
                        timestamp: String::new(),
                        events: vec![Event::MessageSigned {
                            sign_message_display: SignMessageDisplay {
                                message: String::from_utf8_lossy(&hex::decode(&card_text).unwrap())
                                    .to_string(),
                                network_name: "westend".to_string(),
                                signed_by: verifier_value,
                                user_comment: "Pepper tries better".to_string(),
                            },
                        }],
                    },
                    History {
                        order: 0,
                        timestamp: String::new(),
                        events: vec![Event::HistoryCleared],
                    },
                ],
            },
        },
        modal_data: None,
        alert_data: None,
    };
    assert_eq!(
        action, expected_action,
        "Switched to Log from Settings. Expected Log screen.",
    );

    // no init after population
    populate_cold_nav_test(dbname).unwrap();
    let action = do_action(Action::NavbarSettings, "", "").unwrap().unwrap();
    let expected_action = ActionResult {
        screen_label: String::new(),
        back: false,
        footer: true,
        footer_button: Some(FooterButton::Settings),
        right_button: None,
        screen_name_type: ScreenNameType::H4,
        screen_data: ScreenData::Settings {
            f: MSettings {
                error: Some(String::from("Could not find general verifier.")),
                ..Default::default()
            },
        },
        modal_data: None,
        alert_data: None, // None, DO NOT MODIFY THIS
    };

    assert_eq!(
        action, expected_action,
        concat!(
            "Switched to Settings from Log with non-initiated database. ",
            "Expected Settings screen with error on screen, and no alerts ",
            "(we should still allow to reset Signer)"
        )
    );

    std::fs::remove_dir_all(dbname).unwrap();
}
