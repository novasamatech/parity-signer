//use std::path::PathBuf;
//use wasm_loader::Source;
//use wasm_testbed::WasmTestBed;

use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE};
use db_handling::helpers::{open_db, open_tree};
use definitions::{error::{Active, Changed, DatabaseActive, ErrorActive, ErrorSource, Fetch, IncomingMetadataSourceActive, IncomingMetadataSourceActiveStr, MetadataError, MetadataSource, NotFoundActive/*, Wasm*/}, keyring::MetaKeyPrefix, metadata::{AddressBookEntry, MetaValues}};

use crate::parser::{Instruction, Content, Set};
use crate::metadata_db_utils::{add_new, SortedMetaValues, prepare_metadata, write_metadata};
use crate::helpers::{error_occured, filter_address_book_by_url, network_specs_from_entry, Write};
use crate::metadata_shortcut::{MetaShortCut, meta_shortcut};
use crate::output_prep::load_meta_print;


/// Function to generate `load_metadata` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_load_meta (instruction: Instruction) -> Result<(), ErrorActive> {
    
    if instruction.over.encryption.is_some() {return Err(ErrorActive::NotSupported)}
    if instruction.over.token.is_some() {return Err(ErrorActive::NotSupported)}
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
                Content::Address(_) => Err(ErrorActive::NotSupported),
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
                Content::Address(_) => Err(ErrorActive::NotSupported),
            }
        },
        Set::P => {
            let write = Write::None;
            match instruction.content {
                Content::All => meta_kpt_a(&write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n(&name, &write),
                Content::Address(_) => Err(ErrorActive::NotSupported),
            }
        },
        Set::T => {
            let write = Write::All;
            match instruction.content {
                Content::All => meta_kpt_a(&write, instruction.pass_errors),
                Content::Name(name) => meta_kpt_n(&name, &write),
                Content::Address(_) => Err(ErrorActive::NotSupported),
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
fn meta_f_a_element (set_element: &AddressSpecs) -> Result<(), ErrorActive> {
    let meta_key_prefix = MetaKeyPrefix::from_name(&set_element.name);
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked::<Active>(x)?;
        if meta_values.warn_incomplete_extensions {warn(&meta_values.name, meta_values.version);}
        if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
            if prefix_from_meta != set_element.base58prefix {
                return Err(<Active>::faulty_metadata(MetadataError::Base58PrefixSpecsMismatch{specs: set_element.base58prefix, meta: prefix_from_meta}, MetadataSource::Database{name: meta_values.name.to_string(), version: meta_values.version}))
            }
        }
        let shortcut = MetaShortCut {
            meta_values,
            genesis_hash: set_element.genesis_hash,
        };
        load_meta_print(&shortcut)?;
    }
    Ok(())
}

/// Function to process `load_metadata -f -n name` run.
/// Here `name` is network specname, i.e. `polkadot` for any polkadot version and encryption variety
/// (since load_metadata message does not contain info about network encryption).
/// At this moment simple export of only specific version of the metadata is not implemented.
/// Expected behavior:  
/// go through `address_book` and `specs_tree` to collect all unique AddressSpecs entries,
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
fn meta_d_a_element (set_element: &AddressSpecs) -> Result<(), ErrorActive> {
    let shortcut = shortcut_set_element(set_element)?;
    load_meta_print(&shortcut)
}

/// Function to process `load_metadata -d -n name` run.
/// Expected behavior:  
/// go through `address_book` and `specs_tree` to collect all unique AddressSpecs entries,
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
    if let Ok(a) = filter_address_book_by_url(address) {if !a.is_empty() {println!("Database contains entries for {} at {}. With `-d` setting key the fetched metadata integrity with existing database entries is not checked.", a[0].name, address)}}
    let shortcut = meta_shortcut(address)?;
    if shortcut.meta_values.warn_incomplete_extensions {warn(&shortcut.meta_values.name, shortcut.meta_values.version);}
    load_meta_print(&shortcut)
}

/// Function to process `load_metadata -k -a`, `load_metadata -p -a`, `load_metadata -t -a`, `load_metadata -a` runs.
/// Expected behavior:  
/// go through `address_book` and `specs_tree` to collect all unique AddressSpecs entries,
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
fn meta_kpt_a_element (set_element: &AddressSpecs, write: &Write, sorted_meta_values: &SortedMetaValues) -> Result<SortedMetaValues, ErrorActive> {
    let shortcut = shortcut_set_element(set_element)?;
    let upd_sorted = add_new(&shortcut.meta_values, sorted_meta_values)?;
    match write {
        Write::All => load_meta_print(&shortcut)?,
        Write::OnlyNew => if upd_sorted.upd_done {load_meta_print(&shortcut)?},
        Write::None => (),
    }
    if upd_sorted.upd_done {println!("Fetched new metadata {}{}", shortcut.meta_values.name, shortcut.meta_values.version)}
    else {println!("Fetched previously known metadata {}{}", shortcut.meta_values.name, shortcut.meta_values.version)}
    Ok(upd_sorted.sorted)
}

/// Function to process `load_metadata -k -n name`, `load_metadata -p -n name`,
/// `load_metadata -t -n name`, `load_metadata -n name` runs.
/// Expected behavior:  
/// collect known metadata from database and sort it, clear metadata from database,
/// go through `address_book` and `specs_tree` to collect all unique AddressSpecs entries,
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
struct AddressSpecs {
    address: String,
    base58prefix: u16,
    genesis_hash: [u8;32],
    name: String,
}

