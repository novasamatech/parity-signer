use anyhow;
use sled::{Db, open, Tree};
use definitions::{constants::{ADDGENERALVERIFIER, LOADTYPES, SETTREE, TRANSACTION, TYPES, GENERALVERIFIER}, network_specs::Verifier, transactions::LoadTypesDb};
use parity_scale_codec::{Decode, Encode};

use super::error::{Error, ActionFailure};

pub fn accept_types (dbname: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let action = match transaction.remove(LOADTYPES) {
        Ok(a) => match a {
            Some(encoded_load_types) => match <LoadTypesDb>::decode(&mut &encoded_load_types[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadTypes).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::LoadTypes).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match settings.insert(TYPES, action.types_info_encoded) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if let Some(new_verifier) = action.upd_verifier {
        
        match settings.insert(GENERALVERIFIER, new_verifier.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        
        match database.flush() {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    Ok(String::from("Types information successfully loaded."))
}


pub fn add_general_verifier (dbname: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let new_verifier_encoded = match transaction.remove(ADDGENERALVERIFIER) {
        Ok(a) => match a {
            Some(x) => match <Verifier>::decode(&mut &x[..]) {
                Ok(_) => x,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddGeneralVerifier).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::AddGeneralVerifier).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match settings.insert(GENERALVERIFIER, new_verifier_encoded) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    Ok(String::from("Types verifier successfully updated."))
}
