use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use definitions::{constants::{SETTREE, TYPES, GENERALVERIFIER}, defaults::{get_default_types}, network_specs::Verifier};


pub fn load_types (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    
    let settings: Tree = database.open_tree(SETTREE)?;
    settings.remove(TYPES)?;
    
    let types_prep = get_default_types()?;
    
    let types = types_prep.encode();
    settings.insert(TYPES, types)?;
    
    database.flush()?;
    
    Ok(())
}

///Set verifier signature for types definitions and for accepting new networks

pub fn set_general_verifier (database_name: &str, general_verifier: Verifier) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let settings: Tree = database.open_tree(SETTREE)?;
    settings.remove(GENERALVERIFIER)?;
    settings.insert(GENERALVERIFIER, general_verifier.encode())?;
    database.flush()?;
    Ok(())
}

///Function to save terms and conditions ack

pub fn ack_user_agreement(database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;   
    let settings: Tree = database.open_tree(b"settings")?;

    settings.insert(b"terms and conditions", vec![1])?;

    Ok(())
}

///Function to check terms and conditions ack

pub fn check_user_agreement(database_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let settings: Tree = database.open_tree(b"settings")?;

    Ok(settings.contains_key(b"terms and conditions")?)
}
