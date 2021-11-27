use constants::{ADDRESS_BOOK, HOT_DB_NAME};
use sled::{IVec};
use definitions::{crypto::Encryption, keyring::NetworkSpecsKey};
use db_handling::helpers::{open_db, open_tree};
use anyhow;

use crate::parser::{Instruction, Content, Set};
use crate::metadata_shortcut::meta_specs_shortcut;
use crate::output_prep::print_specs;
use crate::error::{Error, NotFound};
use crate::helpers::{get_and_decode_chain_specs_to_send, get_network_specs_from_address_book_entry, get_and_decode_address_book_entry, network_specs_from_address_book_entry_encoded, error_occured, filter_address_book_by_url, process_indices, update_db};


/// Function to generate `add_specs` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_add_specs (instruction: Instruction) -> anyhow::Result<()> {
    
    match instruction.set {
        Set::F => {
            match instruction.content {
                Content::All => {
                    if let Some(_) = instruction.encryption_override {return Err(Error::NotSupported.show())}
                    let mut address_book_set: Vec<IVec> = Vec::new();
                    {
                        let database = open_db(HOT_DB_NAME)?;
                        let address_book = open_tree(&database, ADDRESS_BOOK)?;
                        if address_book.len() == 0 {return Err(Error::AddressBookEmpty.show())}
                        for x in address_book.iter() {
                            if let Ok((_, address_book_entry_encoded)) = x {address_book_set.push(address_book_entry_encoded)}
                        }
                    }
                    for address_book_entry_encoded in address_book_set.iter() {
                        match specs_f_a_element(address_book_entry_encoded) {
                            Ok(()) => (),
                            Err(e) => error_occured(e, instruction.pass_errors)?,
                        }
                    }
                    Ok(())
                },
                Content::Name(name) => {
                    specs_f_n(&name, instruction.encryption_override)
                },
                Content::Address(address) => {
                    specs_f_u(&address, instruction.encryption_override)
                },
            }
        },
        Set::D => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(_) => return Err(Error::NotSupported.show()),
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {specs_d_u(&address, encryption)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
        Set::K => return Err(Error::NotSupported.show()),
        Set::P => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(name) => {
                    if let Some(encryption) = instruction.encryption_override {specs_pt_n(&name, encryption, false)}
                    else {return Err(Error::NotSupported.show())}
                },
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {specs_pt_u(&address, encryption, false)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        },
        Set::T => {
            match instruction.content {
                Content::All => return Err(Error::NotSupported.show()),
                Content::Name(name) => {
                    if let Some(encryption) = instruction.encryption_override {specs_pt_n(&name, encryption, true)}
                    else {return Err(Error::NotSupported.show())}
                },
                Content::Address(address) => {
                    if let Some(encryption) = instruction.encryption_override {specs_pt_u(&address, encryption, true)}
                    else {return Err(Error::NotSupported.show())}
                },
            }
        }
    }
}

/// Function to process individual address book entry in `add_specs -f -a` run.
/// Expected behavior:  
/// generate network key, by network key find network specs in `chainspecs` database tree, print into `sign_me` output file.  
fn specs_f_a_element (address_book_entry_encoded: &IVec) -> anyhow::Result<()> {
    let network_specs = network_specs_from_address_book_entry_encoded (address_book_entry_encoded)?;
    print_specs(&network_specs)
}

/// Function to process `add_specs -f -n name` run.
/// Here `name` is network title from the database, the key for `address_book` entry,
/// i.e. `polkadot` and `polkadot-ed25519` would be different entries
/// (since specs contain info about network encryption).
/// Expected behavior:  
/// get from `address_book` the entry corresponding to the name, generate network key,
/// with it find network specs in `chainspecs` database tree, print into `sign_me` output file.  
fn specs_f_n (name: &str, encryption_override: Option<Encryption>) -> anyhow::Result<()> {
    let mut network_specs = get_network_specs_from_address_book_entry(name)?;
    match encryption_override {
        Some(encryption) => {
            network_specs.encryption = encryption.clone();
            network_specs.title = format!("{}-{}", network_specs.name, encryption.show());
            print_specs(&network_specs)
        }
        None => print_specs(&network_specs)
    }
}

