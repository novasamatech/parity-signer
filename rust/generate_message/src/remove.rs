//! Removing data from the hot database
//!
//! Users can remove from the database a single metadata entry for the network
//! or all information about the network altogether.
//!
//! ## Remove a metadata entry
//!
//! ### Command line
//!
//! `$ cargo run remove -name <network_name> -version <network_version>`
//!
//! ### Example
//!
//! `$ cargo run remove -name westend -version 9200`
//!
//! Note that this will remove only the selected metadata from [`METATREE`] tree
//! and will not affect [`ADDRESS_BOOK`](constants::ADDRESS_BOOK),
//! [`META_HISTORY`] or [`SPECSTREEPREP`](constants::SPECSTREEPREP) trees of the
//! database.
//!
//! If the same version of the network metadata is loaded again, the
//! [`META_HISTORY`] entry get rewritten, so that the metadata and block hash
//! are always matching in the database.
//!
//! ## Remove all data associated with a network
//!
//! ### Command line
//!
//! `$ cargo run remove title <address_book_title>`
//!
//! Network title in the address book is unique identifier for a network with
//! given encryption. It always is constructed as
//! `<network_name>-<network_encryption>`.
//!
//! Removed data associated with the address book title consists of:
//!
//! - address book entry
//!   [`AddressBookEntry`](definitions::metadata::AddressBookEntry) from
//!   [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree
//! - network specs
//!   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) from
//!   [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree
//! - all associated metadata entries from [`METATREE`] if there are no other
//!   address book entries this metadata is associated with
//! - all associated meta block history entries from [`META_HISTORY`] if there
//!   are no other address book entries this block history entries are associated
//!   with
//!
//! Note that single address book entry corresponds to single network specs
//! entry, and they are created and removed always simultaneously.
//!
//! ### Example
//!
//! `$ cargo run remove title westend-sr25519`
//!
//! If one of the default networks gets removed, it could be added back,
//! however, it will not be marked as default anymore.
use constants::{METATREE, META_HISTORY};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{get_meta_values_by_name_version, open_tree},
};
use definitions::keyring::{AddressBookKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey};
use sled::Batch;

use crate::error::Result;
use crate::helpers::{get_address_book_entry, is_specname_in_db};
use crate::parser::Remove;

/// Remove information from the database.
pub fn remove_info(database: &sled::Db, info: Remove) -> Result<()> {
    match info {
        // network data by the address book title
        Remove::Title { t: network_title } => {
            // init `Batch` for `ADDRESS_BOOK`, `SPECSTREEPREP`, `METADATA` and
            // `META_HISTORY`
            let mut address_book_batch = Batch::default();
            let mut metadata_batch = Batch::default();
            let mut meta_history_batch = Batch::default();
            let mut network_specs_prep_batch = Batch::default();

            // get `ADDRESS_BOOK` entry for the title
            let address_book_entry = get_address_book_entry(database, &network_title)?;

            // make `NetworkSpecsKey` using data in `ADDRESS_BOOK` entry
            let network_specs_key = NetworkSpecsKey::from_parts(
                &address_book_entry.genesis_hash,
                &address_book_entry.encryption,
            );

            // finalize `Batch` for `ADDRESS_BOOK`
            address_book_batch.remove(AddressBookKey::from_title(&network_title).key());

            // finalize `Batch` for `SPECSTREEPREP`
            network_specs_prep_batch.remove(network_specs_key.key());

            // if the address book has no more entries with same network name,
            // except the one currently being removed, the metadata and
            // the block history entries get removed
            if !is_specname_in_db(database, &address_book_entry.name, &network_title)? {
                let metadata = open_tree(database, METATREE)?;
                let meta_history = open_tree(database, META_HISTORY)?;
                let meta_key_prefix = MetaKeyPrefix::from_name(&address_book_entry.name);
                for (x, _) in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
                    // add element to `Batch` for `METATREE`
                    metadata_batch.remove(x)
                }
                for (x, _) in meta_history.scan_prefix(meta_key_prefix.prefix()).flatten() {
                    // add element to `Batch` for `META_HISTORY`
                    meta_history_batch.remove(x)
                }
            }
            TrDbHot::new()
                .set_address_book(address_book_batch)
                .set_metadata(metadata_batch)
                .set_meta_history(meta_history_batch)
                .set_network_specs_prep(network_specs_prep_batch)
                .apply(database)?;
        }

        // network metadata by network name and version
        Remove::SpecNameVersion { name, version } => {
            let mut metadata_batch = Batch::default();
            get_meta_values_by_name_version(database, &name, version)?;
            let meta_key = MetaKey::from_parts(&name, version);
            metadata_batch.remove(meta_key.key());
            TrDbHot::new()
                .set_metadata(metadata_batch)
                .apply(database)?;
        }
    };

    Ok(())
}
