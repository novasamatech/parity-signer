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

/// Preparing default address_book batch:
/// (1) purge all existing entries,
/// (2) add default entries with AddressBookKey in key form as a key,
/// and encoded AddressBookEntry as a value.
/// Function applicable only for Active side.
fn default_hot_address_book(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, ADDRESS_BOOK)?;
    for x in default_address_book().iter() {
        let address_book_key = AddressBookKey::from_title(&x.name);
        batch.insert(address_book_key.key(), x.encode());
    }
    Ok(batch)
}

/// Preparing default network_specs_prep batch:
/// (1) purge all existing entries,
/// (2) add default entries with NetworkSpecsKey in key form as a key,
/// and encoded ChainSpecsToSend as a value.
/// Function applicable only for Active side.
fn default_hot_network_specs_prep(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SPECSTREEPREP)?;
    for x in default_chainspecs_to_send().iter() {
        let network_specs_key =
            NetworkSpecsKey::from_parts(x.genesis_hash.as_bytes(), &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Preparing default settings batch:
/// (1) purge all existing entries,
/// (2) add default entry with TYPES as a key,
/// and ContentLoadTypes (i.e. encoded Vec<TypeEntry>) as a value.
fn default_hot_settings(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SETTREE)?;
    let types_prep = default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    Ok(batch)
}

/// Function to reset hot database to defaults:
/// address_book and network_specs_prep with entries for default networks,
/// settings with default types,
/// metadata empty
pub fn reset_hot_database(database_name: &str) -> Result<(), ErrorActive> {
    TrDbHot::new()
        .set_address_book(default_hot_address_book(database_name)?) // set default address_book
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs_prep(default_hot_network_specs_prep(database_name)?) // set default network_specs_prep
        .set_settings(default_hot_settings(database_name)?) // load default types
        .apply(database_name)
}
