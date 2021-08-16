use definitions::{constants::{COLD_DB_NAME, HOT_DB_NAME}, network_specs::Verifier};
use std::fs;
use anyhow;

pub mod address_book;
use address_book::load_address_book;

pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::{load_chainspecs, load_chainspecs_to_send};

pub mod error;
use error::Error;

pub mod identities;
use identities::load_test_identities;

pub mod helpers;
use helpers::{open_db, drop_tree, flush_db};

pub mod manage_history;

pub mod prep_messages;

pub mod settings;
use settings::{load_types, set_general_verifier};

pub mod network_details;
pub mod remove_network;

/// Function to manually purge the database.
/// Used to have issues without purge even if the database was physically removed from the device and created again.
/// Function will remain here and in use for time being.
fn purge (database_name: &str) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    let trees = database.tree_names();
    
    for x in trees.iter() {
        if x != b"__sled__default" {
            drop_tree(&database, x)?;
        }
    }
    flush_db(&database)?;
    Ok(())
}


/// Function to re-populate "cold" database with default values.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold (database_name: &str, metadata_filename: &str, testing: bool) -> anyhow::Result<()> {
    
    populate_cold_no_meta (database_name, testing)?;
    
    let metadata = match fs::read_to_string(metadata_filename) {
        Ok(x) => x,
        Err(e) => return Err(Error::MetadataDefaultFile(e.to_string()).show()),
    };

    load_metadata(database_name, &metadata)?;
    Ok(())
    
}

/// Function to re-populate "cold" database with default values, without any metadata added.
/// For tests.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold_no_meta (database_name: &str, testing: bool) -> anyhow::Result<()> {
    
    populate_cold_no_networks(database_name)?;
    load_chainspecs(database_name)?;
    if testing {load_test_identities(database_name)?}
    Ok(())
    
}

/// Function to re-populate "cold" database with default values, without any network information added.
/// For tests.

pub fn populate_cold_no_networks (database_name: &str) -> anyhow::Result<()> {
    
    purge(database_name)?;

    let general_verifier = Verifier::None;
    
    load_types(database_name)?;
    set_general_verifier(database_name, general_verifier)?;
    
    Ok(())
    
}

/// Function to re-populate default "cold" database with default values.
/// This database should be copied into Signer's resources.

pub fn default_cold_release () -> anyhow::Result<()> {
    
    let database_name = COLD_DB_NAME;
    let metadata_filename = "metadata_database.ts";
    
    populate_cold(database_name, metadata_filename, false)
}

/// Function to re-populate default "cold" database with default values.
/// Currently this cold database is used for transaction_parsing crate
/// and needs Alice & Co identities

pub fn default_cold () -> anyhow::Result<()> {
    
    let database_name = COLD_DB_NAME;
    let metadata_filename = "metadata_database.ts";
    
    populate_cold(database_name, metadata_filename, true)
}


/// Function to re-populate "hot" database with default values.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn populate_hot (database_name: &str) -> anyhow::Result<()> {

    purge(database_name)?;
    
    load_chainspecs_to_send(database_name)?;
    load_address_book(database_name)?;
    load_types(database_name)?;
    
    Ok(())
    
}

/// Function to re-populate default "hot" database with defaults.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn default_hot () -> anyhow::Result<()> {
    
    let database_name = HOT_DB_NAME;
    populate_hot(database_name)
    
}
