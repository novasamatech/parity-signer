use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use regex::Regex;
use lazy_static::lazy_static;
use hex;
use definitions::{constants::{METATREE}, metadata::NameVersioned};




lazy_static! {
    static ref REG_META: Regex = Regex::new(r#"(?i)\["signer_metadata_(?P<name>[^\]]+)_v(?P<version>[0-9]+)","(0x)?(?P<meta>6d657461([0-9a-z][0-9a-z])+)"\]"#).unwrap();
}

pub fn load_metadata (database_name: &str, metadata_contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    metadata.clear()?;
    
    for caps in REG_META.captures_iter(&metadata_contents) {
        let new = NameVersioned {
            name: caps["name"].to_string(),
            version: caps["version"].parse()?,
        };
        let meta_hex = caps["meta"].to_string();
        let meta_to_store = hex::decode(&meta_hex)?;
        metadata.insert(new.encode(), meta_to_store)?;
    }
    
    database.flush()?;
    Ok(())
    
}


/// Function to transfer metadata content hot database into cold database
pub fn transfer_metadata (database_name_from: &str, database_name_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database_from: Db = open(database_name_from)?;
    let metadata_from: Tree = database_from.open_tree(METATREE)?;
    
    let database_to: Db = open(database_name_to)?;
    let metadata_to: Tree = database_to.open_tree(METATREE)?;
    
    for x in metadata_from.iter() {
        if let Ok((key, value)) = x {
            metadata_to.insert(key, value)?;
        }
    }
    
    database_to.flush()?;
    Ok(())
    
}
