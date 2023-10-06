use parity_scale_codec::Decode;
use pretty_assertions::assert_eq;
use sled::Tree;
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::{fmt::Write as _, fs, io::Write, str::FromStr};
use tempfile::tempdir;

use constants::{
    test_values::{
        alice_sr_alice, alice_sr_root, bob, dock_31, ed, empty_png, id_01, id_02, id_04, id_05,
        shell_200, types_known, types_unknown, westend_9070, westend_9111, westend_9122,
    },
    ADDRTREE, ALICE_SEED_PHRASE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, VERIFIERS,
};
use db_handling::{
    cold_default::{populate_cold, populate_cold_no_networks},
    helpers::remove_network,
    identities::{remove_seed, try_create_address, try_create_seed},
    manage_history::{get_history, get_history_entry_by_order},
};
use definitions::navigation::{Identicon, MAddressCard, TransactionSignAction};
use definitions::{
    crypto::Encryption,
    history::{Entry, Event, SignDisplay, SignMessageDisplay},
    keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey},
    navigation::{
        Address, Card, MMetadataRecord, MSCCall, MSCCurrency, MSCEnumVariantName, MSCEraMortal,
        MSCFieldName, MSCId, MSCNameVersion, MTypesInfo, MVerifierDetails, NetworkSpecs,
        TransactionCard, TransactionCardSet,
    },
    network_specs::{
        CurrentVerifier, OrderedNetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue,
    },
    users::AddressDetails,
};
use transaction_parsing::{
    entry_to_transactions_with_decoding, produce_output, StubNav, TransactionAction,
};

use crate::{handle_stub, sign_transaction::create_signature, Error, Result};

const PWD: &str = "";
const USER_COMMENT: &str = "";
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

fn sign_action_test(
    database: &sled::Db,
    checksum: u32,
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    encryption: Encryption,
) -> Result<String> {
    create_signature(
        database,
        seed_phrase,
        pwd_entry,
        user_comment,
        checksum,
        0,
        encryption,
    )
    .map(|r| r.to_string())
}

fn identicon_to_str(identicon: &Identicon) -> &str {
    if let Identicon::Dots {
        identity: identicon,
    } = identicon
    {
        if identicon == &ed() {
            "<ed>"
        } else if identicon == &alice_sr_alice() {
            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        } else if identicon == &empty_png() {
            "<empty>"
        } else {
            "<unknown>"
        }
    } else {
        "<unknown>"
    }
}

fn print_db_content(database: &sled::Db) -> String {
    let mut metadata_set: Vec<String> = Vec::new();
    let metadata: Tree = database.open_tree(METATREE).unwrap();
    for (meta_key_vec, _) in metadata.iter().flatten() {
        let meta_key = MetaKey::from_ivec(&meta_key_vec);
        let (name, version) = meta_key.name_version().unwrap();
        metadata_set.push(format!("{name}{version}"));
    }
    metadata_set.sort();
    let mut metadata_str = String::new();
    for x in metadata_set.iter() {
        let _ = write!(&mut metadata_str, "\n    {x}");
    }

    let mut network_specs_set: Vec<(NetworkSpecsKey, OrderedNetworkSpecs)> = Vec::new();
    let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = OrderedNetworkSpecs::from_entry_with_key_checked(
            &network_specs_key,
            network_specs_encoded,
        )
        .unwrap();
        network_specs_set.push((network_specs_key, network_specs));
    }
    network_specs_set.sort_by(|(_, a), (_, b)| a.specs.title.cmp(&b.specs.title));
    let mut network_specs_str = String::new();
    for (network_specs_key, network_specs) in network_specs_set.iter() {
        let _ = write!(
            &mut network_specs_str,
            "\n    {}: {} ({} with {})",
            hex::encode(network_specs_key.key()),
            network_specs.specs.title,
            network_specs.specs.name,
            network_specs.specs.encryption.show()
        );
    }

    let settings: Tree = database.open_tree(SETTREE).unwrap();
    let general_verifier_encoded = settings.get(GENERALVERIFIER).unwrap().unwrap();
    let general_verifier = Verifier::decode(&mut &general_verifier_encoded[..]).unwrap();

    let mut verifiers_set: Vec<String> = Vec::new();
    let verifiers: Tree = database.open_tree(VERIFIERS).unwrap();
    for (verifier_key_vec, current_verifier_encoded) in verifiers.iter().flatten() {
        let verifier_key = VerifierKey::from_ivec(&verifier_key_vec).unwrap();
        let current_verifier = CurrentVerifier::decode(&mut &current_verifier_encoded[..]).unwrap();
        match current_verifier {
            CurrentVerifier::Valid(a) => {
                let verifier = match a {
                    ValidCurrentVerifier::General => {
                        let card = general_verifier.show_card();
                        let encryption = if card.encryption.is_empty() {
                            "none".to_string()
                        } else {
                            card.encryption
                        };

                        format!(
                            "{}: \"type\":\"general\",\"details\":{{\"public_key\":\"{}\",\"identicon\":\"{}\",\"encryption\":\"{}\"}}",
                            hex::encode(verifier_key.key()),
                            card.public_key,
                            identicon_to_str(&card.identicon),
                            encryption,
                        )
                    }
                    ValidCurrentVerifier::Custom { v } => {
                        let card = v.show_card();
                        let encryption = if card.encryption.is_empty() {
                            "none".to_string()
                        } else {
                            card.encryption
                        };

                        format!(
                            "{}: \"type\":\"custom\",\"details\":{{\"public_key\":\"{}\",\"identicon\":\"{}\",\"encryption\":\"{}\"}}",
                            hex::encode(verifier_key.key()),
                            card.public_key,
                            identicon_to_str(&card.identicon),
                            encryption,
                        )
                    }
                };
                verifiers_set.push(verifier)
            }
        }
    }
    verifiers_set.sort();
    let mut verifiers_str = String::new();
    for x in verifiers_set.iter() {
        let _ = write!(&mut verifiers_str, "\n    {x}");
    }

    let mut identities_set: Vec<String> = Vec::new();
    let identities: Tree = database.open_tree(ADDRTREE).unwrap();
    for (address_key_vec, address_details_encoded) in identities.iter().flatten() {
        let address_key = AddressKey::from_ivec(&address_key_vec).unwrap();
        let address_details = AddressDetails::decode(&mut &address_details_encoded[..]).unwrap();
        let (public_key, encryption) = address_key.public_key_encryption().unwrap();

        let mut networks_set: Vec<String> = Vec::new();
        if let Some(id) = address_details.network_id {
            networks_set.push(hex::encode(id.key()));
        }
        networks_set.sort();
        let mut networks_str = String::new();
        for y in networks_set.iter() {
            let _ = write!(&mut networks_str, "\n        {y}");
        }

        identities_set.push(format!(
            "public_key: {}, encryption: {}, path: {}, available_networks:{}",
            hex::encode(public_key),
            encryption.show(),
            address_details.path,
            networks_str
        ));
    }
    identities_set.sort();
    let mut identities_str = String::new();
    for x in identities_set.iter() {
        let _ = write!(&mut identities_str, "\n    {x}");
    }

    format!("Database contents:\nMetadata:{}\nNetwork Specs:{}\nVerifiers:{}\nGeneral Verifier: {}\nIdentities:{}", metadata_str, network_specs_str, verifiers_str, general_verifier.show_error(), identities_str)
}

fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().any(|e| e.events.contains(event))
}

// can sign a parsed transaction
#[test]
fn can_sign_transaction_1() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs = " Same as the [`transfer`] call, but with a check that the transfer will not kill the\n origin account.\n\n 99% of the time you want [`transfer`] instead.\n\n [`transfer`]: struct.Pallet.html#method.transfer\n # <weight>\n - Cheaper than transfer because account cannot be killed.\n - Base Weight: 51.4 µs\n - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n #</weight>".to_string();

    let set_expected = TransactionCardSet {
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
                card: Card::VarNameCard {
                    f: "dest".to_string(),
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
                        identicon: Identicon::Dots { identity: bob() },
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 2,
                card: Card::VarNameCard {
                    f: "value".to_string(),
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
                        version: "9010".to_string(),
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
                card: Card::BlockHashCard {
                    f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };
    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: concat!(
            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
        .to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };

    let network_info_known = OrderedNetworkSpecs {
        specs: NetworkSpecs {
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
            path_id: "//westend".to_string(),
            secondary_color: "#262626".to_string(),
            title: "Westend".to_string(),
            unit: "WND".to_string(),
        },
        order: 2,
    };

    let output = produce_output(&db, line).unwrap();
    if let TransactionAction::Sign { actions, checksum } = output {
        let TransactionSignAction {
            content: set,
            has_pwd,
            author_info,
            network_info,
        } = &actions[0];

        assert_eq!(actions.len(), 1);
        assert_eq!(set, &set_expected);
        assert_eq!(author_info, &author_info_known);
        assert_eq!(network_info, &network_info_known);
        assert!(!has_pwd, "Expected no password");

        match sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        ) {
            Ok(signature) => assert!(
                (signature.len() == 130) && (signature.starts_with("01")),
                "Wrong signature format,\nReceived: \n{signature}"
            ),
            Err(e) => panic!("Was unable to sign. {e:?}"),
        }

        let history_recorded: Vec<_> = get_history(&db).unwrap().into_iter().map(|e| e.1).collect();
        let transaction = "a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33".to_string();

        let my_event = Event::TransactionSigned {
            sign_display: SignDisplay {
                transaction: hex::decode(transaction).unwrap(),
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        sp_core::sr25519::Public::try_from(
                            hex::decode(
                                "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
                            )
                            .unwrap()
                            .as_ref(),
                        )
                        .unwrap(),
                    ),
                },
                user_comment: String::new(),
            },
        };

        assert!(entries_contain_event(&history_recorded, &my_event));

        let result = sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        );
        if let Err(e) = result {
            if let Error::DbHandling(db_handling::Error::ChecksumMismatch) = e {
            } else {
                panic!("Expected wrong checksum. Got error: {e:?}.")
            }
        } else {
            panic!("Checksum should have changed.")
        }

        let entry = get_history_entry_by_order(&db, 2).unwrap();
        let historic_reply = entry_to_transactions_with_decoding(&db, entry).unwrap();
        let docs = " Same as the [`transfer`] call, but with a check that the transfer will not kill the\n origin account.\n\n 99% of the time you want [`transfer`] instead.\n\n [`transfer`]: struct.Pallet.html#method.transfer\n # <weight>\n - Cheaper than transfer because account cannot be killed.\n - Base Weight: 51.4 µs\n - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n #</weight>".to_string();

        let historic_reply_known = TransactionCardSet {
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
                    card: Card::VarNameCard {
                        f: "dest".to_string(),
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
                            identicon: Identicon::Dots { identity: bob() },
                        },
                    },
                },
                TransactionCard {
                    index: 5,
                    indent: 2,
                    card: Card::VarNameCard {
                        f: "value".to_string(),
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
                            version: "9010".to_string(),
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
                    card: Card::BlockHashCard {
                        f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                            .to_string(),
                    },
                },
            ]),
            ..Default::default()
        };

        assert!(historic_reply
            .iter()
            .any(|m| m.decoded.as_ref() == Some(&historic_reply_known)));
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

