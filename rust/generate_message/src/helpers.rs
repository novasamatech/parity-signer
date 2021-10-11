use sled::{IVec, Tree};
use anyhow;
use definitions::{crypto::Encryption, metadata::{AddressBookEntry, MetaValues, NameVersioned, VersionDecoded}, network_specs::{ChainSpecsToSend, generate_network_key, NetworkKey}};
use meta_reading::decode_metadata::get_meta_const;
use db_handling::helpers::insert_into_tree;
use parity_scale_codec::{Decode, Encode};


use crate::error::{Error, NotDecodeable, NotFound};

/// Wrapper for `get` with crate error
pub fn get_from_tree(key: &Vec<u8>, tree: &Tree) -> anyhow::Result<Option<IVec>> {
    match tree.get(key) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to get SCALE encoded network specs entry by given network_key, decode it
/// as ChainSpecs, and check for genesis hash mismatch. Is used forrom cold database
pub fn get_and_decode_chain_specs_to_send(chainspecs: &Tree, network_key: &NetworkKey) -> anyhow::Result<ChainSpecsToSend> {
    match chainspecs.get(network_key) {
        Ok(Some(chain_specs_to_send_encoded)) => decode_chain_specs_to_send(chain_specs_to_send_encoded, network_key),
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_chain_specs_to_send(chain_specs_to_send_encoded: IVec, network_key: &NetworkKey) -> anyhow::Result<ChainSpecsToSend> {
    match <ChainSpecsToSend>::decode(&mut &chain_specs_to_send_encoded[..]) {
        Ok(a) => {
            if &generate_network_key(&a.genesis_hash.to_vec(), a.encryption) != network_key {return Err(Error::NetworkKeyMismatch(a.title).show())}
            Ok(a)
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecsToSend).show()),
    }
}

/// Function to decode and check for integrity an entry from metadata database
pub fn decode_and_check_meta_entry ((versioned_name_encoded, meta): (IVec, IVec)) -> anyhow::Result<MetaValues> {
// decode what is in the key
    let name_versioned = match NameVersioned::decode(&mut &versioned_name_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::DatabaseVersionedName).show()),
    };
// check the database for corruption
    let version_vector = match get_meta_const(&meta.to_vec()) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name: name_versioned.name, version: name_versioned.version, error: e.to_string()}.show()),
    };
    let version = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name: name_versioned.name, version: name_versioned.version, error: e.to_string()}.show()),
    };
    if (version.specname != name_versioned.name)||(version.spec_version != name_versioned.version) {return Err(Error::DatabaseMetadataMismatch{name1: name_versioned.name, version1: name_versioned.version, name2: version.specname, version2: version.spec_version}.show())}
// output
    Ok(MetaValues {
        name: name_versioned.name,
        version: name_versioned.version,
        meta: meta.to_vec(),
    })
}

/// Function to get ChainSpecsToSend for given address book entry
pub fn network_specs_from_address_book_entry_encoded (address_book_entry_encoded: IVec, chainspecs: &Tree) -> anyhow::Result<ChainSpecsToSend> {
    let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
    };
    let network_key = generate_network_key(&address_book_entry.genesis_hash.to_vec(), address_book_entry.encryption);
    let network_specs = get_and_decode_chain_specs_to_send(&chainspecs, &network_key)?;
    if network_specs.name != address_book_entry.name {return Err(Error::StoredNameMismatch{address_book_name: address_book_entry.name, network_specs_name: network_specs.name}.show())}
    Ok(network_specs)
}

/// Function to update chainspecs and address_book trees of the database
pub fn update_db (address: &str, network_specs: &ChainSpecsToSend, chainspecs: &Tree, address_book: &Tree) -> anyhow::Result<()> {
    insert_into_tree(generate_network_key(&network_specs.genesis_hash.to_vec(), network_specs.encryption), network_specs.encode(), chainspecs)?;
    let address_book_new_key = network_specs.title.encode();
    let address_book_new_entry_encoded = AddressBookEntry {
        name: network_specs.name.to_string(),
        genesis_hash: network_specs.genesis_hash,
        address: address.to_string(),
        encryption: network_specs.encryption,
        def: false,
    }.encode();
    insert_into_tree(address_book_new_key, address_book_new_entry_encoded, address_book)?;
    Ok(())
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
pub fn filter_address_book_by_url (address: &str, address_book: &Tree) -> anyhow::Result<Vec<AddressBookEntry>> {
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
pub fn process_indices (entries: &Vec<AddressBookEntry>, chainspecs: &Tree, encryption: Encryption) -> anyhow::Result<(ChainSpecsToSend, bool)> {
    let indices = get_indices (&entries, encryption)?;
    match indices.index_correct_encryption {
        Some(i) => {
            let network_key = generate_network_key(&entries[i].genesis_hash.to_vec(), entries[i].encryption);
            Ok((get_and_decode_chain_specs_to_send(&chainspecs, &network_key)?, false))
        },
        None => {
            let network_key = match indices.index_default {
                Some(i) => generate_network_key(&entries[i].genesis_hash.to_vec(), entries[i].encryption),
                None => generate_network_key(&entries[0].genesis_hash.to_vec(), entries[0].encryption),
            };
            let mut specs_found = get_and_decode_chain_specs_to_send(&chainspecs, &network_key)?;
            specs_found.encryption = encryption;
            specs_found.title = format!("{}-{}", specs_found.name, encryption.show());
            Ok((specs_found, true))
        },
    }
}

/// Function to search through chainspecs tree of the database for the given genesis hash
pub fn genesis_hash_in_hot_db (genesis_hash: [u8; 32], chainspecs: &Tree) -> anyhow::Result<bool> {
    let mut out = false;
    for x in chainspecs.iter() {
        if let Ok((network_key, chain_specs_to_send_encoded)) = x {
            let network_specs = decode_chain_specs_to_send(chain_specs_to_send_encoded, &network_key.to_vec())?;
            if network_specs.genesis_hash == genesis_hash {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}

/// Function to search through address_book tree of the database for the given natwork spec_name
pub fn specname_in_db (specname: &str, address_book: &Tree) -> anyhow::Result<bool> {
    let mut out = false;
    for x in address_book.iter() {
        if let Ok((_, address_book_entry_encoded)) = x {
            let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            if address_book_entry.name == specname {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}
