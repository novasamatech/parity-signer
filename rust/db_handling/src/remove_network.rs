use definitions::{constants::{ADDRTREE, METATREE, SPECSTREE}, metadata::NameVersioned, network_specs::{ChainSpecs, NetworkKey, generate_network_key}, users::AddressDetails};
use sled::{Db, open, Tree};
use parity_scale_codec::{Decode, Encode};
use hex;
use anyhow;

use super::error::{Error, NotDecodeable, NotFound, NotHex};


pub fn remove_network_by_key (network_key: &NetworkKey, database_name: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let identities: Tree = match database.open_tree(ADDRTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };

// clean up the chainspecs tree
    let network_specs = match chainspecs.remove(&network_key) {
        Ok(a) => match a {
            Some(network_specs_encoded) => {
                match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                    Ok(b) => {
                        if network_key != &generate_network_key(&b.genesis_hash.to_vec()) {return Err(Error::GenesisHashMismatch.show())}
                        b
                    },
                    Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
                }
            },
            None => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
// clean up the existing metadata for this network (with various versions) in metadata tree
    for x in metadata.scan_prefix(network_specs.name.encode()) {
        if let Ok((versioned_name_encoded, _)) = x {
            match metadata.remove(versioned_name_encoded) {
                Ok(_) => (),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            };
        }
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };

// clean up the network_key from identities having it recorded in network_id
    for x in identities.iter() {
        if let Ok((address_key, address_details_encoded)) = x {
            let mut address_details = match <AddressDetails>::decode(&mut &address_details_encoded[..]) {
                Ok(a) => a,
                Err(_) => {
                    match identities.remove(address_key) {
                        Ok(_) => (),
                        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
                    };
                    return Err(Error::NotDecodeable(NotDecodeable::AddressDetailsDel).show())
                },
            };
            address_details.network_id = address_details.network_id.into_iter().filter(|id| id != network_key).collect();
            if address_details.network_id.is_empty() {
                match identities.remove(address_key) {
                    Ok(_) => (),
                    Err(e) => return Err(Error::InternalDatabaseError(e).show()),
                };
            } 
            else {
                match identities.insert(address_key, address_details.encode()) {
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
    Ok(())
}


pub fn remove_network_by_hex (could_be_hex_gen_hash: &str, database_name: &str) -> anyhow::Result<()> {
    
    let hex_gen_hash = {
        if could_be_hex_gen_hash.starts_with("0x") {&could_be_hex_gen_hash[2..]}
        else {could_be_hex_gen_hash}
    };
    let unhex_gen_hash = match hex::decode(hex_gen_hash) {
        Ok(x) => x,
        Err(_) => return Err(Error::NotHex(NotHex::GenesisHash).show()),
    };
    let network_key = generate_network_key(&unhex_gen_hash);
    remove_network_by_key (&network_key, database_name)
    
}


pub fn remove_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<()> {
    let versioned_name = NameVersioned {
        name: network_name.to_string(),
        version: network_version,
    };
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match metadata.remove(versioned_name.encode()) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned(versioned_name)).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{populate_cold, metadata::transfer_metadata};
    use super::*;
    use std::fs;
    use definitions::constants::HOT_DB_NAME;
    
    const METADATA_FILE: &str = "metadata_database.ts";
    
    fn check_for_network (versioned_name: &NameVersioned, dbname: &str) -> bool {
        let database: Db = open(dbname).unwrap();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        metadata.contains_key(versioned_name.encode()).unwrap()
    }

    #[test]
    fn remove_all_westend() {
        let dbname = "tests/remove_all_westend";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        transfer_metadata(HOT_DB_NAME, dbname).unwrap();
        
        let line = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_key = generate_network_key(&hex::decode(line).unwrap());
        remove_network_by_key (&network_key, dbname).unwrap();
        
        let database: Db = open(dbname).unwrap();
        
        let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
        assert!(chainspecs.get(&network_key).unwrap() == None, "Westend network specs were not deleted");
        
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        let prefix_meta = String::from("westend").encode();
        assert!(metadata.scan_prefix(&prefix_meta).next() == None, "Some westend metadata was not deleted");
        
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        for x in identities.iter() {
            if let Ok((_, address_details_encoded)) = x {
                let address_details = <AddressDetails>::decode(&mut &address_details_encoded[..]).unwrap();
                assert!(!address_details.network_id.contains(&network_key), "Some westend identities still remain.");
                assert!(address_details.network_id.len() != 0, "Did not remove address key entried with no network keys associated");
            }
        }
        
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn remove_westend_9010() {
        let dbname = "tests/remove_westend_9010";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        let network_name = "westend";
        let network_version = 9010;
        
        let versioned_name = NameVersioned {
            name: network_name.to_string(),
            version: network_version,
        };
        
        assert!(check_for_network(&versioned_name, dbname), "No westend 9010 to begin with.");
        
        remove_metadata (network_name, network_version, dbname).unwrap();
        
        assert!(!check_for_network(&versioned_name, dbname), "Westend 9010 not removed.");
        
        fs::remove_dir_all(dbname).unwrap();
    }
}

