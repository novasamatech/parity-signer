use db_handling::db_transactions::TrDbColdStub;
use definitions::{history::Event, keyring::{VerifierKey}, network_specs::{CurrentVerifier, Verifier}, qr_transfers::ContentAddSpecs};

use crate::cards::{Action, Card, Warning};
use crate::error::{Error, BadInputData, DatabaseError, CryptoError};
use crate::check_signature::pass_crypto;
use crate::helpers::{GeneralHold, Hold, HoldRelease, get_current_verifier, get_general_verifier, specs_are_new, stub_add_network_specs, stub_store_and_get_checksum};


pub fn add_specs (data_hex: &str, database_name: &str) -> Result<String, Error> {
    let checked_info = pass_crypto(&data_hex)?;
    let specs = match ContentAddSpecs::from_vec(&checked_info.message).specs() {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeAddSpecsMessage)),
    };
    let verifier_key = VerifierKey::from_parts(&specs.genesis_hash.to_vec());
    let possible_current_verifier = get_current_verifier (&verifier_key, &database_name)?;
    let general_verifier = get_general_verifier(&database_name)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    
    match possible_current_verifier {
        None => {
            match checked_info.verifier {
                Verifier(None) => {
                    stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                    stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::Custom(Verifier(None)), &general_verifier, &database_name)?;
                    stub = stub.new_network_verifier(&verifier_key, &CurrentVerifier::Custom(Verifier(None)), &general_verifier);
                    let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                    let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                    let action_card = Action::Stub(checksum).card();
                    Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                },
                _ => {
                    let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
                    match general_verifier {
                        Verifier(None) => {
                            let new_general_verifier = checked_info.verifier;
                            let general_hold = GeneralHold::get(&database_name)?;
                            stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                            stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &new_general_verifier, &database_name)?;
                            stub = stub.new_network_verifier(&verifier_key, &CurrentVerifier::General, &new_general_verifier);
                            let warning_card = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                            let specs_card = Card::NewSpecs(&specs).card(&mut index,0);
                            let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card, specs_card, action_card))
                        },
                        _ => {
                            if checked_info.verifier == general_verifier {
                                stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                                stub = stub.new_network_verifier(&verifier_key, &CurrentVerifier::General, &general_verifier);
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                            }
                            else {
                                stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::Custom(checked_info.verifier.to_owned()), &general_verifier, &database_name)?;
                                stub = stub.new_network_verifier(&verifier_key, &CurrentVerifier::Custom(checked_info.verifier), &general_verifier);
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                            }
                        },
                    }
                },
            }
        },
        Some(CurrentVerifier::Custom(custom_verifier)) => {
            if (custom_verifier == general_verifier)&&(general_verifier != Verifier(None)) {return Err(Error::DatabaseError(DatabaseError::CustomVerifierIsGeneral(verifier_key)))}
            match custom_verifier {
                Verifier(None) => {
                    match checked_info.verifier {
                        Verifier(None) => {
                            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                            let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                            if specs_are_new(&specs, &database_name)? {
                                stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::Custom(Verifier(None)), &general_verifier, &database_name)?;
                                let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                            }
                            else {return Err(Error::BadInputData(BadInputData::SpecsAlreadyThere))} 
                        },
                        _ => {
                            let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
                            let hold = Hold::get(&verifier_key, &database_name)?;
                            if checked_info.verifier == general_verifier {
                                stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &CurrentVerifier::General, HoldRelease::General, &database_name)?;
                                let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                let mut possible_warning = None;
                                if !specs_are_new(&specs, &database_name)? {
                                    possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                    stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                };
                                stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                match possible_warning {
                                    None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                    Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                }
                            }
                            else {
                                if general_verifier == Verifier(None) {
                                    let new_general_verifier = checked_info.verifier;
                                    stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &CurrentVerifier::General, HoldRelease::GeneralSuper, &database_name)?;
                                    let warning_card_1 = Card::Warning(Warning::VerifierGeneralSuper{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                    let general_hold = GeneralHold::get(&database_name)?;
                                    stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                                    let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                                    let mut possible_warning = None;
                                    if !specs_are_new(&specs, &database_name)? {
                                        possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                        stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                    };
                                    stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                    let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                    let action_card = Action::Stub(checksum).card();
                                    match possible_warning {
                                        None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                        Some(warning_card_3) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, specs_card, action_card)),
                                    }
                                }
                                else {
                                    stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &CurrentVerifier::Custom(checked_info.verifier.to_owned()), HoldRelease::Custom, &database_name)?;
                                    let warning_card_1 = Card::Warning(Warning::VerifierChangingToCustom{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                    let mut possible_warning = None;
                                    if !specs_are_new(&specs, &database_name)? {
                                        possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                        stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                    };
                                    stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::Custom(checked_info.verifier.to_owned()), &general_verifier, &database_name)?;
                                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                    let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                    let action_card = Action::Stub(checksum).card();
                                    match possible_warning {
                                        None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                        Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                    }
                                }
                            }
                        },
                    }
                },
                _ => {
                    match checked_info.verifier {
                        Verifier(None) => return Err(Error::CryptoError(CryptoError::VerifierDisappeared)),
                        _ => {
                            let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
                            if checked_info.verifier == general_verifier {
                                let hold = Hold::get(&verifier_key, &database_name)?;
                                stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &CurrentVerifier::General, HoldRelease::General, &database_name)?;
                                let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                let mut possible_warning = None;
                                if !specs_are_new(&specs, &database_name)? {
                                    possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                    stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                };
                                stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                match possible_warning {
                                    None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                    Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                }
                            }
                            else {
                                if checked_info.verifier == custom_verifier {
                                    if specs_are_new(&specs, &database_name)? {
                                        stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::Custom(custom_verifier), &general_verifier, &database_name)?;
                                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                        let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                                        let action_card = Action::Stub(checksum).card();
                                        Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                                    }
                                    else {return Err(Error::BadInputData(BadInputData::SpecsAlreadyThere))}
                                }
                                else {return Err(Error::CryptoError(CryptoError::AddSpecsVerifierChanged{network_name: specs.name, old: custom_verifier, new: checked_info.verifier}))}
                            }
                        },
                    }
                },
            }
        },
        Some(CurrentVerifier::General) => {
            match general_verifier {
                Verifier(None) => {
                    if checked_info.verifier == Verifier(None) {
                        let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                        stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                        if specs_are_new(&specs, &database_name)? {
                            stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                        }
                        else {return Err(Error::BadInputData(BadInputData::SpecsAlreadyThere))}
                    }
                    else {
                        let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
                        let new_general_verifier = checked_info.verifier;
                        let general_hold = GeneralHold::get(&database_name)?;
                        stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                        let warning_card_1 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                        let mut possible_warning = None;
                        if !specs_are_new(&specs, &database_name)? {
                            possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                            stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                        };
                        stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &new_general_verifier, &database_name)?;                        
                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                        let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                        let action_card = Action::Stub(checksum).card();
                        match possible_warning {
                            None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                            Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                        }
                    }
                },
                _ => {
                    if checked_info.verifier == general_verifier {
                        if specs_are_new(&specs, &database_name)? {
                            let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
                            stub = stub_add_network_specs(stub, &specs, &CurrentVerifier::General, &general_verifier, &database_name)?;
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub_store_and_get_checksum(stub, &database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                        }
                        else {return Err(Error::BadInputData(BadInputData::SpecsAlreadyThere))}
                    }
                    else {
                       if checked_info.verifier == Verifier(None) {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
                       else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old: general_verifier, new: checked_info.verifier}))}
                    }
                }
            }
        },
        Some(CurrentVerifier::Dead) => return Err(Error::DatabaseError(DatabaseError::DeadVerifier(specs.name))),
    }
}
