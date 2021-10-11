use parity_scale_codec::Decode;
use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE};
use definitions::metadata::AddressBookEntry;
use db_handling::helpers::{open_db, open_tree};
use anyhow;

use crate::helpers::decode_and_check_meta_entry;
use crate::error::{Error, NotDecodeable};

pub fn show_database() -> anyhow::Result<()> {
    
    let database = open_db(HOT_DB_NAME)?;
    let metadata = open_tree(&database, METATREE)?;
    if metadata.len() == 0 {return Err(Error::MetadataEmpty.show())}
    println!("Database has metadata information for following networks:");
    for x in metadata.iter() {
        if let Ok(a) = x {
            let meta_values = decode_and_check_meta_entry(a)?;
            println!("\t{} {}", meta_values.name, meta_values.version);
        }
    }
    Ok(())
}


pub fn show_address_book() -> anyhow::Result<()> {
    
    let database = open_db(HOT_DB_NAME)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    
    if address_book.len() == 0 {return Err(Error::AddressBookEmpty.show())}
    println!("Address book has entries for following networks:");
    for x in address_book.iter() {
        if let Ok((name_encoded, address_book_entry_encoded)) = x {
            let address_book_entry = match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookEntry).show()),
            };
            let title = match <String>::decode(&mut &name_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressBookKey).show()),
            };
            if address_book_entry.def {println!("\t{} at {}, encryption {} (default)", title, address_book_entry.address, address_book_entry.encryption.show());}
            else {println!("\t{} at {}, encryption {}", title, address_book_entry.address, address_book_entry.encryption.show());}
        }
    }
    Ok(())
}
