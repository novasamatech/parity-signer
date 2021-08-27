use anyhow;
use constants::{ADDGENERALVERIFIER, HISTORY, LOADTYPES, SETTREE, TRANSACTION, TYPES, GENERALVERIFIER};
use definitions::{history::Event, transactions::Transaction, types::TypesUpdate};
use parity_scale_codec::{Decode, Encode};
use db_handling::{helpers::{open_db, open_tree, flush_db, insert_into_tree}, manage_history::enter_events_into_tree};
use blake2_rfc::blake2b::blake2b;

use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;


pub fn accept_types (database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let history = open_tree(&database, HISTORY)?;
    
    let action = match transaction.remove(LOADTYPES) {
        Ok(Some(encoded_action)) => match <Transaction>::decode(&mut &encoded_action[..]) {
            Ok(Transaction::LoadTypes(x)) => x,
            Ok(_) => return Err(Error::NoAction(ActionFailure::LoadTypes).show()),
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadTypes).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::LoadTypes).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    let mut events = action.history;
    let types_update_print = TypesUpdate {
        types_hash: hex::encode(blake2b(32, &[], &action.types_info.encode()).as_bytes()),
        verifier_line: action.verifier.show_card(),
    }.show();
    events.push(Event::TypesInfoUpdated(types_update_print));
    
    insert_into_tree(TYPES.to_vec(), action.types_info.encode(), &settings)?;
    flush_db(&database)?;
    
    if action.upd_verifier {
        events.push(Event::GeneralVerifierAdded(action.verifier.show_card()));
        insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    
    Ok(String::from("Types information successfully loaded."))
}


pub fn add_general_verifier (database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let history = open_tree(&database, HISTORY)?;
    
    let action = match transaction.remove(ADDGENERALVERIFIER) {
        Ok(Some(encoded_action)) => match <Transaction>::decode(&mut &encoded_action[..]) {
            Ok(Transaction::UpdGeneralVerifier(x)) => x,
            Ok(_) => return Err(Error::NoAction(ActionFailure::AddGeneralVerifier).show()),
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddGeneralVerifier).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::AddGeneralVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    let mut events = action.history;
    events.push(Event::GeneralVerifierAdded(action.verifier.show_card()));
    
    insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
    flush_db(&database)?;
    
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    
    Ok(String::from("General verifier successfully updated."))
}
