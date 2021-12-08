use db_handling::{db_transactions::TrDbColdStub, helpers::{try_get_valid_current_verifier, get_general_verifier}};
use definitions::{error::{ErrorSigner, GeneralVerifierForContent, InputSigner, Signer, TransferContent}, history::Event, keyring::{VerifierKey}, network_specs::{ValidCurrentVerifier, Verifier}, qr_transfers::ContentAddSpecs};

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::helpers::{specs_are_new};

use crate::holds::{GeneralHold, Hold, HoldRelease};

pub fn add_specs (data_hex: &str, database_name: &str) -> Result<String, ErrorSigner> {
    let checked_info = pass_crypto(&data_hex, TransferContent::AddSpecs)?;
    let specs = ContentAddSpecs::from_vec(&checked_info.message).specs::<Signer>()?;
    let verifier_key = VerifierKey::from_parts(&specs.genesis_hash.to_vec());
    let possible_valid_current_verifier = try_get_valid_current_verifier (&verifier_key, &database_name)?;
    let general_verifier = get_general_verifier(&database_name)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    
    match possible_valid_current_verifier {
        None => {
            match checked_info.verifier {
                Verifier(None) => {
                    stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                    stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::Custom(Verifier(None)), &general_verifier, &database_name)?;
                    stub = stub.new_network_verifier(&verifier_key, &ValidCurrentVerifier::Custom(Verifier(None)), &general_verifier);
                    let checksum = stub.store_and_get_checksum(&database_name)?;
                    let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                    let action_card = Action::Stub(checksum).card();
                    Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                },
                Verifier(Some(ref new_verifier_value)) => {
                    let verifier_card = Card::Verifier(&new_verifier_value).card(&mut index,0);
                    match general_verifier {
                        Verifier(None) => {
                            let new_general_verifier = checked_info.verifier;
                            let general_hold = GeneralHold::get(&database_name)?;
                            stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                            stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &new_general_verifier, &database_name)?;
                            stub = stub.new_network_verifier(&verifier_key, &ValidCurrentVerifier::General, &new_general_verifier);
                            let warning_card = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                            let specs_card = Card::NewSpecs(&specs).card(&mut index,0);
                            let checksum = stub.store_and_get_checksum(&database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card, specs_card, action_card))
                        },
                        _ => {
                            if checked_info.verifier == general_verifier {
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                                stub = stub.new_network_verifier(&verifier_key, &ValidCurrentVerifier::General, &general_verifier);
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                            }
                            else {
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::Custom(checked_info.verifier.to_owned()), &general_verifier, &database_name)?;
                                stub = stub.new_network_verifier(&verifier_key, &ValidCurrentVerifier::Custom(checked_info.verifier), &general_verifier);
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                            }
                        },
                    }
                },
            }
        },
        Some(ValidCurrentVerifier::Custom(custom_verifier)) => {
            match custom_verifier {
                Verifier(None) => {
                    match checked_info.verifier {
                        Verifier(None) => {
                            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                            let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                            if specs_are_new(&specs, &database_name)? {
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::Custom(Verifier(None)), &general_verifier, &database_name)?;
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                            }
                            else {return Err(ErrorSigner::Input(InputSigner::SpecsKnown{name: specs.name, encryption: specs.encryption}))} 
                        },
                        Verifier(Some(ref new_verifier_value)) => {
                            let verifier_card = Card::Verifier(&new_verifier_value).card(&mut index,0);
                            let hold = Hold::get(&verifier_key, &database_name)?;
                            if checked_info.verifier == general_verifier {
                                stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &ValidCurrentVerifier::General, HoldRelease::General, &database_name)?;
                                let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                let mut possible_warning = None;
                                if !specs_are_new(&specs, &database_name)? {
                                    possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                    stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                };
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                match possible_warning {
                                    None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                    Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                }
                            }
                            else {
                                if general_verifier == Verifier(None) {
                                    let new_general_verifier = checked_info.verifier;
                                    stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &ValidCurrentVerifier::General, HoldRelease::GeneralSuper, &database_name)?;
                                    let warning_card_1 = Card::Warning(Warning::VerifierGeneralSuper{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                    let general_hold = GeneralHold::get(&database_name)?;
                                    stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                                    let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                                    let mut possible_warning = None;
                                    if !specs_are_new(&specs, &database_name)? {
                                        possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                        stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                    };
                                    stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                    let checksum = stub.store_and_get_checksum(&database_name)?;
                                    let action_card = Action::Stub(checksum).card();
                                    match possible_warning {
                                        None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                        Some(warning_card_3) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, specs_card, action_card)),
                                    }
                                }
                                else {
                                    stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &ValidCurrentVerifier::Custom(checked_info.verifier.to_owned()), HoldRelease::Custom, &database_name)?;
                                    let warning_card_1 = Card::Warning(Warning::VerifierChangingToCustom{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                    let mut possible_warning = None;
                                    if !specs_are_new(&specs, &database_name)? {
                                        possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                        stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                    };
                                    stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::Custom(checked_info.verifier.to_owned()), &general_verifier, &database_name)?;
                                    let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                    let checksum = stub.store_and_get_checksum(&database_name)?;
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
                Verifier(Some(ref old_verifier_value)) => {
                    match checked_info.verifier {
                        Verifier(None) => return Err(ErrorSigner::Input(InputSigner::NeedVerifier{name: specs.name, verifier_value: old_verifier_value.to_owned()})),
                        Verifier(Some(ref new_verifier_value)) => {
                            let verifier_card = Card::Verifier(&new_verifier_value).card(&mut index,0);
                            if checked_info.verifier == general_verifier {
                                let hold = Hold::get(&verifier_key, &database_name)?;
                                stub = hold.upd_stub(stub, &verifier_key, &custom_verifier, &ValidCurrentVerifier::General, HoldRelease::General, &database_name)?;
                                let warning_card_1 = Card::Warning(Warning::VerifierChangingToGeneral{verifier_key: &verifier_key, hold: &hold}).card(&mut index,0);
                                let mut possible_warning = None;
                                if !specs_are_new(&specs, &database_name)? {
                                    possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                    stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                                };
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                match possible_warning {
                                    None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                    Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                                }
                            }
                            else {
                                if new_verifier_value == old_verifier_value {
                                    if specs_are_new(&specs, &database_name)? {
                                        stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::Custom(custom_verifier), &general_verifier, &database_name)?;
                                        let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                        let checksum = stub.store_and_get_checksum(&database_name)?;
                                        let action_card = Action::Stub(checksum).card();
                                        Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                                    }
                                    else {return Err(ErrorSigner::Input(InputSigner::SpecsKnown{name: specs.name, encryption: specs.encryption}))}
                                }
                                else {return Err(ErrorSigner::Input(InputSigner::AddSpecsVerifierChanged{name: specs.name, old_verifier_value: old_verifier_value.to_owned(), new_verifier_value: new_verifier_value.to_owned()}))}
                            }
                        },
                    }
                },
            }
        },
        Some(ValidCurrentVerifier::General) => {
            match general_verifier {
                Verifier(None) => {
                    match checked_info.verifier {
                        Verifier(None) => {
                            let warning_card = Card::Warning(Warning::NotVerified).card(&mut index,0);
                            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
                            if specs_are_new(&specs, &database_name)? {
                                stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                                let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                                let checksum = stub.store_and_get_checksum(&database_name)?;
                                let action_card = Action::Stub(checksum).card();
                                Ok(format!("{{\"warning\":[{}],\"new_specs\":[{}],{}}}", warning_card, specs_card, action_card))
                            }
                            else {return Err(ErrorSigner::Input(InputSigner::SpecsKnown{name: specs.name, encryption: specs.encryption}))}
                        },
                        Verifier(Some(ref new_verifier_value)) => {
                            let verifier_card = Card::Verifier(&new_verifier_value).card(&mut index,0);
                            let new_general_verifier = checked_info.verifier;
                            let general_hold = GeneralHold::get(&database_name)?;
                            stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                            let warning_card_1 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                            let mut possible_warning = None;
                            if !specs_are_new(&specs, &database_name)? {
                                possible_warning = Some(Card::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title)).card(&mut index,0));
                                stub = stub.new_history_entry(Event::Warning(Warning::NetworkSpecsAlreadyThere(&specs.title).show()));
                            };
                            stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &new_general_verifier, &database_name)?;                        
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub.store_and_get_checksum(&database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            match possible_warning {
                                None => Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, specs_card, action_card)),
                                Some(warning_card_2) => Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"new_specs\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, specs_card, action_card)),
                            }
                        },
                    }
                },
                Verifier(Some(ref old_general_verifier_value)) => {
                    if checked_info.verifier == general_verifier {
                        if specs_are_new(&specs, &database_name)? {
                            let verifier_card = Card::Verifier(&old_general_verifier_value).card(&mut index,0);
                            stub = stub.add_network_specs(&specs, &ValidCurrentVerifier::General, &general_verifier, &database_name)?;
                            let specs_card = Card::NewSpecs(&specs).card(&mut index, 0);
                            let checksum = stub.store_and_get_checksum(&database_name)?;
                            let action_card = Action::Stub(checksum).card();
                            Ok(format!("{{\"verifier\":[{}],\"new_specs\":[{}],{}}}", verifier_card, specs_card, action_card))
                        }
                        else {return Err(ErrorSigner::Input(InputSigner::SpecsKnown{name: specs.name, encryption: specs.encryption}))}
                    }
                    else {
                        match checked_info.verifier {
                            Verifier(None) => return Err(ErrorSigner::Input(InputSigner::NeedGeneralVerifier{content: GeneralVerifierForContent::Network{name: specs.name}, verifier_value: old_general_verifier_value.to_owned()})),
                            Verifier(Some(new_general_verifier_value)) => return Err(ErrorSigner::Input(InputSigner::GeneralVerifierChanged{content: GeneralVerifierForContent::Network{name: specs.name}, old_general_verifier_value: old_general_verifier_value.to_owned(), new_general_verifier_value: new_general_verifier_value.to_owned()})),
                        }
                    }
                }
            }
        },
    }
}
