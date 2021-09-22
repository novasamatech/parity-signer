use parity_scale_codec::Encode;
use definitions::{constants::ADDRESS_BOOK, defaults::get_default_address_book};
use anyhow;

use crate::helpers::{open_db, open_tree, flush_db, clear_tree, insert_into_tree};

pub fn load_address_book (database_name: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let address_book = open_tree(&database, ADDRESS_BOOK)?;
    clear_tree(&address_book)?;
    
    let address_book_vec = get_default_address_book();
    
    for x in address_book_vec.iter() {
        let name = x.name.to_string();
        insert_into_tree(name.encode(), x.encode(), &address_book)?;
    }
    
    flush_db(&database)?;
    Ok(())
}
