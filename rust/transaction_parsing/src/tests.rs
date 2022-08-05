use crate::{produce_output, StubNav};
use constants::test_values::{
    alice_sr_alice, bob, ed, id_01, id_02, id_03, types_known, types_unknown, westend_9070,
};
use db_handling::{
    cold_default::{populate_cold, populate_cold_no_metadata, populate_cold_no_networks},
    manage_history::get_history,
};
use definitions::{
    crypto::Encryption,
    history::{Entry, Event},
    keyring::NetworkSpecsKey,
    navigation::{
        Address, Card, MMetadataRecord, MSCAuthorPlain, MSCCall, MSCCurrency, MSCEnumVariantName,
        MSCEraMortal, MSCId, MSCNameVersion, MTypesInfo, MVerifierDetails, NetworkSpecsToSend,
        TransactionAction, TransactionCard, TransactionCardSet,
    },
    network_specs::{NetworkSpecs, Verifier, VerifierValue},
};
use pretty_assertions::assert_eq;
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::{fs, str::FromStr};

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

fn verifier_alice_ed25519() -> Verifier {
    Verifier {
        v: Some(VerifierValue::Standard {
            m: MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(ALICE)),
        }),
    }
}

fn entries_contain_event(entries: &[Entry], event: &Event) -> bool {
    entries.iter().flat_map(|e| &e.events).any(|e| e == event)
}

fn westend_spec() -> NetworkSpecs {
    NetworkSpecs {
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
    }
}

