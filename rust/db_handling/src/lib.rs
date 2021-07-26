use sled::{Db, open};
use definitions::{constants::{COLD_DB_NAME, HOT_DB_NAME}, network_specs::Verifier};
use std::fs;
use anyhow;

pub mod address_book;
use address_book::load_address_book;

pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::{load_chainspecs, load_chainspecs_to_send};

mod error;
use error::{Error};

pub mod identities;
use identities::load_test_identities;

pub mod prep_messages;

pub mod settings;
use settings::{load_types, set_general_verifier};

pub mod network_details;
pub mod remove_network;

/// Function to manually purge the database.
/// Used to have issues without purge even if the database was physically removed from the device and created again.
/// Function will remain here and in use for time being.
fn purge (dbname: &str) -> anyhow::Result<()> {
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let trees = database.tree_names();
    
    for x in trees.iter() {
        if x != b"__sled__default" {
            match database.drop_tree(x) {
                Ok(_) => (),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            };
        }
    }
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    Ok(())
}


/// Function to re-populate "cold" database with default values.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold (dbname: &str, metadata_filename: &str, testing: bool) -> anyhow::Result<()> {
    
    populate_cold_no_meta (dbname, testing)?;
    
    let metadata = match fs::read_to_string(metadata_filename) {
        Ok(x) => x,
        Err(e) => return Err(Error::MetadataDefaultFile(e.to_string()).show()),
    };

    load_metadata(dbname, &metadata)?;
    Ok(())
    
}

/// Function to re-populate "cold" database with default values, without any metadata added.
/// For tests.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold_no_meta (dbname: &str, testing: bool) -> anyhow::Result<()> {
    
    populate_cold_no_networks(dbname)?;
    load_chainspecs(dbname)?;
    if testing {load_test_identities(dbname)?}
    Ok(())
    
}

/// Function to re-populate "cold" database with default values, without any network information added.
/// For tests.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold_no_networks (dbname: &str) -> anyhow::Result<()> {
    
    purge(dbname)?;

    let general_verifier = Verifier::None;
    
    load_types(dbname)?;
    set_general_verifier(dbname, general_verifier)?;
    
    Ok(())
    
}


/// Function to re-populate default "cold" database with default values.
/// Currently this cold database is used for transaction_parsing crate
/// and needs Alice & Co identities

pub fn default_cold () -> anyhow::Result<()> {
    
    let dbname = COLD_DB_NAME;
    let metadata_filename = "metadata_database.ts";
    
    populate_cold(dbname, metadata_filename, true)
}


/// Function to re-populate "hot" database with default values.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn populate_hot (dbname: &str) -> anyhow::Result<()> {

    purge(dbname)?;
    
    load_chainspecs_to_send(dbname)?;
    load_address_book(dbname)?;
    load_types(dbname)?;
    
    Ok(())
    
}

/// Function to re-populate default "hot" database with defaults.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn default_hot () -> anyhow::Result<()> {
    
    let dbname = HOT_DB_NAME;
    populate_hot(dbname)
    
}
