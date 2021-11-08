use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE};
use sled::Tree;
use anyhow;
use db_handling::helpers::{open_db, open_tree};
use parity_scale_codec::{Decode, Encode};
use definitions::metadata::AddressBookEntry;

use crate::parser::{Instruction, Content, Set};
use crate::metadata_db_utils::{add_new, SortedMetaValues, prepare_metadata, write_metadata};
use crate::error::{Error, NotFound, NotDecodeable};
use crate::helpers::{decode_and_check_meta_entry, error_occured, Write};
use crate::metadata_shortcut::{MetaShortCut, meta_shortcut};
use crate::output_prep::load_meta_print;


/// Function to generate `load_metadata` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_load_meta (instruction: Instruction) -> anyhow::Result<()> {
    
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    let metadata = open_tree(&database, METATREE)?;
    if let Some(_) = instruction.encryption_override {return Err(Error::NotSupported.show())}
    match instruction.set {
        Set::F => {
            match instruction.content {
                Content::All => {
                    let set = get_address_book_set(&address_book)?;
                    for x in set.iter() {
                        match meta_f_a_element (x, &metadata) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Name(name) => meta_f_n (&name, &address_book, &metadata),
                Content::Address(_) => return Err(Error::NotSupported.show()),
            }
        },
        Set::D => {
            match instruction.content {
                Content::All => {
                    let set = get_address_book_set(&address_book)?;
                    for x in set.iter() {
                        match meta_d_a_element (x) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Name(name) => meta_d_n (&name, &address_book),
                Content::Address(address) => meta_d_u (&address),
            }
        },
        Set::K => {
            let write = Write::OnlyNew;
            match instruction.content {
                Content::All => meta_kpt_a(&address_book, &metadata, &write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n (&name, &write, &address_book, &metadata),
                Content::Address(_) => return Err(Error::NotSupported.show()),
            }
        },
        Set::P => {
            let write = Write::None;
            match instruction.content {
                Content::All => meta_kpt_a(&address_book, &metadata, &write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n (&name, &write, &address_book, &metadata),
                Content::Address(_) => return Err(Error::NotSupported.show()),
            }
        },
        Set::T => {
            let write = Write::All;
            match instruction.content {
                Content::All => meta_kpt_a(&address_book, &metadata, &write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n (&name, &write, &address_book, &metadata),
                Content::Address(_) => return Err(Error::NotSupported.show()),
            }
        },
    }
}

/// Function to process an individual address book entry in `load_metadata -f -a` run.
/// Expected behavior:  
/// scan prefix in `metadata` database tree in search of network specname
/// to get all versions available in the database (max 2),
/// check meta_values integrity (network specname and spec_version),
/// and print into `sign_me` output file.  
fn meta_f_a_element (set_element: &NameHashAddress, metadata: &Tree) -> anyhow::Result<()> {
    for x in metadata.scan_prefix(set_element.name.encode()) {
        if let Ok((versioned_name_encoded, meta)) = x {
            let meta_values = decode_and_check_meta_entry((versioned_name_encoded, meta))?;
            let shortcut = MetaShortCut {
                meta_values,
                genesis_hash: set_element.genesis_hash,
            };
            load_meta_print(&shortcut)?;
        }
    }
    Ok(())
}

/// Function to process `load_metadata -f -n name` run.
/// Here `name` is network specname, i.e. `polkadot` for any polkadot version and encryption variety
/// (since load_metadata message does not contain info about network encryption).
/// At this moment simple export of only specific version of the metadata is not implemented.
/// Expected behavior:  
/// go through `address_book` and collect all unique NameHashAddress entries,
/// search through this set for the entry corresponding to the requested name,
/// scan prefix in `metadata` database tree in search of network specname
/// to get all versions available in the database (max 2),
/// check `meta_values` integrity (network specname and `spec_version`),
/// and print into `sign_me` output file.  
fn meta_f_n (name: &str, address_book: &Tree, metadata: &Tree) -> anyhow::Result<()> {
    meta_f_a_element(&search_name(name, address_book)?, metadata)
}

/// Function to process individual address book entry in `load_metadata -d -a` run.
/// Expected behavior:  
/// fetch information from address, check it,
/// and print into `sign_me` output file.  
fn meta_d_a_element (set_element: &NameHashAddress) -> anyhow::Result<()> {
    let shortcut = shortcut_set_element(set_element)?;
    load_meta_print(&shortcut)
}

/// Function to process `load_metadata -d -n name` run.
/// Expected behavior:  
/// go through `address_book` and collect all unique NameHashAddress entries,
/// search through this set for the entry corresponding to the requested name,
/// fetch information from address, check it,
/// and print into `sign_me` output file.
fn meta_d_n (name: &str, address_book: &Tree) -> anyhow::Result<()> {
    meta_d_a_element(&search_name(name, address_book)?)
}

/// Function to process `load_metadata -d -u url` run.
/// Expected behavior:  
/// fetch information from address, check it,
/// and print into `sign_me` output file.
fn meta_d_u (address: &str) -> anyhow::Result<()> {
    let shortcut = meta_shortcut(address)?;
    load_meta_print(&shortcut)
}

/// Function to process `load_metadata -k -a`, `load_metadata -p -a`, `load_metadata -t -a`, `load_metadata -a` runs.
/// Expected behavior:  
/// go through `address_book` and collect all unique NameHashAddress entries,
/// collect known metadata from database and sort it, clear metadata from database,
/// process each element and update sorted metadata set (so that there are
/// max 2 most recent metadata entries for each network),
/// record resulting metadata into database.
/// `write` determines which `sign_me` files are produced.
fn meta_kpt_a (address_book: &Tree, metadata: &Tree, write: &Write, pass_errors: bool) -> anyhow::Result<()> {
    let set = get_address_book_set(&address_book)?;
    let mut sorted_meta_values = prepare_metadata(&metadata)?;
    for x in set.iter() {
        sorted_meta_values = match meta_kpt_a_element (x, write, &sorted_meta_values) {
            Ok(a) => a,
            Err(e) => {
                error_occured(e, pass_errors)?;
                sorted_meta_values
            },
        };
    }
    write_metadata(sorted_meta_values, &metadata)
}

/// Function to process an individual element of
/// `load_metadata -k -a`, `load_metadata -p -a`, `load_metadata -t -a`, `load_metadata -a` runs.
/// Expected behavior:  
/// fetch information from address, check it,
/// insert in the sorted `meta_values`,
/// and print into `sign_me` output file depending on value of `write`.
fn meta_kpt_a_element (set_element: &NameHashAddress, write: &Write, sorted_meta_values: &SortedMetaValues) -> anyhow::Result<SortedMetaValues> {
    let shortcut = shortcut_set_element(set_element)?;
    let upd_sorted = add_new(&shortcut.meta_values, sorted_meta_values)?;
    match write {
        Write::All => load_meta_print(&shortcut)?,
        Write::OnlyNew => if upd_sorted.upd_done {load_meta_print(&shortcut)?},
        Write::None => (),
    }
    Ok(upd_sorted.sorted)
}

/// Function to process `load_metadata -k -n name`, `load_metadata -p -n name`,
/// `load_metadata -t -n name`, `load_metadata -n name` runs.
/// Expected behavior:  
/// collect known metadata from database and sort it, clear metadata from database,
/// go through `address_book` and collect all unique NameHashAddress entries,
/// search through this set for the entry corresponding to the requested name,
/// fetch information from address, check it, insert into sorted metadata
/// and print into `sign_me` output file depending on `write` value,
/// record resulting metadata into database.
fn meta_kpt_n (name: &str, write: &Write, address_book: &Tree, metadata: &Tree) -> anyhow::Result<()> {
    let mut sorted_meta_values = prepare_metadata(&metadata)?;
    sorted_meta_values = meta_kpt_a_element(&search_name(name, address_book)?, write, &sorted_meta_values)?;
    write_metadata(sorted_meta_values, &metadata)
}

/// Struct to collect network specname, genesis hash and fetching address from address book
#[derive(PartialEq)]
struct NameHashAddress {
    name: String,
    genesis_hash: [u8; 32],
    address: String,
}

/// Function to collect a vector of unique NameHashAddress entries from address book
fn get_address_book_set (address_book: &Tree) -> anyhow::Result<Vec<NameHashAddress>> {
    let mut set: Vec<NameHashAddress> = Vec::new();
    for x in address_book.iter() {
        if let Ok((_, address_book_entry_encoded)) = x {
            let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            // check that each name gets only one genesis hash
            for a in set.iter() {
                if a.name == address_book_entry.name {
                    if a.genesis_hash != address_book_entry.genesis_hash {return Err(Error::TwoGenHash(address_book_entry.name).show())}
                    if a.address != address_book_entry.address {return Err(Error::TwoAddresses(address_book_entry.name).show())}
                }
            }
            let new = NameHashAddress{
                name: address_book_entry.name.to_string(),
                genesis_hash: address_book_entry.genesis_hash,
                address: address_book_entry.address.to_string(),
            };
            if !set.contains(&new) {set.push(new)}
        }
    }
    if set.len() == 0 {return Err(Error::AddressBookEmpty.show())}
    Ok(set)
}

/// Function to collect a vector of unique NameHashAddress entries from address book
/// and search for particular name in it,
/// outputs NameHashAddress
fn search_name (name: &str, address_book: &Tree) -> anyhow::Result<NameHashAddress> {
    let set = get_address_book_set(&address_book)?;
    let mut found = None;
    for x in set.into_iter() {
        if x.name == name {
            found = Some(x);
            break;
        }
    }
    match found {
        Some(a) => Ok(a),
        None => return Err(Error::NotFound(NotFound::AddressBookNetworkName(name.to_string())).show())
    }
}

/// Function to process individual NameHashAddress entry:
/// do fetch, check fetched metadata for version,
/// check that genesis hash and network name are same in address book and in fetch
/// output MetaShortCut value
fn shortcut_set_element (set_element: &NameHashAddress) -> anyhow::Result<MetaShortCut> {
    let shortcut = meta_shortcut(&set_element.address)?;
    if shortcut.meta_values.name != set_element.name {return Err(Error::NameChanged(set_element.address.to_string()).show())}
    if shortcut.genesis_hash != set_element.genesis_hash {return Err(Error::GenesisHashChanged{address: set_element.address.to_string(), old_genesis_hash: set_element.genesis_hash, new_genesis_hash: shortcut.genesis_hash}.show())}
    Ok(shortcut)
}


