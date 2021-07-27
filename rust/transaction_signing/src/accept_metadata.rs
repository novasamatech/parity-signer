use definitions::{constants::{ADDMETAVERIFIER, GENERALVERIFIER, LOADMETA, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::{LoadMetaDb, UpdSpecs}};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use db_handling::helpers::{open_db, open_tree, flush_db, insert_into_tree, get_and_decode_chain_specs};

use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;

/// function to add approved metadata for known network to the database;

pub fn accept_metadata (database_name: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let action = match transaction.remove(LOADMETA) {
        Ok(Some(encoded_action)) => match <LoadMetaDb>::decode(&mut &encoded_action[..]) {
            Ok(x) => x,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadMeta).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::LoadMeta).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    flush_db(&database)?;
    
    let metadata = open_tree(&database, METATREE)?;
    insert_into_tree(action.versioned_name, action.meta, &metadata)?;
    flush_db(&database)?;
    
    if upd_general {
        let settings = open_tree(&database, SETTREE)?;
        insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    if let Some(network_key) = action.upd_network {
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let mut specs_to_load = get_and_decode_chain_specs(&chainspecs, &network_key.to_vec())?;
        specs_to_load.verifier = action.verifier;
        insert_into_tree(network_key.to_vec(), specs_to_load.encode(), &chainspecs)?;
        flush_db(&database)?;
    }
    
    if upd_general {Ok(String::from("Metadata successfully loaded. General verifier successfully updated."))}
    else {Ok(String::from("Metadata successfully loaded."))}
    
}


/// function to add approved metadata for known network to the database;
/// flag upd_general indicates if general verifier should be updated as well;

pub fn add_meta_verifier (database_name: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let upd_specs = match transaction.remove(ADDMETAVERIFIER) {
        Ok(Some(encoded_upd_specs)) => match <UpdSpecs>::decode(&mut &encoded_upd_specs[..]) {
            Ok(x) => x,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddVerifier).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::AddVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    if upd_general {
        let settings = open_tree(&database, SETTREE)?;
        insert_into_tree(GENERALVERIFIER.to_vec(), upd_specs.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut specs_to_load = get_and_decode_chain_specs(&chainspecs, &upd_specs.network_key)?;
    specs_to_load.verifier = upd_specs.verifier;
    insert_into_tree(upd_specs.network_key, specs_to_load.encode(), &chainspecs)?;
    flush_db(&database)?;
    
    if upd_general {Ok(String::from("Network verifier successfully updated. General verifier successfully updated."))}
    else {Ok(String::from("Network verifier successfully updated."))}
}
