use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use definitions::{constants::ADDRESS_BOOK, defaults::get_default_address_book};

pub fn load_address_book (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let address_book: Tree = database.open_tree(ADDRESS_BOOK)?;
    address_book.clear()?;
    
    let address_book_vec = get_default_address_book();
    
    for x in address_book_vec.iter() {
        let name = x.name.to_string();
        address_book.insert(name.encode(), x.encode())?;
    }
    
    database.flush()?;
    Ok(())

}
