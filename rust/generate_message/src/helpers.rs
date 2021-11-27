use sled::{IVec, Batch};
use anyhow;
use constants::{ADDRESS_BOOK, HOT_DB_NAME, SPECSTREEPREP};
use db_handling::{db_transactions::TrDbHot, helpers::{open_db, open_tree}};
use definitions::{crypto::Encryption, keyring::{AddressBookKey, NetworkSpecsKey, MetaKey}, metadata::{AddressBookEntry, MetaValues, VersionDecoded}, network_specs::ChainSpecsToSend};
use meta_reading::decode_metadata::get_meta_const;
use parity_scale_codec::{Decode, Encode};


use crate::error::{Error, NotDecodeable, NotFound};

fn get_address_book_entry (title: &str) -> anyhow::Result<IVec> {
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    match address_book.get (AddressBookKey::from_title(title).key()) {
        Ok(Some(a)) => Ok(a),
        Ok(None) => return Err(Error::NotFound(NotFound::AddressBookKey(title.to_string())).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

pub fn get_network_specs_from_address_book_entry (title: &str) -> anyhow::Result<ChainSpecsToSend> {
    network_specs_from_address_book_entry_encoded (&get_address_book_entry(title)?)
}

pub fn get_and_decode_address_book_entry (title: &str) -> anyhow::Result<AddressBookEntry> {
    let address_book_entry_encoded = get_address_book_entry(title)?;
    match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
    }
}

/// Function to get SCALE encoded network specs entry by given network_key, decode it
/// as ChainSpecsToSend, and check for genesis hash mismatch. Is used for hot db only
pub fn get_and_decode_chain_specs_to_send(network_specs_key: &NetworkSpecsKey) -> anyhow::Result<Option<ChainSpecsToSend>> {
    let database = open_db(HOT_DB_NAME)?;
    let chainspecs = open_tree(&database, SPECSTREEPREP)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(chain_specs_to_send_encoded)) => Ok(Some(decode_chain_specs_to_send(chain_specs_to_send_encoded, network_specs_key)?)),
        Ok(None) => Ok(None),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecsToSend,
/// and check for genesis hash mismatch
pub fn decode_chain_specs_to_send(chain_specs_to_send_encoded: IVec, network_specs_key: &NetworkSpecsKey) -> anyhow::Result<ChainSpecsToSend> {
    match <ChainSpecsToSend>::decode(&mut &chain_specs_to_send_encoded[..]) {
        Ok(a) => {
            if network_specs_key != &NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption) {return Err(Error::NetworkSpecsKeyMismatch(a.name).show())}
            Ok(a)
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecsToSend).show()),
    }
}

/// Function to decode and check for integrity an entry from metadata database
pub fn decode_and_check_meta_entry ((meta_key_vec, meta): (IVec, IVec)) -> anyhow::Result<MetaValues> {
// decode what is in the key
    let (name, version) = match MetaKey::from_vec(&meta_key_vec.to_vec()).name_version() {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::DatabaseVersionedName).show()),
    };
// check the database for corruption
    let version_vector = match get_meta_const(&meta.to_vec()) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name, version, error: e.to_string()}.show()),
    };
    let version_decoded = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name, version, error: e.to_string()}.show()),
    };
    if (version_decoded.specname != name)||(version_decoded.spec_version != version) {return Err(Error::DatabaseMetadataMismatch{name1: name, version1: version, name2: version_decoded.specname, version2: version_decoded.spec_version}.show())}
// output
    Ok(MetaValues {
        name,
        version,
        meta: meta.to_vec(),
    })
}

/// Function to get ChainSpecsToSend for given address book entry
pub fn network_specs_from_address_book_entry_encoded (address_book_entry_encoded: &IVec) -> anyhow::Result<ChainSpecsToSend> {
    let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
    };
    let network_specs_key = NetworkSpecsKey::from_parts(&address_book_entry.genesis_hash.to_vec(), &address_book_entry.encryption);
    let network_specs = match get_and_decode_chain_specs_to_send(&network_specs_key)? {
        Some(a) => a,
        None => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
    };
    if network_specs.name != address_book_entry.name {return Err(Error::StoredNameMismatch{address_book_name: address_book_entry.name, network_specs_name: network_specs.name}.show())}
    Ok(network_specs)
}

