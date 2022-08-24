use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{get_general_verifier, try_get_types},
};
use definitions::{
    error::TransferContent, error_signer::GeneralVerifierForContent, history::Event,
    navigation::TransactionCardSet, network_specs::Verifier, qr_transfers::ContentLoadTypes,
    types::TypeEntry,
};
use std::path::Path;

use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, Result};
use crate::holds::GeneralHold;
use crate::{StubNav, TransactionAction};

pub fn load_types<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let checked_info = pass_crypto(data_hex, TransferContent::LoadTypes)?;
    let content_new_types = ContentLoadTypes::from_slice(&checked_info.message);
    let new_types = content_new_types.types()?;
    let old_types: Vec<TypeEntry> = try_get_types(&db_path)?.unwrap_or_default();
    let general_verifier = get_general_verifier(&db_path)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    match checked_info.verifier {
        Verifier { v: None } => match general_verifier {
            Verifier { v: None } => {
                if new_types == old_types {
                    Err(Error::TypesKnown)
                } else {
                    stub = stub.new_history_entry(Event::Warning {
                        warning: Warning::TypesNotVerified.show(),
                    });
                    stub = stub.new_history_entry(Event::Warning {
                        warning: Warning::UpdatingTypes.show(),
                    });
                    stub = stub.add_types(&content_new_types, &checked_info.verifier);
                    let checksum = stub.store_and_get_checksum(&db_path)?;
                    let warning_card_1 =
                        Card::Warning(Warning::TypesNotVerified).card(&mut index, 0);
                    let warning_card_2 = Card::Warning(Warning::UpdatingTypes).card(&mut index, 0);
                    let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                    Ok(TransactionAction::Stub {
                        s: TransactionCardSet {
                            warning: Some(vec![warning_card_1, warning_card_2]),
                            types_info: Some(vec![types_card]),
                            ..Default::default()
                        },
                        u: checksum,
                        stub: StubNav::LoadTypes,
                    })
                }
            }
            Verifier {
                v: Some(old_general_verifier_value),
            } => Err(Error::NeedGeneralVerifier {
                content: GeneralVerifierForContent::Types,
                verifier_value: old_general_verifier_value,
            }),
        },
        Verifier {
            v: Some(ref new_general_verifier_value),
        } => {
            let verifier_card = Card::Verifier(new_general_verifier_value).card(&mut index, 0);
            if general_verifier == checked_info.verifier {
                if new_types == old_types {
                    Err(Error::TypesKnown)
                } else {
                    stub = stub.new_history_entry(Event::Warning {
                        warning: Warning::UpdatingTypes.show(),
                    });
                    stub = stub.add_types(&content_new_types, &checked_info.verifier);
                    let checksum = stub.store_and_get_checksum(&db_path)?;
                    let warning_card = Card::Warning(Warning::UpdatingTypes).card(&mut index, 0);
                    let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                    Ok(TransactionAction::Stub {
                        s: TransactionCardSet {
                            verifier: Some(vec![verifier_card]),
                            warning: Some(vec![warning_card]),
                            types_info: Some(vec![types_card]),
                            ..Default::default()
                        },
                        u: checksum,
                        stub: StubNav::LoadTypes,
                    })
                }
            } else {
                match general_verifier {
                    Verifier { v: None } => {
                        let new_general_verifier = checked_info.verifier;
                        let general_hold = GeneralHold::get(&db_path)?;
                        stub = general_hold.upd_stub(stub, &new_general_verifier, &db_path)?;
                        stub = stub.add_types(&content_new_types, &new_general_verifier);
                        let warning_card_1 =
                            Card::Warning(Warning::GeneralVerifierAppeared(&general_hold))
                                .card(&mut index, 0);
                        let warning_card_2 = {
                            if new_types == old_types {
                                stub = stub.new_history_entry(Event::Warning {
                                    warning: Warning::TypesAlreadyThere.show(),
                                });
                                Card::Warning(Warning::TypesAlreadyThere).card(&mut index, 0)
                            } else {
                                stub = stub.new_history_entry(Event::Warning {
                                    warning: Warning::UpdatingTypes.show(),
                                });
                                Card::Warning(Warning::UpdatingTypes).card(&mut index, 0)
                            }
                        };
                        let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&db_path)?;
                        Ok(TransactionAction::Stub {
                            s: TransactionCardSet {
                                verifier: Some(vec![verifier_card]),
                                warning: Some(vec![warning_card_1, warning_card_2]),
                                types_info: Some(vec![types_card]),
                                ..Default::default()
                            },
                            u: checksum,
                            stub: StubNav::LoadTypes,
                        })
                    }
                    Verifier {
                        v: Some(old_general_verifier_value),
                    } => Err(Error::GeneralVerifierChanged {
                        content: GeneralVerifierForContent::Types,
                        old_general_verifier_value,
                        new_general_verifier_value: new_general_verifier_value.to_owned(),
                    }),
                }
            }
        }
    }
}
