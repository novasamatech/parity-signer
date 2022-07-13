use std::fmt::Write as _;

use parity_scale_codec::{Decode, Encode};
use pretty_assertions::assert_eq;
use sled::{open, Db, Tree};
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::{fs, io::Write, str::FromStr};

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
use definitions::{
    crypto::Encryption,
    error::{AddressKeySource, ErrorSource},
    error_signer::{DatabaseSigner, ErrorSigner, Signer},
    history::{Entry, Event, SignDisplay, SignMessageDisplay},
    keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey},
    navigation::{
        Address, Card, MMetadataRecord, MSCAuthorPlain, MSCCall, MSCCurrency, MSCEnumVariantName,
        MSCEraMortal, MSCFieldName, MSCId, MSCNameVersion, MTypesInfo, MVerifierDetails,
        NetworkSpecsToSend, TransactionCard, TransactionCardSet,
    },
    network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue},
    users::AddressDetails,
};
use transaction_parsing::{
    entry_to_transactions_with_decoding, produce_output, StubNav, TransactionAction,
};

use crate::{handle_stub, sign_transaction::create_signature};

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
    checksum: u32,
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    dbname: &str,
) -> Result<String, ErrorSigner> {
    Ok(hex::encode(
        create_signature(seed_phrase, pwd_entry, user_comment, dbname, checksum)?.encode(),
    ))
}

fn identicon_to_str(identicon: &[u8]) -> &str {
    if identicon == ed() {
        "<ed>"
    } else if identicon == alice_sr_alice() {
        "<alice_sr25519_//Alice>"
    } else if identicon == empty_png() {
        "<empty>"
    } else {
        "<unknown>"
    }
}

fn print_db_content(dbname: &str) -> String {
    let database: Db = open(dbname).unwrap();

    let mut metadata_set: Vec<String> = Vec::new();
    let metadata: Tree = database.open_tree(METATREE).unwrap();
    for (meta_key_vec, _) in metadata.iter().flatten() {
        let meta_key = MetaKey::from_ivec(&meta_key_vec);
        let (name, version) = meta_key.name_version::<Signer>().unwrap();
        metadata_set.push(format!("{}{}", name, version));
    }
    metadata_set.sort();
    let mut metadata_str = String::new();
    for x in metadata_set.iter() {
        let _ = write!(metadata_str, "\n    {}", x);
    }

    let mut network_specs_set: Vec<(NetworkSpecsKey, NetworkSpecs)> = Vec::new();
    let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(
            &network_specs_key,
            network_specs_encoded,
        )
        .unwrap();
        network_specs_set.push((network_specs_key, network_specs));
    }
    network_specs_set.sort_by(|(_, a), (_, b)| a.title.cmp(&b.title));
    let mut network_specs_str = String::new();
    for (network_specs_key, network_specs) in network_specs_set.iter() {
        let _ = write!(
            network_specs_str,
            "\n    {}: {} ({} with {})",
            hex::encode(network_specs_key.key()),
            network_specs.title,
            network_specs.name,
            network_specs.encryption.show()
        );
    }

    let settings: Tree = database.open_tree(SETTREE).unwrap();
    let general_verifier_encoded = settings.get(&GENERALVERIFIER).unwrap().unwrap();
    let general_verifier = Verifier::decode(&mut &general_verifier_encoded[..]).unwrap();

    let mut verifiers_set: Vec<String> = Vec::new();
    let verifiers: Tree = database.open_tree(VERIFIERS).unwrap();
    for (verifier_key_vec, current_verifier_encoded) in verifiers.iter().flatten() {
        let verifier_key = VerifierKey::from_ivec::<Signer>(&verifier_key_vec).unwrap();
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
            CurrentVerifier::Dead => verifiers_set.push(format!(
                "{}: network inactivated",
                hex::encode(verifier_key.key())
            )),
        }
    }
    verifiers_set.sort();
    let mut verifiers_str = String::new();
    for x in verifiers_set.iter() {
        let _ = write!(verifiers_str, "\n    {}", x);
    }

    let mut identities_set: Vec<String> = Vec::new();
    let identities: Tree = database.open_tree(ADDRTREE).unwrap();
    for (address_key_vec, address_details_encoded) in identities.iter().flatten() {
        let address_key = AddressKey::from_ivec(&address_key_vec);
        let address_details = AddressDetails::decode(&mut &address_details_encoded[..]).unwrap();
        let (public_key, encryption) = address_key
            .public_key_encryption::<Signer>(AddressKeySource::AddrTree)
            .unwrap();

        let mut networks_set: Vec<String> = Vec::new();
        for y in address_details.network_id.iter() {
            networks_set.push(hex::encode(y.key()))
        }
        networks_set.sort();
        let mut networks_str = String::new();
        for y in networks_set.iter() {
            let _ = write!(networks_str, "\n        {}", y);
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
        let _ = write!(identities_str, "\n    {}", x);
    }

    format!("Database contents:\nMetadata:{}\nNetwork Specs:{}\nVerifiers:{}\nGeneral Verifier: {}\nIdentities:{}", metadata_str, network_specs_str, verifiers_str, general_verifier.show_error(), identities_str)
}

fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().any(|e| e.events.contains(event))
}

// can sign a parsed transaction
#[test]
fn can_sign_transaction_1() {
    let dbname = "for_tests/can_sign_transaction_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs = "2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e".to_string();

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
                        identicon: bob().to_vec(),
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
    let author_info_known = Address {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        identicon: alice_sr_alice().to_vec(),
        seed_name: "Alice".to_string(),
        path: "//Alice".to_string(),
        has_pwd: false,
        multiselect: None,
    };

    let network_info_known = NetworkSpecs {
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
        order: 2,
        path_id: "//westend".to_string(),
        secondary_color: "#262626".to_string(),
        title: "Westend".to_string(),
        unit: "WND".to_string(),
    };

    let output = produce_output(line, dbname);
    if let TransactionAction::Sign {
        content: set,
        checksum,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        assert_eq!(set, set_expected);
        assert_eq!(author_info, author_info_known);
        assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");

        match sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(signature) => assert!(
                (signature.len() == 130) && (signature.starts_with("01")),
                "Wrong signature format,\nReceived: \n{}",
                signature
            ),
            Err(e) => panic!("Was unable to sign. {:?}", e),
        }

        let history_recorded: Vec<_> = get_history(dbname)
            .unwrap()
            .into_iter()
            .map(|e| e.1)
            .collect();
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

        let result = sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {
            let expected_err = ErrorSigner::Database(DatabaseSigner::ChecksumMismatch);
            if <Signer>::show(&e) != <Signer>::show(&expected_err) {
                panic!("Expected wrong checksum. Got error: {:?}.", e)
            }
        } else {
            panic!("Checksum should have changed.")
        }

        let entry = get_history_entry_by_order(2, dbname).unwrap();
        let historic_reply = entry_to_transactions_with_decoding(entry, dbname).unwrap();
        let docs = "2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e".to_string();

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
                            identicon: bob().to_vec(),
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
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

// can sign a message
#[test]
fn can_sign_message_1() {
    let dbname = "for_tests/can_sign_message_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d3c42797465733e4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e3c2f42797465733ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let output = produce_output(line, dbname);
    let message = "4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e".to_string();

    let content_known = TransactionCardSet {
        message: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::TextCard { f: message.clone() },
        }]),
        ..Default::default()
    };

    let author_info_known = Address {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        identicon: alice_sr_alice().to_vec(),
        seed_name: "Alice".to_string(),
        path: "//Alice".to_string(),
        has_pwd: false,
        multiselect: None,
    };

    let network_info_known = NetworkSpecs {
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
        order: 2,
        path_id: "//westend".to_string(),
        secondary_color: "#262626".to_string(),
        title: "Westend".to_string(),
        unit: "WND".to_string(),
    };

    if let TransactionAction::Sign {
        content,
        checksum,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");

        match sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(signature) => assert!(
                (signature.len() == 130) && (signature.starts_with("01")),
                "Wrong signature format,\nReceived: \n{}",
                signature
            ),
            Err(e) => panic!("Was unable to sign. {:?}", e),
        }

        let history_recorded: Vec<_> = get_history(dbname)
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
            "Recorded {:?}",
            history_recorded
        );

        let result = sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {
            let expected_err = ErrorSigner::Database(DatabaseSigner::ChecksumMismatch);
            if <Signer>::show(&e) != <Signer>::show(&expected_err) {
                panic!("Expected wrong checksum. Got error: {:?}.", e)
            }
        } else {
            panic!("Checksum should have changed.")
        }
    } else {
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_no_network_info_not_signed() {
    let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
    populate_cold_no_networks(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(dbname);
        let expected_print_before = "Database contents:\nMetadata:\nNetwork Specs:\nVerifiers:\nGeneral Verifier: none\nIdentities:";
        assert!(
            print_before == expected_print_before,
            "Received: \n{}",
            print_before
        );

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_signed() {
    let dbname = "for_tests/add_specs_westend_ed25519_not_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before =
            print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        let mut file1 = std::fs::File::create("/tmp/a").unwrap();
        let mut file2 = std::fs::File::create("/tmp/b").unwrap();
        file1.write_all(print_before.as_bytes()).unwrap();
        file2.write_all(expected_print_before.as_bytes()).unwrap();
        assert_eq!(print_before, expected_print_before);

        handle_stub(checksum, dbname).unwrap();
        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_after, expected_print_after);

        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "",
            &NetworkSpecsKey::from_parts(
                &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                    .unwrap(),
                &Encryption::Ed25519,
            ),
            dbname,
        )
        .unwrap();
        try_create_address(
            "Alice",
            ALICE_SEED_PHRASE,
            "//westend",
            &NetworkSpecsKey::from_parts(
                &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                    .unwrap(),
                &Encryption::Ed25519,
            ),
            dbname,
        )
        .unwrap();
        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks:
        00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_after, expected_print_after);

        remove_seed(dbname, "Alice").unwrap();
        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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

        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: a52095ee77497ba94588d61c3f71c4cfa0d6a4f389cef43ebadc76c29c4f42de, encryption: ed25519, path: //westend, available_networks:
        00e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070() {
    let dbname = "for_tests/load_westend9070";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    meta_id_pic: westend_9070().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before =
            print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_known_types_upd_general_verifier() {
    let dbname = "for_tests/load_known_types_upd_general_verifier";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    identicon: alice_sr_alice().to_vec(),
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
                    types_id_pic: Some(types_known().to_vec()),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before =
            print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_before, expected_print_before);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert_eq!(print_after, expected_print_after);
    } else {
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_new_types_verified() {
    let dbname = "for_tests/load_new_types_verified";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
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
                    types_hash: Some(
                        "d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"
                            .to_string(),
                    ),
                    types_id_pic: Some(types_unknown().to_vec()),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_before == expected_print_before,
            "Received: \n{}",
            print_before
        );

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn dock_adventures_1() {
    let dbname = "for_tests/dock_adventures_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before =
            print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_before == expected_print_before,
            "Received: \n{}",
            print_before
        );

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    meta_id_pic: dock_31().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    identicon: alice_sr_alice().to_vec(),
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
        let expected_print_after = r#"Database contents:
Metadata:
Network Specs:
    016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: dock-pos-main-runtime-sr25519 (dock-pos-main-runtime with sr25519)
Verifiers:
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn dock_adventures_2() {
    let dbname = "for_tests/dock_adventures_2";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_before == expected_print_before,
            "Received: \n{}",
            print_before
        );

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#)
            .replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    meta_id_pic: dock_31().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#)
            .replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    identicon: ed().to_vec(),
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#)
            .replace(&hex::encode(ed()), r#"<ed>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let warning = "Received message is verified by the general verifier. Current verifier for network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. Affected network specs entries: dock-pos-main-runtime-sr25519; affected metadata entries: none.".to_string();
    let warning_2 = "Received network specs information for dock-pos-main-runtime-sr25519 is same as the one already in the database.".to_string();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: alice_sr_alice().to_vec(),
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
                f: NetworkSpecsToSend {
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname)
            .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
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
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn can_parse_westend_with_v14() {
    let dbname = "for_tests/can_parse_westend_with_v14";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/load_metadata_westendV9111_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    meta_id_pic: westend_9111().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);

        let print_before =
            print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_before == expected_print_before,
            "Received: \n{}",
            print_before
        );

        handle_stub(checksum, dbname).unwrap();

        let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
        assert!(
            print_after == expected_print_after,
            "Received: \n{}",
            print_after
        );
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = "530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d9c0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480284d717d5031504025a62029723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs = "53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a23203c7765696768743e0a2d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a2d2042617365205765696768743a2035312e3420c2b5730a2d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a233c2f7765696768743e".to_string();
    let output = produce_output(line, dbname);
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

    let author_info_known = Address {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        identicon: alice_sr_alice().to_vec(),
        seed_name: "Alice".to_string(),
        path: "//Alice".to_string(),
        has_pwd: false,
        multiselect: None,
    };
    // TODO: let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign {
        content,
        checksum,
        has_pwd,
        author_info,
        network_info: _,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        // TODO: assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
        sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ffe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let output = produce_output(line, dbname);
    let docs1 = "53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e".to_string();
    let docs2 = "54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e".to_string();

    let docs3 = "4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e".to_string();

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
                        identicon: alice_sr_root().to_vec(),
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
                        identicon: id_04().to_vec(),
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
                        identicon: id_01().to_vec(),
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
                        identicon: id_02().to_vec(),
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

    let author_info_known = Address {
        base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
        identicon: alice_sr_root().to_vec(),
        seed_name: "Alice".to_string(),
        path: String::new(),
        has_pwd: false,
        multiselect: None,
    };
    // TODO let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign {
        content,
        checksum,
        has_pwd,
        author_info,
        network_info: _,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        // TODO assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
        sign_action_test(checksum, ALICE_SEED_PHRASE, PWD, USER_COMMENT, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let entry = get_history_entry_by_order(3, dbname).unwrap();
    let _historic_reply = entry_to_transactions_with_decoding(entry, dbname).unwrap();

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
        let historic_reply_known = r#""method":[{"index":0,"indent":0,"type":"pallet","payload":"Utility"},{"index":1,"indent":1,"type":"method","payload":{"method_name":"batch_all","docs":"53656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a5468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a4d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a2d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e20546865206e756d626572206f662063616c6c206d757374206e6f740a20206578636565642074686520636f6e7374616e743a2060626174636865645f63616c6c735f6c696d6974602028617661696c61626c6520696e20636f6e7374616e74206d65746164617461292e0a0a4966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a23203c7765696768743e0a2d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a23203c2f7765696768743e"}},{"index":2,"indent":2,"type":"field_name","payload":{"name":"calls","docs_field_name":"","path_type":"","docs_type":""}},{"index":3,"indent":3,"type":"pallet","payload":"Staking"},{"index":4,"indent":4,"type":"method","payload":{"method_name":"bond","docs":"54616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a6076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a456d6974732060426f6e646564602e0a23203c7765696768743e0a2d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a2d204f2831292e0a2d20546872656520657874726120444220656e74726965732e0a0a4e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a23203c2f7765696768743e"}},{"index":5,"indent":5,"type":"field_name","payload":{"name":"controller","docs_field_name":"","path_type":"sp_runtime >> multiaddress >> MultiAddress","docs_type":""}},{"index":6,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":7,"indent":7,"type":"Id","payload":{"base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"<alice_sr25519_root>"}},{"index":8,"indent":5,"type":"field_name","payload":{"name":"value","docs_field_name":"","path_type":"","docs_type":""}},{"index":9,"indent":6,"type":"balance","payload":{"amount":"1.061900000000","units":"WND"}},{"index":10,"indent":5,"type":"field_name","payload":{"name":"payee","docs_field_name":"","path_type":"pallet_staking >> RewardDestination","docs_type":""}},{"index":11,"indent":6,"type":"enum_variant_name","payload":{"name":"Staked","docs_enum_variant":""}},{"index":12,"indent":3,"type":"pallet","payload":"Staking"},{"index":13,"indent":4,"type":"method","payload":{"method_name":"nominate","docs":"4465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a0a23203c7765696768743e0a2d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a77686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a2d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a23203c2f7765696768743e"}},{"index":14,"indent":5,"type":"field_name","payload":{"name":"targets","docs_field_name":"","path_type":"","docs_type":""}},{"index":15,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":16,"indent":7,"type":"Id","payload":{"base58":"5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh","identicon":"<id_04>"}},{"index":17,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":18,"indent":7,"type":"Id","payload":{"base58":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ","identicon":"<id_01>"}},{"index":19,"indent":6,"type":"enum_variant_name","payload":{"name":"Id","docs_enum_variant":""}},{"index":20,"indent":7,"type":"Id","payload":{"base58":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f","identicon":"<id_02>"}}],"extensions":[{"index":21,"indent":0,"type":"era","payload":{"era":"Mortal","phase":"5","period":"64"}},{"index":22,"indent":0,"type":"nonce","payload":"2"},{"index":23,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":24,"indent":0,"type":"name_version","payload":{"name":"westend","version":"9111"}},{"index":25,"indent":0,"type":"tx_version","payload":"7"},{"index":26,"indent":0,"type":"block_hash","payload":"5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"}]"#;
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
    let dbname = "for_tests/parse_transaction_alice_remarks_westend9122";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/load_metadata_westendV9122_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
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
                    meta_id_pic: westend_9122().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = "53010246ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a2509000115094c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e2045022c00a223000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66ae143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let output = produce_output(line, dbname);

    let text = "4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e20".to_string();

    let docs = "4d616b6520736f6d65206f6e2d636861696e2072656d61726b2e0a0a23203c7765696768743e0a2d20604f283129600a23203c2f7765696768743e".to_string();

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
    let author_info_known = Address {
        base58: "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV".to_string(),
        identicon: alice_sr_root().to_vec(),
        seed_name: "Alice".to_string(),
        path: String::new(),
        has_pwd: false,
        multiselect: None,
    };
    // TODO let network_info_known = r#""network_title":"Westend","network_logo":"westend""#;

    if let TransactionAction::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info: _,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        // TODO: assert_eq!(network_info == network_info_known);
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn proper_hold_display() {
    let dbname = "for_tests/proper_hold_display";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let warning_1 = "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend, westend-ed25519; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string();
    let warning_2 =
        "Received types information is identical to the one that was in the database.".to_string();

    let output = produce_output(line.trim(), dbname);
    let reply_known = TransactionCardSet {
        verifier: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::VerifierCard {
                f: MVerifierDetails {
                    public_key: "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        .to_string(),
                    identicon: alice_sr_alice().to_vec(),
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
                    types_id_pic: Some(types_known().to_vec()),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn delete_westend_try_load_metadata() {
    let dbname = "for_tests/delete_westend_try_load_metadata";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    remove_network(
        &NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        dbname,
    )
    .unwrap();
    let print_before = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
    let expected_print_before = r#"Database contents:
Metadata:
    kusama2030
    polkadot30
Network Specs:
    01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: Kusama (kusama with sr25519)
    0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: Polkadot (polkadot with sr25519)
Verifiers:
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
    assert_eq!(print_before, expected_print_before);

    let line =
        fs::read_to_string("for_tests/load_metadata_westendV9122_Alice-sr25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    let error = "Bad input data. Network westend was previously known to the database with verifier public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata.".to_string();

    let reply_known = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        ..Default::default()
    };

    if let TransactionAction::Read { r: reply } = output {
        assert_eq!(reply, reply_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn dock_adventures_3() {
    let dbname = "for_tests/dock_adventures_3";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print_before = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#)
        .replace(&hex::encode(ed()), r#"<ed>"#);
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
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        016bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
    assert_eq!(print_before, expected_print_before);

    remove_network(
        &NetworkSpecsKey::from_parts(
            &H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        dbname,
    )
    .unwrap();

    let print_after = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
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
    6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae: network inactivated
    91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
    e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:
    public_key: 3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34, encryption: sr25519, path: //westend, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a, encryption: sr25519, path: , available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: 64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05, encryption: sr25519, path: //kusama, available_networks:
        01b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe
    public_key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519, path: //Alice, available_networks:
        01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e
    public_key: f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730, encryption: sr25519, path: //polkadot, available_networks:
        0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"#;
    assert_eq!(print_after, expected_print_after);

    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-ed25519.txt")
            .unwrap();
    let error = "Network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is disabled. It could be enabled again only after complete wipe and re-installation of Signer.".to_string();

    let output = produce_output(line.trim(), dbname);
    let reply_known = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        ..Default::default()
    };

    if let TransactionAction::Read { r: reply } = output {
        assert_eq!(reply, reply_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV34_Alice-ed25519.txt")
            .unwrap();
    let output = produce_output(line.trim(), dbname);

    let error = "Network with genesis hash 6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae is disabled. It could be enabled again only after complete wipe and re-installation of Signer.".to_string();
    let reply_known = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        ..Default::default()
    };

    if let TransactionAction::Read { r: reply } = output {
        assert_eq!(reply, reply_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn acala_adventures() {
    let dbname = "for_tests/acala_adventures";
    populate_cold_no_networks(dbname, Verifier { v: None }).unwrap();

    let line = fs::read_to_string("for_tests/add_specs_acala-sr25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print_after = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
        "Received: \n{}",
        print_after
    );

    let line = fs::read_to_string("for_tests/load_metadata_acalaV2012_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = "530102dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce97359a80a0000dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce973590b00407a10f35a24010000dc07000001000000fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c";

    let docs = "5472616e7366657220736f6d65206c697175696420667265652062616c616e636520746f20616e6f74686572206163636f756e742e0a0a607472616e73666572602077696c6c207365742074686520604672656542616c616e636560206f66207468652073656e64657220616e642072656365697665722e0a49742077696c6c2064656372656173652074686520746f74616c2069737375616e6365206f66207468652073797374656d2062792074686520605472616e73666572466565602e0a4966207468652073656e6465722773206163636f756e742069732062656c6f7720746865206578697374656e7469616c206465706f736974206173206120726573756c740a6f6620746865207472616e736665722c20746865206163636f756e742077696c6c206265207265617065642e0a0a546865206469737061746368206f726967696e20666f7220746869732063616c6c206d75737420626520605369676e65646020627920746865207472616e736163746f722e0a0a23203c7765696768743e0a2d20446570656e64656e74206f6e20617267756d656e747320627574206e6f7420637269746963616c2c20676976656e2070726f70657220696d706c656d656e746174696f6e7320666f7220696e70757420636f6e6669670a202074797065732e205365652072656c617465642066756e6374696f6e732062656c6f772e0a2d20497420636f6e7461696e732061206c696d69746564206e756d626572206f6620726561647320616e642077726974657320696e7465726e616c6c7920616e64206e6f20636f6d706c65780a2020636f6d7075746174696f6e2e0a0a52656c617465642066756e6374696f6e733a0a0a20202d2060656e737572655f63616e5f77697468647261776020697320616c776179732063616c6c656420696e7465726e616c6c792062757420686173206120626f756e64656420636f6d706c65786974792e0a20202d205472616e7366657272696e672062616c616e63657320746f206163636f756e7473207468617420646964206e6f74206578697374206265666f72652077696c6c2063617573650a2020202060543a3a4f6e4e65774163636f756e743a3a6f6e5f6e65775f6163636f756e746020746f2062652063616c6c65642e0a20202d2052656d6f76696e6720656e6f7567682066756e64732066726f6d20616e206163636f756e742077696c6c20747269676765722060543a3a4475737452656d6f76616c3a3a6f6e5f756e62616c616e636564602e0a20202d20607472616e736665725f6b6565705f616c6976656020776f726b73207468652073616d652077617920617320607472616e73666572602c206275742068617320616e206164646974696f6e616c20636865636b0a202020207468617420746865207472616e736665722077696c6c206e6f74206b696c6c20746865206f726967696e206163636f756e742e0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a2d204f726967696e206163636f756e7420697320616c726561647920696e206d656d6f72792c20736f206e6f204442206f7065726174696f6e7320666f72207468656d2e0a23203c2f7765696768743e".to_string();

    let output = produce_output(line, dbname);
    let content_known = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorPlainCard {
                f: MSCAuthorPlain {
                    base58: "25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C".to_string(),
                    identicon: id_05().to_vec(),
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
                        identicon: id_05().to_vec(),
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
        assert_eq!(content, content_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn shell_no_token_warning_on_metadata() {
    let dbname = "for_tests/shell_no_token_warning_on_metadata";
    populate_cold_no_networks(dbname, Verifier { v: None }).unwrap();

    let line = fs::read_to_string("for_tests/add_specs_shell-sr25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let line = fs::read_to_string("for_tests/load_metadata_shellV200_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);

    let warning_1 = "Received metadata has incomplete set of signed extensions. As a result, Signer may be unable to parse signable transactions using this metadata.".to_string();
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
                    meta_id_pic: shell_200().to_vec(),
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
        assert_eq!(reply, reply_known);
        assert_eq!(stub_nav, stub_nav_known);
    } else {
        panic!("Wrong action: {:?}", output)
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_1() {
    let dbname = "for_tests/rococo_and_verifiers_1";
    populate_cold_no_networks(dbname, verifier_alice_sr25519()).unwrap();

    // added rococo specs with ed25519, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-ed25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    // added rococo specs with sr25519, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname).replace(&hex::encode(ed()), r#"<ed>"#);
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
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        dbname,
    )
    .unwrap();

    let print = print_db_content(dbname);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: network inactivated
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_2() {
    let dbname = "for_tests/rococo_and_verifiers_2";
    populate_cold_no_networks(dbname, verifier_alice_sr25519()).unwrap();

    // added rococo specs with sr25519, general verifier, specified one
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // remove it
    remove_network(
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        dbname,
    )
    .unwrap();

    let print = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_3() {
    let dbname = "for_tests/rococo_and_verifiers_3";
    populate_cold_no_networks(dbname, verifier_alice_sr25519()).unwrap();

    // added rococo specs with sr25519, custom verifier None
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
        &NetworkSpecsKey::from_parts(
            &H256::from_str("27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184")
                .unwrap(),
            &Encryption::Sr25519,
        ),
        dbname,
    )
    .unwrap();

    let print = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
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
    let dbname = "for_tests/rococo_and_verifiers_4";
    populate_cold_no_networks(dbname, verifier_alice_sr25519()).unwrap();

    // added rococo specs with sr25519, custom verifier None
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_unverified.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname).replace(&hex::encode(empty_png()), r#"<empty>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"","identicon":"<empty>","encryption":"none"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // added rococo specs with sr25519, general verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn rococo_and_verifiers_5() {
    let dbname = "for_tests/rococo_and_verifiers_5";
    populate_cold_no_networks(dbname, verifier_alice_sr25519()).unwrap();

    // added rococo specs with sr25519, custom verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-ed25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname).replace(&hex::encode(ed()), r#"<ed>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"custom","details":{"public_key":"88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee","identicon":"<ed>","encryption":"ed25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    // added rococo specs with sr25519, general verifier
    let line = fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: _,
        u: checksum,
        stub: _,
    } = output
    {
        handle_stub(checksum, dbname).unwrap();
    } else {
        panic!("Wrong action: {:?}", output)
    }

    let print = print_db_content(dbname)
        .replace(&hex::encode(alice_sr_alice()), r#"<alice_sr25519_//Alice>"#);
    let expected_print = r#"Database contents:
Metadata:
Network Specs:
    0127b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: rococo-sr25519 (rococo with sr25519)
Verifiers:
    27b0e1604364f6a7309d31ad60cdfb820666c3095b9f948c4a7d7894b6b3c184: "type":"general","details":{"public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","encryption":"sr25519"}
General Verifier: public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519
Identities:"#;
    assert_eq!(print, expected_print);

    fs::remove_dir_all(dbname).unwrap();
}
