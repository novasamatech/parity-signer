use constants::{HOT_DB_NAME, METATREE};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{open_db, open_tree},
};
use definitions::{
    error::{Active, ErrorActive},
    keyring::{AddressBookKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey},
};
use sled::Batch;

use crate::helpers::{get_address_book_entry, specname_in_db};
use crate::parser::Remove;

/// Function to remove information from the database.
pub fn remove_info(info: Remove) -> Result<(), ErrorActive> {
    match info {
        Remove::Title(network_title) => {
            let mut address_book_batch = Batch::default();
            let mut metadata_batch = Batch::default();
            let mut network_specs_prep_batch = Batch::default();
            let address_book_entry = get_address_book_entry(&network_title)?;
            let network_specs_key = NetworkSpecsKey::from_parts(
                &address_book_entry.genesis_hash,
                &address_book_entry.encryption,
            );
            address_book_batch.remove(AddressBookKey::from_title(&network_title).key());
            network_specs_prep_batch.remove(network_specs_key.key());
            let mut meta_to_del: Vec<MetaKey> = Vec::new();
            if !specname_in_db(&address_book_entry.name, &network_title)? {
                let database = open_db::<Active>(HOT_DB_NAME)?;
                let metadata = open_tree::<Active>(&database, METATREE)?;
                let meta_key_prefix = MetaKeyPrefix::from_name(&address_book_entry.name);
                for (x, _) in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
                    meta_to_del.push(MetaKey::from_ivec(&x))
                }
            }
            for x in meta_to_del.iter() {
                metadata_batch.remove(x.key())
            }
            TrDbHot::new()
                .set_address_book(address_book_batch)
                .set_metadata(metadata_batch)
                .set_network_specs_prep(network_specs_prep_batch)
                .apply(HOT_DB_NAME)
        }
        Remove::SpecNameVersion { name, version } => {
            let mut metadata_batch = Batch::default();
            let meta_key = MetaKey::from_parts(&name, version);
            metadata_batch.remove(meta_key.key());
            TrDbHot::new()
                .set_metadata(metadata_batch)
                .apply(HOT_DB_NAME)
        }
    }
}
