use sled::Batch;

use constants::{METATREE, SPECSTREE};
use definitions::{
    error_active::{Active, ErrorActive},
    keyring::MetaKeyPrefix,
    network_specs::NetworkSpecs,
};

use crate::db_transactions::TrDbCold;
use crate::helpers::{open_db, open_tree};

/// Function to transfer metadata content into cold database
/// Checks that only networks with network specs already in cold database are processed,
/// so that no metadata without associated network specs enters the cold database.
/// Function applicable only to Active side.
pub fn transfer_metadata_to_cold(
    database_name_hot: &str,
    database_name_cold: &str,
) -> Result<(), ErrorActive> {
    let mut for_metadata = Batch::default();
    {
        let database_hot = open_db::<Active>(database_name_hot)?;
        let metadata_hot = open_tree::<Active>(&database_hot, METATREE)?;
        let database_cold = open_db::<Active>(database_name_cold)?;
        let chainspecs_cold = open_tree::<Active>(&database_cold, SPECSTREE)?;
        for x in chainspecs_cold.iter().flatten() {
            let network_specs = NetworkSpecs::from_entry_checked::<Active>(x)?;
            for (key, value) in metadata_hot
                .scan_prefix(MetaKeyPrefix::from_name(&network_specs.name).prefix())
                .flatten()
            {
                for_metadata.insert(key, value)
            }
        }
    }
    TrDbCold::new()
        .set_metadata(for_metadata)
        .apply::<Active>(database_name_cold)
}
