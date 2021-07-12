use sled::{Db, open};
use definitions::{constants::{COLD_DB_NAME, HOT_DB_NAME}, network_specs::Verifier};

pub mod address_book;
use address_book::load_address_book;

pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::{load_chainspecs, load_chainspecs_to_send};

pub mod identities;

pub mod settings;
use settings::{load_types, set_general_verifier};

mod db_utils;

/// struct to store three important databases: chain_spec, metadata, and types_info
pub struct DataFiles<'a> {
    pub metadata_contents: &'a str,
}


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
/// Used for tests in signing crate

pub fn populate_cold (dbname: &str, datafiles: DataFiles) -> Result<(), Box<dyn std::error::Error>> {
    
    purge(dbname)?;

    let general_verifier = Verifier::None;

    load_metadata(dbname, datafiles.metadata_contents)?;
    load_chainspecs(dbname)?;
    load_types(dbname)?;
    set_general_verifier(dbname, general_verifier)?;
    
    Ok(())
    
}


/// Function to re-populate default "cold" database with default values.

pub fn default_cold (datafiles: DataFiles) -> Result<(), Box<dyn std::error::Error>> {
    
    let dbname = COLD_DB_NAME;
    populate_cold(dbname, datafiles)
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
