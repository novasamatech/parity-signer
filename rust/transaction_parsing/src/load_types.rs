use hex;
use sled::{Db, Tree};
use definitions::{network_specs::Verifier, transactions::{LoadTypes, Transaction, UpdGeneralVerifier}, types::TypeEntry, constants::{ADDGENERALVERIFIER, LOADTYPES, SETTREE, TRANSACTION}, history::Event};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, BadInputData, CryptoError};
use crate::helpers::{open_db, open_tree, flush_db, insert_into_tree, get_checksum};
use crate::utils::{get_types, get_general_verifier};


pub fn load_types (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and removing the previous (if any) load_types saves
    let database = open_db(dbname)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let current_types = get_types(&settings)?;
    
    let current_verifier = get_general_verifier(&settings)?;
    
    let checked_info = pass_crypto(&data_hex)?;
    
    let new_types = match <Vec<TypeEntry>>::decode(&mut &checked_info.message[..]) {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
    };
    
    match checked_info.verifier {
        Verifier::None => {
            if current_verifier == Verifier::None {
            // both verifiers None, can only update types information if it is good and not already in the system
                let warning_card_1 = Card::Warning(Warning::TypesNotVerified).card(0,0);
                let warning_card_2 = Card::Warning(Warning::UpdatingTypes).card(1,0);
                let history = vec![Event::Warning(Warning::TypesNotVerified.show())];
                let index = 2;
                let upd_verifier = false;
                let (types_card, action_card) = process_received_types(&current_types, new_types, history, checked_info.verifier, upd_verifier, index, &transaction, &database)?;
                Ok(format!("{{\"warning\":[{},{}],\"types_info\":[{}],{}}}", warning_card_1, warning_card_2, types_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
        },
        _ => {
            let verifier_card = Card::Verifier(checked_info.verifier.show_card()).card(0,0);
            if current_verifier == checked_info.verifier {
            // verifiers equal, can only update types information if it is good and not already in the system
                let warning_card = Card::Warning(Warning::UpdatingTypes).card(1,0);
                let history: Vec<Event> = Vec::new();
                let index = 2;
                let upd_verifier = false;
                let (types_card, action_card) = process_received_types(&current_types, new_types, history, checked_info.verifier, upd_verifier, index, &transaction, &database)?;
                Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"types_info\":[{}],{}}}", verifier_card, warning_card, types_card, action_card))
            }
            else {
                if current_verifier == Verifier::None {
                // going to update verifier if types information is good,
                // maybe going to add types information if it is good and not already in the system
                    let warning_card_1 = Card::Warning(Warning::GeneralVerifierAppeared).card(1,0);
                    let history = vec![Event::Warning(Warning::GeneralVerifierAppeared.show())];
                    let warning_types_upd = Card::Warning(Warning::UpdatingTypes).card(2,0);
                    let warning_no_types_upd = Card::Warning(Warning::TypesAlreadyThere).card(2,0);
                    let index = 3;
                    let upd_verifier = true;
                    let (types_card, action_card) = process_received_types(&current_types, new_types, history, checked_info.verifier, upd_verifier, index, &transaction, &database)?;
                    if types_card == warning_no_types_upd {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],{}}}", verifier_card, warning_card_1, warning_no_types_upd, action_card))}
                    else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"types_info\":[{}],{}}}", verifier_card, warning_card_1, warning_types_upd, types_card, action_card))}
                }
                else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old_show: current_verifier.show_error(), new_show: checked_info.verifier.show_error()}))}
            }
        },
    }
}


fn process_received_types (current_types: &Vec<TypeEntry>, new_types: Vec<TypeEntry>, mut history: Vec<Event>, verifier: Verifier, upd_verifier: bool, index: u32, transaction: &Tree, database: &Db) -> Result<(String, String), Error> {
    if &new_types == current_types {
        if upd_verifier {
        // adding only types verifier
            let types_card = Card::Warning(Warning::TypesAlreadyThere).card(index-1, 0);
            history.push(Event::Warning(Warning::TypesAlreadyThere.show()));
                
            let upd_general_verifier = Transaction::UpdGeneralVerifier(UpdGeneralVerifier{
                verifier,
                history,
            });
        // making action entry into database
            insert_into_tree(ADDGENERALVERIFIER.to_vec(), upd_general_verifier.encode(), transaction)?;
            flush_db(database)?;
            let checksum = get_checksum(database)?;
        // action card
            let action_card = Action::AddGeneralVerifier(checksum).card();
            Ok((types_card, action_card))
        }
        else {return Err(Error::BadInputData(BadInputData::TypesAlreadyThere))}
    }
    else {
    // loading types
        let types_card = Card::TypesInfo(&hex::encode(blake2b(32, &[], &new_types.encode()).as_bytes())).card(index, 0);
    // adding history entry
        history.push(Event::Warning(Warning::UpdatingTypes.show()));
    // making action entry into database
        let load_types = Transaction::LoadTypes(LoadTypes{types_info: new_types, verifier, upd_verifier, history});
        
        insert_into_tree(LOADTYPES.to_vec(), load_types.encode(), transaction)?;
        flush_db(database)?;
        let checksum = get_checksum(database)?;
        
    // action card
        let action_card = Action::LoadTypes(checksum).card();
        Ok((types_card, action_card))
    }
}

