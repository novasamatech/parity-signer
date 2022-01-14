use sled::Batch;

use constants::{METATREE, SPECSTREE};
use definitions::{error::{Active, ErrorActive}, keyring::MetaKeyPrefix, network_specs::NetworkSpecs};

use crate::helpers::{open_db, open_tree};
use crate::db_transactions::TrDbCold;

/// Function to transfer metadata content into cold database
/// Checks that only networks with network specs already in cold database are processed,
/// so that no metadata without associated network specs enters the cold database.
/// Function applicable only to Active side.
pub fn transfer_metadata_to_cold (database_name_hot: &str, database_name_cold: &str) -> Result<(), ErrorActive> {
    let mut for_metadata = Batch::default();
    {
        let database_hot = open_db::<Active>(database_name_hot)?;
        let metadata_hot = open_tree::<Active>(&database_hot, METATREE)?;
        let database_cold = open_db::<Active>(database_name_cold)?;
        let chainspecs_cold = open_tree::<Active>(&database_cold, SPECSTREE)?;
        for x in chainspecs_cold.iter() {
            if let Ok(a) = x {
                let network_specs = NetworkSpecs::from_entry_checked::<Active>(a)?;
                for y in metadata_hot.scan_prefix(MetaKeyPrefix::from_name(&network_specs.name).prefix()) {
                    if let Ok((key, value)) = y {for_metadata.insert(key, value);}
                }
            }
        }
    }
    TrDbCold::new()
        .set_metadata(for_metadata)
        .apply::<Active>(database_name_cold)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cold_default::populate_cold, db_transactions::TrDbCold, helpers::{open_db, open_tree}, hot_default::reset_hot_database};
    use constants::METATREE;
    use definitions::{error::{Active, IncomingMetadataSourceActive}, keyring::MetaKey, metadata::MetaValues, network_specs::Verifier};
    
    fn insert_metadata_from_file (database_name: &str, filename: &str) {
        let meta_str = std::fs::read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta_str.trim(), IncomingMetadataSourceActive::Default{filename: filename.to_string()}).unwrap();
        let mut meta_batch = Batch::default();
        meta_batch.insert(MetaKey::from_parts(&meta_values.name, meta_values.version).key(), meta_values.meta);
        TrDbCold::new()
            .set_metadata(meta_batch)
            .apply::<Active>(database_name).unwrap();
    }
    
    fn metadata_len(database_name: &str) -> usize {
        let database = open_db::<Active>(database_name).unwrap();
        let metadata = open_tree::<Active>(&database, METATREE).unwrap();
        metadata.len()
    }
    fn metadata_contents(database_name: &str) -> Vec<(String, u32)> {
        let database = open_db::<Active>(database_name).unwrap();
        let metadata = open_tree::<Active>(&database, METATREE).unwrap();
        let mut out: Vec<(String, u32)> = Vec::new();
        for x in metadata.iter() {
            if let Ok((meta_key_vec, _)) = x {
                let new = MetaKey::from_ivec(&meta_key_vec).name_version::<Active>().unwrap();
                out.push(new);
            }
        }
        out
    }
    
    #[test]
    fn test_metadata_transfer() {
        let dbname_hot = "for_tests/test_metadata_transfer_mock_hot";
        reset_hot_database(dbname_hot).unwrap();
        let dbname_cold = "for_tests/test_metadata_transfer_mock_cold";
        populate_cold(dbname_cold, Verifier(None)).unwrap();
        
        insert_metadata_from_file(dbname_hot, "for_tests/westend9010");
        assert!(metadata_len(dbname_hot) == 1, "Fresh hot database, should have only the single network added.");
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        insert_metadata_from_file(dbname_hot, "for_tests/westend9090");
        assert!(metadata_len(dbname_hot) == 2, "Now 2 entries in hot db.");
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("westend", 9000), ("westend", 9010), ("westend", 9090), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        std::fs::remove_dir_all(dbname_hot).unwrap();
        std::fs::remove_dir_all(dbname_cold).unwrap();
    }
}