/// Function to collect a vector of unique AddressSpecs entries from address book
fn get_address_book_set () -> Result<Vec<AddressSpecs>, ErrorActive> {
    let mut set: Vec<AddressBookEntry> = Vec::new();
    {
        let database = open_db::<Active>(HOT_DB_NAME)?;
        let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
        for x in address_book.iter().flatten() {
            let address_book_entry = AddressBookEntry::from_entry(x)?;
            set.push(address_book_entry);
        }
    }
    if set.is_empty() {return Err(ErrorActive::Database(DatabaseActive::AddressBookEmpty))}
    let mut out: Vec<AddressSpecs> = Vec::new();
    for x in set.iter() {
        let specs = network_specs_from_entry(x)?;
        for y in out.iter() {
            if y.name == specs.name {
                if y.genesis_hash != specs.genesis_hash {return Err(ErrorActive::Database(DatabaseActive::TwoGenesisHashVariantsForName{name: x.name.to_string()}))}
                if y.address != x.address {return Err(ErrorActive::Database(DatabaseActive::TwoUrlVariantsForName{name: x.name.to_string()}))}
                if y.base58prefix != specs.base58prefix {return Err(ErrorActive::Database(DatabaseActive::TwoBase58ForName{name: x.name.to_string()}))}
            }
        }
        let new = AddressSpecs {
            address: x.address.to_string(),
            base58prefix: specs.base58prefix,
            genesis_hash: specs.genesis_hash,
            name: specs.name.to_string(),
        };
        if !out.contains(&new) {out.push(new)}
    }
    Ok(out)
}

/// Function to collect a vector of unique AddressSpecs entries from address book
/// and search for particular name in it,
/// outputs AddressSpecs
fn search_name (name: &str) -> Result<AddressSpecs, ErrorActive> {
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
        None => Err(ErrorActive::NotFound(NotFoundActive::AddressBookEntryWithName{name: name.to_string()})),
    }
}

/// Function to process individual AddressSpecs entry:
/// do fetch, check fetched metadata for version,
/// check that genesis hash and network name are same in address book and in fetch
/// output MetaShortCut value
fn shortcut_set_element (set_element: &AddressSpecs) -> Result<MetaShortCut, ErrorActive> {
    let shortcut = meta_shortcut(&set_element.address)?;
    if shortcut.meta_values.name != set_element.name {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url: set_element.address.to_string(), what: Changed::Name{old: set_element.name.to_string(), new: shortcut.meta_values.name}}))}
    if shortcut.genesis_hash != set_element.genesis_hash {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url: set_element.address.to_string(), what: Changed::GenesisHash{old: set_element.genesis_hash, new: shortcut.genesis_hash}}))}
    if let Some(prefix_from_meta) = shortcut.meta_values.optional_base58prefix {
        if prefix_from_meta != set_element.base58prefix {
            return Err(<Active>::faulty_metadata(MetadataError::Base58PrefixSpecsMismatch{specs: set_element.base58prefix, meta: prefix_from_meta}, MetadataSource::Incoming(IncomingMetadataSourceActive::Str(IncomingMetadataSourceActiveStr::Fetch{url: set_element.address.to_string()}))))
        }
    }
    if shortcut.meta_values.warn_incomplete_extensions {warn(&shortcut.meta_values.name, shortcut.meta_values.version);}
    Ok(shortcut)
}

/// Print warning if the metadata (v14) has incomplete set of signed extensions
fn warn(name: &str, version: u32) {
    println!("Warning. Metadata {}{} has incomplete set of signed extensions, and could cause Signer to fail in parsing signable transactions using this metadata.", name, version);
}
/*
/// Function to process .wasm files into signable entities and add metadata into the database
pub fn unwasm (filename: &str, update_db: bool) -> Result<(), ErrorActive> {
    let testbed = match WasmTestBed::new(&Source::File(PathBuf::from(&filename))) {
        Ok(a) => a,
        Err(e) => return Err(ErrorActive::Wasm(Wasm::WasmTestbed(e))),
    };
    let meta_values = MetaValues::from_runtime_metadata(testbed.metadata(), IncomingMetadataSourceActive::Wasm{filename: filename.to_string()})?;
    let set_element = search_name(&meta_values.name)?;
    if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
        if prefix_from_meta != set_element.base58prefix {
            return Err(<Active>::faulty_metadata(MetadataError::Base58PrefixSpecsMismatch{specs: set_element.base58prefix, meta: prefix_from_meta}, MetadataSource::Incoming(IncomingMetadataSourceActive::Wasm{filename: filename.to_string()})))
        }
    }
    let genesis_hash = set_element.genesis_hash;
    if update_db {
        let upd_sorted = add_new(&meta_values, &prepare_metadata()?)?;
        if upd_sorted.upd_done {println!("Unwasmed new metadata {}{}", meta_values.name, meta_values.version)}
        else {println!("Unwasmed previously known metadata {}{}", meta_values.name, meta_values.version)}
        write_metadata(upd_sorted.sorted)?;
    }
    let shortcut = MetaShortCut {
        meta_values,
        genesis_hash,
    };
    load_meta_print(&shortcut)
}
*/