/// Function to process `add_specs -f -u url` run.
/// Expected behavior for NO encryption override:  
/// go through `address_book` database tree in search of all entries corresponding to url, generate network keys,
/// and with it find network specs in `chainspecs` database tree, print into `sign_me` output files.
/// Expected behavior for encryption override:  
/// go through `address_book` database tree in search of entry: (1) already with correct encryption,
/// (2) the one marked default, (3) any entry corresponding to url;
/// generate network key with old encryption, and with it find network specs in `chainspecs` database tree,
/// generate modified network specs (if not in case (1)) set with encryption override,
/// print into `sign_me` output file.
fn specs_f_u(address: &str, encryption_override: Option<Encryption>) -> anyhow::Result<()> {
    let entries = filter_address_book_by_url(address)?;
    if entries.len() == 0 {return Err(Error::NotFound(NotFound::Url(address.to_string())).show())}
    match encryption_override {
        Some(encryption) => {
            let network_specs = process_indices(&entries, encryption)?.0;
            print_specs(&network_specs)
        },
        None => {
            for x in entries.iter() {
                let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption);
                let network_specs = match get_and_decode_chain_specs_to_send(&network_specs_key)? {
                    Some(a) => a,
                    None => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
                };
                print_specs(&network_specs)?;
            }
            Ok(())
        }
    }
}

/// Function to process `add_specs -d -u url -encryption` run.
/// Expected behavior:  
/// go through address book in the database and search for given address;
/// if no entries found, do fetch (throw error if chainspecs turn up in the database), print `sign_me` file;
/// if entries found, search for appropriate network specs to modify, and print `sign_me` file.
fn specs_d_u(address: &str, encryption: Encryption) -> anyhow::Result<()> {
    let shortcut = meta_specs_shortcut (address, encryption)?;
    print_specs(&shortcut.specs)
}


/// Function to process `add_specs -p -n name -encryption`, `add_specs -t -n name -encryption` and `add_specs -n name -encryption` run.
/// Expected behavior:  
/// get from address book AddressBookEntry#1 corresponding to exact name;
/// generate NetworkSpecsKey#1 using encryption from AddressBookEntry#1,  
/// search through `chainspecs` tree for network specs ChainSpecsToSend#1,
/// if the encryption is matching, print `sign_me` file according to the key;
/// if not, generate NetworkSpecsKey#2 using override encryption,  
/// search through `chainspecs` tree for NetworkSpecsKey#2: if found, do nothing with database (chainspecs are already
/// in place meaning address book also should be in place and was not found only because name used in query was not exact fit),
/// print `sign_me` file according to the key;
/// if not found:
/// (1) modify ChainSpecsToSent#1 (encryption and title fields) and insert in `chainspecs` tree with NetworkSpecsKey#2,
/// (2) modify AddressBookEntry#1 (encryption and `def = false`) and insert in `address_book` tree with encoded `name-encryption` as a key  
/// and print `sign_me` file according to the key;
fn specs_pt_n(name: &str, encryption: Encryption, printing: bool) -> anyhow::Result<()> {
    let address_book_entry = get_and_decode_address_book_entry(name)?;
    let network_specs_key_existing = NetworkSpecsKey::from_parts(&address_book_entry.genesis_hash.to_vec(), &address_book_entry.encryption);
    let network_specs_existing = match get_and_decode_chain_specs_to_send(&network_specs_key_existing)? {
        Some(a) => a,
        None => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
    };
    if address_book_entry.encryption == encryption {
        if printing {print_specs(&network_specs_existing)}
        else {return Err(Error::SpecsInDb{name: name.to_string(), encryption}.show())}
    }
    else {
        let network_specs_key_possible = NetworkSpecsKey::from_parts(&address_book_entry.genesis_hash.to_vec(), &encryption);
        match get_and_decode_chain_specs_to_send(&network_specs_key_possible)? {
            Some(network_specs_found) => {
                if printing {print_specs(&network_specs_found)}
                else {return Err(Error::SpecsInDb{name: name.to_string(), encryption}.show())}
            },
            None => {
                // this encryption is not on record
                let mut network_specs = network_specs_existing;
                network_specs.encryption = encryption.clone();
                network_specs.title = format!("{}-{}", network_specs.name, encryption.show());
                update_db (&address_book_entry.address, &network_specs)?;
                if printing {print_specs(&network_specs)}
                else {Ok(())}
            },
        }
    }
}

/// Function to process `add_specs -p -u url -encryption`, `add_specs -t -u url -encryption` and `add_specs -u url -encryption` run.
/// Expected behavior:  
/// get from address book set of entries corresponding to given url address;
/// if no entries found, the network is new, and network specs are fetched;
/// if there are entries, search for appropriate network specs to modify, print `sign_me` file according to the key and update the database.
fn specs_pt_u(address: &str, encryption: Encryption, printing: bool) -> anyhow::Result<()> {
    let shortcut = meta_specs_shortcut (address, encryption)?;
    if shortcut.update {update_db (address, &shortcut.specs)?}
    if printing {print_specs(&shortcut.specs)?}
    Ok(())
}


