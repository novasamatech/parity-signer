//! Making and restoring **hot** database with default content
//!
//! Hot database is the database that exists on network-connected device and
//! that could be used to manage Signer updates.
//!
//! For release, the cold database is generated on the hot side and then copied
//! verbatim into Signer files during the build.
//!
//! Hot database contains following trees:
//!
//! - [`ADDRESS_BOOK`] with information needed to perform rpc calls on networks
//! - [`METATREE`] with network metadata fetched through rpc calls, maximum two
//! entries are allowed for each network
//! - [`SETTREE`] with types information
//! - [`SPECSTREEPREP`] with network specs entries
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRESS_BOOK, METATREE, SETTREE, SPECSTREEPREP, TYPES};
use defaults::{default_address_book, default_chainspecs_to_send, default_types_content};
use definitions::{
    error_active::{Active, ErrorActive},
    keyring::{AddressBookKey, NetworkSpecsKey},
};

use crate::db_transactions::TrDbHot;
use crate::helpers::make_batch_clear_tree;

/// Make [`Batch`] with default
/// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) values, for
/// [`ADDRESS_BOOK`] tree.
///
/// - Purge all existing entries
/// - Add default `AddressBookEntry` values
fn default_hot_address_book(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, ADDRESS_BOOK)?;
    for x in default_address_book().iter() {
        let address_book_key = AddressBookKey::from_title(&x.name);
        batch.insert(address_book_key.key(), x.encode());
    }
    Ok(batch)
}

/// Make [`Batch`] with default
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
/// values, for [`SPECSTREEPREP`] tree.
///
/// - Purge all existing entries
/// - Add default `NetworkSpecsToSend` values
fn default_hot_network_specs_prep(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SPECSTREEPREP)?;
    for x in default_chainspecs_to_send().iter() {
        let network_specs_key =
            NetworkSpecsKey::from_parts(x.genesis_hash.as_bytes(), &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Make [`Batch`] with default settings, for [`SETTREE`] tree.
///
/// - Purge all existing entries
/// - Add default types information
/// [`ContentLoadTypes`](definitions::qr_transfers::ContentLoadTypes)
fn default_hot_settings(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SETTREE)?;
    let types_prep = default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    Ok(batch)
}

/// Generate hot database with default content.
///
/// Function clears all database trees and loads into database defaults for:
///
/// - network specs
/// - types information
/// - network verifiers
///
/// Note that no metadata entries are loaded. It is intended that all metadata
/// is received only through rpc calls.
pub fn reset_hot_database(database_name: &str) -> Result<(), ErrorActive> {
    TrDbHot::new()
        .set_address_book(default_hot_address_book(database_name)?) // set default address book
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs_prep(default_hot_network_specs_prep(database_name)?) // set default network specs
        .set_settings(default_hot_settings(database_name)?) // load default types
        .apply(database_name)
}
