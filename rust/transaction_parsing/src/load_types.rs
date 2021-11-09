use db_handling::db_transactions::TrDbColdStub;
use definitions::{network_specs::Verifier, history::Event, qr_transfers::ContentLoadTypes};

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, BadInputData, CryptoError, DatabaseError};
use crate::helpers::{get_types, get_general_verifier, GeneralHold};


pub fn load_types(data_hex: &str, database_name: &str) -> Result<String, Error> {
    let checked_info = pass_crypto(&data_hex)?;
    let content_new_types = ContentLoadTypes::from_vec(&checked_info.message);
    let new_types = match content_new_types.types() {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
    };
    let old_types = get_types(&database_name)?;
    let general_verifier = get_general_verifier(&database_name)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    if checked_info.verifier == Verifier(None) {
        if general_verifier == Verifier(None) {
            if new_types == old_types {return Err(Error::BadInputData(BadInputData::TypesAlreadyThere))}
            else {
                stub = stub.new_history_entry(Event::Warning(Warning::TypesNotVerified.show()));
                stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                stub = stub.add_types(&content_new_types, &checked_info.verifier);
                let checksum = match stub.store_and_get_checksum(&database_name) {
                    Ok(a) => a,
                    Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
                };
                let warning_card_1 = Card::Warning(Warning::TypesNotVerified).card(&mut index,0);
                let warning_card_2 = Card::Warning(Warning::UpdatingTypes).card(&mut index,0);
                let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                let action_card = Action::Stub(checksum).card();
                Ok(format!("{{\"warning\":[{},{}],\"types_info\":[{}],{}}}", warning_card_1, warning_card_2, types_card, action_card))
            }
        }
        else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
    }
    else {
        let verifier_card = Card::Verifier(&checked_info.verifier).card(&mut index,0);
        if general_verifier == checked_info.verifier {
            if new_types == old_types {return Err(Error::BadInputData(BadInputData::TypesAlreadyThere))}
            else {
                stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                stub = stub.add_types(&content_new_types, &checked_info.verifier);
                let checksum = match stub.store_and_get_checksum(&database_name) {
                    Ok(a) => a,
                    Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
                };
                let warning_card = Card::Warning(Warning::UpdatingTypes).card(&mut index,0);
                let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                let action_card = Action::Stub(checksum).card();
                Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"types_info\":[{}],{}}}", verifier_card, warning_card, types_card, action_card))
            }
        }
        else {
            if general_verifier == Verifier(None) {
                let new_general_verifier = checked_info.verifier;
                let general_hold = GeneralHold::get(&database_name)?;
                stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                stub = stub.add_types(&content_new_types, &new_general_verifier);
                let warning_card_1 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                let warning_card_2 = {
                    if new_types == old_types {
                        stub = stub.new_history_entry(Event::Warning(Warning::TypesAlreadyThere.show()));
                        Card::Warning(Warning::TypesAlreadyThere).card(&mut index,0)
                    }
                    else {
                        stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                        Card::Warning(Warning::UpdatingTypes).card(&mut index,0)
                    }
                };
                let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                let checksum = match stub.store_and_get_checksum(&database_name) {
                    Ok(a) => a,
                    Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
                };
                let action_card = Action::Stub(checksum).card();
                Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"types_info\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, types_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old: general_verifier, new: checked_info.verifier}))}
        }
    }
}
