use constants::{ADDRESS_BOOK, HOT_DB_NAME, METATREE};
use db_handling::helpers::{open_db, open_tree};
use definitions::{
    error::{Active, ErrorActive},
    metadata::{AddressBookEntry, MetaValues},
};

pub fn show_database() -> Result<(), ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    if metadata.len() == 0 {
        println!("Database has no metadata entries.");
        return Ok(());
    }
    println!("Database has metadata information for following networks:");
    for x in metadata.iter() {
        if let Ok(a) = x {
            let meta_values = MetaValues::from_entry_checked::<Active>(a)?;
            println!("\t{} {}", meta_values.name, meta_values.version);
        }
    }
    Ok(())
}

pub fn show_address_book() -> Result<(), ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    if address_book.len() == 0 {
        println!("Address book is empty.");
        return Ok(());
    }
    println!("Address book has entries for following networks:");
    for x in address_book.iter() {
        if let Ok(a) = x {
            let (title, address_book_entry) = AddressBookEntry::process_entry(a)?;
            if address_book_entry.def {
                println!(
                    "\t{} at {}, encryption {} (default)",
                    title,
                    address_book_entry.address,
                    address_book_entry.encryption.show()
                );
            } else {
                println!(
                    "\t{} at {}, encryption {}",
                    title,
                    address_book_entry.address,
                    address_book_entry.encryption.show()
                );
            }
        }
    }
    Ok(())
}