// can sign a message
#[test]
fn can_sign_message_1() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();
    populate_cold(&db, Verifier { v: None }).unwrap();

    let card_text = String::from("uuid-abcd");
    let message = hex::encode(b"<Bytes>uuid-abcd</Bytes>");
    let line = format!("530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d{message}e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e");
    let output = produce_output(&db, &line).unwrap();

    let content_known = TransactionCardSet {
        message: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::TextCard { f: card_text },
        }]),
        ..Default::default()
    };

    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: concat!(
            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
        .to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };

    let network_info_known = OrderedNetworkSpecs {
        specs: NetworkSpecs {
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
            path_id: "//westend".to_string(),
            secondary_color: "#262626".to_string(),
            title: "Westend".to_string(),
            unit: "WND".to_string(),
        },
        order: 2,
    };

    if let TransactionAction::Sign { actions, checksum } = output {
        let TransactionSignAction {
            content,
            has_pwd,
            author_info,
            network_info,
        } = &actions[0];
        assert_eq!(actions.len(), 1);
        assert_eq!(content, &content_known);
        assert_eq!(author_info, &author_info_known);
        assert_eq!(network_info, &network_info_known);
        assert!(!has_pwd, "Expected no password");

        match sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        ) {
            Ok(signature) => assert_eq!(
                signature.len(),
                128,
                "Wrong signature format,\nReceived: \n{}",
                signature
            ),
            Err(e) => panic!("Was unable to sign. {e:?}"),
        }

        let history_recorded: Vec<_> = get_history(&db)
            .unwrap()
            .into_iter()
            .flat_map(|e| e.1.events)
            .collect();

        let message = String::from_utf8(hex::decode(message).unwrap()).unwrap();
        let my_event = Event::MessageSigned {
            sign_message_display: SignMessageDisplay {
                message,
                network_name: "westend".to_string(),
                signed_by: VerifierValue::Standard {
                    m: MultiSigner::Sr25519(
                        sp_core::sr25519::Public::try_from(
                            hex::decode(
                                "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
                            )
                            .unwrap()
                            .as_ref(),
                        )
                        .unwrap(),
                    ),
                },
                user_comment: String::new(),
            },
        };

        // TODO: fails since .message has to be encoded (or decoded) everywhere.
        assert!(
            history_recorded.contains(&my_event),
            "Recorded {history_recorded:?}"
        );

        let result = sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        );
        if let Err(e) = result {
            if let Error::DbHandling(db_handling::Error::ChecksumMismatch) = e {
            } else {
                panic!("Expected wrong checksum. Got error: {e:?}.")
            }
        } else {
            panic!("Checksum should have changed.")
        }
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_no_network_info_not_signed() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
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
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "Westend".to_string(),
                    unit: "WND".to_string(),
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = "Database contents:\nMetadata:\nNetwork Specs:\nVerifiers:\nGeneral Verifier: none\nIdentities:";
        assert!(
            print_before == expected_print_before,
            "Received: \n{print_before}"
        );

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_signed() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 42,
                    color: "#660D35".to_string(),
                    decimals: 12,
                    encryption: Encryption::Ed25519,
                    genesis_hash: H256::from_str(
                        "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
                    )
                    .unwrap(),
                    logo: "westend".to_string(),
                    name: "westend".to_string(),
                    path_id: "//westend".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "westend-ed25519".to_string(),
                    unit: "WND".to_string(),
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Ed25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        let mut file1 = std::fs::File::create("/tmp/a").unwrap();
        let mut file2 = std::fs::File::create("/tmp/b").unwrap();
        file1.write_all(print_before.as_bytes()).unwrap();
        file2.write_all(expected_print_before.as_bytes()).unwrap();
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();
        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);

        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "",
            &NetworkSpecsKey::from_parts(
                &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                    .unwrap(),
                &Encryption::Ed25519,
            ),
        )
        .unwrap();
        try_create_address(
            &db,
            "Alice",
            ALICE_SEED_PHRASE,
            "//westend",
            &NetworkSpecsKey::from_parts(
                &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                    .unwrap(),
                &Encryption::Ed25519,
            ),
        )
        .unwrap();
        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef, encryption: ed25519, path: , available_networks:
        00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks:
        00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);

        remove_seed(&db, "Alice").unwrap();
        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:"#;
        assert_eq!(print_after, expected_print_after);

        try_create_seed(&db, "Alice", ALICE_SEED_PHRASE, true).unwrap();
        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: westend-ed25519 (westend with ed25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        meta: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "westend".to_string(),
                    specs_version: "9070".to_string(),
                    meta_hash: "e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: westend_9070(),
                    },
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
    westend9070
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_known_types_upd_general_verifier() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let warning = "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string();

    let warning2 =
        "Received types information is identical to the one that was in the database.".to_string();

    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: Identicon::Dots {
                        identity: alice_sr_alice(),
                    },
                    encryption: "sr25519".to_string(),
                },
            },
        }]),
        warning: Some(vec![
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning },
            },
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::WarningCard { f: warning2 },
            },
        ]),
        types_info: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::TypesInfoCard {
                f: MTypesInfo {
                    types_on_file: false,
                    types_hash: Some(
                        "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"
                            .to_string(),
                    ),
                    types_id_pic: Some(Identicon::Dots {
                        identity: types_known(),
                    }),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::LoadTypes;

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_new_types_verified() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: Identicon::Dots {
                        identity: alice_sr_alice(),
                    },
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
                    types_hash: Some(
                        "d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"
                            .to_string(),
                    ),
                    types_id_pic: Some(Identicon::Dots {
                        identity: types_unknown(),
                    }),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::LoadTypes;

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn dock_adventures_1() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae",
                    )
                    .unwrap(),
                    logo: "dock-pos-main-runtime".to_string(),
                    name: "dock-pos-main-runtime".to_string(),
                    path_id: "//dock-pos-main-runtime".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "dock-pos-main-runtime-sr25519".to_string(),
                    unit: "DOCK".to_string(),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        meta: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "dock-pos-main-runtime".to_string(),
                    specs_version: "31".to_string(),
                    meta_hash: "28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: dock_31(),
                    },
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    dock-pos-main-runtime31
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let warning_1 = "Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31.".to_string();
    let warning_2 = "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string();
    let warning_3 = "Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database.".to_string();

    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: Identicon::Dots {
                        identity: alice_sr_alice(),
                    },
                    encryption: "sr25519".to_string(),
                },
            },
        }]),
        warning: Some(vec![
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning_1 },
            },
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::WarningCard { f: warning_2 },
            },
            TransactionCard {
                index: 3,
                indent: 0,
                card: Card::WarningCard { f: warning_3 },
            },
        ]),
        new_specs: Some(vec![TransactionCard {
            index: 4,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae",
                    )
                    .unwrap(),
                    logo: "dock-pos-main-runtime".to_string(),
                    name: "dock-pos-main-runtime".to_string(),
                    path_id: "//dock-pos-main-runtime".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "dock-pos-main-runtime-sr25519".to_string(),
                    unit: "DOCK".to_string(),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn dock_adventures_2() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, verifier_alice_sr25519()).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae",
                    )
                    .unwrap(),
                    logo: "dock-pos-main-runtime".to_string(),
                    name: "dock-pos-main-runtime".to_string(),
                    path_id: "//dock-pos-main-runtime".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "dock-pos-main-runtime-sr25519".to_string(),
                    unit: "DOCK".to_string(),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before,);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        meta: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "dock-pos-main-runtime".to_string(),
                    specs_version: "31".to_string(),
                    meta_hash: "28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: dock_31(),
                    },
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    dock-pos-main-runtime31
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let warning_1 = "Received message is verified. Currently no verifier is set for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: dock-pos-main-runtime31.".to_string();

    let warning_2 = "Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database.".to_string();

    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee"
                        .to_string(),
                    identicon: Identicon::Dots { identity: ed() },
                    encryption: "ed25519".to_string(),
                },
            },
        }]),
        warning: Some(vec![
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning_1 },
            },
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::WarningCard { f: warning_2 },
            },
        ]),
        new_specs: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae",
                    )
                    .unwrap(),
                    logo: "dock-pos-main-runtime".to_string(),
                    name: "dock-pos-main-runtime".to_string(),
                    path_id: "//dock-pos-main-runtime".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "dock-pos-main-runtime-sr25519".to_string(),
                    unit: "DOCK".to_string(),
                },
            },
        }]),
        ..Default::default()
    };

    //
    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let warning = "Received message is verified by the general verifier. Current verifier for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: none.".to_string();
    let warning_2 = "Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database.".to_string();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: Identicon::Dots {
                        identity: alice_sr_alice(),
                    },
                    encryption: "sr25519".to_string(),
                },
            },
        }]),
        warning: Some(vec![
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning },
            },
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::WarningCard { f: warning_2 },
            },
        ]),
        new_specs: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecs {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str(
                        "6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae",
                    )
                    .unwrap(),
                    logo: "dock-pos-main-runtime".to_string(),
                    name: "dock-pos-main-runtime".to_string(),
                    path_id: "//dock-pos-main-runtime".to_string(),
                    secondary_color: "#262626".to_string(),
                    title: "dock-pos-main-runtime-sr25519".to_string(),
                    unit: "DOCK".to_string(),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn can_parse_westend_with_v14() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/load_metadata_westendV9111_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        meta: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "westend".to_string(),
                    specs_version: "9111".to_string(),
                    meta_hash: "207956815bc7b3234fa8827ef40df5fd2879e93f18a680e22bc6801bca27312d"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: westend_9111(),
                    },
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(&db);
        let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(&db, checksum).unwrap();

        let print_after = print_db_content(&db);
        let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
    westend9111
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d9c0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480284d717d5031504025a62029723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs = "Same as the [`transfer`] call, but with a check that the transfer will not kill the\norigin account.\n\n99% of the time you want [`transfer`] instead.\n\n[`transfer`]: struct.Pallet.html#method.transfer\n# <weight>\n- Cheaper than transfer because account cannot be killed.\n- Base Weight: 51.4 µs\n- DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n#</weight>".to_string();
    let output = produce_output(&db, line).unwrap();
    let content_known = TransactionCardSet {
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
                        identicon: Identicon::Dots { identity: bob() },
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
                        amount: "100.000000".to_string(),
                        units: "uWND".to_string(),
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
                        phase: "61".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 0,
                card: Card::NonceCard {
                    f: "261".to_string(),
                },
            },
            TransactionCard {
                index: 9,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "10.000000".to_string(),
                        units: "uWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 10,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9111".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 11,
                indent: 0,
                card: Card::TxSpecCard { f: "7".to_string() },
            },
            TransactionCard {
                index: 12,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: concat!(
            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
        .to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };
    // TODO: let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign { actions, checksum } = output {
        let TransactionSignAction {
            content,
            has_pwd,
            author_info,
            network_info,
        } = &actions[0];
        assert_eq!(actions.len(), 1);
        assert_eq!(content, &content_known);
        assert_eq!(author_info, &author_info_known);
        // TODO: assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
        sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        )
        .unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ffe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let output = produce_output(&db, line).unwrap();
    let docs1 = "Send a batch of dispatch calls and atomically execute them.\nThe whole transaction will rollback and fail if any of the calls failed.\n\nMay be called from any origin.\n\n- `calls`: The calls to be dispatched from the same origin. The number of call must not\n  exceed the constant: `batched_calls_limit` (available in constant metadata).\n\nIf origin is root then call are dispatch without checking origin filter. (This includes\nbypassing `frame_system::Config::BaseCallFilter`).\n\n# <weight>\n- Complexity: O(C) where C is the number of calls to be batched.\n# </weight>".to_string();
    let docs2 = "Take the origin account as a stash and lock up `value` of its balance. `controller` will\nbe the account that controls it.\n\n`value` must be more than the `minimum_balance` specified by `T::Currency`.\n\nThe dispatch origin for this call must be _Signed_ by the stash account.\n\nEmits `Bonded`.\n# <weight>\n- Independent of the arguments. Moderate complexity.\n- O(1).\n- Three extra DB entries.\n\nNOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned\nunless the `origin` falls below _existential deposit_ and gets removed as dust.\n------------------\n# </weight>".to_string();

    let docs3 = "Declare the desire to nominate `targets` for the origin controller.\n\nEffects will be felt at the beginning of the next era.\n\nThe dispatch origin for this call must be _Signed_ by the controller, not the stash.\n\n# <weight>\n- The transaction's complexity is proportional to the size of `targets` (N)\nwhich is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS).\n- Both the reads and writes follow a similar pattern.\n# </weight>".to_string();

    let content_known = TransactionCardSet {
        method: Some(vec![
            TransactionCard {
                index: 0,
                indent: 0,
                card: Card::PalletCard {
                    f: "Utility".to_string(),
                },
            },
            TransactionCard {
                index: 1,
                indent: 1,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "batch_all".to_string(),
                        docs: docs1,
                    },
                },
            },
            TransactionCard {
                index: 2,
                indent: 2,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "calls".to_string(),
                        docs_field_name: String::new(),
                        path_type: String::new(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 3,
                indent: 3,
                card: Card::PalletCard {
                    f: "Staking".to_string(),
                },
            },
            TransactionCard {
                index: 4,
                indent: 4,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "bond".to_string(),
                        docs: docs2,
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 5,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "controller".to_string(),
                        docs_field_name: String::new(),
                        path_type: "sp_runtime >> multiaddress >> MultiAddress".to_string(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 7,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
                        identicon: Identicon::Dots {
                            identity: alice_sr_root(),
                        },
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 5,
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
                index: 9,
                indent: 6,
                card: Card::BalanceCard {
                    f: MSCCurrency {
                        amount: "1.061900000000".to_string(),
                        units: "WND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 10,
                indent: 5,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "payee".to_string(),
                        docs_field_name: String::new(),
                        path_type: "pallet_staking >> RewardDestination".to_string(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 11,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Staked".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 12,
                indent: 3,
                card: Card::PalletCard {
                    f: "Staking".to_string(),
                },
            },
            TransactionCard {
                index: 13,
                indent: 4,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "nominate".to_string(),
                        docs: docs3,
                    },
                },
            },
            TransactionCard {
                index: 14,
                indent: 5,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "targets".to_string(),
                        docs_field_name: String::new(),
                        path_type: String::new(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 15,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 16,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh".to_string(),
                        identicon: Identicon::Dots { identity: id_04() },
                    },
                },
            },
            TransactionCard {
                index: 17,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 18,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ".to_string(),
                        identicon: Identicon::Dots { identity: id_01() },
                    },
                },
            },
            TransactionCard {
                index: 19,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 20,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f".to_string(),
                        identicon: Identicon::Dots { identity: id_02() },
                    },
                },
            },
        ]),
        extensions: Some(vec![
            TransactionCard {
                index: 21,
                indent: 0,
                card: Card::EraMortalCard {
                    f: MSCEraMortal {
                        era: "Mortal".to_string(),
                        phase: "5".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 22,
                indent: 0,
                card: Card::NonceCard { f: "2".to_string() },
            },
            TransactionCard {
                index: 23,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 24,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9111".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 25,
                indent: 0,
                card: Card::TxSpecCard { f: "7".to_string() },
            },
            TransactionCard {
                index: 26,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: concat!(
            "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
        .to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };
    // TODO let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign { actions, checksum } = output {
        let TransactionSignAction {
            content,
            has_pwd,
            author_info,
            network_info,
        } = &actions[0];

        assert_eq!(actions.len(), 1);
        assert_eq!(content, &content_known);
        assert_eq!(author_info, &author_info_known);
        // TODO assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
        sign_action_test(
            &db,
            checksum,
            ALICE_SEED_PHRASE,
            PWD,
            USER_COMMENT,
            network_info.specs.encryption,
        )
        .unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let entry = get_history_entry_by_order(&db, 3).unwrap();
    let _historic_reply = entry_to_transactions_with_decoding(&db, entry).unwrap();

    /*
        r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Balances"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"transfer_keep_alive","docs":"53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"dest","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":3,"indent":3,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":4,"indent":4,"type":"Id","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","identicon":"<bob>"}},{"index":5,"indent":2,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":6,"indent":3,"type":"balance","payload":{"amount":"100.000000","units":"uWND"}}],"extensions":[{"index":7,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"61","period":"64"}},{"index":8,"indent":0,"type":"nonce","payload":"261"},{"index":9,"indent":0,"type":"tip","payload":{"amount":"10.000000","units":"uWND"}},{"index":10,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":11,"indent":0,"type":"tx_version","payload":"7"},{"index":12,"indent":0,"type":"block_hash","payload":"98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84"}]"#;
        assert!(
            historic_reply.contains(historic_reply_known),
            "Received different historic reply for order 3: \n{}\n{}",
            historic_reply,
            print_history(dbname).unwrap()
        );

        let historic_reply = print_history_entry_by_order_with_decoding(4, dbname)
            .unwrap()
            .replace(&alice_sr_root(), r#"<alice_sr25519_root>"#)
            .replace(&id_04(), r#"<id_04>"#)
            .replace(&id_01(), r#"<id_01>"#)
            .replace(&id_02(), r#"<id_02>"#);
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"ake the origin account as a stash and lock up `value` of its balance. `controller` will\nbe the account that controls it.\n\n`value` must be more than the `minimum_balance` specified by `T::Currency`.\n\nThe dispatch origin for this call must be _Signed_ by the stash account.\n\nEmits `Bonded`.\n# <weight>\n- Independent of the arguments. Moderate complexity.\n- O(1).\n- Three extra DB entries.\n\nNOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned\nunless the `origin` falls below _existential deposit_ and gets removed as dust.\n------------------\n# </weight>"}},{"index":5,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"<alice_sr25519_root>"}},{"index":8,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":9,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":10,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":14,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh","identicon":"<id_04>"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"<id_01>"}},{"index":19,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":20,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"<id_02>"}}],"extensions":[{"index":21,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":22,"indent":0,"type":"nonce","payload":"2"},{"index":23,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":24,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":25,"indent":0,"type":"tx_version","payload":"7"},{"index":26,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}]"#;
        assert!(
            historic_reply.contains(historic_reply_known),
            "Received different historic reply for order 4: \n{}",
            historic_reply
        );

    */
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_alice_remarks_westend9122() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/load_metadata_westendV9122_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        meta: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "westend".to_string(),
                    specs_version: "9122".to_string(),
                    meta_hash: "d656951f4c58c9fdbe029be33b02a7095abc3007586656be7ff68fd0550d6ced"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: westend_9122(),
                    },
                },
            },
        }]),
        ..Default::default()
    };
    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: checksum,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d2509000115094c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e2045022c00a223000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66ae143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let output = produce_output(&db, line).unwrap();

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Congue eu consequat ac felis donec. Turpis egestas integer eget aliquet nibh praesent. Neque convallis a cras semper auctor neque. Netus et malesuada fames ac turpis egestas sed tempus. Pellentesque habitant morbi tristique senectus et netus et. Pretium vulputate sapien nec sagittis aliquam. Convallis aenean et tortor at risus viverra. Vivamus arcu felis bibendum ut tristique et egestas quis ipsum. Malesuada proin libero nunc consequat interdum varius. ".to_string();

    let docs = "Make some on-chain remark.\n\n# <weight>\n- `O(1)`\n# </weight>".to_string();

    let content_known = TransactionCardSet {
        method: Some(vec![
            TransactionCard {
                index: 0,
                indent: 0,
                card: Card::PalletCard {
                    f: "System".to_string(),
                },
            },
            TransactionCard {
                index: 1,
                indent: 1,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "remark".to_string(),
                        docs,
                    },
                },
            },
            TransactionCard {
                index: 2,
                indent: 2,
                card: Card::FieldNameCard {
                    f: MSCFieldName {
                        name: "remark".to_string(),
                        docs_field_name: String::new(),
                        path_type: String::new(),
                        docs_type: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 3,
                indent: 3,
                card: Card::TextCard { f: text },
            },
        ]),
        extensions: Some(vec![
            TransactionCard {
                index: 4,
                indent: 0,
                card: Card::EraMortalCard {
                    f: MSCEraMortal {
                        era: "Mortal".to_string(),
                        phase: "36".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 0,
                card: Card::NonceCard {
                    f: "11".to_string(),
                },
            },
            TransactionCard {
                index: 6,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 7,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9122".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 0,
                card: Card::TxSpecCard { f: "7".to_string() },
            },
            TransactionCard {
                index: 9,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };
    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key:
"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
            .to_string(),
        address: Address {
            identicon: Identicon::Dots {
                identity: alice_sr_alice(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };
    // TODO let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign { actions, .. } = output {
        let TransactionSignAction {
            content,
            has_pwd,
            author_info,
            ..
        } = &actions[0];
        assert_eq!(actions.len(), 1);
        assert_eq!(content, &content_known);
        assert_eq!(author_info, &author_info_known);
        // TODO: assert_eq!(network_info == network_info_known);
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn proper_hold_display() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let warning_1 = "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend, westend-ed25519; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string();
    let warning_2 =
        "Received types information is identical to the one that was in the database.".to_string();

    let output = produce_output(&db, line.trim()).unwrap();
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: Identicon::Dots {
                        identity: alice_sr_alice(),
                    },
                    encryption: "sr25519".to_string(),
                },
            },
        }]),
        warning: Some(vec![
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning_1 },
            },
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::WarningCard { f: warning_2 },
            },
        ]),
        types_info: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::TypesInfoCard {
                f: MTypesInfo {
                    types_on_file: false,
                    types_hash: Some(
                        "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb"
                            .to_string(),
                    ),
                    types_id_pic: Some(Identicon::Dots {
                        identity: types_known(),
                    }),
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::LoadTypes;

    if let TransactionAction::Stub {
        s: reply,
        u: _,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn delete_westend_try_load_metadata() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, verifier_alice_sr25519()).unwrap();
    remove_network(
        &db,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();
    let print_before = print_db_content(&db).replace(
        &hex::encode(alice_sr_alice()),
        r#"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"#,
    );
    let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:"#;
    assert_eq!(print_before, expected_print_before);

    let line =
        fs::read_to_string("for_tests/load_metadata_westendV9122_Alice-sr25519.txt").unwrap();
    let error = produce_output(&db, line.trim()).unwrap_err();

    if let transaction_parsing::Error::LoadMetaNoSpecs {
        name,
        valid_current_verifier: _,
        general_verifier,
    } = error
    {
        assert_eq!(name, "westend");
        assert_eq!(general_verifier.show_error(),
            "public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519"
        );
    } else {
        panic!("Unexpected error {error:?}");
    }
}

#[test]
fn dock_adventures_3() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold(&db, verifier_alice_sr25519()).unwrap();

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print_before = print_db_content(&db);
    let expected_print_before = r#"Database contents:
Metadata:
    dock-pos-main-runtime34
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
    assert_eq!(print_before, expected_print_before);

    remove_network(
        &db,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();

    let print_after = print_db_content(&db);
    let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
    assert_eq!(print_after, expected_print_after);

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();

    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }
    let print_after = print_db_content(&db);
    let expected_print_after = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
    assert_eq!(print_after, expected_print_after);

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }
    let print_after = print_db_content(&db);
    let expected_print_after = r#"Database contents:
Metadata:
    dock-pos-main-runtime34
    kusama2030
    polkadot30
    westend9000
    westend9010
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
    01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: Westend (westend with sr25519)
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"#;
    assert_eq!(print_after, expected_print_after);
}

