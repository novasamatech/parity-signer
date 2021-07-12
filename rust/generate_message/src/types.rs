use definitions::{constants::{HOT_DB_NAME, SETTREE, TYLO, TYPES}, types::TypeEntry};
use parity_scale_codec::Decode;
use sled::{Db, open, Tree};
use std::fs;

pub fn gen_types() -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(HOT_DB_NAME)?;
    let settings: Tree = database.open_tree(SETTREE)?;
    
    let types_info = match settings.get(TYPES)? {
        Some(a) => a,
        None => return Err(Box::from("Types info not found in the hot database.")),
    };
    
    match <Vec<TypeEntry>>::decode(&mut &types_info[..]) {
        Ok(_) => (),
        Err(_) => return Err(Box::from("Types vector from hot database is not decodeable.")),
    }
    
    match fs::write(&TYLO, &types_info) {
        Ok(()) => Ok(()),
        Err(e) => {
            let err_text = format!("Problem writing types export file. {}", e);
            let err: Box<dyn std::error::Error> = From::from(err_text);
            return Err(err)
        },
    }
}
