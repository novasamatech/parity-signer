use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use definitions::{constants::{SETTREE, TYPES, GENERALVERIFIER}, defaults::{get_default_types}, network_specs::Verifier};
use anyhow;

use super::error::Error;


/// Load default types

pub fn load_types (database_name: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match settings.remove(TYPES) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let types_prep = match get_default_types() {
        Ok(x) => x,
        Err(e) => return Err(Error::BadTypesFile(e).show()),
    };
    
    let types = types_prep.encode();
    match settings.insert(TYPES, types) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    Ok(())
}

/// Set verifier signature for types definitions and for accepting new networks

pub fn set_general_verifier (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match settings.remove(GENERALVERIFIER) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match settings.insert(GENERALVERIFIER, general_verifier.encode()) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    Ok(())
}

