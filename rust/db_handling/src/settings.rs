use parity_scale_codec::Encode;
use constants::{SETTREE, TYPES, GENERALVERIFIER};
use definitions::{defaults::get_default_types, network_specs::Verifier};
use anyhow;

use crate::error::Error;
use crate::helpers::{open_db, open_tree, flush_db, insert_into_tree, remove_from_tree};


/// Load default types

pub fn load_types (database_name: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    remove_from_tree(TYPES.to_vec(), &settings)?;
    
    let types_prep = match get_default_types() {
        Ok(x) => x,
        Err(e) => return Err(Error::BadTypesFile(e).show()),
    };
    insert_into_tree(TYPES.to_vec(), types_prep.encode(), &settings)?;
    flush_db(&database)?;
    Ok(())
}

/// Set verifier signature for types definitions and for accepting new networks

pub fn set_general_verifier (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    remove_from_tree(GENERALVERIFIER.to_vec(), &settings)?;
    insert_into_tree(GENERALVERIFIER.to_vec(), general_verifier.encode(), &settings)?;
    flush_db(&database)?;
    Ok(())
}

