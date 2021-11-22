use constants::{METATREE, SPECSTREE};
use definitions::{keyring::{MetaKeyPrefix, NetworkSpecsKey}};
use anyhow;
use sled::Batch;

use crate::helpers::{open_db, open_tree, decode_chain_specs};
use crate::db_transactions::TrDbCold;

/// Function to transfer metadata content into cold database
/// Checks that only networks with network specs already in cold database are processed,
/// so that no metadata without associated network specs enters the cold database
pub fn transfer_metadata_to_cold (database_name_hot: &str, database_name_cold: &str) -> anyhow::Result<()> {
    let mut for_metadata = Batch::default();
    {
        let database_hot = open_db(database_name_hot)?;
        let metadata_hot = open_tree(&database_hot, METATREE)?;
        let database_cold = open_db(database_name_cold)?;
        let chainspecs_cold = open_tree(&database_cold, SPECSTREE)?;
        for x in chainspecs_cold.iter() {
            if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
                let network_specs = decode_chain_specs(network_specs_encoded, &NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec()))?;
                for y in metadata_hot.scan_prefix(MetaKeyPrefix::from_name(&network_specs.name).prefix()) {
                    if let Ok((key, value)) = y {for_metadata.insert(key, value);}
                }
            }
        }
    }
    TrDbCold::new()
        .set_metadata(for_metadata)
        .apply(database_name_cold)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cold_default::reset_cold_database_no_addresses, db_transactions::TrDbCold, helpers::{open_db, open_tree}, hot_default::reset_hot_database};
    use constants::METATREE;
    use definitions::{keyring::MetaKey, network_specs::Verifier};
    use meta_reading::decode_metadata::decode_version;
    
    /// This is a crude function intended for tests only.
    /// Probably will want at some point same function for at least cold databases,
    /// carefully written.
    fn insert_metadata_from_file (database_name: &str, filename: &str) {
        let meta_from_file = std::fs::read_to_string(filename).unwrap();
        let meta_values = decode_version(&meta_from_file.trim()).unwrap();
        let mut meta_batch = Batch::default();
        meta_batch.insert(MetaKey::from_parts(&meta_values.name, meta_values.version).key(), meta_values.meta);
        TrDbCold::new()
            .set_metadata(meta_batch)
            .apply(database_name).unwrap();
    }
    
    fn metadata_len(database_name: &str) -> usize {
        let database = open_db(database_name).unwrap();
        let metadata = open_tree(&database, METATREE).unwrap();
        metadata.len()
    }
    fn metadata_contents(database_name: &str) -> Vec<(String, u32)> {
        let database = open_db(database_name).unwrap();
        let metadata = open_tree(&database, METATREE).unwrap();
        let mut out: Vec<(String, u32)> = Vec::new();
        for x in metadata.iter() {
            if let Ok((meta_key_vec, _)) = x {
                out.push(MetaKey::from_vec(&meta_key_vec.to_vec()).name_version().unwrap());
            }
        }
        out
    }
    
    #[test]
    fn test_metadata_transfer() {
        let dbname_hot = "tests/test_metadata_transfer_mock_hot";
        reset_hot_database(dbname_hot).unwrap();
        let dbname_cold = "tests/test_metadata_transfer_mock_cold";
        reset_cold_database_no_addresses(dbname_cold, Verifier(None)).unwrap();
        
        insert_metadata_from_file(dbname_hot, "tests/westend9010");
        assert!(metadata_len(dbname_hot) == 1, "Fresh hot database, should have only the single network added.");
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("rococo", 9103), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("rococo", 9103), ("westend", 9000), ("westend", 9010), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        insert_metadata_from_file(dbname_hot, "tests/westend9090");
        assert!(metadata_len(dbname_hot) == 2, "Now 2 entries in hot db.");
        transfer_metadata_to_cold(dbname_hot, dbname_cold).unwrap();
        assert!(format!("{:?}", metadata_contents(dbname_cold)) == r#"[("kusama", 2030), ("rococo", 9103), ("westend", 9000), ("westend", 9010), ("westend", 9090), ("polkadot", 30)]"#, "expected: \n{:?}", metadata_contents(dbname_cold));
        
        std::fs::remove_dir_all(dbname_hot).unwrap();
        std::fs::remove_dir_all(dbname_cold).unwrap();
    }
}
