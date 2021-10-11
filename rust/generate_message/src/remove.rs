use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE, SPECSTREEPREP};
use sled::IVec;
use anyhow;
use db_handling::helpers::{open_db, open_tree};
use parity_scale_codec::{Decode, Encode};
use definitions::{metadata::{AddressBookEntry, NameVersioned}, network_specs::generate_network_key};

use crate::parser::Remove;
use crate::error::{Error, NotFound, NotDecodeable};
use crate::helpers::specname_in_db;


/// Function to remove information from the database.
pub fn remove_info (info: Remove) -> anyhow::Result<()> {
    
    let database = open_db(HOT_DB_NAME)?;
    let metadata = open_tree(&database, METATREE)?;
    
    match info {
        Remove::Title(network_title) => {
            let address_book = open_tree(&database, ADDRESS_BOOK)?;
            let chainspecs = open_tree(&database, SPECSTREEPREP)?;
            let address_book_entry_encoded = match address_book.remove(&network_title.encode()) {
                Ok(Some(a)) => a,
                Ok(None) => return Err(Error::NotFound(NotFound::AddressBookKey(network_title)).show()),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            };
            let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            let network_key = generate_network_key(&address_book_entry.genesis_hash.to_vec(), address_book_entry.encryption);
            match chainspecs.remove(&network_key) {
                Ok(Some(_)) => (),
                Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            }
            if !specname_in_db(&address_book_entry.name, &address_book)? {
                let mut to_del: Vec<IVec> = Vec::new();
                for x in metadata.scan_prefix(address_book_entry.name.encode()) {if let Ok((a, _)) = x {to_del.push(a)}}
                for x in to_del.iter() {if let Err(e) = metadata.remove(x) {return Err(Error::InternalDatabaseError(e).show())}}
            }
            Ok(())
        },
        Remove::SpecNameVersion{name, version} => {
            let versioned_name = NameVersioned {name, version};
            match metadata.remove(&versioned_name.encode()) {
                Ok(Some(_)) => Ok(()),
                Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned(versioned_name)).show()),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            }
        },
    }
}