#[test]
fn add_specs_westend_no_network_info_not_signed() {
    let dbname = "for_tests/add_specs_westend_no_network_info_not_signed";
    populate_cold_no_networks(dbname, Verifier { v: None }).unwrap();
    let current_history: Vec<_> = get_history(dbname)
        .unwrap()
        .into_iter()
        .map(|e| e.1)
        .collect();
    assert!(entries_contain_event(
        &current_history,
        &Event::DatabaseInitiated
    ));

    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    //r##""warning":
    //[{"index":0,"indent":0,"type":"warning","payload":"Received network information is not verified."}],
    //
    //"new_specs":[{"index":1,"indent":0,"type":"new_specs","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND"}}]"##;
    let stub_nav_known = StubNav::AddSpecs {
        n: NetworkSpecsKey::from_parts(
            &H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
                .unwrap(),
            &Encryption::Sr25519,
        ),
    };

    let card_set_known = TransactionCardSet {
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
        warning: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::WarningCard {
                f: "Received network information is not verified.".to_string(),
            },
        }]),
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);

    if let TransactionAction::Stub { s, u: _, stub } = output {
        assert_eq!(s, card_set_known);
        assert_eq!(stub, stub_nav_known)
    } else {
        panic!("expected TansactionAction::Stub, got {:?}", output);
    }

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_not_signed() {
    let dbname = "for_tests/add_specs_westend_not_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let action = produce_output(line.trim(), dbname);
    let expected_action = TransactionAction::Read {
        r: TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard { f:  "Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database.".to_string()},
            }]),
            ..Default::default()
        }
    };
    assert_eq!(action, expected_action);
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_not_signed_general_verifier_disappear() {
    let dbname = "for_tests/add_specs_westend_not_signed_general_verifier_disappear";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend_unverified.txt").unwrap();
    let action = produce_output(line.trim(), dbname);
    let expected_action = TransactionAction::Read {
        r: TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard { f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier.".to_string()},
            }]),
            ..Default::default()
        }
    };
    assert_eq!(action, expected_action);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_not_signed() {
    let dbname = "for_tests/load_types_known_not_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
    let action = produce_output(line.trim(), dbname);
    let expected_action = TransactionAction::Read {
        r: TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard {
                    f: "Bad input data. Exactly same types information is already in the database."
                        .to_string(),
                },
            }]),
            ..Default::default()
        },
    };
    assert_eq!(action, expected_action);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_not_signed_general_verifier_disappear() {
    let dbname = "for_tests/load_types_known_not_signed_general_verifier_disappear";
    populate_cold_no_metadata(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
    let action = produce_output(line.trim(), dbname);
    let expected_action = TransactionAction::Read {
        r: TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard {
                    f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier.".to_string(),
                },
            }]),
            ..Default::default()
        },
    };
    assert_eq!(action, expected_action);

    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed() {
    let dbname = "for_tests/load_types_known_alice_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();

    let expected_warning_1 =  "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged.".to_string();

    let warning = Some(vec![
        TransactionCard {
            index: 1,
            indent: 0,
            card: Card::WarningCard {
                f: expected_warning_1,
            },
        },
        TransactionCard {
            index: 2,
            indent: 0,
            card: Card::WarningCard {
                f: "Received types information is identical to the one that was in the database."
                    .to_string(),
            },
        },
    ]);
    let types_info = Some(vec![TransactionCard {
        index: 3,
        indent: 0,
        card: Card::TypesInfoCard {
            f: MTypesInfo {
                types_on_file: false,
                types_hash: Some(
                    "d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string(),
                ),
                types_id_pic: Some(types_known().to_vec()),
            },
        },
    }]);

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
        warning,
        types_info,
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub {
        s: reply,
        u: _,
        stub,
    } = output
    {
        assert_eq!(reply, reply_known);
        assert_eq!(stub, StubNav::LoadTypes);
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
    let reply_known = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Exactly same types information is already in the database."
                    .to_string(),
            },
        }]),
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: reply } = output {
        assert_eq!(reply, reply_known);
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
    let action_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received types information could be accepted only if verified by the same general verifier. Current message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519.".to_string() },
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: reply } = action {
        assert_eq!(reply, action_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_known_alice_signed_metadata_hold() {
    let dbname = "for_tests/load_types_known_alice_signed_metadata_hold";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
    let set_expected = TransactionCardSet {
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
            card: Card::WarningCard { f: "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string() },
        },
        TransactionCard {
            index: 2,
            indent: 0,
            card: Card::WarningCard { f: "Received types information is identical to the one that was in the database.".to_string() },
        }
        ]),
        types_info: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::TypesInfoCard { f: MTypesInfo {
                types_on_file: false,
                types_hash: Some("d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb".to_string()),
                types_id_pic: Some(types_known().to_vec()),
            }
            }
        }]),
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = output {
        assert_eq!(set, set_expected);
        assert_eq!(stub, StubNav::LoadTypes);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_unknown_not_signed() {
    let dbname = "for_tests/load_types_unknown_not_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
    let expected_set = TransactionCardSet {
        warning: Some(vec![
            TransactionCard {
                index: 0,
                indent: 0,
                card: Card::WarningCard {
                    f: "Received types information is not verified.".to_string(),
                },
            },
            TransactionCard {
                index: 1,
                indent: 0,
                card: Card::WarningCard {
                    f: "Updating types (really rare operation).".to_string(),
                },
            },
        ]),
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

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(stub, StubNav::LoadTypes);
        assert_eq!(set, expected_set);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_types_unknown_alice_signed() {
    let dbname = "for_tests/load_types_unknown_alice_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
    let expected_set = TransactionCardSet {
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
            card: Card::WarningCard { f: "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: none. Types information is purged.".to_string() },
        }, TransactionCard {
            index: 2,
            indent: 0,
            card: Card::WarningCard { f: "Updating types (really rare operation).".to_string() },
        }]),
        types_info: Some(vec![TransactionCard {
            index: 3,
            indent: 0,
            card: Card::TypesInfoCard {
                f: MTypesInfo {
                    types_on_file: false,
                    types_hash: Some("d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574".to_string()),
                    types_id_pic: Some(types_unknown().to_vec()),
                }
            }
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, expected_set);
        assert_eq!(stub, StubNav::LoadTypes);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_westend_50_not_in_db() {
    let dbname = "for_tests/parse_transaction_westend_50_not_in_db";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003200000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let expected_set = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9000).".to_string() },
        }]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, expected_set);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_1() {
    let dbname = "for_tests/parse_transaction_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

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
                        docs: "2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e".to_string(), 
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
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        assert_eq!(network_info, network_info_known);
        assert_eq!(has_pwd, false)
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_2() {
    let dbname = "for_tests/parse_transaction_2";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d550210020c060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700b864d9450006050800aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d0608008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f501b4003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let docs1 = "2053656e642061206261746368206f662064697370617463682063616c6c7320616e642061746f6d6963616c6c792065786563757465207468656d2e0a205468652077686f6c65207472616e73616374696f6e2077696c6c20726f6c6c6261636b20616e64206661696c20696620616e79206f66207468652063616c6c73206661696c65642e0a0a204d61792062652063616c6c65642066726f6d20616e79206f726967696e2e0a0a202d206063616c6c73603a205468652063616c6c7320746f20626520646973706174636865642066726f6d207468652073616d65206f726967696e2e0a0a204966206f726967696e20697320726f6f74207468656e2063616c6c2061726520646973706174636820776974686f757420636865636b696e67206f726967696e2066696c7465722e20285468697320696e636c756465730a20627970617373696e6720606672616d655f73797374656d3a3a436f6e6669673a3a4261736543616c6c46696c74657260292e0a0a2023203c7765696768743e0a202d20436f6d706c65786974793a204f284329207768657265204320697320746865206e756d626572206f662063616c6c7320746f20626520626174636865642e0a2023203c2f7765696768743e".to_string();

    let docs2 = "2054616b6520746865206f726967696e206163636f756e74206173206120737461736820616e64206c6f636b207570206076616c756560206f66206974732062616c616e63652e2060636f6e74726f6c6c6572602077696c6c0a20626520746865206163636f756e74207468617420636f6e74726f6c732069742e0a0a206076616c756560206d757374206265206d6f7265207468616e2074686520606d696e696d756d5f62616c616e636560207370656369666965642062792060543a3a43757272656e6379602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f20627920746865207374617368206163636f756e742e0a0a20456d6974732060426f6e646564602e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e204d6f64657261746520636f6d706c65786974792e0a202d204f2831292e0a202d20546872656520657874726120444220656e74726965732e0a0a204e4f54453a2054776f206f66207468652073746f726167652077726974657320286053656c663a3a626f6e646564602c206053656c663a3a7061796565602920617265205f6e657665725f20636c65616e65640a20756e6c6573732074686520606f726967696e602066616c6c732062656c6f77205f6578697374656e7469616c206465706f7369745f20616e6420676574732072656d6f76656420617320647573742e0a202d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c65646765722c205b4f726967696e204163636f756e745d2c2043757272656e74204572612c20486973746f72792044657074682c204c6f636b730a202d2057726974653a20426f6e6465642c2050617965652c205b4f726967696e204163636f756e745d2c204c6f636b732c204c65646765720a2023203c2f7765696768743e".to_string();

    let docs3 = "204465636c617265207468652064657369726520746f206e6f6d696e6174652060746172676574736020666f7220746865206f726967696e20636f6e74726f6c6c65722e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e20546869732063616e206f6e6c792062652063616c6c6564207768656e0a205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f2062792074686520636f6e74726f6c6c65722c206e6f74207468652073746173682e0a20416e642c2069742063616e206265206f6e6c792063616c6c6564207768656e205b60457261456c656374696f6e537461747573605d2069732060436c6f736564602e0a0a2023203c7765696768743e0a202d20546865207472616e73616374696f6e277320636f6d706c65786974792069732070726f706f7274696f6e616c20746f207468652073697a65206f662060746172676574736020284e290a2077686963682069732063617070656420617420436f6d7061637441737369676e6d656e74733a3a4c494d495420284d41585f4e4f4d494e4154494f4e53292e0a202d20426f74682074686520726561647320616e642077726974657320666f6c6c6f7720612073696d696c6172207061747465726e2e0a202d2d2d2d2d2d2d2d2d0a205765696768743a204f284e290a207768657265204e20697320746865206e756d626572206f6620746172676574730a204442205765696768743a0a202d2052656164733a2045726120456c656374696f6e205374617475732c204c65646765722c2043757272656e74204572610a202d205772697465733a2056616c696461746f72732c204e6f6d696e61746f72730a2023203c2f7765696768743e".to_string();

    let docs4 = "202852652d297365742074686520636f6e74726f6c6c6572206f6620612073746173682e0a0a20456666656374732077696c6c2062652066656c742061742074686520626567696e6e696e67206f6620746865206e657874206572612e0a0a20546865206469737061746368206f726967696e20666f7220746869732063616c6c206d757374206265205f5369676e65645f206279207468652073746173682c206e6f742074686520636f6e74726f6c6c65722e0a0a2023203c7765696768743e0a202d20496e646570656e64656e74206f662074686520617267756d656e74732e20496e7369676e69666963616e7420636f6d706c65786974792e0a202d20436f6e7461696e732061206c696d69746564206e756d626572206f662072656164732e0a202d2057726974657320617265206c696d6974656420746f2074686520606f726967696e60206163636f756e74206b65792e0a202d2d2d2d2d2d2d2d2d2d0a205765696768743a204f2831290a204442205765696768743a0a202d20526561643a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a202d2057726974653a20426f6e6465642c204c6564676572204e657720436f6e74726f6c6c65722c204c6564676572204f6c6420436f6e74726f6c6c65720a2023203c2f7765696768743e".to_string();

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
                card: Card::VarNameCard {
                    f: "calls".to_string(),
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
                card: Card::VarNameCard {
                    f: "controller".to_string(),
                },
            },
            TransactionCard {
                index: 6,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: "".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 7,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 5,
                card: Card::VarNameCard {
                    f: "value".to_string(),
                },
            },
            TransactionCard {
                index: 9,
                indent: 6,
                card: Card::BalanceCard {
                    f: MSCCurrency {
                        amount: "300.000000000".to_string(),
                        units: "mWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 10,
                indent: 5,
                card: Card::VarNameCard {
                    f: "payee".to_string(),
                },
            },
            TransactionCard {
                index: 11,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Staked".to_string(),
                        docs_enum_variant: "".to_string(),
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
                card: Card::VarNameCard {
                    f: "targets".to_string(),
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
                        base58: "5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ".to_string(),
                        identicon: id_01().to_vec(),
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
                        base58: "5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f".to_string(),
                        identicon: id_02().to_vec(),
                    },
                },
            },
            TransactionCard {
                index: 19,
                indent: 3,
                card: Card::PalletCard {
                    f: "Staking".to_string(),
                },
            },
            TransactionCard {
                index: 20,
                indent: 4,
                card: Card::CallCard {
                    f: MSCCall {
                        method_name: "set_controller".to_string(),
                        docs: docs4,
                    },
                },
            },
            TransactionCard {
                index: 21,
                indent: 5,
                card: Card::VarNameCard {
                    f: "controller".to_string(),
                },
            },
            TransactionCard {
                index: 22,
                indent: 6,
                card: Card::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: "Id".to_string(),
                        docs_enum_variant: String::new(),
                    },
                },
            },
            TransactionCard {
                index: 23,
                indent: 7,
                card: Card::IdCard {
                    f: MSCId {
                        base58: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                        identicon: bob().to_vec(),
                    },
                },
            },
        ]),
        extensions: Some(vec![
            TransactionCard {
                index: 24,
                indent: 0,
                card: Card::EraMortalCard {
                    f: MSCEraMortal {
                        era: "Mortal".to_string(),
                        phase: "31".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 25,
                indent: 0,
                card: Card::NonceCard {
                    f: "45".to_string(),
                },
            },
            TransactionCard {
                index: 26,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 27,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9010".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 28,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 29,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"
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
    let network_info_known = westend_spec();

    let action = produce_output(line, dbname);
    if let TransactionAction::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = action
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_3() {
    let dbname = "for_tests/parse_transaction_3";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001750365010f00c06e31d910013223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ea8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cde143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs1 = "2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e".to_string();

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
                        docs: docs1,
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
                        amount: "300.000000000000".to_string(),
                        units: "WND".to_string(),
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
                        phase: "55".to_string(),
                        period: "64".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 8,
                indent: 0,
                card: Card::NonceCard {
                    f: "89".to_string(),
                },
            },
            TransactionCard {
                index: 9,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "300.000000000000".to_string(),
                        units: "WND".to_string(),
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
                    f: "a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"
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
    let network_info_known = westend_spec();
    let output = produce_output(line, dbname);
    if let TransactionAction::Sign {
        content,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = output
    {
        assert_eq!(content, content_known);
        assert_eq!(author_info, author_info_known);
        assert!(!has_pwd, "Expected no password");
        assert_eq!(network_info, network_info_known);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070_not_signed() {
    let dbname = "for_tests/load_westend9070_not_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9070_alice_signed() {
    let dbname = "for_tests/load_westend9070_alice_signed";
    populate_cold_no_metadata(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.".to_string() },
        }]),
        ..Default::default()
    };
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: reply } = action {
        assert_eq!(reply, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_not_signed() {
    let dbname = "for_tests/load_westend9000_already_in_db_not_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_None.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Metadata for westend9000 is already in the database."
                    .to_string(),
            },
        }]),
        ..Default::default()
    };
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_alice_signed() {
    let dbname = "for_tests/load_westend9000_already_in_db_alice_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.".to_string()
            },
        }]),
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = output {
        assert_eq!(set, set_expected);
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
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Metadata for westend9000 is already in the database."
                    .to_string(),
            },
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_westend9000_already_in_db_alice_signed_bad_general_verifier() {
    let dbname = "for_tests/load_westend9000_already_in_db_alice_signed_bad_general_verifier";
    populate_cold(dbname, verifier_alice_ed25519()).unwrap();
    let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f:  "Bad input data. Network westend is verified by the general verifier which currently is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Received load_metadata message is verified by public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.".to_string() } 
        }]),
        ..Default::default()
    };
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn load_dock31_unknown_network() {
    let dbname = "for_tests/load_dock31_unknown_network";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/load_metadata_dock-pos-main-runtimeV31_unverified.txt")
            .unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. Network dock-pos-main-runtime is not in the database. Add network specs before loading the metadata.".to_string() },
        }]),
       ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = output {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", output)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_not_verified_db_not_verified() {
    let dbname = "for_tests/add_specs_dock_not_verified_db_not_verified";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_unverified.txt")
            .unwrap();
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_dock_alice_verified_db_not_verified() {
    let dbname = "for_tests/add_specs_dock_alice_verified_db_not_verified";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_dock-pos-main-runtime-sr25519_Alice-sr25519.txt")
            .unwrap();

    let set_expected = TransactionCardSet {
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
            card: Card::WarningCard { f: "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string() },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 2,
            indent: 0,
            card: Card::NewSpecsCard {
                f: NetworkSpecsToSend {
                    base58prefix: 22,
                    color: "#660D35".to_string(),
                    decimals: 6,
                    encryption: Encryption::Sr25519,
                    genesis_hash: H256::from_str("6bfe24dca2a3be10f22212678ac13a6446ec764103c0f3471c71609eac384aae").unwrap(),
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
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
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
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
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
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
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_signed() {
    let dbname = "for_tests/add_specs_westend_ed25519_not_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_bad_westend_ed25519_not_signed() {
    let dbname = "for_tests/add_specs_bad_westend_ed25519_not_signed";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line =
        fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified_bad_ones.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. Network westend with genesis hash e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e already has entries in the database with base58 prefix 42. Received network specs have same genesis hash and different base58 prefix 115.".to_string()
            },
        }]),
        ..Default::default()
    };
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_alice_signed_db_not_verified() {
    let dbname = "for_tests/add_specs_westend_ed25519_alice_signed_db_not_verified";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
    let warning_str = "Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. Affected network specs entries: Kusama, Polkadot, Westend; affected metadata entries: kusama2030, polkadot30, westend9000, westend9010. Types information is purged.".to_string();

    let set_expected = TransactionCardSet {
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
            card: Card::WarningCard { f: warning_str },
        }]),
        new_specs: Some(vec![TransactionCard {
            index: 2,
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_not_verified_db_alice_verified() {
    let dbname = "for_tests/add_specs_westend_ed25519_not_verified_db_alice_verified";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_unverified.txt").unwrap();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier.".to_string() },
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_both_verified_same() {
    let dbname = "for_tests/add_specs_westend_ed25519_both_verified_same";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-sr25519.txt").unwrap();
    let set_expected = TransactionCardSet {
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(set, set_expected);
        assert_eq!(stub, stub_nav_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn add_specs_westend_ed25519_both_verified_different() {
    let dbname = "for_tests/add_specs_westend_ed25519_both_verified_different";
    populate_cold(dbname, verifier_alice_sr25519()).unwrap();
    let line = fs::read_to_string("for_tests/add_specs_westend-ed25519_Alice-ed25519.txt").unwrap();
    let error = "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received network westend specs could be accepted only if verified by the same general verifier. Current message is verified by public key: 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee, encryption: ed25519.".to_string();
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        ..Default::default()
    };
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_4_unknown_author() {
    let dbname = "for_tests/parse_transaction_4_unknown_author";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "5301008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48a4040300d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let docs = "2053616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a206f726967696e206163636f756e742e0a0a20393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a205b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e736665720a2023203c7765696768743e0a202d2043686561706572207468616e207472616e736665722062656361757365206163636f756e742063616e6e6f74206265206b696c6c65642e0a202d2042617365205765696768743a2035312e3420c2b5730a202d204442205765696768743a2031205265616420616e64203120577269746520746f2064657374202873656e64657220697320696e206f7665726c617920616c7265616479290a20233c2f7765696768743e".to_string();

    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorPlainCard {
                f: MSCAuthorPlain {
                    base58: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                    identicon: bob().to_vec(),
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
                        method_name: "transfer_keep_alive".to_string(),
                        docs,
                    },
                },
            },
            TransactionCard {
                index: 4,
                indent: 2,
                card: Card::VarNameCard {
                    f: "dest".to_string(),
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
                        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                        identicon: alice_sr_alice().to_vec(),
                    },
                },
            },
            TransactionCard {
                index: 7,
                indent: 2,
                card: Card::VarNameCard {
                    f: "value".to_string(),
                },
            },
            TransactionCard {
                index: 8,
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
                index: 9,
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
                index: 10,
                indent: 0,
                card: Card::NonceCard {
                    f: "46".to_string(),
                },
            },
            TransactionCard {
                index: 11,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 12,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9010".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 13,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 14,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_5_unknown_network() {
    let dbname = "for_tests/parse_transaction_5_unknown_network";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530102761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62a8030300761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c620b00407a10f35aa707000b00a0724e1809140000000a000000f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769badc21d36b69bae1e8a41dedb34758567ba4efe711412f33d1461f795ffcd1de13f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba";
    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorPublicKeyCard {
                f: MVerifierDetails {
                    public_key: "761291ee5faf5b5b67b028aa7e28fb1271bf40af17a486b368e8c7de86ad3c62"
                        .to_string(),
                    identicon: id_03().to_vec(),
                    encryption: "sr25519".to_string(),
                },
            },
        }]),
        error: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::ErrorCard { f: "Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash f7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba and encryption sr25519.".to_string() },
        }]),
        ..Default::default()
    };
    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_6_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_6_error_on_parsing";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403018eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let error = "Error parsing incoming transaction content. After decoding the method some data remained unused.".to_string();
    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: Address {
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    identicon: alice_sr_alice().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//Alice".to_string(),
                    has_pwd: false,
                    multiselect: None,
                },
            },
        }]),
        error: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        extensions: Some(vec![
            TransactionCard {
                index: 2,
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
                index: 3,
                indent: 0,
                card: Card::NonceCard {
                    f: "46".to_string(),
                },
            },
            TransactionCard {
                index: 4,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9010".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 7,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_7_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_7_error_on_parsing";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403068eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let error = "Error parsing incoming transaction content. Encountered unexpected enum variant."
        .to_string();
    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: Address {
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    identicon: alice_sr_alice().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//Alice".to_string(),
                    has_pwd: false,
                    multiselect: None,
                },
            },
        }]),
        error: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::ErrorCard { f: error },
        }]),
        extensions: Some(vec![
            TransactionCard {
                index: 2,
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
                index: 3,
                indent: 0,
                card: Card::NonceCard {
                    f: "46".to_string(),
                },
            },
            TransactionCard {
                index: 4,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9010".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 7,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_transaction_8_error_on_parsing() {
    let dbname = "for_tests/parse_transaction_8_error_on_parsing";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403028eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let data = "Error parsing incoming transaction content. Data too short for expected content."
        .to_string();

    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: Address {
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    identicon: alice_sr_alice().to_vec(),
                    seed_name: "Alice".to_string(),
                    path: "//Alice".to_string(),
                    has_pwd: false,
                    multiselect: None,
                },
            },
        }]),
        error: Some(vec![TransactionCard {
            index: 1,
            indent: 0,
            card: Card::ErrorCard { f: data },
        }]),
        extensions: Some(vec![
            TransactionCard {
                index: 2,
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
                index: 3,
                indent: 0,
                card: Card::NonceCard {
                    f: "46".to_string(),
                },
            },
            TransactionCard {
                index: 4,
                indent: 0,
                card: Card::TipCard {
                    f: MSCCurrency {
                        amount: "0".to_string(),
                        units: "pWND".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 5,
                indent: 0,
                card: Card::NameVersionCard {
                    f: MSCNameVersion {
                        name: "westend".to_string(),
                        version: "9010".to_string(),
                    },
                },
            },
            TransactionCard {
                index: 6,
                indent: 0,
                card: Card::TxSpecCard { f: "5".to_string() },
            },
            TransactionCard {
                index: 7,
                indent: 0,
                card: Card::BlockHashCard {
                    f: "538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"
                        .to_string(),
                },
            },
        ]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_1() {
    let dbname = "for_tests/parse_msg_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d3c42797465733e4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e3c2f42797465733ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";

    let text = "4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e".to_string();

    let set_expected = TransactionCardSet {
        message: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::TextCard { f: text },
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

    let network_info_known = westend_spec();
    let action = produce_output(line, dbname);

    if let TransactionAction::Sign {
        content: set,
        checksum: _,
        has_pwd,
        author_info,
        network_info,
    } = action
    {
        assert_eq!(set, set_expected);
        assert_eq!(author_info, author_info_known);
        assert_eq!(network_info, network_info_known);
        assert!(!has_pwd, "Expected no password");
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_2() {
    let dbname = "for_tests/parse_msg_2";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    // message scale encoded
    let line = "530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27df5064c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2ee143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Received message has no `<Bytes></Bytes>` wrapper.".to_string(),
            },
        }]),
        ..Default::default()
    };
    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn import_derivations() {
    let dbname = "for_tests/import_derivations";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "53ffde01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e141c2f2f416c6963653c2f2f416c6963652f77657374656e64582f2f416c6963652f7365637265742f2f7365637265740c2f2f300c2f2f31";
    let set_expected = TransactionCardSet {
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
    };

    let network_info_known = westend_spec();
    let action = produce_output(line, dbname);
    if let TransactionAction::Derivations {
        content: set,
        network_info,
        checksum: _,
        network_specs_key: _,
    } = action
    {
        assert_eq!(set, set_expected);
        assert_eq!(network_info, network_info_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn import_derivations_some_passworded() {
    let dbname = "for_tests/import_derivations_some_passworded";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "53ffde01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1c1c2f2f416c6963653c2f2f416c6963652f77657374656e64582f2f416c6963652f7365637265742f2f7365637265740c2f2f300c2f2f31702f2f416c6963652f2f2f6d795f7365637265745f70617373776f7264182f2f2f303030";
    let set_expected = TransactionCardSet {
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
                    "//Alice///my_secret_password".to_string(),
                    "///000".to_string(),
                ],
            },
        }]),
        ..Default::default()
    };

    let network_info_known = westend_spec();
    let action = produce_output(line, dbname);
    if let TransactionAction::Derivations {
        content: set,
        network_info,
        checksum: _,
        network_specs_key: _,
    } = action
    {
        assert_eq!(set, set_expected);
        assert_eq!(network_info, network_info_known);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}