/// Function to update chainspecs and address_book trees of the database
pub fn update_db (address: &str, network_specs: &ChainSpecsToSend) -> anyhow::Result<()> {
    let mut network_specs_prep_batch = Batch::default();
    network_specs_prep_batch.insert(NetworkSpecsKey::from_parts(&network_specs.genesis_hash.to_vec(), &network_specs.encryption).key(), network_specs.encode());
    let address_book_new_key = AddressBookKey::from_title(&network_specs.title);
    let address_book_new_entry_encoded = AddressBookEntry {
        name: network_specs.name.to_string(),
        genesis_hash: network_specs.genesis_hash,
        address: address.to_string(),
        encryption: network_specs.encryption.clone(),
        def: false,
    }.encode();
    let mut address_book_batch = Batch::default();
    address_book_batch.insert(address_book_new_key.key(), address_book_new_entry_encoded);
    TrDbHot::new()
        .set_address_book(address_book_batch)
        .set_network_specs_prep(network_specs_prep_batch)
        .apply(HOT_DB_NAME)
}

/// Function to process error depending on pass_errors flag
pub fn error_occured (e: anyhow::Error, pass_errors: bool) -> anyhow::Result<()> {
    if pass_errors {Ok(println!("Error encountered. {} Skipping it.", e))}
    else {return Err(e)}
}

/// Enum to indicate what need to be printed in `load_metadata` and `add_network` messages
pub enum Write {
    All, // -t key or no set key
    OnlyNew, // -k key
    None, // -p key
}

/// Function to filter address_book entries by url address
pub fn filter_address_book_by_url (address: &str) -> anyhow::Result<Vec<AddressBookEntry>> {
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    let mut out: Vec<AddressBookEntry> = Vec::new();
    for x in address_book.iter() {
        if let Ok((_, address_book_entry_encoded)) = x {
            let new_address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            if new_address_book_entry.address == address {out.push(new_address_book_entry)}
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
fn get_indices (entries: &Vec<AddressBookEntry>, encryption: Encryption) -> anyhow::Result<Indices> {
    let mut index_correct_encryption = None;
    let mut index_default = None;
    for (i, x) in entries.iter().enumerate() {
        if x.encryption == encryption {
            match index_correct_encryption {
                Some(_) => return Err(Error::TwoEntriesAddressEncryption{address: x.address.to_string(), encryption}.show()),
                None => {index_correct_encryption = Some(i)}
            }
        }
        if x.def {
            match index_default {
                Some(_) => return Err(Error::TwoDefaultsAddress(x.address.to_string()).show()),
                None => {index_default = Some(i)}
            }
        }
    }
    Ok(Indices{
        index_correct_encryption,
        index_default,
    })
}

/// Function to use the indices to get the most appropriate chainspecs entry to modify, 
/// and modify its encryption and title
pub fn process_indices (entries: &Vec<AddressBookEntry>, encryption: Encryption) -> anyhow::Result<(ChainSpecsToSend, bool)> {
    let indices = get_indices (&entries, encryption.clone())?;
    match indices.index_correct_encryption {
        Some(i) => {
            let network_specs_key = NetworkSpecsKey::from_parts(&entries[i].genesis_hash.to_vec(), &entries[i].encryption);
            let network_specs = match get_and_decode_chain_specs_to_send(&network_specs_key)? {
                Some(a) => a,
                None => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
            };
            Ok((network_specs, false))
        },
        None => {
            let network_specs_key = match indices.index_default {
                Some(i) => NetworkSpecsKey::from_parts(&entries[i].genesis_hash.to_vec(), &entries[i].encryption),
                None => NetworkSpecsKey::from_parts(&entries[0].genesis_hash.to_vec(), &entries[0].encryption),
            };
            let mut specs_found = match get_and_decode_chain_specs_to_send(&network_specs_key)? {
                Some(a) => a,
                None => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
            };
            specs_found.encryption = encryption.clone();
            specs_found.title = format!("{}-{}", specs_found.name, encryption.show());
            Ok((specs_found, true))
        },
    }
}

/// Function to search through chainspecs tree of the database for the given genesis hash
pub fn genesis_hash_in_hot_db (genesis_hash: [u8; 32]) -> anyhow::Result<bool> {
    let database = open_db(HOT_DB_NAME)?;
    let chainspecs = open_tree(&database, SPECSTREEPREP)?;
    let mut out = false;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, chain_specs_to_send_encoded)) = x {
            let network_specs = decode_chain_specs_to_send(chain_specs_to_send_encoded, &NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec()))?;
            if network_specs.genesis_hash == genesis_hash {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}

/// Function to search through address_book tree of the database for the given network spec_name
pub fn specname_in_db (specname: &str, except_title: &str) -> anyhow::Result<bool> {
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    let mut out = false;
    for x in address_book.iter() {
        if let Ok((address_book_key_vec, address_book_entry_encoded)) = x {
            let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            let title = match AddressBookKey::from_vec(&address_book_key_vec.to_vec()).title() {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookKey).show()),
            };
            if (address_book_entry.name == specname)&&(title != except_title) {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}
