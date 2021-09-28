use parity_scale_codec::Encode;
use regex::Regex;
use lazy_static::lazy_static;
use constants::{METATREE, SPECSTREE};
use definitions::metadata::NameVersioned;
use anyhow;

use crate::error::{Error, NotHex};
use crate::helpers::{open_db, open_tree, flush_db, clear_tree, insert_into_tree, unhex, decode_chain_specs};



lazy_static! {
    static ref REG_META: Regex = Regex::new(r#"(?i)\["signer_metadata_(?P<name>[^\]]+)_v(?P<version>[0-9]+)","(0x)?(?P<meta>6d657461([0-9a-z][0-9a-z])+)"\]"#).unwrap();
}

pub fn load_metadata (database_name: &str, metadata_contents: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    clear_tree(&metadata)?;
    
    for caps in REG_META.captures_iter(&metadata_contents) {
        let version: u32 = match caps["version"].parse() {
            Ok(x) => x,
            Err(_) => return Err(Error::RegexVersion.show()),
        };
        let new = NameVersioned {
            name: caps["name"].to_string(),
            version,
        };
        let meta_hex = caps["meta"].to_string();
        
        let meta_to_store = unhex(&meta_hex, NotHex::DefaultMeta)?;
        insert_into_tree(new.encode(), meta_to_store, &metadata)?;
    }
    
    flush_db(&database)?;
    Ok(())
}


/// Function to transfer metadata content hot database into cold database
/// Checks that only networks with network specs already in "to" database are processed,
/// so that no metadata without associated network specs enters the database
pub fn transfer_metadata (database_name_from: &str, database_name_to: &str) -> anyhow::Result<()> {
    
    let database_from = open_db(database_name_from)?;
    let metadata_from = open_tree(&database_from, METATREE)?;
    let database_to = open_db(database_name_to)?;
    let metadata_to = open_tree(&database_to, METATREE)?;
    let chainspecs_to = open_tree(&database_to, SPECSTREE)?;
    
    for x in chainspecs_to.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
            for y in metadata_from.scan_prefix(network_specs.name.encode()) {
                if let Ok((key, value)) = y {
                    insert_into_tree(key.to_vec(), value.to_vec(), &metadata_to)?;
                }
            }
        }
    }
    flush_db(&database_to)?;
    Ok(())
    
}
