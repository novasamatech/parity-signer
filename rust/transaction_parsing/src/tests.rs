use crate::{produce_output, StubNav};
use constants::test_values::{
    alice_sr_alice, alice_sr_westend_0, bob, ed, id_01, id_02, id_03, types_known, types_unknown,
    westend_9070,
};

use db_handling::{
    cold_default::{populate_cold, populate_cold_no_metadata, populate_cold_no_networks},
    manage_history::get_history,
};
use definitions::derivations::{DerivedKeyPreview, SeedKeysPreview};
use definitions::navigation::{MAddressCard, SignerImage, TransactionSignAction};
use definitions::{
    crypto::Encryption,
    history::{Entry, Event},
    keyring::NetworkSpecsKey,
    navigation::{
        Address, Card, MMetadataRecord, MSCCall, MSCCurrency, MSCEnumVariantName, MSCEraMortal,
        MSCId, MSCNameVersion, MTypesInfo, MVerifierDetails, NetworkSpecs, TransactionAction,
        TransactionCard, TransactionCardSet,
    },
    network_specs::{OrderedNetworkSpecs, Verifier, VerifierValue},
};

use pretty_assertions::assert_eq;
use sp_core::sr25519::Public;
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::convert::TryFrom;
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

fn westend_spec() -> OrderedNetworkSpecs {
    OrderedNetworkSpecs {
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
        assert_eq!(*s, card_set_known);
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
        r: Box::new(TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard { f:  "Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database.".to_string()},
            }]),
            ..Default::default()
        })
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
        r: Box::new(TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard { f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier.".to_string()},
            }]),
            ..Default::default()
        })
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
        r: Box::new(TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard {
                    f: "Bad input data. Exactly same types information is already in the database."
                        .to_string(),
                },
            }]),
            ..Default::default()
        }),
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
        r: Box::new(TransactionCardSet {
            error: Some(vec![TransactionCard {
                index: 0,
                indent: 0,
                card: Card::ErrorCard {
                    f: "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier.".to_string(),
                },
            }]),
            ..Default::default()
        }),
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
                types_id_pic: Some(SignerImage::Png {
                    image: types_known().to_vec(),
                }),
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
                    identicon: SignerImage::Png {
                        image: alice_sr_alice().to_vec(),
                    },
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
        assert_eq!(*reply, reply_known);
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
        assert_eq!(*reply, reply_known);
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
        assert_eq!(*reply, action_expected);
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
                    identicon: SignerImage::Png{ image: alice_sr_alice().to_vec() },
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
                types_id_pic: Some(SignerImage::Png{ image: types_known().to_vec() }),
            }
            }
        }]),
        ..Default::default()
    };

    let output = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = output {
        assert_eq!(*set, set_expected);
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
                    types_id_pic: Some(SignerImage::Png {
                        image: types_unknown().to_vec(),
                    }),
                },
            },
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(stub, StubNav::LoadTypes);
        assert_eq!(*set, expected_set);
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
                    identicon: SignerImage::Png{ image: alice_sr_alice().to_vec() },
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
                    types_id_pic: Some(SignerImage::Png{ image: types_unknown().to_vec() }),
                }
            }
        }]),
        ..Default::default()
    };

    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, expected_set);
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
            card: Card::ErrorCard { f: "Bad input data. Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (50) differs from the version in metadata (9000).".to_string() },
        }]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(*set, expected_set);
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
                        docs: " Same as the [`transfer`] call, but with a check that the transfer will not kill the\n origin account.\n\n 99% of the time you want [`transfer`] instead.\n\n [`transfer`]: struct.Pallet.html#method.transfer\n # <weight>\n - Cheaper than transfer because account cannot be killed.\n - Base Weight: 51.4 µs\n - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n #</weight>".to_string(),
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
                        identicon: SignerImage::Png { image: bob().to_vec() },
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
        address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            .to_string(),
        address: Address {
            identicon: SignerImage::Png {
                image: alice_sr_alice().to_vec(),
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
    let output = produce_output(line, dbname);
    if let TransactionAction::Sign { actions, .. } = output {
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
        assert_eq!(*has_pwd, false)
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
    let docs1 = " Send a batch of dispatch calls and atomically execute them.\n The whole transaction will rollback and fail if any of the calls failed.\n\n May be called from any origin.\n\n - `calls`: The calls to be dispatched from the same origin.\n\n If origin is root then call are dispatch without checking origin filter. (This includes\n bypassing `frame_system::Config::BaseCallFilter`).\n\n # <weight>\n - Complexity: O(C) where C is the number of calls to be batched.\n # </weight>".to_string();

    let docs2 = " Take the origin account as a stash and lock up `value` of its balance. `controller` will\n be the account that controls it.\n\n `value` must be more than the `minimum_balance` specified by `T::Currency`.\n\n The dispatch origin for this call must be _Signed_ by the stash account.\n\n Emits `Bonded`.\n\n # <weight>\n - Independent of the arguments. Moderate complexity.\n - O(1).\n - Three extra DB entries.\n\n NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned\n unless the `origin` falls below _existential deposit_ and gets removed as dust.\n ------------------\n Weight: O(1)\n DB Weight:\n - Read: Bonded, Ledger, [Origin Account], Current Era, History Depth, Locks\n - Write: Bonded, Payee, [Origin Account], Locks, Ledger\n # </weight>".to_string();

    let docs3 = " Declare the desire to nominate `targets` for the origin controller.\n\n Effects will be felt at the beginning of the next era. This can only be called when\n [`EraElectionStatus`] is `Closed`.\n\n The dispatch origin for this call must be _Signed_ by the controller, not the stash.\n And, it can be only called when [`EraElectionStatus`] is `Closed`.\n\n # <weight>\n - The transaction's complexity is proportional to the size of `targets` (N)\n which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS).\n - Both the reads and writes follow a similar pattern.\n ---------\n Weight: O(N)\n where N is the number of targets\n DB Weight:\n - Reads: Era Election Status, Ledger, Current Era\n - Writes: Validators, Nominators\n # </weight>".to_string();

    let docs4 = " (Re-)set the controller of a stash.\n\n Effects will be felt at the beginning of the next era.\n\n The dispatch origin for this call must be _Signed_ by the stash, not the controller.\n\n # <weight>\n - Independent of the arguments. Insignificant complexity.\n - Contains a limited number of reads.\n - Writes are limited to the `origin` account key.\n ----------\n Weight: O(1)\n DB Weight:\n - Read: Bonded, Ledger New Controller, Ledger Old Controller\n - Write: Bonded, Ledger New Controller, Ledger Old Controller\n # </weight>".to_string();

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
                        identicon: SignerImage::Png {
                            image: alice_sr_alice().to_vec(),
                        },
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
                        identicon: SignerImage::Png {
                            image: id_01().to_vec(),
                        },
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
                        identicon: SignerImage::Png {
                            image: id_02().to_vec(),
                        },
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
                        identicon: SignerImage::Png {
                            image: bob().to_vec(),
                        },
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

    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            .to_string(),
        address: Address {
            identicon: SignerImage::Png {
                image: alice_sr_alice().to_vec(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };
    let network_info_known = westend_spec();

    let action = produce_output(line, dbname);
    if let TransactionAction::Sign { actions, .. } = action {
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

    let docs1 = " Same as the [`transfer`] call, but with a check that the transfer will not kill the\n origin account.\n\n 99% of the time you want [`transfer`] instead.\n\n [`transfer`]: struct.Pallet.html#method.transfer\n # <weight>\n - Cheaper than transfer because account cannot be killed.\n - Base Weight: 51.4 µs\n - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n #</weight>".to_string();

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
                        identicon: SignerImage::Png {
                            image: bob().to_vec(),
                        },
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

    let author_info_known = MAddressCard {
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            .to_string(),
        address: Address {
            identicon: SignerImage::Png {
                image: alice_sr_alice().to_vec(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };
    let network_info_known = westend_spec();
    let output = produce_output(line, dbname);
    if let TransactionAction::Sign { actions, .. } = output {
        let TransactionSignAction {
            content,
            has_pwd,
            author_info,
            network_info,
        } = &actions[0];

        assert_eq!(actions.len(), 1);
        assert_eq!(content, &content_known);
        assert_eq!(author_info, &author_info_known);
        assert!(!has_pwd, "Expected no password");
        assert_eq!(network_info, &network_info_known);
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
                    meta_id_pic: SignerImage::Png {
                        image: westend_9070().to_vec(),
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
        assert_eq!(*reply, set_expected);
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
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png{ image: alice_sr_alice().to_vec() },
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
                f: NetworkSpecs {
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
        assert_eq!(*set, set_expected);
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png {
                        image: alice_sr_alice().to_vec(),
                    },
                    encryption: "sr25519".to_string(),
                },
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png {
                        image: ed().to_vec(),
                    },
                    encryption: "ed25519".to_string(),
                },
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png {
                        image: alice_sr_alice().to_vec(),
                    },
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png {
                        image: alice_sr_alice().to_vec(),
                    },
                    encryption: "sr25519".to_string(),
                },
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
    let action = produce_output(line.trim(), dbname);
    if let TransactionAction::Stub { s: set, u: _, stub } = action {
        assert_eq!(*set, set_expected);
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
    let error = "Bad input data. General verifier in the database is public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: sr25519. Received westend network information could be accepted only if verified by the same general verifier. Current message is verified by public key: 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee, encryption: ed25519.".to_string();
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
        assert_eq!(*set, set_expected);
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

    let docs = " Same as the [`transfer`] call, but with a check that the transfer will not kill the\n origin account.\n\n 99% of the time you want [`transfer`] instead.\n\n [`transfer`]: struct.Pallet.html#method.transfer\n # <weight>\n - Cheaper than transfer because account cannot be killed.\n - Base Weight: 51.4 µs\n - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)\n #</weight>".to_string();

    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorPlainCard {
                f: MSCId {
                    base58: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                    identicon: SignerImage::Png {
                        image: bob().to_vec(),
                    },
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
                        identicon: SignerImage::Png {
                            image: alice_sr_alice().to_vec(),
                        },
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
        assert_eq!(*set, set_expected);
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
                    identicon: SignerImage::Png{ image: id_03().to_vec() } ,
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
        assert_eq!(*set, set_expected);
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

    let error = "Bad input data. Error parsing incoming transaction content. After decoding the method some data remained unused.".to_string();
    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: MAddressCard {
                    address_key:
                        "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                            .to_string(),
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    address: Address {
                        identicon: SignerImage::Png {
                            image: alice_sr_alice().to_vec(),
                        },
                        seed_name: "Alice".to_string(),
                        path: "//Alice".to_string(),
                        has_pwd: false,
                        secret_exposed: false,
                    },
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
        assert_eq!(*set, set_expected);
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

    let error = "Bad input data. Error parsing incoming transaction content. Encountered unexpected enum variant."
        .to_string();
    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: MAddressCard {
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    address_key:
                        "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                            .to_string(),
                    address: Address {
                        identicon: SignerImage::Png {
                            image: alice_sr_alice().to_vec(),
                        },
                        seed_name: "Alice".to_string(),
                        path: "//Alice".to_string(),
                        has_pwd: false,
                        secret_exposed: false,
                    },
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
        assert_eq!(*set, set_expected);
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

    let data = "Bad input data. Error parsing incoming transaction content. Data too short for expected content."
        .to_string();

    let set_expected = TransactionCardSet {
        author: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::AuthorCard {
                f: MAddressCard {
                    base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    address_key:
                        "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                            .to_string(),
                    address: Address {
                        identicon: SignerImage::Png {
                            image: alice_sr_alice().to_vec(),
                        },
                        seed_name: "Alice".to_string(),
                        path: "//Alice".to_string(),
                        has_pwd: false,
                        secret_exposed: false,
                    },
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
        assert_eq!(*set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_1() {
    let dbname = "for_tests/parse_msg_1";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let sign_msg = hex::encode(b"<Bytes>uuid-abcd</Bytes>");
    let text = String::from("uuid-abcd");
    let line = format!("530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d{}e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", sign_msg);

    let set_expected = TransactionCardSet {
        message: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::TextCard { f: text },
        }]),
        ..Default::default()
    };

    let author_info_known = MAddressCard {
        address_key: "01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            .to_string(),
        base58: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        address: Address {
            identicon: SignerImage::Png {
                image: alice_sr_alice().to_vec(),
            },
            seed_name: "Alice".to_string(),
            path: "//Alice".to_string(),
            has_pwd: false,
            secret_exposed: false,
        },
    };

    let network_info_known = westend_spec();
    let action = produce_output(&line, dbname);

    if let TransactionAction::Sign { actions, .. } = action {
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
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn parse_msg_2() {
    let dbname = "for_tests/parse_msg_2";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    // sneaking one extra byte in the text body
    let sign_msg = hex::encode(b"<Bytes>uuid-abcd");
    let line = format!("530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d{}e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", sign_msg);
    let set_expected = TransactionCardSet {
        error: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::ErrorCard {
                f: "Bad input data. Parser error: Error(Error { input: \"uuid-abcd\", code: TakeUntil })".to_string(),
            },
        }]),
        ..Default::default()
    };
    let action = produce_output(&line, dbname);
    if let TransactionAction::Read { r: set } = action {
        assert_eq!(*set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}

#[test]
fn import_derivations() {
    let dbname = "for_tests/import_derivations";
    populate_cold(dbname, Verifier { v: None }).unwrap();
    let line = "53ffde00041c6d7920736565640146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a04c0354847694263466745424d6754364745756f395341393873426e4767774874504b44586955756b5436617143724b457801302f2f77657374656e642f2f3001e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
    let set_expected = TransactionCardSet {
        importing_derivations: Some(vec![TransactionCard {
            index: 0,
            indent: 0,
            card: Card::DerivationsCard {
                f: vec![SeedKeysPreview {
                    name: "my seed".to_string(),
                    multisigner: MultiSigner::Sr25519(
                        Public::try_from(
                            hex::decode(
                                "46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a",
                            )
                            .unwrap()
                            .as_ref(),
                        )
                        .unwrap(),
                    ),
                    derived_keys: vec![DerivedKeyPreview {
                        address: "5HGiBcFgEBMgT6GEuo9SA98sBnGgwHtPKDXiUukT6aqCrKEx".to_string(),
                        derivation_path: Some("//westend//0".to_string()),
                        encryption: Encryption::Sr25519,
                        genesis_hash:
                            "0xe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
                                .parse()
                                .unwrap(),
                        identicon: SignerImage::Png {
                            image: alice_sr_westend_0().to_vec(),
                        },
                        has_pwd: None,
                        network_title: Some("Westend".to_string()),
                    }],
                }],
            },
        }]),
        ..Default::default()
    };

    let action = produce_output(line, dbname);
    if let TransactionAction::Derivations { content: set } = action {
        assert_eq!(*set, set_expected);
    } else {
        panic!("Wrong action {:?}", action)
    }
    fs::remove_dir_all(dbname).unwrap();
}
