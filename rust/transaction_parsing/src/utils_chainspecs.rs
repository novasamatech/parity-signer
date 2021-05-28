use regex::Regex;
use frame_metadata::{RuntimeMetadataV12};
use meta_reading::{get_meta_const_light, decode_version};
use hex;
use parity_scale_codec::{Decode};


/// struct to store chain name, version and full metadata
/// as derived from metadata_contents database output

pub struct MetaValuesVersioned {
    pub name: String,
    pub version: u32,
    pub meta: String,
}


/// struct to store chain specs as derived from chain spec database

pub struct ChainSpecEntry {
    pub name: String,
    pub genesis_hash: String,
    pub base58prefix: u8,
    pub decimals: u8,
    pub unit: String,
}


/// function to collect MetaValuesVersioned entries from metadata database

pub fn collect_meta (metadata_contents: &str) -> Vec<MetaValuesVersioned> {
    let re = Regex::new(r#"(?i)\["signer_metadata_(?P<name>[\w]+)_v(?P<version>[0-9]+)","(0x)?6d657461(?P<meta>([0-9a-z][0-9a-z])+)"\]"#).unwrap();
    let mut metadata: Vec<MetaValuesVersioned> = Vec::new();
    for caps in re.captures_iter(&metadata_contents) {
        let new = MetaValuesVersioned {
            name: caps["name"].to_string(),
            version: caps["version"].parse().unwrap(),
            meta: caps["meta"].to_string(),
        };
        metadata.push(new);
    }
    metadata
}


/// function to generate ChainSpecEntry vector from chain spec database

pub fn get_genesis_hash (chain_spec_database: &str) -> Vec<ChainSpecEntry> {
    let re = Regex::new(r#"(?i)"decimals":(?P<decimals>[0-9]+),"genesisHash":"0x(?P<gen_hash>[0-9a-f]{64})"[^]]*"specName":"(?P<name>[^"]+)"[^]]*"prefix":(?P<prefix>[0-9]+)[^]]*"unit":"(?P<unit>[a-z]+)""#).unwrap();
    let mut out: Vec<ChainSpecEntry> = Vec::new();
    for caps in re.captures_iter(chain_spec_database) {
        let new = ChainSpecEntry {
            name: caps["name"].to_string(),
            genesis_hash: caps["gen_hash"].to_string(),
            base58prefix: caps["prefix"].parse().unwrap(),
            decimals: caps["decimals"].parse().unwrap(),
            unit: caps["unit"].to_string(),
        };
        out.push(new);
    }
    out
}


/// function to find the chain in spec database based on genesis hash,
/// and produce ChainSpecEntry for it

pub fn specs_from_genesis_hash (chain_spec_database: &str, genesis_hash_found: &str) -> Result<ChainSpecEntry, &'static str> {

// checking the input - should not get here ever
    if genesis_hash_found.len() != 64 {return Err("Wrong genesis hash length.");}

// reading the genesis hash database
    let hash_book = get_genesis_hash(chain_spec_database);

// find the corresponding chain name
    let mut entry_found = None;
    for x in hash_book {
        if &x.genesis_hash == genesis_hash_found {
            entry_found = Some(x);
            break;
        }
    }
    
    match entry_found {
        Some(x) => Ok(x),
        None => return Err("No matching genesis hash found.")
    }
}


/// function searches for full metadata for certain chain name and version in metadata database;
/// checks that found full metadata indeed corresponds to the queried name and version;
/// in case of successful find produces a tuple of corresponding RuntimeMetadataV12 and Option<u32>;
/// Option is None if the version of chain is the latest known one,
/// and Some(latest_version) if there are later versions available

pub fn find_meta(chain_name: &str, version: u32, metadata_contents: &str) -> Result<(RuntimeMetadataV12, Option<u32>), &'static str> {
   
    let metadata = collect_meta(metadata_contents);
    
    let mut meta = None;
    let mut other = false;
    let mut latest_version = version;
    for x in metadata.iter() {
        if x.name == chain_name {
            if x.version == version {
                meta = Some((&x.meta).to_string());
            }
            else {
                if x.version > latest_version {latest_version = x.version}
                other = true;
            }
        }
    }
    
    match meta {
        Some(m) => {
            let part1 = &m[0..2];
            let part1_vec = hex::decode(part1).expect("System error. Metadata should have been hex-decodeable.");
            let part1_decoded = u8::decode(&mut &part1_vec[..]).expect("System error. Unable to decode RuntimeMetadata version.");
            if part1_decoded < 12 {
                return Err("System error. Metadata version incompatible. Check frame_metadata crate version.");
            }
            let meta_unhex = hex::decode(&m[2..]).expect("System error. Metadata should have been hex-decodeable.");
            let data_back = RuntimeMetadataV12::decode(&mut &meta_unhex[..]);
            match data_back {
                Ok(metadata) => {
                // check if the name and version are same in metadata, i.e. the database is not damaged
                    match get_meta_const_light(&metadata) {
                        Ok(x) => {
                            let check = decode_version(x);
                            if (check.spec_version != version) || (check.specname != chain_name) {return Err("Network metadata entry corrupted in database. Please remove the entry and download the metadata for this network.")}
                        },
                        Err(_) => return Err("System error. No version in metadata.")
                    };
                    if version < latest_version {
                        Ok((metadata, Some(latest_version)))
                    }
                    else {Ok((metadata, None))}
                },
                Err(_) => return Err("System error. Unable to decode metadata."),
            }
        },
        None => {
            if other {return Err("No metadata on file for this version.")}
            else {return Err("No metadata on file for this network.")}
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_chain_specs() {
        let spec_book = fs::read_to_string("database_output").unwrap();
        let spec_book = get_genesis_hash(&spec_book);
        assert!(spec_book.len()==9, "Used to be 9. Found: {}", spec_book.len());
    }
    
    #[test]
    fn read_metadata_database() {
        let meta_book = fs::read_to_string("metadata_database.ts").unwrap();
        let meta_book = collect_meta(&meta_book);
        assert!(meta_book.len()==5, "Used to be 5. Found: {}", meta_book.len());
    }
}

