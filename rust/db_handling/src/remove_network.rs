use anyhow;
use constants::{ADDRTREE, METATREE, SPECSTREE, VERIFIERS};
use definitions::{history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay}, keyring::{AddressKey, NetworkSpecsKey, VerifierKey, MetaKeyPrefix, MetaKey}, metadata::MetaValues, network_specs::{CurrentVerifier, Verifier}};
use parity_scale_codec::{Decode, Encode};
use sled::Batch;

use crate::db_transactions::TrDbCold;
use crate::error::{Error, NotDecodeable, NotFound, NotHex};
use crate::helpers::{open_db, open_tree, get_general_verifier, unhex, decode_chain_specs, reverse_address_key, decode_address_details};
use crate::manage_history::events_to_batch;

/// Function to remove the network with given NetworkSpecsKey from the database.
/// Removes network specs for all entries with same genesis hash.
/// Removes all metadata entries for corresponding network specname.
/// Removes all addresses corresponding to the networks removed (all encryptions).
/// If CurrentVerifier is Custom(Verifier(None)), leave it as that. If CurrentVerifier is General, leave it as General.
/// If CurrentVerifier is Custom with some Verifier set, transform it into CurrentVerifier::Dead to disable the network
/// permanently until Signer is wiped altogether.
pub fn remove_network_by_key (network_specs_key: &NetworkSpecsKey, database_name: &str) -> anyhow::Result<()> {
    let mut address_batch = Batch::default();
    let mut meta_batch = Batch::default();
    let mut network_specs_batch = Batch::default();
    let mut verifiers_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    
    let general_verifier = get_general_verifier(&database_name)?;
    
    {
        let database = open_db(database_name)?;
        let metadata = open_tree(&database, METATREE)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let verifiers = open_tree(&database, VERIFIERS)?;
        let identities = open_tree(&database, ADDRTREE)?;
    
    // get network_specs from chainspecs tree
        let network_specs = match chainspecs.get(network_specs_key.key()) {
            Ok(Some(network_specs_encoded)) => decode_chain_specs(network_specs_encoded, network_specs_key)?,
            Ok(None) => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    // scan through verifiers tree to possibly modify verifier
        let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
        let current_verifier = match verifiers.get(verifier_key.key()) {
            Ok(Some(current_verifier_encoded)) => match <CurrentVerifier>::decode(&mut &current_verifier_encoded[..]) {
                Ok(a) => {
                    match a {
                        CurrentVerifier::General => (),
                        CurrentVerifier::Custom(ref b) => {
                            match b {
                                Verifier(None) => (),
                                _ => {
                                    verifiers_batch.remove(verifier_key.key());
                                    verifiers_batch.insert(verifier_key.key(), (CurrentVerifier::Dead).encode());
                                }
                            }
                        },
                        CurrentVerifier::Dead => return Err(Error::DeadVerifier.show()),
                    }
                    a
                },
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::CurrentVerifier).show()),
            },
            Ok(None) => return Err(Error::NotFound(NotFound::CurrentVerifier).show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    // scan through chainspecs tree to mark for removal all networks with target genesis hash
        let mut keys_to_wipe: Vec<NetworkSpecsKey> = Vec::new();
        for x in chainspecs.iter() {
            if let Ok((x_network_specs_key_vec, x_network_specs_encoded)) = x {
                let x_network_specs_key = NetworkSpecsKey::from_vec(&x_network_specs_key_vec.to_vec());
                let x_network_specs = decode_chain_specs(x_network_specs_encoded, &x_network_specs_key)?;
                if x_network_specs.genesis_hash == network_specs.genesis_hash {
                    network_specs_batch.remove(x_network_specs_key.key());
                    events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&x_network_specs, &current_verifier, &general_verifier)));
                    keys_to_wipe.push(x_network_specs_key);
                }
            }
        }
    // scan through metadata tree to mark for removal all networks with target name
        let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
        for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
            if let Ok((meta_key_vec, meta_stored)) = x {
                let meta_key = MetaKey::from_vec(&meta_key_vec.to_vec());
                meta_batch.remove(meta_key.key());
                if let Ok((name, version)) = meta_key.name_version() {
                    let meta_values_display = MetaValuesDisplay::get(&MetaValues{name, version, meta: meta_stored.to_vec()});
                    events.push(Event::MetadataRemoved(meta_values_display));
                }
            }
        }
    // scan through address tree to clean up the network_key(s) from identities
        for x in identities.iter() {
            if let Ok((address_key_vec, address_details_encoded)) = x {
                let mut address_details = decode_address_details(address_details_encoded)?;
                for key in keys_to_wipe.iter() {
                    let (public_key, encryption) = reverse_address_key(&AddressKey::from_vec(&address_key_vec.to_vec()))?;
                    if address_details.network_id.contains(key) {
                        let identity_history = IdentityHistory::get(&address_details.seed_name, &encryption, &public_key, &address_details.path, &network_specs.genesis_hash.to_vec());
                        events.push(Event::IdentityRemoved(identity_history));
                        address_details.network_id = address_details.network_id.into_iter().filter(|id| id != network_specs_key).collect();
                    }
                }
                if address_details.network_id.is_empty() {address_batch.remove(address_key_vec.to_vec())}
                else {address_batch.insert(address_key_vec.to_vec(), address_details.encode())}
            }
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch(&database_name, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply(&database_name)
}


pub fn remove_network_by_hex (network_specs_key_string: &str, database_name: &str) -> anyhow::Result<()> {
    let network_specs_key = NetworkSpecsKey::from_vec(&unhex(network_specs_key_string, NotHex::NetworkSpecsKey)?);
    remove_network_by_key (&network_specs_key, database_name)
}


pub fn remove_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<()> {
    let meta_key = MetaKey::from_parts(network_name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());
    let events = {
        let database = open_db(database_name)?;
        let metadata = open_tree(&database, METATREE)?;
        match metadata.get(meta_key.key()) {
            Ok(Some(meta_stored)) => {
                let meta_values_display = MetaValuesDisplay::get(&MetaValues{name: network_name.to_string(), version: network_version, meta: meta_stored.to_vec()});
                vec![Event::MetadataRemoved(meta_values_display)]
            },
            Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned{name: network_name.to_string(), version: network_version}).show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        }
    };
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(events_to_batch(&database_name, events)?) // add corresponding history
        .apply(&database_name)
}

