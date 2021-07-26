use sled::{Db, open, Tree};
use definitions::{network_specs::ChainSpecs, constants::{ADDMETAVERIFIER, GENERALVERIFIER, LOADMETA, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::{LoadMetaDb, UpdSpecs}};
use parity_scale_codec::{Decode, Encode};
use anyhow;

use super::error::{Error, ActionFailure, DBFailure};


/// function to add approved metadata for known network to the database;

pub fn accept_metadata (dbname: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let action = match transaction.remove(LOADMETA) {
        Ok(a) => match a {
            Some(encoded_action) => match <LoadMetaDb>::decode(&mut &encoded_action[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadMeta).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::LoadMeta).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match metadata.insert(action.versioned_name, action.meta) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if upd_general {
    
        let settings: Tree = match database.open_tree(SETTREE) {
            Ok(x) => x,
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match settings.insert(GENERALVERIFIER, action.verifier.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match database.flush() {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    if let Some(network_key) = action.upd_network {
        let chainspecs: Tree = match database.open_tree(SPECSTREE) {
            Ok(x) => x,
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        let mut specs_to_load = match chainspecs.remove(&network_key) {
            Ok(a) => match a {
                Some(specs_encoded) => match <ChainSpecs>::decode(&mut &specs_encoded[..]) {
                    Ok(b) => b,
                    Err(_) => return Err(Error::BadDatabaseDecode(DBFailure::ChainSpecs).show()),
                },
                None => return Err(Error::NotFound(DBFailure::ChainSpecs).show()),
            },
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match database.flush() {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        specs_to_load.verifier = action.verifier;
        match chainspecs.insert(network_key, specs_to_load.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match database.flush() {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    if upd_general {Ok(String::from("Metadata successfully loaded. General verifier successfully updated."))}
    else {Ok(String::from("Metadata successfully loaded."))}
    
}


/// function to add approved metadata for known network to the database;
/// flag upd_general indicates if general verifier should be updated as well;

pub fn add_meta_verifier (dbname: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let upd_specs = match transaction.remove(ADDMETAVERIFIER) {
        Ok(a) => match a {
            Some(encoded_upd_specs) => match <UpdSpecs>::decode(&mut &encoded_upd_specs[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddVerifier).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::AddVerifier).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show())
    };
    
    if upd_general {
        let settings: Tree = match database.open_tree(SETTREE) {
            Ok(x) => x,
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match settings.insert(GENERALVERIFIER, upd_specs.verifier.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
        match database.flush() {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show())
        };
    }
    
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let mut specs_to_load = match chainspecs.remove(&upd_specs.network_key) {
        Ok(a) => match a {
            Some(specs_encoded) => match <ChainSpecs>::decode(&mut &specs_encoded[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadDatabaseDecode(DBFailure::ChainSpecs).show()),
            },
            None => return Err(Error::NotFound(DBFailure::ChainSpecs).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    specs_to_load.verifier = upd_specs.verifier;
    
    match chainspecs.insert(upd_specs.network_key, specs_to_load.encode()) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if upd_general {Ok(String::from("Network verifier successfully updated. General verifier successfully updated."))}
    else {Ok(String::from("Network verifier successfully updated."))}
}
