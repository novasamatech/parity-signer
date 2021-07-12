use definitions::{constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE, SPECSTREEPREP}};
use sled::{Db, open, Tree};

use super::parser::{Instruction, Content, Set};
use super::metadata_db_utils::{load_f_from_metadata_entry, load_f_from_name, do_d_from_address_book_entry, do_d_from_name, sign_me_prep_load_meta_from_address, upd_from_address_book_entry, upd_from_name, insert_from_address, error_occured};


/// Function to generate `load_meta` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_load_meta (instruction: Instruction) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(HOT_DB_NAME)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    let address_book: Tree = database.open_tree(ADDRESS_BOOK)?;
    let chainspecs: Tree = database.open_tree(SPECSTREEPREP)?;
    
    match instruction.set {
        Set::F => {
        // `-f` key: do NOT run rps calls, produce ALL requested files from existing database
            match instruction.content {
                Content::All => {
                // creating load_meta files for all existing database metadata entries
                    if metadata.len() == 0 {return Err(Box::from("No metadata entries in the database."))}
                    for x in metadata.iter() {
                        if let Ok(a) = x {
                            match load_f_from_metadata_entry(a, &address_book, &chainspecs) {
                                Ok(()) => (),
                                Err(e) => error_occured(e, instruction.pass_errors)?,
                            }
                        }
                    }
                    Ok(())
                },
                Content::Name(names) => {
                // creating load_meta files for a given set of names from existing database metadata entries
                    for name in names.iter() {
                        match load_f_from_name(name, &address_book, &chainspecs, &metadata) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Address(_) => {return Err(Box::from("Could not process url addresses without rpc queries."))}
            }
        },
        Set::D => {
        // `-d` key: do NOT update the database, make rpc calls, and produce ALL requested output files
            let flag = true; // going to make `load_meta` message
            match instruction.content {
                Content::All => {
                // creating load_meta for all address book entries
                    if address_book.len() == 0 {return Err(Box::from("No entries in the address book."))}
                    for x in address_book.iter() {
                        if let Ok(a) = x {
                            match do_d_from_address_book_entry(a, flag, &chainspecs) {
                                Ok(()) => (),
                                Err(e) => error_occured(e, instruction.pass_errors)?,
                            }
                        }
                    }
                    Ok(())
                },
                Content::Name(names) => {
                // creating load_meta for a given set of names in address book
                    for name in names.iter() {
                        match do_d_from_name (name, flag, &address_book, &chainspecs) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Address(addresses) => {
                // creating load_meta for a given set of addresses
                    for address in addresses.iter() {
                        match sign_me_prep_load_meta_from_address(&address) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
            }
        },
        _ => {
        // remaining set keys `-p`, '-k', and default `-t` all update database
            
            let flag = true; // going to make `load_meta` message
            
            let mut output_all = true;
            let mut output_only_new = false;
            if let Set::P = instruction.set {
                output_all = false;
                output_only_new = false;
            }
            if let Set::K = instruction.set {
                output_all = false;
                output_only_new = true;
            }
            match instruction.content {
                Content::All => {
                // use all address book entries
                    if address_book.len() == 0 {return Err(Box::from("No entries in the address book."))}
                    for x in address_book.iter() {
                        if let Ok(a) = x {
                            match upd_from_address_book_entry (a, flag, &chainspecs, &metadata, output_all, output_only_new) {
                                Ok(()) => (),
                                Err(e) => error_occured(e, instruction.pass_errors)?,
                            }
                        }
                    }
                    Ok(())
                },
                Content::Name(names) => {
                // use only a set of address book entries corresponding to given names
                    for name in names.iter() {
                        match upd_from_name (name, flag, &address_book, &chainspecs, &metadata, output_all, output_only_new) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Address(addresses) => {
                // use only a set of addresses
                // key suggests database update, so in case of missing network specs and/or missing address book entries
                // corresponding additions are made
                    for address in addresses.iter() {
                        match insert_from_address (flag, address, &metadata, &chainspecs, &address_book, output_all, output_only_new) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
            }
        },
    }
}

