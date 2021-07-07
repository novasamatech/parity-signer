use sled::{Db, Tree};
use db_handling::{chainspecs::ChainSpecs, constants::{ADDMETAVERIFIER, LOADMETA, METATREE, SETTREE, SPECSTREE}, settings::{LoadMetaDb, UpdSpecs}};
use parity_scale_codec::{Decode, Encode};

pub fn accept_metadata (database: Db) -> Result<String, Box<dyn std::error::Error>> {
    
    let settings: Tree = database.open_tree(SETTREE)?;
    let action = match settings.remove(LOADMETA)? {
        Some(x) => {<LoadMetaDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved metadata found."))}
    };
    database.flush()?;
    
    let metadata: Tree = database.open_tree(METATREE)?;
    metadata.insert(action.versioned_name, action.meta)?;
    database.flush()?;
    
    if let Some(upd_specs) = action.upd_specs {
        let chainspecs: Tree = database.open_tree(SPECSTREE)?;
        let specs_to_change = match chainspecs.remove(&upd_specs.gen_hash)? {
            Some(x) => {<ChainSpecs>::decode(&mut &x[..])?},
            None => {return Err(Box::from("Network specs gone missing."))}
        };
        database.flush()?;
        let mut specs_to_load = specs_to_change;
        specs_to_load.verifier = upd_specs.verifier;
        chainspecs.insert(upd_specs.gen_hash, specs_to_load.encode())?;
        database.flush()?;
    }
    
    Ok(String::from("Metadata successfully loaded."))
    
}


pub fn add_meta_verifier (database: Db) -> Result<String, Box<dyn std::error::Error>> {
    let settings: Tree = database.open_tree(SETTREE)?;
    let upd_specs = match settings.remove(ADDMETAVERIFIER)? {
        Some(x) => {<UpdSpecs>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved metadata verifier found."))}
    };
    database.flush()?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    let specs_to_change = match chainspecs.remove(&upd_specs.gen_hash)? {
        Some(x) => {<ChainSpecs>::decode(&mut &x[..])?},
        None => {return Err(Box::from("Network specs gone missing."))}
    };
    database.flush()?;
    let mut specs_to_load = specs_to_change;
    specs_to_load.verifier = upd_specs.verifier;
    chainspecs.insert(upd_specs.gen_hash, specs_to_load.encode())?;
    database.flush()?;
    
    Ok(String::from("Verifier successfully updated."))
    
}
