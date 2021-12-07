use anyhow;
use constants::{ADDRTREE, METATREE, SPECSTREE};
use definitions::{error::{ErrorSigner, ErrorSource, NotFoundSigner, Signer}, helpers::multisigner_to_public, history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay}, keyring::{AddressKey, NetworkSpecsKey, VerifierKey, MetaKeyPrefix, MetaKey}, metadata::MetaValues, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}, users::AddressDetails};
use parity_scale_codec::Encode;
use sled::Batch;

use crate::db_transactions::TrDbCold;
use crate::helpers::{open_db, open_tree, get_valid_current_verifier, get_general_verifier};
use crate::manage_history::events_to_batch;
use crate::network_details::get_network_specs_by_hex_key;

/// Function to remove the network with given NetworkSpecsKey from the database.
/// Removes network specs for all entries with same genesis hash.
/// Removes all metadata entries for corresponding network specname.
/// Removes all addresses corresponding to the networks removed (all encryptions).
/// If ValidCurrentVerifier is Custom(Verifier(None)), leaves it as that. If ValidCurrentVerifier is General, leaves it as General.
/// If ValidCurrentVerifier is Custom with some Verifier set, transforms CurrentVerifier from Valid into Dead to disable the network
/// permanently until Signer is wiped altogether.
/// Function is used only on the Signer side.
fn remove_network (network_specs_key_string: &str, database_name: &str) -> Result<(), ErrorSigner> {
    let mut address_batch = Batch::default();
    let mut meta_batch = Batch::default();
    let mut network_specs_batch = Batch::default();
    let mut verifiers_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    
    let general_verifier = get_general_verifier(&database_name)?;
    let network_specs = get_network_specs_by_hex_key(database_name, network_specs_key_string)?;
    
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, &database_name)?;

// modify verifier as needed    
    if let ValidCurrentVerifier::Custom(ref a) = valid_current_verifier {
        match a {
            Verifier(None) => (),
            _ => {
                verifiers_batch.remove(verifier_key.key());
                verifiers_batch.insert(verifier_key.key(), (CurrentVerifier::Dead).encode());
            },
        }
    }

    {
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
        let identities = open_tree::<Signer>(&database, ADDRTREE)?;

    // scan through chainspecs tree to mark for removal all networks with target genesis hash
        let mut keys_to_wipe: Vec<NetworkSpecsKey> = Vec::new();
        for x in chainspecs.iter() {
            if let Ok((network_specs_key_vec, entry)) = x {
                let x_network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
                let x_network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&x_network_specs_key, entry)?;
                if x_network_specs.genesis_hash == network_specs.genesis_hash {
                    network_specs_batch.remove(x_network_specs_key.key());
                    events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&x_network_specs, &valid_current_verifier, &general_verifier)));
                    keys_to_wipe.push(x_network_specs_key);
                }
            }
        }
    // scan through metadata tree to mark for removal all networks with target name
        let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
        for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
            if let Ok((meta_key_vec, meta_stored)) = x {
                let meta_key = MetaKey::from_ivec(&meta_key_vec);
                meta_batch.remove(meta_key.key());
                if let Ok((name, version)) = meta_key.name_version::<Signer>() {
                    let meta_values_display = MetaValuesDisplay::get(&MetaValues{name, version, meta: meta_stored.to_vec()});
                    events.push(Event::MetadataRemoved(meta_values_display));
                }
            }
        }
    // scan through address tree to clean up the network_key(s) from identities
        for x in identities.iter() {
            if let Ok((address_key_vec, entry)) = x {
                let address_key = AddressKey::from_ivec(&address_key_vec);
                let (multisigner, mut address_details) = AddressDetails::process_entry_checked::<Signer>((address_key_vec, entry))?;
                for key in keys_to_wipe.iter() {
                    if address_details.network_id.contains(key) {
                        let identity_history = IdentityHistory::get(&address_details.seed_name, &address_details.encryption, &multisigner_to_public(&multisigner), &address_details.path, &network_specs.genesis_hash.to_vec());
                        events.push(Event::IdentityRemoved(identity_history));
                        address_details.network_id = address_details.network_id.into_iter().filter(|id| id != key).collect();
                    }
                }
                if address_details.network_id.is_empty() {address_batch.remove(address_key.key())}
                else {address_batch.insert(address_key.key(), address_details.encode())}
            }
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply::<Signer>(&database_name)
}

/// Wrapper for remove_network open to user interface.
/// Used only on the Signer side.
pub fn remove_network_by_hex (network_specs_key_string: &str, database_name: &str) -> anyhow::Result<()> {
    remove_network(network_specs_key_string, database_name).map_err(|e| e.anyhow())
}

pub fn remove_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<()> {
    let meta_key = MetaKey::from_parts(network_name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());
    let history_batch = get_batch_remove_unchecked_meta(database_name, network_name, network_version).map_err(|e| e.anyhow())?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply::<Signer>(&database_name)
        .map_err(|e| e.anyhow())
}


fn get_batch_remove_unchecked_meta (database_name: &str, network_name: &str, network_version: u32) -> Result<Batch, ErrorSigner> {
    let events = {
        let meta_key = MetaKey::from_parts(network_name, network_version);
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        match metadata.get(meta_key.key()) {
            Ok(Some(meta_stored)) => {
                let meta_values_display = MetaValuesDisplay::get(&MetaValues{name: network_name.to_string(), version: network_version, meta: meta_stored.to_vec()});
                vec![Event::MetadataRemoved(meta_values_display)]
            },
            Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Metadata{name: network_name.to_string(), version: network_version})),
            Err(e) => return Err(<Signer>::db_internal(e)),
        }
    };
    events_to_batch::<Signer>(&database_name, events)
}

#[cfg(test)]
mod tests {
    use crate::{cold_default::{populate_cold, reset_cold_database_no_addresses}, manage_history::{print_history}};
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
        let dbname = "for_tests/remove_all_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        
        let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode(genesis_hash).unwrap(), &Encryption::Sr25519);
        let network_specs_key_string = hex::encode(network_specs_key.key());
        remove_network_by_hex (&network_specs_key_string, dbname).unwrap();
        
        {
            let database: Db = open(dbname).unwrap();
            let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
            assert!(chainspecs.get(&network_specs_key.key()).unwrap() == None, "Westend network specs were not deleted");
            let metadata: Tree = database.open_tree(METATREE).unwrap();
            let prefix_meta = String::from("westend").encode();
            assert!(metadata.scan_prefix(&prefix_meta).next() == None, "Some westend metadata was not deleted");
            let identities: Tree = database.open_tree(ADDRTREE).unwrap();
            for x in identities.iter() {
                if let Ok(a) = x {
                    let (_, address_details) = AddressDetails::process_entry_checked::<Signer>(a).unwrap();
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
        let dbname = "for_tests/remove_westend_9010";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        let network_name = "westend";
        let network_version = 9010;
        assert!(check_for_network(network_name, network_version, dbname), "No westend 9010 to begin with.");
        remove_metadata (network_name, network_version, dbname).unwrap();
        assert!(!check_for_network(network_name, network_version, dbname), "Westend 9010 not removed.");
        fs::remove_dir_all(dbname).unwrap();
    }
}