#[cfg(test)]
mod tests {
    use crate::{cold_default::reset_cold_database_no_addresses, identities::generate_test_identities, manage_history::{print_history}};
    use super::*;
    use std::fs;
    use sled::{Db, Tree, open};
    use definitions::{crypto::Encryption, keyring::{MetaKey, NetworkSpecsKey}, network_specs::Verifier, users::AddressDetails};
    
    fn check_for_network (name: &str, version: u32, dbname: &str) -> bool {
        let database: Db = open(dbname).unwrap();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        let meta_key = MetaKey::from_parts(name, version);
        metadata.contains_key(meta_key.key()).unwrap()
    }

    #[test]
    fn remove_all_westend() {
        let dbname = "tests/remove_all_westend";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        generate_test_identities(dbname).unwrap();
        
        let line = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode(line).unwrap(), &Encryption::Sr25519);
        remove_network_by_key (&network_specs_key, dbname).unwrap();
        
        {
            let database: Db = open(dbname).unwrap();
        
            let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
            assert!(chainspecs.get(&network_specs_key.key()).unwrap() == None, "Westend network specs were not deleted");
        
            let metadata: Tree = database.open_tree(METATREE).unwrap();
            let prefix_meta = String::from("westend").encode();
            assert!(metadata.scan_prefix(&prefix_meta).next() == None, "Some westend metadata was not deleted");
        
            let identities: Tree = database.open_tree(ADDRTREE).unwrap();
            for x in identities.iter() {
                if let Ok((_, address_details_encoded)) = x {
                    let address_details = <AddressDetails>::decode(&mut &address_details_encoded[..]).unwrap();
                    assert!(!address_details.network_id.contains(&network_specs_key), "Some westend identities still remain.");
                    assert!(address_details.network_id.len() != 0, "Did not remove address key entried with no network keys associated");
                }
            }
        }
        
        let history_printed = print_history(dbname).unwrap();
        assert!(history_printed.contains(r#"{"event":"database_initiated"}"#) && history_printed.contains(r##"{"event":"network_removed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"","encryption":"none"}}}}"##) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"}}"#) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","path":"//Alice","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#), "Expected different history:\n{}", history_printed);
        
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn remove_westend_9010() {
        let dbname = "tests/remove_westend_9010";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        
        let network_name = "westend";
        let network_version = 9010;
        
        assert!(check_for_network(network_name, network_version, dbname), "No westend 9010 to begin with.");
        
        remove_metadata (network_name, network_version, dbname).unwrap();
        
        assert!(!check_for_network(network_name, network_version, dbname), "Westend 9010 not removed.");
        
        fs::remove_dir_all(dbname).unwrap();
    }
}

