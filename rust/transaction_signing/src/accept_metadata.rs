use sled::{Db, open, Tree};
use definitions::{network_specs::ChainSpecs, constants::{ADDMETAVERIFIER, GENERALVERIFIER, LOADMETA, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::{LoadMetaDb, UpdSpecs}};
use parity_scale_codec::{Decode, Encode};


/// function to add approved metadata for known network to the database;

pub fn accept_metadata (dbname: &str, checksum: u32, upd_general: bool) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    let action = match transaction.remove(LOADMETA)? {
        Some(x) => {<LoadMetaDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved metadata found."))}
    };
    database.flush()?;
    
    let metadata: Tree = database.open_tree(METATREE)?;
    metadata.insert(action.versioned_name, action.meta)?;
    database.flush()?;
    
    if upd_general {
        let settings: Tree = database.open_tree(SETTREE)?;
        settings.insert(GENERALVERIFIER, action.verifier.encode())?;
        database.flush()?;
    }
    
    if let Some(network_key) = action.upd_network {
        let chainspecs: Tree = database.open_tree(SPECSTREE)?;
        let specs_to_change = match chainspecs.remove(&network_key)? {
            Some(x) => {<ChainSpecs>::decode(&mut &x[..])?},
            None => {return Err(Box::from("Network specs gone missing."))}
        };
        database.flush()?;
        let mut specs_to_load = specs_to_change;
        specs_to_load.verifier = action.verifier;
        chainspecs.insert(network_key, specs_to_load.encode())?;
        database.flush()?;
    }
    
    if upd_general {Ok(String::from("Metadata successfully loaded. General verifier successfully updated."))}
    else {Ok(String::from("Metadata successfully loaded."))}
    
}


/// function to add approved metadata for known network to the database;
/// flag upd_general indicates if general verifier should be updated as well;

pub fn add_meta_verifier (dbname: &str, checksum: u32, upd_general: bool) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    let upd_specs = match transaction.remove(ADDMETAVERIFIER)? {
        Some(x) => {<UpdSpecs>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved verifier found."))}
    };
    database.flush()?;
    
    if upd_general {
        let settings: Tree = database.open_tree(SETTREE)?;
        settings.insert(GENERALVERIFIER, upd_specs.verifier.encode())?;
        database.flush()?;
    }
    
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    let specs_to_change = match chainspecs.remove(&upd_specs.network_key)? {
        Some(x) => {<ChainSpecs>::decode(&mut &x[..])?},
        None => {return Err(Box::from("Network specs gone missing."))}
    };
    database.flush()?;
    let mut specs_to_load = specs_to_change;
    specs_to_load.verifier = upd_specs.verifier;
    chainspecs.insert(upd_specs.network_key, specs_to_load.encode())?;
    database.flush()?;
    
    if upd_general {Ok(String::from("Network verifier successfully updated. General verifier successfully updated."))}
    else {Ok(String::from("Network verifier successfully updated."))}
}
