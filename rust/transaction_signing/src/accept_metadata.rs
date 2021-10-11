use constants::{ADDMETAVERIFIER, GENERALVERIFIER, HISTORY, LOADMETA, METATREE, SETTREE, VERIFIERS, TRANSACTION};
use definitions::{history::Event, metadata::{MetaValuesDisplay}, network_specs::NetworkVerifier, transactions::Transaction};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use db_handling::{helpers::{open_db, open_tree, flush_db, insert_into_tree}, manage_history::{enter_events_into_tree}};
use blake2_rfc::blake2b::blake2b;

use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;

/// function to add approved metadata for known network to the database;

pub fn accept_metadata (database_name: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let history = open_tree(&database, HISTORY)?;
    
    let action = match transaction.remove(LOADMETA) {
        Ok(Some(encoded_action)) => match <Transaction>::decode(&mut &encoded_action[..]) {
            Ok(Transaction::LoadMeta(x)) => x,
            Ok(_) => return Err(Error::NoAction(ActionFailure::LoadMeta).show()),
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::LoadMeta).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::LoadMeta).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    flush_db(&database)?;
    
    let mut events = action.history;
    let meta_values_display = MetaValuesDisplay {
        name: &action.versioned_name.name,
        version: action.versioned_name.version,
        meta_hash: &hex::encode(blake2b(32, &[], &action.meta).as_bytes()),
    }.show();
    events.push(Event::MetadataAdded(meta_values_display));
    
    let metadata = open_tree(&database, METATREE)?;
    insert_into_tree(action.versioned_name.encode(), action.meta, &metadata)?;
    flush_db(&database)?;
    
    if upd_general {
        events.push(Event::GeneralVerifierAdded(action.verifier.show_card()));
        let settings = open_tree(&database, SETTREE)?;
        insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    if let Some(verifier_key) = action.upd_network {
        let network_verifier_show = NetworkVerifier {
            verifier_key: &hex::encode(&verifier_key),
            verifier_line: action.verifier.show_card(),
        }.show();
        events.push(Event::MetadataVerifierAdded(network_verifier_show));
        let verifiers = open_tree(&database, VERIFIERS)?;
        insert_into_tree(verifier_key.to_vec(), action.verifier.encode(), &verifiers)?;
        flush_db(&database)?;
    }
    
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    
    if upd_general {Ok(String::from("Metadata successfully loaded. General verifier successfully updated."))}
    else {Ok(String::from("Metadata successfully loaded."))}
    
}


/// function to add approved metadata for known network to the database;
/// flag upd_general indicates if general verifier should be updated as well;

pub fn add_meta_verifier (database_name: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let history = open_tree(&database, HISTORY)?;
    
    let action = match transaction.remove(ADDMETAVERIFIER) {
        Ok(Some(encoded_action)) => match <Transaction>::decode(&mut &encoded_action[..]) {
            Ok(Transaction::UpdMetaVerifier(x)) => x,
            Ok(_) => return Err(Error::NoAction(ActionFailure::AddVerifier).show()),
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddVerifier).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::AddVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    let mut events = action.history;
    let network_verifier_show = NetworkVerifier {
        verifier_key: &hex::encode(&action.verifier_key),
        verifier_line: action.verifier.show_card(),
    }.show();
    events.push(Event::MetadataVerifierAdded(network_verifier_show));
    let verifiers = open_tree(&database, VERIFIERS)?;
    insert_into_tree(action.verifier_key.to_vec(), action.verifier.encode(), &verifiers)?;
    flush_db(&database)?;
    
    if upd_general {
        events.push(Event::GeneralVerifierAdded(action.verifier.show_card()));
        let settings = open_tree(&database, SETTREE)?;
        insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }
    
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    
    if upd_general {Ok(String::from("Network verifier successfully updated. General verifier successfully updated."))}
    else {Ok(String::from("Network verifier successfully updated."))}
}
