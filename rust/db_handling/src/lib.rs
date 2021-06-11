pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::load_chainspecs;

pub mod users;
use users::load_users;

pub mod settings;
use settings::load_types;

mod db_utils;
mod constants;

/// struct to store three important databases: chain_spec, metadata, and types_info
pub struct DataFiles<'a> {
    pub chain_spec_database: &'a str,
    pub metadata_contents: &'a str,
    pub types_info: &'a str,
    pub identities: &'a str,
}

pub fn fill_database_from_files (dbname: &str, datafiles: DataFiles) -> Result<(), Box<dyn std::error::Error>> {
    
    load_metadata(dbname, datafiles.metadata_contents)?;
    load_chainspecs(dbname, datafiles.chain_spec_database)?;
    load_users(dbname, datafiles.identities)?;
    load_types(dbname, datafiles.types_info)?;
    
    Ok(())
    
}

#[cfg(tests)]
mod tests {
    use super::*;
}