#[test]
fn acala_adventures() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, Verifier { v: None }).unwrap();

    let line = fs::read_to_string("for_tests/add_specs_acala-sr25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print_after = print_db_content(&db);
    let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
    01fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c: Acala (acala with sr25519)
Verifiers:
    fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: none
Identities:"#;
    assert!(
        print_after == expected_print_after,
        "Received: \n{print_after}"
    );

    let line = fs::read_to_string("for_tests/load_metadata_acalaV2012_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = "530102dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce97359a80a0000dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce973590b00407a10f35a24010000dc07000001000000fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c";

    let docs = "Transfer some liquid free balance to another account.\n\n`transfer` will set the `FreeBalance` of the sender and receiver.\nIt will decrease the total issuance of the system by the `TransferFee`.\nIf the sender's account is below the existential deposit as a result\nof the transfer, the account will be reaped.\n\nThe dispatch origin for this call must be `Signed` by the transactor.\n\n# <weight>\n- Dependent on arguments but not critical, given proper implementations for input config\n  types. See related functions below.\n- It contains a limited number of reads and writes internally and no complex\n  computation.\n\nRelated functions:\n\n  - `ensure_can_withdraw` is always called internally but has a bounded complexity.\n  - Transferring balances to accounts that did not exist before will cause\n    `T::OnNewAccount::on_new_account` to be called.\n  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.\n  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check\n    that the transfer will not kill the origin account.\n---------------------------------\n- Origin account is already in memory, so no DB operations for them.\n# </weight>".to_string();

    let output = produce_output(&db, line).unwrap();
    let content_known = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorPlainCard {
                f: MSCId {
                    base58: "25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C".to_string(),
                    identicon: Identicon::Dots { identity: id_05() },
                },
            },
        }]),
        warning: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::WarningCard {
                f: "Transaction author public key not found.".to_string(),
            },
        }]),
        method: Some(vec![
            TransactionCard {
                index: 2,
                indent: 0,
                card: Card::PalletCard {
                    f: "Balances".to_string(),
                },
            },
            TransactionCard {
                index: 3,
                indent: 1,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "transfer".to_string(),
                        docs,
                    },
                },
            },
            TransactionCard {
                index: 4,
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
                index: 5,
                indent: 3,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 4,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C".to_string(),
                        identicon: Identicon::Dots { identity: id_05() },
                    },
                },
            },
            TransactionCard {
                index: 7,
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
                index: 8,
                indent: 3,
                card: Card::BalanceCard {
                    f: MSCCurrency {
                        amount: "100.000000000000".to_string(),
                        units: "ACA".to_string(),
                    },
                },
            },
        ]),
        extensions: Some(vec![
            TransactionCard {
                index: 9,
                indent: 0,
                card: Card::EraMortalCard {
                    f: MSCEraMortal {
                        era: "Mortal".to_string(),
                        phase: "18".to_string(),
                        period: "32".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 10,
                indent: 0,
                card: Card::NonceCard { f: "0".to_string() },
            },
            TransactionCard {
                index: 11,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pACA".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 12,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "acala".to_string(),
                        version: "2012".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 13,
                indent: 0,
                card: Card::TxSpecCard { f: "1".to_string() },
            },
            TransactionCard {
                index: 14,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    if let TransactionAction::Read { r: content } = output {
        assert_eq!(*content, content_known);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn shell_no_token_warning_on_metadata() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, Verifier { v: None }).unwrap();

    let line = fs::read_to_string("for_tests/add_specs_shell-sr25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let line = fs::read_to_string("for_tests/load_metadata_shellV200_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();

    let warning_1 = "Received metadata has incomplete set of signed extensions. As a result, Vault may be unable to parse signable transactions using this metadata.".to_string();
    let warning_2 = "Received network information is not verified.".to_string();

    let reply_known = TransactionCardSet {
        warning: Some(vec![
            TransactionCard {
                index: 0,
                indent: 0,
                card: Card::WarningCard { f: warning_1 },
            },
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard { f: warning_2 },
            },
        ]),
        meta: Some(vec![TransactionCard {
            index: 2,
            indent: 0,
            card: Card::MetaCard {
                f: MMetadataRecord {
                    specname: "shell".to_string(),
                    specs_version: "200".to_string(),
                    meta_hash: "65f0d394de10396c6c1800092f9a95c48ec1365d9302dbf5df736c5e0c54fde3"
                        .to_string(),
                    meta_id_pic: Identicon::Dots {
                        identity: shell_200(),
                    },
                },
            },
        }]),
        ..Default::default()
    };

    let stub_nav_known = StubNav::LoadMeta {
        l: NetworkSpecsKey::from_parts(
            &H256::from_str("a216666c2d1b8745bbeba02293b6dabbe30685ca29a25f481a82ef8443447258")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    if let TransactionAction::Stub {
        s: reply,
        u: _,
        stub: stub_nav,
    } = output
    {
        assert_eq!(*reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
    } else {
        panic!("Wrong action: {output:?}")
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_1() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, verifier_alice_sr25519()).unwrap();

    // added rococo specs with `ed25519`, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-ed25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    // added rococo specs with `sr25519`, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0027b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-ed25519 (rococo with ed25519)
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // remove only one of the rococo's
    remove_network(
        &db,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_2() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, verifier_alice_sr25519()).unwrap();

    // added rococo specs with `sr25519`, general verifier, specified one
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // remove it
    remove_network(
        &db,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_3() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, verifier_alice_sr25519()).unwrap();

    // added rococo specs with `sr25519`, custom verifier None
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // remove it
    remove_network(
        &db,
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    )
    .unwrap();

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_4() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, verifier_alice_sr25519()).unwrap();

    // added rococo specs with `sr25519`, custom verifier None
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_unverified.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // added rococo specs with `sr25519`, general verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_5() {
    let dbname = &tempdir().unwrap().into_path().to_str().unwrap().to_string();
    let db = sled::open(dbname).unwrap();

    populate_cold_no_networks(&db, verifier_alice_sr25519()).unwrap();

    // added rococo specs with `sr25519`, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // added rococo specs with `sr25519`, general verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(&db, line.trim()).unwrap();
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(&db, checksum).unwrap();
    } else {
        panic!("Wrong action: {output:?}")
    }

    let print = print_db_content(&db);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}
