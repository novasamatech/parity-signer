use sled::{Db, Tree, open};
use parity_scale_codec::{Decode, Encode};
use regex::Regex;
use lazy_static::lazy_static;
use hex;
use definitions::{constants::{METATREE, SPECSTREE}, metadata::NameVersioned, network_specs::{ChainSpecs, generate_network_key}};




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
/// Checks that only networks with network specs already in "to" database are processed,
/// so that no metadata without associated network specs enters the database
pub fn transfer_metadata (database_name_from: &str, database_name_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database_from: Db = open(database_name_from)?;
    let metadata_from: Tree = database_from.open_tree(METATREE)?;
    
    let database_to: Db = open(database_name_to)?;
    let metadata_to: Tree = database_to.open_tree(METATREE)?;
    let chainspecs_to: Tree = database_to.open_tree(SPECSTREE)?;
    
    for x in chainspecs_to.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = <ChainSpecs>::decode(&mut &network_specs_encoded[..])?;
            if network_key != generate_network_key(&network_specs.genesis_hash.to_vec()) {return Err(Box::from("Database corrupted. Genesis hash mismatch found."))}
            for y in metadata_from.scan_prefix(network_specs.name.encode()) {
                if let Ok((key, value)) = y {
                    metadata_to.insert(key, value)?;
                }
            }
        }
    }
    
    database_to.flush()?;
    Ok(())
    
}
