use sled::{Db, Tree};
use db_handling::{constants::{ADDTYPESVERIFIER, LOADTYPES, SETTREE, TYPES, TYPESVERIFIER}, settings::LoadTypesDb};
use parity_scale_codec::{Decode, Encode};

pub fn accept_types (database: Db) -> Result<String, Box<dyn std::error::Error>> {
    
    let settings: Tree = database.open_tree(SETTREE)?;
    let action = match settings.remove(LOADTYPES)? {
        Some(x) => {<LoadTypesDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved types information found."))}
    };
    database.flush()?;
    
    settings.insert(TYPES, action.types_info_encoded)?;
    database.flush()?;
    
    if let Some(new_verifier) = action.upd_verifier {
        settings.insert(TYPESVERIFIER, new_verifier.encode())?;
        database.flush()?;
    }
    
    Ok(String::from("Types information successfully loaded."))
}


pub fn add_types_verifier (database: Db) -> Result<String, Box<dyn std::error::Error>> {
    
    let settings: Tree = database.open_tree(SETTREE)?;
    let new_verifier_encoded = match settings.remove(ADDTYPESVERIFIER)? {
        Some(x) => x,
        None => {return Err(Box::from("No approved types verifier found."))}
    };
    database.flush()?;
    
    settings.insert(TYPESVERIFIER, new_verifier_encoded)?;
    database.flush()?;
    
    Ok(String::from("Types verifier successfully updated."))
}
