use sled::{Db, open, Tree};
use definitions::{constants::{ADDGENERALVERIFIER, LOADTYPES, SETTREE, TRANSACTION, TYPES, GENERALVERIFIER}, transactions::LoadTypesDb};
use parity_scale_codec::{Decode, Encode};

pub fn accept_types (dbname: &str, checksum: u32) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let settings: Tree = database.open_tree(SETTREE)?;
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    let action = match transaction.remove(LOADTYPES)? {
        Some(x) => {<LoadTypesDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved types information found."))}
    };
    database.flush()?;
    
    settings.insert(TYPES, action.types_info_encoded)?;
    database.flush()?;
    
    if let Some(new_verifier) = action.upd_verifier {
        settings.insert(GENERALVERIFIER, new_verifier.encode())?;
        database.flush()?;
    }
    
    Ok(String::from("Types information successfully loaded."))
}


pub fn add_general_verifier (dbname: &str, checksum: u32) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let settings: Tree = database.open_tree(SETTREE)?;
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    
    let new_verifier_encoded = match transaction.remove(ADDGENERALVERIFIER)? {
        Some(x) => x,
        None => {return Err(Box::from("No approved general verifier found."))}
    };
    database.flush()?;
    
    settings.insert(GENERALVERIFIER, new_verifier_encoded)?;
    database.flush()?;
    
    Ok(String::from("Types verifier successfully updated."))
}
