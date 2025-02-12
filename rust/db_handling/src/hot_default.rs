//! Making and restoring **hot** database with default content
//!
//! Hot database is the database that exists on network-connected device and
//! that could be used to manage Vault updates.
//!
//! Hot database contains following trees:
//!
//! - [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) with information needed to
//!   perform RPC calls on networks
//! - [`METATREE`](constants::METATREE) with network metadata fetched through
//!   RPC calls, maximum two entries are allowed for each network, empty by
//!   default
//! - [`META_HISTORY`](constants::META_HISTORY) with block hashes for metadata
//!   fetched through RPC calls, empty by default
//! - [`SETTREE`](constants::SETTREE) with types information
//! - [`SPECSTREEPREP`](constants::SPECSTREEPREP) with network specs entries
//!   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
use parity_scale_codec::Encode;
use sled::Batch;

use constants::TYPES;
use defaults::{default_address_book, default_chainspecs_to_send, default_types_content};
use definitions::keyring::{AddressBookKey, NetworkSpecsKey};

use crate::db_transactions::TrDbHot;
use crate::Result;

/// Make [`Batch`] with default
/// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) values, for
/// [`ADDRESS_BOOK`] tree, in purged database.
///
/// - Add default `AddressBookEntry` values
fn default_hot_address_book() -> Result<Batch> {
    let mut batch = Batch::default();
    for x in default_address_book().iter() {
        let address_book_key =
            AddressBookKey::from_title(&format!("{}-{}", x.name, x.encryption.show()));
        batch.insert(address_book_key.key(), x.encode());
    }
    Ok(batch)
}

/// Make [`Batch`] with default
/// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
/// values, for [`SPECSTREEPREP`] tree, in purged database.
///
/// - Add default `NetworkSpecs` values
fn default_hot_network_specs_prep() -> Result<Batch> {
    let mut batch = Batch::default();
    for x in default_chainspecs_to_send().iter() {
        let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Make [`Batch`] with default settings, for [`SETTREE`] tree, in purged
/// database.
///
/// Adds default types information
/// [`ContentLoadTypes`](definitions::qr_transfers::ContentLoadTypes).
fn default_hot_settings() -> Result<Batch> {
    let mut batch = Batch::default();
    let types_prep = default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    Ok(batch)
}

/// Generate hot database with default content.
///
/// Function wipes everything in the database directory and loads into database
/// defaults for:
///
/// - network specs
/// - types information
/// - network verifiers
///
/// Note that no metadata entries are loaded. It is intended that all metadata
/// entries appear during the database use.
pub fn reset_hot_database(database: &sled::Db) -> Result<()> {
    database.clear()?;
    TrDbHot::new()
        .set_address_book(default_hot_address_book()?) // set default address book
        .set_network_specs_prep(default_hot_network_specs_prep()?) // set default network specs
        .set_settings(default_hot_settings()?) // load default types
        .apply(database)
}
