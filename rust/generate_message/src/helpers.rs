use constants::{ADDRESS_BOOK, HOT_DB_NAME, SPECSTREEPREP};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{open_db, open_tree},
};
use definitions::{
    crypto::Encryption,
    error::{Active, DatabaseActive, ErrorActive, ErrorSource, MismatchActive, NotFoundActive},
    keyring::{AddressBookKey, NetworkSpecsKey},
    metadata::AddressBookEntry,
    network_specs::NetworkSpecsToSend,
};
use parity_scale_codec::Encode;
use sled::Batch;

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

/// Function to get NetworkSpecsToSend for given address book title
pub fn network_specs_from_title(title: &str) -> Result<NetworkSpecsToSend, ErrorActive> {
    network_specs_from_entry(&get_address_book_entry(title)?)
}

/// Function to get NetworkSpecsToSend for given address book entry
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

/// Function to get SCALE encoded network specs entry by given network_key, decode it
/// as NetworkSpecsToSend, and check for genesis hash mismatch. Is used for hot db only
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

/// Function to update chainspecs and address_book trees of the database
pub fn update_db(address: &str, network_specs: &NetworkSpecsToSend) -> Result<(), ErrorActive> {
    let mut network_specs_prep_batch = Batch::default();
    network_specs_prep_batch.insert(
        NetworkSpecsKey::from_parts(&network_specs.genesis_hash, &network_specs.encryption).key(),
        network_specs.encode(),
    );
    let address_book_new_key = AddressBookKey::from_title(&network_specs.title);
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

/// Function to filter address_book entries by url address
pub fn filter_address_book_by_url(address: &str) -> Result<Vec<AddressBookEntry>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut out: Vec<AddressBookEntry> = Vec::new();
    let mut found_name = None;
    for x in address_book.iter().flatten() {
        let new_address_book_entry = AddressBookEntry::from_entry(x)?;
        if new_address_book_entry.address == address {
            found_name = match found_name {
                Some(name) => {
                    if name == new_address_book_entry.name {
                        Some(name)
                    } else {
                        return Err(ErrorActive::Database(DatabaseActive::TwoNamesForUrl {
                            url: address.to_string(),
                        }));
                    }
                }
                None => Some(new_address_book_entry.name.to_string()),
            };
            out.push(new_address_book_entry)
        }
    }
    Ok(out)
}

/// Struct to store indices (id found) for correct encryption and for default entry
struct Indices {
    index_correct_encryption: Option<usize>,
    index_default: Option<usize>,
}

/// Function to search through a vector of AddressBookEntry (for use with sets having the same address)
/// for entry with given encryption and for default entry;
/// Checks that there is only one default entry and only one entry with given encryption for this address
fn get_indices(
    entries: &[AddressBookEntry],
    encryption: Encryption,
) -> Result<Indices, ErrorActive> {
    let mut index_correct_encryption = None;
    let mut index_default = None;
    for (i, x) in entries.iter().enumerate() {
        if x.encryption == encryption {
            match index_correct_encryption {
                Some(_) => {
                    return Err(ErrorActive::Database(
                        DatabaseActive::TwoEntriesAddressEncryption {
                            url: x.address.to_string(),
                            encryption,
                        },
                    ))
                }
                None => index_correct_encryption = Some(i),
            }
        }
        if x.def {
            match index_default {
                Some(_) => {
                    return Err(ErrorActive::Database(DatabaseActive::TwoDefaultsAddress {
                        url: x.address.to_string(),
                    }))
                }
                None => index_default = Some(i),
            }
        }
    }
    Ok(Indices {
        index_correct_encryption,
        index_default,
    })
}

/// Function to use the indices to get the most appropriate chainspecs entry to modify,
/// and modify its encryption and title
pub fn process_indices(
    entries: &[AddressBookEntry],
    encryption: Encryption,
) -> Result<(NetworkSpecsToSend, bool), ErrorActive> {
    let indices = get_indices(entries, encryption.to_owned())?;
    match indices.index_correct_encryption {
        Some(i) => {
            let network_specs_key =
                NetworkSpecsKey::from_parts(&entries[i].genesis_hash, &entries[i].encryption);
            let network_specs = get_network_specs_to_send(&network_specs_key)?;
            Ok((network_specs, false))
        }
        None => {
            let network_specs_key = match indices.index_default {
                Some(i) => {
                    NetworkSpecsKey::from_parts(&entries[i].genesis_hash, &entries[i].encryption)
                }
                None => {
                    NetworkSpecsKey::from_parts(&entries[0].genesis_hash, &entries[0].encryption)
                }
            };
            let mut specs_found = get_network_specs_to_send(&network_specs_key)?;
            specs_found.encryption = encryption.clone();
            specs_found.title = format!("{}-{}", specs_found.name, encryption.show());
            Ok((specs_found, true))
        }
    }
}

/// Function to search through chainspecs tree of the database for the given genesis hash
pub fn genesis_hash_in_hot_db(genesis_hash: [u8; 32]) -> Result<bool, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let chainspecs = open_tree::<Active>(&database, SPECSTREEPREP)?;
    let mut out = false;
    for x in chainspecs.iter().flatten() {
        let network_specs = NetworkSpecsToSend::from_entry_checked(x)?;
        if network_specs.genesis_hash == genesis_hash {
            out = true;
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
