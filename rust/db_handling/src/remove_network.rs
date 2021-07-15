use definitions::{constants::{ADDRTREE, METATREE, SPECSTREE}, metadata::NameVersioned, network_specs::{ChainSpecs, NetworkKey, generate_network_key}, users::AddressDetails};
use sled::{Db, open, Tree};
use parity_scale_codec::{Decode, Encode};
use hex;


pub fn remove_network_by_key (network_key: &NetworkKey, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    let identities: Tree = database.open_tree(ADDRTREE)?;

// clean up the chainspecs tree
    let network_specs = match chainspecs.remove(&network_key)? {
        Some(network_specs_encoded) => {
            match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Network specs were damaged and not decodeable. Removed the record from chainspecs tree.")),
            }
        },
        None => return Err(Box::from("Network key not found in chainspecs tree of the database."))
    };
    database.flush()?;
    
// clean up the existing metadata for this network (with various versions) in metadata tree
    for x in metadata.scan_prefix(network_specs.name.encode()) {
        if let Ok((versioned_name_encoded, _)) = x {
            metadata.remove(versioned_name_encoded)?;
        }
    }
    database.flush()?;

// clean up the network_key from identities having it recorded in network_id
    for x in identities.iter() {
        if let Ok((address_key, address_details_encoded)) = x {
            let mut address_details = match <AddressDetails>::decode(&mut &address_details_encoded[..]) {
                Ok(a) => a,
                Err(_) => {
                    identities.remove(address_key)?;
                    return Err(Box::from("Address details were damaged and not decodeable. Removed the record from identities tree."))
                },
            };
            address_details.network_id = address_details.network_id.into_iter().filter(|id| id != network_key).collect();
            if address_details.network_id.is_empty() {identities.remove(address_key)?;} 
            else {identities.insert(address_key, address_details.encode())?;}
        }
    }
    database.flush()?;
    Ok(())
}


pub fn remove_network_by_hex (could_be_hex_gen_hash: &str, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let hex_gen_hash = {
        if could_be_hex_gen_hash.starts_with("0x") {&could_be_hex_gen_hash[2..]}
        else {could_be_hex_gen_hash}
    };
    let network_key = generate_network_key(&hex::decode(hex_gen_hash)?);
    remove_network_by_key (&network_key, database_name)
    
}


pub fn remove_metadata (network_name: &str, network_version: u32, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let versioned_name = NameVersioned {
        name: network_name.to_string(),
        version: network_version,
    };
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    match metadata.remove(versioned_name.encode())? {
        Some(_) => Ok(()),
        None => return Err(Box::from(format!("Metadata for {} version {} not in the database.", network_name, network_version)))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{populate_cold, metadata::transfer_metadata};
    use super::*;
    use std::fs;
    use definitions::constants::HOT_DB_NAME;
    
    const METADATA_FILE: &str = "metadata_database.ts";
    
    fn check_for_network (versioned_name: &NameVersioned, dbname: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let database: Db = open(dbname).unwrap();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        let flag = metadata.contains_key(versioned_name.encode())?;
        Ok(flag)
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
        
        assert!(check_for_network(&versioned_name, dbname).unwrap(), "No westend 9010 to begin with.");
        
        remove_metadata (network_name, network_version, dbname).unwrap();
        
        assert!(!check_for_network(&versioned_name, dbname).unwrap(), "Westend 9010 not removed.");
        
        fs::remove_dir_all(dbname).unwrap();
    }
}

