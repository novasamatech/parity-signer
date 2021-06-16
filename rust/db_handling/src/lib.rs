pub mod metadata;
use metadata::load_metadata;

pub mod chainspecs;
use chainspecs::load_chainspecs;

pub mod identities;
use identities::load_users;

pub mod settings;
use settings::load_types;

mod db_utils;
mod constants;
mod default_type_defs;

/// struct to store three important databases: chain_spec, metadata, and types_info
pub struct DataFiles<'a> {
    pub metadata_contents: &'a str,
    pub identities: &'a str,
}

pub fn fill_database_from_files (dbname: &str, datafiles: DataFiles) -> Result<(), Box<dyn std::error::Error>> {
    let type_defs = default_type_defs::get_default_type_def();

    load_metadata(dbname, datafiles.metadata_contents)?;
    load_chainspecs(dbname)?;
    load_users(dbname, datafiles.identities)?;
    load_types(dbname, &type_defs)?;
    
    Ok(())
    
}

#[cfg(tests)]
mod tests {
    use super::*;
}
