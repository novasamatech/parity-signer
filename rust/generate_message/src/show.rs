use sled::{Db, open, Tree};
use parity_scale_codec::Decode;
use definitions::{constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE}, metadata::{AddressBookEntry, NameVersioned}};


pub fn show_database() -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(HOT_DB_NAME)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    
    if metadata.len() == 0 {println!("No metadata entries in the database.")}
    else {
        println!("Database has metadata information for following networks:");
    
        for x in metadata.iter() {
            if let Ok((versioned_name_encoded, _)) = x {
                let versioned_name = <NameVersioned>::decode(&mut &versioned_name_encoded[..])?;
                println!("\t{} {}", versioned_name.name, versioned_name.version);
            }
        }
    }
    Ok(())
}


pub fn show_address_book() -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(HOT_DB_NAME)?;
    let address_book: Tree = database.open_tree(ADDRESS_BOOK)?;
    
    if address_book.len() == 0 {println!("No address book entries in the database.")}
    else {
        println!("Address book has entries for following networks:");
    
        for x in address_book.iter() {
            if let Ok((name_encoded, address_entry_encoded)) = x {
                let address_entry = <AddressBookEntry>::decode(&mut &address_entry_encoded[..])?;
                let name = <String>::decode(&mut &name_encoded[..])?;
                if name == address_entry.name {println!("\t{} at {}", name, address_entry.address);}
                else {println!("\t{} entry got corrupted, and should be deleted.", name);}
            }
        }
    }
    Ok(())
}
