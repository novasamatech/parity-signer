use anyhow;
use definitions::{constants::{ADDGENERALVERIFIER, LOADTYPES, SETTREE, TRANSACTION, TYPES, GENERALVERIFIER}, network_specs::Verifier, transactions::LoadTypesDb};
use parity_scale_codec::{Decode, Encode};
use db_handling::helpers::{open_db, open_tree, flush_db, insert_into_tree};

use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;


pub fn accept_types (database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let action = match transaction.remove(LOADTYPES) {
        Ok(Some(encoded_load_types)) => match <LoadTypesDb>::decode(&mut &encoded_load_types[..]) {
            Ok(x) => x,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadTypes).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::LoadTypes).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    insert_into_tree(TYPES.to_vec(), action.types_info_encoded, &settings)?;
    flush_db(&database)?;
    
    if let Some(new_verifier) = action.upd_verifier {
        insert_into_tree(GENERALVERIFIER.to_vec(), new_verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    Ok(String::from("Types information successfully loaded."))
}


pub fn add_general_verifier (database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let new_verifier_encoded = match transaction.remove(ADDGENERALVERIFIER) {
        Ok(Some(x)) => match <Verifier>::decode(&mut &x[..]) {
            Ok(_) => x,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddGeneralVerifier).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::AddGeneralVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    insert_into_tree(GENERALVERIFIER.to_vec(), new_verifier_encoded.to_vec(), &settings)?;
    flush_db(&database)?;
    
    Ok(String::from("Types verifier successfully updated."))
}
