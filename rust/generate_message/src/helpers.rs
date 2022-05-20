//! Hot database helpers
use constants::{ADDRESS_BOOK, HOT_DB_NAME, SPECSTREEPREP};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{open_db, open_tree},
};
use definitions::{
    error::ErrorSource,
    error_active::{Active, DatabaseActive, ErrorActive, MismatchActive, NotFoundActive},
    keyring::{AddressBookKey, NetworkSpecsKey},
    metadata::AddressBookEntry,
    network_specs::NetworkSpecsToSend,
};
use parity_scale_codec::Encode;
use sled::Batch;

/// Get [`AddressBookEntry`] from the database for given address book title
pub fn get_address_book_entry(title: &str) -> Result<AddressBookEntry, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    match address_book.get(AddressBookKey::from_title(title).key()) {
        Ok(Some(a)) => AddressBookEntry::from_entry_with_title(title, &a),
        Ok(None) => Err(ErrorActive::NotFound(NotFoundActive::AddressBookEntry {
            title: title.to_string(),
        })),
        Err(e) => Err(<Active>::db_internal(e)),
    }
}

/// Get [`NetworkSpecsToSend`] from the database for given address book title
pub fn network_specs_from_title(title: &str) -> Result<NetworkSpecsToSend, ErrorActive> {
    network_specs_from_entry(&get_address_book_entry(title)?)
}

/// Get [`NetworkSpecsToSend`] corresponding to the given entry in
/// [`ADDRESS_BOOK`] tree.
///
/// Entries in [`ADDRESS_BOOK`] and [`SPECSTREEPREP`] trees for any network can
/// be added and removed only simultaneously.
// TODO consider combining those, key would be address book title, network specs
// key will stay only in cold database then?
pub fn network_specs_from_entry(
    address_book_entry: &AddressBookEntry,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    let network_specs_key = NetworkSpecsKey::from_parts(
        &address_book_entry.genesis_hash,
        &address_book_entry.encryption,
    );
    let network_specs = get_network_specs_to_send(&network_specs_key)?;
    if network_specs.name != address_book_entry.name {
        return Err(ErrorActive::Database(DatabaseActive::Mismatch(
            MismatchActive::AddressBookSpecsName {
                address_book_name: address_book_entry.name.to_string(),
                specs_name: network_specs.name,
            },
        )));
    }
    Ok(network_specs)
}

/// Try to get network specs [`NetworkSpecsToSend`] from the hot database.
///
/// If the [`NetworkSpecsKey`] and associated [`NetworkSpecsToSend`] are not
/// found in the [`SPECSTREEPREP`], the result is `Ok(None)`.
pub fn try_get_network_specs_to_send(
    network_specs_key: &NetworkSpecsKey,
) -> Result<Option<NetworkSpecsToSend>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let chainspecs = open_tree::<Active>(&database, SPECSTREEPREP)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(specs_encoded)) => Ok(Some(NetworkSpecsToSend::from_entry_with_key_checked(
            network_specs_key,
            specs_encoded,
        )?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Active>::db_internal(e)),
    }
}

/// Get network specs [`NetworkSpecsToSend`] from the hot database.
///
/// Network specs here are expected to be found, not finding them results in an
/// error.
pub fn get_network_specs_to_send(
    network_specs_key: &NetworkSpecsKey,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    match try_get_network_specs_to_send(network_specs_key)? {
        Some(a) => Ok(a),
        None => Err(ErrorActive::NotFound(NotFoundActive::NetworkSpecsToSend(
            network_specs_key.to_owned(),
        ))),
    }
}

/// Update the database when introducing a new network.
///
/// Inputs `&str` url address that is used for rpc calls and already prepared
/// [`NetworkSpecsToSend`].
///
/// Adds simultaneously [`AddressBookEntry`] to [`ADDRESS_BOOK`] and
/// [`NetworkSpecsToSend`] to [`SPECSTREEPREP`].
///
/// Key for [`AddressBookEntry`], the network address book title coincides with
/// `title` field of the [`NetworkSpecsToSend`] entry.
pub fn update_db(address: &str, network_specs: &NetworkSpecsToSend) -> Result<(), ErrorActive> {
    let mut network_specs_prep_batch = Batch::default();
    network_specs_prep_batch.insert(
        NetworkSpecsKey::from_parts(&network_specs.genesis_hash, &network_specs.encryption).key(),
        network_specs.encode(),
    );
    let address_book_new_key = AddressBookKey::from_title(&format!(
        "{}-{}",
        network_specs.name,
        network_specs.encryption.show()
    ));
    let address_book_new_entry_encoded = AddressBookEntry {
        name: network_specs.name.to_string(),
        genesis_hash: network_specs.genesis_hash,
        address: address.to_string(),
        encryption: network_specs.encryption.clone(),
        def: false,
    }
    .encode();
    let mut address_book_batch = Batch::default();
    address_book_batch.insert(address_book_new_key.key(), address_book_new_entry_encoded);
    TrDbHot::new()
        .set_address_book(address_book_batch)
        .set_network_specs_prep(network_specs_prep_batch)
        .apply(HOT_DB_NAME)
}

/// Function to process error depending on pass_errors flag
pub fn error_occured(e: ErrorActive, pass_errors: bool) -> Result<(), ErrorActive> {
    if pass_errors {
        println!("Error encountered. {} Skipping it.", e);
        Ok(())
    } else {
        Err(e)
    }
}

/// Enum to indicate what need to be printed in `load_metadata` and `add_network` messages
pub enum Write {
    All,     // -t key or no set key
    OnlyNew, // -k key
    None,    // -p key
}

pub fn address_book_content() -> Result<Vec<(String, AddressBookEntry)>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    for x in address_book.iter().flatten() {
        out.push(AddressBookEntry::process_entry(x)?)
    }
    Ok(out)
}

/// Function to filter address_book entries by url address
pub fn filter_address_book_by_url(
    address: &str,
) -> Result<Vec<(String, AddressBookEntry)>, ErrorActive> {
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    let mut found_name = None;
    for (title, address_book_entry) in address_book_content()?.into_iter() {
        if address_book_entry.address == address {
            found_name = match found_name {
                Some(name) => {
                    if name == address_book_entry.name {
                        Some(name)
                    } else {
                        return Err(ErrorActive::Database(DatabaseActive::TwoNamesForUrl {
                            url: address.to_string(),
                        }));
                    }
                }
                None => Some(address_book_entry.name.to_string()),
            };
            out.push((title, address_book_entry))
        }
    }
    Ok(out)
}

/// Function to search through address book tree of the database for the given genesis hash
pub fn genesis_hash_in_hot_db(
    genesis_hash: [u8; 32],
) -> Result<Option<AddressBookEntry>, ErrorActive> {
    let mut out = None;
    for (_, address_book_entry) in address_book_content()?.into_iter() {
        if address_book_entry.genesis_hash == genesis_hash {
            out = Some(address_book_entry);
            break;
        }
    }
    Ok(out)
}

/// Function to search through address_book tree of the database for the given network spec_name
pub fn specname_in_db(specname: &str, except_title: &str) -> Result<bool, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut out = false;
    for x in address_book.iter().flatten() {
        let (title, address_book_entry) = <AddressBookEntry>::process_entry(x)?;
        if (address_book_entry.name == specname) && (title != except_title) {
            out = true;
            break;
        }
    }
    Ok(out)
}
