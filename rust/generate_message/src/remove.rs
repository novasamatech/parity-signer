//! Removing data from the hot database
//!
//! Users can remove from the database a single metadata entry for the network
//! or all information about the network altogether.
//!
//! ## Remove a metadata entry
//!
//! ### Command line
//!
//! `$ cargo run remove -name <network name> -version <network version>`
//!
//! ### Example
//!
//! `$ cargo run remove -name westend -version 9200`
//!
//! Note that this will remove only the selected metadata from [`METATREE`] tree
//! and will not affect [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) or
//! [`SPECSTREEPREP`](constants::SPECSTREEPREP) trees of the database.
//!
//! ## Remove all data associated with a network
//!
//! ### Command line
//!
//! `$ cargo run remove -title <network address book title>`
//!
//! Network title in the address book is unique identifier for a network with
//! given encryption.
//!
//! Removed data associated with the address book title consists of:
//!
//! - address book entry
//! [`AddressBookEntry`](definitions::metadata::AddressBookEntry) from
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree
//! - network specs
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) from
//! [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree
//! - all associated metadata entries from [`METATREE`] if there are no other
//! address book entries this metadata is associated with
//!
//! Note that single address book entry corresponds to single network specs
//! entry, and they are created and removed always simultaneously.
//!
//! ### Example
//!
//! Removing default Westend network:
//!
//! `$ cargo run remove -title westend`
//!
//! Removing user-generated Westend network with Ed25519 encryption:
//!
//! `$ cargo run remove -title westend-ed25519`
//!
//! If one of the default networks gets removed, it could be added back,
//! however, it will not be marked as default anymore.
use constants::{HOT_DB_NAME, METATREE};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{open_db, open_tree},
};
use definitions::{
    error_active::{Active, ErrorActive},
    keyring::{AddressBookKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey},
};
use sled::Batch;

use crate::helpers::{get_address_book_entry, is_specname_in_db};
use crate::parser::Remove;

/// Remove information from the database
pub fn remove_info(info: Remove) -> Result<(), ErrorActive> {
    match info {
        // network data by the address book title
        Remove::Title(network_title) => {
            // init `Batch` for `ADDRESS_BOOK`, `SPECSTREEPREP`, `METADATA`
            let mut address_book_batch = Batch::default();
            let mut metadata_batch = Batch::default();
            let mut network_specs_prep_batch = Batch::default();

            // get `ADDRESS_BOOK` entry for the title
            let address_book_entry = get_address_book_entry(&network_title)?;

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
            // except the one currently being removed, the metadata gets removed
            if !is_specname_in_db(&address_book_entry.name, &network_title)? {
                let database = open_db::<Active>(HOT_DB_NAME)?;
                let metadata = open_tree::<Active>(&database, METATREE)?;
                let meta_key_prefix = MetaKeyPrefix::from_name(&address_book_entry.name);
                for (x, _) in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
                    // add element to `Batch` for `METATREE`
                    metadata_batch.remove(x)
                }
            }
            TrDbHot::new()
                .set_address_book(address_book_batch)
                .set_metadata(metadata_batch)
                .set_network_specs_prep(network_specs_prep_batch)
                .apply(HOT_DB_NAME)
        }

        // network metadata by ntework name and version
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
