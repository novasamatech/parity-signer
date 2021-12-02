use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE};
use db_handling::helpers::{open_db, open_tree};
use definitions::{error::{Active, Changed, DatabaseActive, ErrorActive, Fetch, NotFoundActive}, keyring::MetaKeyPrefix, metadata::{AddressBookEntry, MetaValues}};

use crate::parser::{Instruction, Content, Set};
use crate::metadata_db_utils::{add_new, SortedMetaValues, prepare_metadata, write_metadata};
use crate::helpers::{error_occured, Write};
use crate::metadata_shortcut::{MetaShortCut, meta_shortcut};
use crate::output_prep::load_meta_print;


/// Function to generate `load_metadata` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_load_meta (instruction: Instruction) -> Result<(), ErrorActive> {
    
    if let Some(_) = instruction.encryption_override {return Err(ErrorActive::NotSupported)}
    match instruction.set {
        Set::F => {
            match instruction.content {
                Content::All => {
                    let set = get_address_book_set()?;
                    for x in set.iter() {
                        match meta_f_a_element(x) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Name(name) => meta_f_n(&name),
                Content::Address(_) => return Err(ErrorActive::NotSupported),
            }
        },
        Set::D => {
            match instruction.content {
                Content::All => {
                    let set = get_address_book_set()?;
                    for x in set.iter() {
                        match meta_d_a_element (x) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Name(name) => meta_d_n(&name),
                Content::Address(address) => meta_d_u(&address),
            }
        },
        Set::K => {
            let write = Write::OnlyNew;
            match instruction.content {
                Content::All => meta_kpt_a(&write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n(&name, &write),
                Content::Address(_) => return Err(ErrorActive::NotSupported),
            }
        },
        Set::P => {
            let write = Write::None;
            match instruction.content {
                Content::All => meta_kpt_a(&write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n(&name, &write),
                Content::Address(_) => return Err(ErrorActive::NotSupported),
            }
        },
        Set::T => {
            let write = Write::All;
            match instruction.content {
                Content::All => meta_kpt_a(&write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n(&name, &write),
                Content::Address(_) => return Err(ErrorActive::NotSupported),
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
fn meta_f_a_element (set_element: &NameHashAddress) -> Result<(), ErrorActive> {
    let meta_key_prefix = MetaKeyPrefix::from_name(&set_element.name);
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
        if let Ok(a) = x {
            let meta_values = MetaValues::from_entry_checked::<Active>(a)?;
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
fn meta_f_n (name: &str) -> Result<(), ErrorActive> {
    meta_f_a_element(&search_name(name)?)
}

/// Function to process individual address book entry in `load_metadata -d -a` run.
/// Expected behavior:  
/// fetch information from address, check it,
/// and print into `sign_me` output file.  
fn meta_d_a_element (set_element: &NameHashAddress) -> Result<(), ErrorActive> {
    let shortcut = shortcut_set_element(set_element)?;
    load_meta_print(&shortcut)
}

/// Function to process `load_metadata -d -n name` run.
/// Expected behavior:  
/// go through `address_book` and collect all unique NameHashAddress entries,
/// search through this set for the entry corresponding to the requested name,
/// fetch information from address, check it,
/// and print into `sign_me` output file.
fn meta_d_n (name: &str) -> Result<(), ErrorActive> {
    meta_d_a_element(&search_name(name)?)
}

/// Function to process `load_metadata -d -u url` run.
/// Expected behavior:  
/// fetch information from address, check it,
/// and print into `sign_me` output file.
fn meta_d_u (address: &str) -> Result<(), ErrorActive> {
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
fn meta_kpt_a (write: &Write, pass_errors: bool) -> Result<(), ErrorActive> {
    let set = get_address_book_set()?;
    let mut sorted_meta_values = prepare_metadata()?;
    for x in set.iter() {
        sorted_meta_values = match meta_kpt_a_element (x, write, &sorted_meta_values) {
            Ok(a) => a,
            Err(e) => {
                error_occured(e, pass_errors)?;
                sorted_meta_values
            },
        };
    }
    write_metadata(sorted_meta_values)
}

/// Function to process an individual element of
/// `load_metadata -k -a`, `load_metadata -p -a`, `load_metadata -t -a`, `load_metadata -a` runs.
/// Expected behavior:  
/// fetch information from address, check it,
/// insert in the sorted `meta_values`,
/// and print into `sign_me` output file depending on value of `write`.
fn meta_kpt_a_element (set_element: &NameHashAddress, write: &Write, sorted_meta_values: &SortedMetaValues) -> Result<SortedMetaValues, ErrorActive> {
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
fn meta_kpt_n (name: &str, write: &Write) -> Result<(), ErrorActive> {
    let mut sorted_meta_values = prepare_metadata()?;
    sorted_meta_values = meta_kpt_a_element(&search_name(name)?, write, &sorted_meta_values)?;
    write_metadata(sorted_meta_values)
}

/// Struct to collect network specname, genesis hash and fetching address from address book
#[derive(PartialEq)]
struct NameHashAddress {
    name: String,
    genesis_hash: [u8; 32],
    address: String,
}

/// Function to collect a vector of unique NameHashAddress entries from address book
fn get_address_book_set () -> Result<Vec<NameHashAddress>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut set: Vec<NameHashAddress> = Vec::new();
    for x in address_book.iter() {
        if let Ok(a) = x {
            let address_book_entry = AddressBookEntry::from_entry(a)?;
            // check that each name gets only one genesis hash
            for a in set.iter() {
                if a.name == address_book_entry.name {
                    if a.genesis_hash != address_book_entry.genesis_hash {return Err(ErrorActive::Database(DatabaseActive::TwoGenesisHashVariantsForName{name: address_book_entry.name.to_string()}))}
                    if a.address != address_book_entry.address {return Err(ErrorActive::Database(DatabaseActive::TwoUrlVariantsForName{name: address_book_entry.name.to_string()}))}
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
    if set.len() == 0 {return Err(ErrorActive::Database(DatabaseActive::AddressBookEmpty))}
    Ok(set)
}

/// Function to collect a vector of unique NameHashAddress entries from address book
/// and search for particular name in it,
/// outputs NameHashAddress
fn search_name (name: &str) -> Result<NameHashAddress, ErrorActive> {
    let set = get_address_book_set()?;
    let mut found = None;
    for x in set.into_iter() {
        if x.name == name {
            found = Some(x);
            break;
        }
    }
    match found {
        Some(a) => Ok(a),
        None => return Err(ErrorActive::NotFound(NotFoundActive::AddressBookEntryWithName{name: name.to_string()})),
    }
}

/// Function to process individual NameHashAddress entry:
/// do fetch, check fetched metadata for version,
/// check that genesis hash and network name are same in address book and in fetch
/// output MetaShortCut value
fn shortcut_set_element (set_element: &NameHashAddress) -> Result<MetaShortCut, ErrorActive> {
    let shortcut = meta_shortcut(&set_element.address)?;
    if shortcut.meta_values.name != set_element.name {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url: set_element.address.to_string(), what: Changed::Name{old: set_element.name.to_string(), new: shortcut.meta_values.name.to_string()}}))}
    if shortcut.genesis_hash != set_element.genesis_hash {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url: set_element.address.to_string(), what: Changed::GenesisHash{old: set_element.genesis_hash, new: shortcut.genesis_hash}}))}
    Ok(shortcut)
}


