use sled::{Db, Tree, open};
use parity_scale_codec::{Decode, Encode};
use regex::Regex;
use lazy_static::lazy_static;
use hex;
use definitions::{constants::{METATREE, SPECSTREE}, metadata::NameVersioned, network_specs::{ChainSpecs, generate_network_key}};
use anyhow;

use super::error::{Error, NotDecodeable, NotHex};



lazy_static! {
    static ref REG_META: Regex = Regex::new(r#"(?i)\["signer_metadata_(?P<name>[^\]]+)_v(?P<version>[0-9]+)","(0x)?(?P<meta>6d657461([0-9a-z][0-9a-z])+)"\]"#).unwrap();
}

pub fn load_metadata (database_name: &str, metadata_contents: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match metadata.clear() {
        Ok(()) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
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
        let meta_to_store = match hex::decode(&meta_hex) {
            Ok(x) => x,
            Err(_) => return Err(Error::NotHex(NotHex::DefaultMeta).show()),
        };
        match metadata.insert(new.encode(), meta_to_store) {
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


/// Function to transfer metadata content hot database into cold database
/// Checks that only networks with network specs already in "to" database are processed,
/// so that no metadata without associated network specs enters the database
pub fn transfer_metadata (database_name_from: &str, database_name_to: &str) -> anyhow::Result<()> {
    
    let database_from: Db = match open(database_name_from) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let metadata_from: Tree = match database_from.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let database_to: Db = match open(database_name_to) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let metadata_to: Tree = match database_to.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs_to: Tree = match database_to.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    for x in chainspecs_to.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
            };
            if network_key != generate_network_key(&network_specs.genesis_hash.to_vec()) {return Err(Error::GenesisHashMismatch.show())}
            for y in metadata_from.scan_prefix(network_specs.name.encode()) {
                if let Ok((key, value)) = y {
                    match metadata_to.insert(key, value) {
                        Ok(_) => (),
                        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
                    };
                }
            }
        }
    }
    
    match database_to.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    Ok(())
    
}
