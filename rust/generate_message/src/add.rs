use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE, SPECSTREEPREP};
use sled::{Tree, IVec};
use anyhow;
use db_handling::helpers::{open_db, open_tree};
use parity_scale_codec::Encode;
use definitions::crypto::Encryption;

use crate::parser::{Instruction, Content, Set};
use crate::metadata_db_utils::{add_new, prepare_metadata, write_metadata};
use crate::error::{Error, NotFound};
use crate::helpers::{decode_and_check_meta_entry, error_occured, network_specs_from_address_book_entry_encoded, get_from_tree, Write, update_db};
use crate::metadata_shortcut::{MetaSpecsShortCut, meta_specs_shortcut};
use crate::output_prep::add_network_print;


/// Function to generate `add_network` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_add_network (instruction: Instruction) -> anyhow::Result<()> {
    
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    let metadata = open_tree(&database, METATREE)?;
    let chainspecs = open_tree(&database, SPECSTREEPREP)?;
    
    match instruction.set {
        Set::F => {
            match instruction.content {
                Content::All => {
                    if let Some(_) = instruction.encryption_override {return Err(Error::NotSupported.show())}
                    else {
                        if address_book.len() == 0 {return Err(Error::AddressBookEmpty.show())}
                        for x in address_book.iter() {
                            if let Ok((_, address_book_entry_encoded)) = x {
                                match network_f_a_element(address_book_entry_encoded, &chainspecs, &metadata) {
                                    Ok(()) => (),
                                    Err(e) => error_occured(e, instruction.pass_errors)?,
                                }
                            }
                        }
                        Ok(())
                    }
                },
                Content::Name(name) => {
                    if let Some(_) = instruction.encryption_override {return Err(Error::NotSupported.show())}
                    else {network_f_n(&name, &address_book, &chainspecs, &metadata)}
                },
                Content::Address(_) => return Err(Error::NotSupported.show()),
            }
        },
        Set::D => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(_) => return Err(Error::NotSupported.show()),
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {network_d_u(&address, &address_book, &chainspecs, encryption)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
        Set::K => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(_) => return Err(Error::NotSupported.show()),
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {network_kpt_u(&address, &address_book, &chainspecs, &metadata, encryption, Write::OnlyNew)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
        Set::P => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(_) => return Err(Error::NotSupported.show()),
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {network_kpt_u(&address, &address_book, &chainspecs, &metadata, encryption, Write::None)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
        Set::T => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(_) => return Err(Error::NotSupported.show()),
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {network_kpt_u(&address, &address_book, &chainspecs, &metadata, encryption, Write::All)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
    }
}

/// Function to process an individual address book entry in `add_network -f -a` run.
/// Expected behavior:  
/// get network specs for the entry,
/// scan prefix in `metadata` database tree in search of network specname
/// to get all versions available in the database (max 2),
/// check meta_values integrity (network specname and spec_version),
/// and print into `sign_me` output file.  
fn network_f_a_element(address_book_entry_encoded: IVec, chainspecs: &Tree, metadata: &Tree) -> anyhow::Result<()> {
    let network_specs = network_specs_from_address_book_entry_encoded (address_book_entry_encoded, chainspecs)?;
    for x in metadata.scan_prefix(network_specs.name.encode()) {
        if let Ok(a) = x {
            let meta_values = decode_and_check_meta_entry(a)?;
            let specs_shortcut = MetaSpecsShortCut {
                meta_values,
                specs: network_specs.to_owned(),
                update: false,
            };
            add_network_print(&specs_shortcut)?;
        }
    }
    Ok(())
}

/// Function to process `add_network -f -n name` run.
/// Here `name` is network title from the database, the key for `address_book` entry.
/// Expected behavior:  
/// get from `address_book` the entry corresponding to the name, generate network key,
/// with it find network specs in `chainspecs` database tree,
/// scan prefix in `metadata` database tree in search of network specname
/// to get all versions available in the database (max 2),
/// check `meta_values` integrity (network specname and `spec_version`),
/// and print into `sign_me` output file.  
fn network_f_n(name: &str, address_book: &Tree, chainspecs: &Tree, metadata: &Tree) -> anyhow::Result<()> {
    match get_from_tree (&name.encode(), address_book)? {
        Some(address_book_entry_encoded) => network_f_a_element (address_book_entry_encoded, chainspecs, metadata),
        None => return Err(Error::NotFound(NotFound::AddressBookKey(name.to_string())).show()),
    }
}

/// Function to process `add_network -d -u url` run.
/// Expected behavior:  
/// fetch information from address, check it,
/// and print into `sign_me` output file.
fn network_d_u(address: &str, address_book: &Tree, chainspecs: &Tree, encryption: Encryption) -> anyhow::Result<()> {
    let shortcut = meta_specs_shortcut (address, address_book, chainspecs, encryption)?;
    add_network_print(&shortcut)
}

/// Function to process `add_network -k -u url`, `add_network -p -u url`, `add_network -t -u url` and `add_network -u url` runs.
/// Expected behavior:  
/// fetch information from address, check it, update the database with it,
/// and print into `sign_me` output file if needed.
fn network_kpt_u (address: &str, address_book: &Tree, chainspecs: &Tree, metadata: &Tree, encryption: Encryption, write: Write) -> anyhow::Result<()> {
    let shortcut = meta_specs_shortcut (address, address_book, chainspecs, encryption)?;
    if shortcut.update {update_db (address, &shortcut.specs, chainspecs, address_book)?}
    let sorted_meta_values = prepare_metadata(&metadata)?;
    let upd_sorted = add_new(&shortcut.meta_values, &sorted_meta_values)?;
    match write {
        Write::All => add_network_print(&shortcut)?,
        Write::OnlyNew => if upd_sorted.upd_done {add_network_print(&shortcut)?},
        Write::None => (),
    }
    write_metadata(sorted_meta_values, &metadata)
}
