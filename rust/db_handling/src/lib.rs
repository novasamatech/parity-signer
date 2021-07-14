use sled::{Db, open};
use definitions::{constants::{COLD_DB_NAME, HOT_DB_NAME}, network_specs::Verifier};
use std::fs;

pub mod address_book;
use address_book::load_address_book;

pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::{load_chainspecs, load_chainspecs_to_send};

pub mod identities;
use identities::load_test_identities;

pub mod settings;
use settings::{load_types, set_general_verifier};

pub mod network_details;

/// Function to manually purge the database.
/// Used to have issues without purge even if the database was physically removed from the device and created again.
/// Function will remain here and in use for time being.
fn purge (dbname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(dbname)?;
    let trees = database.tree_names();
    
    for x in trees.iter() {
        if x != b"__sled__default" {
            database.drop_tree(x)?;
        }
    }
    database.flush()?;
    Ok(())
}


/// Function to re-populate "cold" database with default values.
/// Flag testing = true indicates if Alice & Co test identities should be added to ADDRTREE

pub fn populate_cold (dbname: &str, metadata_filename: &str, testing: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    purge(dbname)?;

    let general_verifier = Verifier::None;
    
    let metadata = match fs::read_to_string(metadata_filename) {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Metadata database missing")),
    };

    load_metadata(dbname, &metadata)?;
    load_chainspecs(dbname)?;
    load_types(dbname)?;
    set_general_verifier(dbname, general_verifier)?;
    if testing {load_test_identities(dbname)?}
    
    Ok(())
    
}


/// Function to re-populate default "cold" database with default values.
/// Currently this cold database is used for transaction_parsing crate
/// and needs Alice & Co identities

pub fn default_cold () -> Result<(), Box<dyn std::error::Error>> {
    
    let dbname = COLD_DB_NAME;
    let metadata_filename = "metadata_database.ts";
    
    populate_cold(dbname, metadata_filename, true)
}


/// Function to re-populate "hot" database with default values.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn populate_hot (dbname: &str) -> Result<(), Box<dyn std::error::Error>> {

    purge(dbname)?;
    
    load_chainspecs_to_send(dbname)?;
    load_address_book(dbname)?;
    load_types(dbname)?;
    
    Ok(())
    
}

/// Function to re-populate default "hot" database with defaults.
/// No metadata is added here, all metadata entries will come from
/// meta_reading and/or generate_message

pub fn default_hot () -> Result<(), Box<dyn std::error::Error>> {
    
    let dbname = HOT_DB_NAME;
    populate_hot(dbname)
    
}
