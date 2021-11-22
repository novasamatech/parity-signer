use sled::Batch;
use parity_scale_codec::Encode;
use constants::{ADDRESS_BOOK, METATREE, SETTREE, SPECSTREEPREP, TYPES};
use defaults::{get_default_address_book, get_default_chainspecs_to_send, get_default_types};
use definitions::keyring::{AddressBookKey, NetworkSpecsKey};
use anyhow;

use crate::db_transactions::TrDbHot;
use crate::error::Error;
use crate::helpers::{make_batch_clear_tree};

/// Preparing default address_book batch:
/// (1) purge all existing entries,
/// (2) add default entries with AddressBookKey in key form as a key,
/// and encoded AddressBookEntry as a value.
fn default_hot_address_book (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(database_name, ADDRESS_BOOK)?;
    for x in get_default_address_book().iter() {
        let address_book_key = AddressBookKey::from_title(&x.name);
        batch.insert(address_book_key.key(), x.encode());
    }
    Ok(batch)
}

/// Preparing default network_specs_prep batch:
/// (1) purge all existing entries,
/// (2) add default entries with NetworkSpecsKey in key form as a key,
/// and encoded ChainSpecsToSend as a value.
fn default_hot_network_specs_prep (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(database_name, SPECSTREEPREP)?;
    for x in get_default_chainspecs_to_send().iter() {
        let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Preparing default settings batch:
/// (1) purge all existing entries,
/// (2) add default entry with TYPES as a key,
/// and ContentLoadTypes (i.e. encoded Vec<TypeEntry>) as a value.
fn default_hot_settings (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(database_name, SETTREE)?;
    let types_prep = match get_default_types() {
        Ok(x) => x,
        Err(e) => return Err(Error::BadTypesFile(e).show()),
    };
    batch.insert(TYPES.to_vec(), types_prep.store());
    Ok(batch)
}

/// Function to reset hot database to defaults:
/// address_book and network_specs_prep with entries for default networks,
/// settings with default types,
/// metadata empty
pub fn reset_hot_database (database_name: &str) -> anyhow::Result<()> {
    TrDbHot::new()
        .set_address_book(default_hot_address_book(&database_name)?) // set default address_book
        .set_metadata(make_batch_clear_tree(&database_name, METATREE)?) // clear metadata
        .set_network_specs_prep(default_hot_network_specs_prep(&database_name)?) // set default network_specs_prep
        .set_settings(default_hot_settings(&database_name)?) // load default types
        .apply(&database_name)
}
