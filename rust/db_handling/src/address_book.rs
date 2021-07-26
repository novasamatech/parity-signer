use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use definitions::{constants::ADDRESS_BOOK, defaults::get_default_address_book};
use anyhow;

use super::error::Error;

pub fn load_address_book (database_name: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let address_book: Tree = match database.open_tree(ADDRESS_BOOK) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match address_book.clear() {
        Ok(()) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let address_book_vec = get_default_address_book();
    
    for x in address_book_vec.iter() {
        let name = x.name.to_string();
        match address_book.insert(name.encode(), x.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    Ok(())

}
