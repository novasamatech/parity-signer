use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use parity_scale_codec_derive;
use regex::Regex;
use lazy_static::lazy_static;
use hex;

use super::constants::{METATREE};

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct NameVersioned {
    pub name: String,
    pub version: u32,
}

lazy_static! {
    static ref REG_META: Regex = Regex::new(r#"(?i)\["signer_metadata_(?P<name>[^\]]+)_v(?P<version>[0-9]+)","(0x)?(?P<meta>6d657461([0-9a-z][0-9a-z])+)"\]"#).unwrap();
}

pub fn load_metadata (database_name: &str, metadata_contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    
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
