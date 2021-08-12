use definitions::{constants::{ADDRTREE, HISTORY, METATREE, SPECSTREE}, history::Event, metadata::{NameVersioned, MetaValuesDisplay}, network_specs::{NetworkKey, generate_network_key}, users::{AddressDetails, IdentityHistory}};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use blake2_rfc::blake2b::blake2b;

use crate::error::{Error, NotDecodeable, NotFound, NotHex};
use crate::helpers::{open_db, open_tree, flush_db, insert_into_tree, remove_from_tree, unhex, decode_chain_specs};
use crate::manage_history::enter_events_into_tree;


pub fn remove_network_by_key (network_key: &NetworkKey, database_name: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let identities = open_tree(&database, ADDRTREE)?;
    let history = open_tree(&database, HISTORY)?;
    
// clean up the chainspecs tree
    let network_specs = match chainspecs.remove(&network_key) {
        Ok(Some(network_specs_encoded)) => decode_chain_specs(network_specs_encoded, &network_key.to_vec())?,
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
// record that in the history
    enter_events_into_tree(&history, vec![Event::NetworkRemoved(network_specs.show())])?;
    flush_db(&database)?;
    
// clean up the existing metadata for this network (with various versions) in metadata tree
    let mut events: Vec<Event> = Vec::new();
    for x in metadata.scan_prefix(network_specs.name.encode()) {
        if let Ok((versioned_name_encoded, meta_stored)) = x {
            remove_from_tree(versioned_name_encoded.to_vec(), &metadata)?;
            if let Ok(versioned_name) = <NameVersioned>::decode(&mut &versioned_name_encoded[..]) {
                let meta_values_display = MetaValuesDisplay {
                    name: &versioned_name.name,
                    version: versioned_name.version,
                    meta_hash: &hex::encode(blake2b(32, &[], &meta_stored).as_bytes()),
                }.show();
                events.push(Event::MetadataRemoved(meta_values_display));
            }
        }
    }
    if events.len()>0 {enter_events_into_tree(&history, events)?}
    flush_db(&database)?;

// clean up the network_key from identities having it recorded in network_id
    let mut events: Vec<Event> = Vec::new();
    for x in identities.iter() {
        if let Ok((address_key, address_details_encoded)) = x {
            let mut address_details = match <AddressDetails>::decode(&mut &address_details_encoded[..]) {
                Ok(a) => a,
                Err(_) => {
                    remove_from_tree(address_key.to_vec(), &identities)?;
                    events.push(Event::Error(Error::NotDecodeable(NotDecodeable::AddressDetailsDel).show().to_string()));
                    enter_events_into_tree(&history, events)?;
                    return Err(Error::NotDecodeable(NotDecodeable::AddressDetailsDel).show())
                },
            };
            let identity_history_print = IdentityHistory {
                seed_name: &address_details.seed_name,
                public_key: &hex::encode(&address_key),
                path: &address_details.path,
                network_key: &hex::encode(&network_key),
            }.show();
            events.push(Event::IdentityRemoved(identity_history_print));
            address_details.network_id = address_details.network_id.into_iter().filter(|id| id != network_key).collect();
            if address_details.network_id.is_empty() {remove_from_tree(address_key.to_vec(), &identities)?}
            else {insert_into_tree(address_key.to_vec(), address_details.encode(), &identities)?}
        }
    }
    if events.len()>0 {enter_events_into_tree(&history, events)?}
    flush_db(&database)?;
    Ok(())
}


pub fn remove_network_by_hex (genesis_hash: &str, database_name: &str) -> anyhow::Result<()> {
    
    let network_key = generate_network_key(&unhex(genesis_hash, NotHex::GenesisHash)?);
    remove_network_by_key (&network_key, database_name)
    
}


pub fn remove_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<()> {
    let versioned_name = NameVersioned {
        name: network_name.to_string(),
        version: network_version,
    };
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    let history = open_tree(&database, HISTORY)?;
    match metadata.remove(versioned_name.encode()) {
        Ok(Some(meta_stored)) => {
            let meta_values_display = MetaValuesDisplay {
                name: &network_name,
                version: network_version,
                meta_hash: &hex::encode(blake2b(32, &[], &meta_stored).as_bytes()),
            }.show();
            let events = vec![Event::MetadataRemoved(meta_values_display)];
            enter_events_into_tree(&history, events)?;
            Ok(())
        },
        Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned(versioned_name)).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{populate_cold, metadata::transfer_metadata, manage_history::{init_history, print_history_tree}};
    use super::*;
    use std::fs;
    use sled::{Db, Tree, open};
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
        init_history(dbname).unwrap();
        
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
        
        let history_printed = print_history_tree(&database).unwrap();
        assert!(history_printed.contains(r#""events":[{"event":"database_initiated"}]"#) && history_printed.contains(r##""events":[{"event":"network_removed","payload":["base58prefix":"42","color":"#660D35","decimals":"12","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":"none"]}]"##) && history_printed.contains(r#""events":[{"event":"metadata_removed","payload":["specname":"westend","spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"]},{"event":"metadata_removed","payload":["specname":"westend","spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"]},{"event":"metadata_removed","payload":["specname":"westend","spec_version":"9080","meta_hash":"44e8d52c5af362b3279309ca7476424391902470f363fae097cd8bb620d0e6a7"]}]"#) && history_printed.contains(r#"[{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]},{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]},{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","path":"//kusama","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]},{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04","path":"//rococo","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]},{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","path":"//Alice","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]},{"event":"identity_removed","payload":["seed_name":"Alice","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","path":"//polkadot","network_key":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"]}]"#), "Expected different history:\n{}", history_printed);
        
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

