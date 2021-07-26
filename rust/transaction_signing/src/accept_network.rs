use anyhow;
use sled::{Db, open, Tree};
use definitions::{network_specs::{ChainSpecs, generate_network_key}, constants::{ADDNETWORK, ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::AddNetworkDb, users::AddressDetails};
use parity_scale_codec::{Decode, Encode};

use super::error::{Error, ActionFailure, DBFailure};

/// function to add approved network to the database;
/// flag upd_general indicates if general verifier should be updated as well

pub fn add_network (dbname: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
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
    
    let action = match transaction.remove(ADDNETWORK) {
        Ok(a) => match a {
            Some(encoded_action) => match <AddNetworkDb>::decode(&mut &encoded_action[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddNetwork).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::AddNetwork).show()),
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
    
// updating general verifier if requested
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

// creating chainspecs tree entry
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let order = chainspecs.len() as u8;
    let network_key = generate_network_key(&action.chainspecs.genesis_hash.to_vec());
    let new_chainspecs = ChainSpecs {
        base58prefix: action.chainspecs.base58prefix,
        color: action.chainspecs.color,
        decimals: action.chainspecs.decimals,
        genesis_hash: action.chainspecs.genesis_hash,
        logo: action.chainspecs.logo,
        name: action.chainspecs.name,
        order,
        path_id: action.chainspecs.path_id,
        secondary_color: action.chainspecs.secondary_color,
        title: action.chainspecs.title,
        unit: action.chainspecs.unit,
        verifier: action.verifier,
    };
    
    match chainspecs.insert(&network_key, new_chainspecs.encode()) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };

// adding network in network_id vector of all existing identities records
// with default path "", and no password (has_pwd = false)
    let identities: Tree = match database.open_tree(ADDRTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let mut address_details = match <AddressDetails>::decode(&mut &value[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::BadDatabaseDecode(DBFailure::AddressDetails).show())
            };
            if (address_details.path.as_str() == "") && !address_details.has_pwd {
                address_details.network_id.push(network_key.to_vec());
                match identities.insert(key, address_details.encode()) {
                    Ok(_) => (),
                    Err(e) => return Err(Error::InternalDatabaseError(e).show()),
                };
            }
        }
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if upd_general {Ok(String::from("Network successfully added. General verifier successfully updated."))}
    else {Ok(String::from("Network successfully added."))}
}

