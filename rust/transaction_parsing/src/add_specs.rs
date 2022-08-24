use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{
        genesis_hash_in_specs, get_general_verifier, open_db, try_get_valid_current_verifier,
    },
};
use definitions::{
    error::TransferContent,
    error_signer::GeneralVerifierForContent,
    history::Event,
    keyring::{NetworkSpecsKey, VerifierKey},
    navigation::TransactionCardSet,
    network_specs::{ValidCurrentVerifier, Verifier},
    qr_transfers::ContentAddSpecs,
};
use std::path::Path;

use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, Result};
use crate::helpers::specs_are_new;
use crate::{StubNav, TransactionAction};

use crate::holds::{GeneralHold, Hold, HoldRelease};

pub fn add_specs<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let checked_info = pass_crypto(data_hex, TransferContent::AddSpecs)?;
    let specs = ContentAddSpecs::from_slice(&checked_info.message).specs()?;
    let network_specs_key = NetworkSpecsKey::from_parts(&specs.genesis_hash, &specs.encryption);
    let verifier_key = VerifierKey::from_parts(specs.genesis_hash);
    let possible_valid_current_verifier = try_get_valid_current_verifier(&verifier_key, &db_path)?;
    let general_verifier = get_general_verifier(&db_path)?;
    if let Some(specs_invariants) = genesis_hash_in_specs(specs.genesis_hash, &open_db(&db_path)?)?
    {
        if specs.name != specs_invariants.name {
            return Err(Error::AddSpecsDifferentName {
                genesis_hash: specs_invariants.genesis_hash,
                name_database: specs_invariants.name,
                name_input: specs.name,
            });
        }
        if specs.base58prefix != specs_invariants.base58prefix {
            return Err(Error::AddSpecsDifferentBase58 {
                genesis_hash: specs_invariants.genesis_hash,
                name: specs_invariants.name,
                base58_database: specs_invariants.base58prefix,
                base58_input: specs.base58prefix,
            });
        }
    }
    let mut stub = TrDbColdStub::new();
    let mut index = 0;

    match possible_valid_current_verifier {
        None => match checked_info.verifier {
            Verifier { v: None } => {
                stub = stub.new_history_entry(Event::Warning {
                    warning: Warning::NotVerified.show(),
                });
                stub = stub.add_network_specs(
                    &specs,
                    &ValidCurrentVerifier::Custom {
                        v: Verifier { v: None },
                    },
                    &general_verifier,
                    &db_path,
                )?;
                stub = stub.new_network_verifier(
                    &verifier_key,
                    &ValidCurrentVerifier::Custom {
                        v: Verifier { v: None },
                    },
                    &general_verifier,
                );
                let checksum = stub.store_and_get_checksum(&db_path)?;
                let warning_card = Card::Warning(Warning::NotVerified).card(&mut index, 0);
                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                Ok(TransactionAction::Stub {
                    s: TransactionCardSet {
                        warning: Some(vec![warning_card]),
                        new_specs: Some(vec![specs_card]),
                        ..Default::default()
                    },
                    u: checksum,
                    stub: StubNav::AddSpecs {
                        n: network_specs_key,
                    },
                })
            }
            Verifier {
                v: Some(ref new_verifier_value),
            } => {
                let verifier_card = Card::Verifier(new_verifier_value).card(&mut index, 0);
                match general_verifier {
                    Verifier { v: None } => {
                        let new_general_verifier = checked_info.verifier;
                        let general_hold = GeneralHold::get(&db_path)?;
                        stub = general_hold.upd_stub(stub, &new_general_verifier, &db_path)?;
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &new_general_verifier,
                            &db_path,
                        )?;
                        stub = stub.new_network_verifier(
                            &verifier_key,
                            &ValidCurrentVerifier::General,
                            &new_general_verifier,
                        );
                        let warning_card =
                            Card::Warning(Warning::GeneralVerifierAppeared(&general_hold))
                                .card(&mut index, 0);
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(db_path)?;
                        Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                verifier: Some(vec![verifier_card]),
                                warning: Some(vec![warning_card]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        })
                    }
                    _ => {
                        if checked_info.verifier == general_verifier {
                            stub = stub.add_network_specs(
                                &specs,
                                &ValidCurrentVerifier::General,
                                &general_verifier,
                                &db_path,
                            )?;
                            stub = stub.new_network_verifier(
                                &verifier_key,
                                &ValidCurrentVerifier::General,
                                &general_verifier,
                            );
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub.store_and_get_checksum(&db_path)?;
                            Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            })
                        } else {
                            stub = stub.add_network_specs(
                                &specs,
                                &ValidCurrentVerifier::Custom {
                                    v: checked_info.verifier.to_owned(),
                                },
                                &general_verifier,
                                &db_path,
                            )?;
                            stub = stub.new_network_verifier(
                                &verifier_key,
                                &ValidCurrentVerifier::Custom {
                                    v: checked_info.verifier,
                                },
                                &general_verifier,
                            );
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub.store_and_get_checksum(&db_path)?;
                            Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            })
                        }
                    }
                }
            }
        },
        Some(ValidCurrentVerifier::Custom { v: custom_verifier }) => match custom_verifier {
            Verifier { v: None } => match checked_info.verifier {
                Verifier { v: None } => {
                    stub = stub.new_history_entry(Event::Warning {
                        warning: Warning::NotVerified.show(),
                    });
                    let warning_card = Card::Warning(Warning::NotVerified).card(&mut index, 0);
                    if specs_are_new(&specs, &db_path)? {
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::Custom {
                                v: Verifier { v: None },
                            },
                            &general_verifier,
                            &db_path,
                        )?;
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                warning: Some(vec![warning_card]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        })
                    } else {
                        Err(Error::SpecsKnown {
                            name: specs.name,
                            encryption: specs.encryption,
                        })
                    }
                }
                Verifier {
                    v: Some(ref new_verifier_value),
                } => {
                    let verifier_card = Card::Verifier(new_verifier_value).card(&mut index, 0);
                    let hold = Hold::get(&verifier_key, &db_path)?;
                    if checked_info.verifier == general_verifier {
                        stub = hold.upd_stub(
                            stub,
                            &verifier_key,
                            &custom_verifier,
                            &ValidCurrentVerifier::General,
                            HoldRelease::General,
                            &db_path,
                        )?;
                        let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral {
                            verifier_key: &verifier_key,
                            hold: &hold,
                        })
                        .card(&mut index, 0);
                        let mut possible_warning = None;
                        if !specs_are_new(&specs, &db_path)? {
                            possible_warning = Some(
                                Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title))
                                    .card(&mut index, 0),
                            );
                            stub = stub.new_history_entry(Event::Warning {
                                warning: Warning::NetworkSpecsAlreadyThere(&specs.title).show(),
                            });
                        };
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        match possible_warning {
                            None => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                            Some(warning_card_2) => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1, warning_card_2]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                        }
                    } else if general_verifier.v.is_none() {
                        let new_general_verifier = checked_info.verifier;
                        stub = hold.upd_stub(
                            stub,
                            &verifier_key,
                            &custom_verifier,
                            &ValidCurrentVerifier::General,
                            HoldRelease::GeneralSuper,
                            &db_path,
                        )?;
                        let warning_card_1 = Card::Warning(Warning::VerifierGeneralSuper {
                            verifier_key: &verifier_key,
                            hold: &hold,
                        })
                        .card(&mut index, 0);
                        let general_hold = GeneralHold::get(&db_path)?;
                        stub = general_hold.upd_stub(stub, &new_general_verifier, &db_path)?;
                        let warning_card_2 =
                            Card::Warning(Warning::GeneralVerifierAppeared(&general_hold))
                                .card(&mut index, 0);
                        let mut possible_warning = None;
                        if !specs_are_new(&specs, &db_path)? {
                            possible_warning = Some(
                                Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title))
                                    .card(&mut index, 0),
                            );
                            stub = stub.new_history_entry(Event::Warning {
                                warning: Warning::NetworkSpecsAlreadyThere(&specs.title).show(),
                            });
                        };
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        match possible_warning {
                            None => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1, warning_card_2]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                            Some(warning_card_3) => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![
                                        warning_card_1,
                                        warning_card_2,
                                        warning_card_3,
                                    ]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                        }
                    } else {
                        stub = hold.upd_stub(
                            stub,
                            &verifier_key,
                            &custom_verifier,
                            &ValidCurrentVerifier::Custom {
                                v: checked_info.verifier.to_owned(),
                            },
                            HoldRelease::Custom,
                            &db_path,
                        )?;
                        let warning_card_1 = Card::Warning(Warning::VerifierChangingToCustom {
                            verifier_key: &verifier_key,
                            hold: &hold,
                        })
                        .card(&mut index, 0);
                        let mut possible_warning = None;
                        if !specs_are_new(&specs, &db_path)? {
                            possible_warning = Some(
                                Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title))
                                    .card(&mut index, 0),
                            );
                            stub = stub.new_history_entry(Event::Warning {
                                warning: Warning::NetworkSpecsAlreadyThere(&specs.title).show(),
                            });
                        };
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::Custom {
                                v: checked_info.verifier.to_owned(),
                            },
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        match possible_warning {
                            None => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                            Some(warning_card_2) => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1, warning_card_2]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                        }
                    }
                }
            },
            Verifier {
                v: Some(ref old_verifier_value),
            } => match checked_info.verifier {
                Verifier { v: None } => Err(Error::NeedVerifier {
                    name: specs.name,
                    verifier_value: old_verifier_value.to_owned(),
                }),
                Verifier {
                    v: Some(ref new_verifier_value),
                } => {
                    let verifier_card = Card::Verifier(new_verifier_value).card(&mut index, 0);
                    if checked_info.verifier == general_verifier {
                        let hold = Hold::get(&verifier_key, &db_path)?;
                        stub = hold.upd_stub(
                            stub,
                            &verifier_key,
                            &custom_verifier,
                            &ValidCurrentVerifier::General,
                            HoldRelease::General,
                            &db_path,
                        )?;
                        let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral {
                            verifier_key: &verifier_key,
                            hold: &hold,
                        })
                        .card(&mut index, 0);
                        let mut possible_warning = None;
                        if !specs_are_new(&specs, &db_path)? {
                            possible_warning = Some(
                                Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title))
                                    .card(&mut index, 0),
                            );
                            stub = stub.new_history_entry(Event::Warning {
                                warning: Warning::NetworkSpecsAlreadyThere(&specs.title).show(),
                            });
                        };
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        match possible_warning {
                            None => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                            Some(warning_card_2) => Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    warning: Some(vec![warning_card_1, warning_card_2]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            }),
                        }
                    } else if new_verifier_value == old_verifier_value {
                        if specs_are_new(&specs, &db_path)? {
                            stub = stub.add_network_specs(
                                &specs,
                                &ValidCurrentVerifier::Custom { v: custom_verifier },
                                &general_verifier,
                                &db_path,
                            )?;
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub.store_and_get_checksum(&db_path)?;
                            Ok(TransactionAction::Stub {
                                s: TransactionCardSet {
                                    verifier: Some(vec![verifier_card]),
                                    new_specs: Some(vec![specs_card]),
                                    ..Default::default()
                                },
                                u: checksum,
                                stub: StubNav::AddSpecs {
                                    n: network_specs_key,
                                },
                            })
                        } else {
                            Err(Error::SpecsKnown {
                                name: specs.name,
                                encryption: specs.encryption,
                            })
                        }
                    } else {
                        Err(Error::AddSpecsVerifierChanged {
                            name: specs.name,
                            old_verifier_value: old_verifier_value.to_owned(),
                            new_verifier_value: new_verifier_value.to_owned(),
                        })
                    }
                }
            },
        },
        Some(ValidCurrentVerifier::General) => match general_verifier {
            Verifier { v: None } => match checked_info.verifier {
                Verifier { v: None } => {
                    let warning_card = Card::Warning(Warning::NotVerified).card(&mut index, 0);
                    stub = stub.new_history_entry(Event::Warning {
                        warning: Warning::NotVerified.show(),
                    });
                    if specs_are_new(&specs, &db_path)? {
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                warning: Some(vec![warning_card]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        })
                    } else {
                        Err(Error::SpecsKnown {
                            name: specs.name,
                            encryption: specs.encryption,
                        })
                    }
                }
                Verifier {
                    v: Some(ref new_verifier_value),
                } => {
                    let verifier_card = Card::Verifier(new_verifier_value).card(&mut index, 0);
                    let new_general_verifier = checked_info.verifier;
                    let general_hold = GeneralHold::get(&db_path)?;
                    stub = general_hold.upd_stub(stub, &new_general_verifier, &db_path)?;
                    let warning_card_1 =
                        Card::Warning(Warning::GeneralVerifierAppeared(&general_hold))
                            .card(&mut index, 0);
                    let mut possible_warning = None;
                    if !specs_are_new(&specs, &db_path)? {
                        possible_warning = Some(
                            Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title))
                                .card(&mut index, 0),
                        );
                        stub = stub.new_history_entry(Event::Warning {
                            warning: Warning::NetworkSpecsAlreadyThere(&specs.title).show(),
                        });
                    };
                    stub = stub.add_network_specs(
                        &specs,
                        &ValidCurrentVerifier::General,
                        &new_general_verifier,
                        &db_path,
                    )?;
                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                    let checksum = stub.store_and_get_checksum(&db_path)?;
                    match possible_warning {
                        None => Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                verifier: Some(vec![verifier_card]),
                                warning: Some(vec![warning_card_1]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        }),
                        Some(warning_card_2) => Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                verifier: Some(vec![verifier_card]),
                                warning: Some(vec![warning_card_1, warning_card_2]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        }),
                    }
                }
            },
            Verifier {
                v: Some(ref old_general_verifier_value),
            } => {
                if checked_info.verifier == general_verifier {
                    if specs_are_new(&specs, &db_path)? {
                        let verifier_card =
                            Card::Verifier(old_general_verifier_value).card(&mut index, 0);
                        stub = stub.add_network_specs(
                            &specs,
                            &ValidCurrentVerifier::General,
                            &general_verifier,
                            &db_path,
                        )?;
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                verifier: Some(vec![verifier_card]),
                                new_specs: Some(vec![specs_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::AddSpecs {
                                n: network_specs_key,
                            },
                        })
                    } else {
                        Err(Error::SpecsKnown {
                            name: specs.name,
                            encryption: specs.encryption,
                        })
                    }
                } else {
                    match checked_info.verifier {
                        Verifier { v: None } => Err(Error::NeedGeneralVerifier {
                            content: GeneralVerifierForContent::Network { name: specs.name },
                            verifier_value: old_general_verifier_value.to_owned(),
                        }),
                        Verifier {
                            v: Some(new_general_verifier_value),
                        } => Err(Error::GeneralVerifierChanged {
                            content: GeneralVerifierForContent::Network { name: specs.name },
                            old_general_verifier_value: old_general_verifier_value.to_owned(),
                            new_general_verifier_value,
                        }),
                    }
                }
            }
        },
    }
}
