use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use hex;
use parity_scale_codec::{Encode, Decode};
use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};
use jsonrpsee_types::{
	jsonrpc::{JsonValue, Params},
	traits::Client,
};
use jsonrpsee_ws_client::WsClientBuilder;

/// struct to store the metadata values (chain name, chain
/// version, full meta data as hex string)

pub struct MetaValues {
    pub name: String,
    pub version: Option<u32>,
    pub meta: String,
}

/// struct for splitting metavalues

pub struct SplitMetaValues {
    pub latest: Vec<MetaValues>,
    pub historical: Vec<MetaValues>,
}

/// struct for splitting metavalues

pub struct UpdatingMetaValues {
    pub data: SplitMetaValues,
    pub flag: bool,
}

/// struct for address book import (chain name, wss address
/// to fetch metadata from)

pub struct AddressBookEntry <'a> {
    pub name: &'a str,
    pub address: &'a str,
}

/// struct to store raw freshly fetched metadata
/// (chain name, full metadata)

pub struct MetaEntry {
    pub name: String,
    pub meta: Result<String, Box<dyn std::error::Error>>,
}

/// struct to decode the version metadata

#[derive(Debug, Encode, Decode)]
pub struct VersionDecoded {
    pub specname: String,
    implname: String,
    auth_version: u32,
    pub spec_version: u32,
    impl_version: u32,
    apis: Vec<(u8, u32)>,
    trans_version: u32,
}

/// function takes string slice formatted as
/// 'export const blablaMetadataV** = '0x******''
/// and cuts out of it the chain name, version and full metadata,
/// doing some checking along the way

pub fn split_properly(x: &str) -> MetaValues {

    let mut meta_borders = Vec::new();
    let mut equals = Vec::new();
    let mut spaces = Vec::new();
    for (i, c) in x.chars().enumerate() {
        if c == '\'' {meta_borders.push(i)};
        if c == '=' {equals.push(i)};
        if c == ' ' {spaces.push(i)};
    }
    assert!(x.starts_with("export const "), "unexpected format of metadata: line does not start with 'export const '");
    assert!((meta_borders.len()==2), "expected exactly two quotation marks in metadata line");
    assert!((equals.len()==1), "expected exactly one equality sign");
    assert!((spaces[2]+1==equals[0])&&(spaces[3]-1==equals[0])&&(meta_borders[0]==equals[0]+2), "expected expression 'smthMetadata = '0x***'' within metadata line");
    
    let name_long = &x[(spaces[1]+1)..(spaces[2])];
    assert!(name_long.contains("Metadata"), "Unexpected name formatting");
    
    let mut version: Option<u32> = None;
    let mut vp = name_long.len();
    for (i, c) in name_long.char_indices().rev() {
        if c == 'V' {
            vp = i;
            version = match name_long[vp+1..].parse() {
                Ok(num) => Some(num),
                Err(_) => panic!("cannot parse the version"),
            };
        break;
        }
    }
    assert!(&name_long[vp-8..vp]=="Metadata", "unexpected metadata name formatting");
    
    assert!((&x[(meta_borders[0]+1)..(meta_borders[0]+3)]=="0x"), "expected '0x' as first two characters in hexadecimal metadata value");
    assert!(((&x[(meta_borders[0]+1)..(meta_borders[1])]).len()%2==0), "odd number of characters in metadata");
    for c in x[(meta_borders[0]+3)..(meta_borders[1])].chars() {
        assert!(c.is_ascii_hexdigit(), "unexpected characters in hexadecimal metadata value");
    }

    let out = MetaValues {
        name: (&name_long[..vp-8]).to_string(),
        version: version,
        meta: (&x[(meta_borders[0]+1)..(meta_borders[1])]).to_string(),
    };
    
    out
}

/// function takes full metadata in format '0x******', decodes
/// using RuntimeMetadataV12, finds version in constants block
/// within system module, and outputs version as a decodeable 
/// vector; some checking done along the way

// TODO: change unwrap() to error propagation thingy

pub fn get_meta_const (meta: &str) -> Result<Vec<u8>, &str> {

    if !meta.starts_with("0x6d657461") {
        return Err("No 'meta' starting sequence in metadata");
    }
    
    let part1 = &meta[10..12];
    let part1_vec = hex::decode(part1).unwrap();
    let part1_decoded = u8::decode(&mut &part1_vec[..]).unwrap();
    if part1_decoded != 12 {
        return Err("RuntimeMetadata version incompatible");
    }
    
    let meta_str = &(String::from(meta))[12..];
    let meta_work = hex::decode(meta_str).unwrap();
    let meta_back = RuntimeMetadataV12::decode(&mut &meta_work[..]).unwrap();
    
    let mut out = Vec::new();
    let mut system_block = false;
    let mut constants_version = false;
    
    if let DecodeDifferent::Decoded(meta_vector) = meta_back.modules {
        for x in meta_vector.iter() {
            if x.name==DecodeDifferent::Encode("System") {
                system_block = true;
                if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                    for y in constants_vector.iter() {
                        if y.name==DecodeDifferent::Encode("Version") {
                            constants_version = true;
                            if let DecodeDifferent::Decoded(fin) = &y.value {out = fin.to_vec();}
                            break;
                        }
                    }
                }
                break;
            }
        }
    }
    if !system_block {
        return Err("No system block found");
    }
    if !constants_version {
        return Err("No version found in constants");
    }
    if out.len()==0 {
        return Err("No version retrieved from constants");
    }

    Ok(out)
}


/// function takes version metadata vector (such as one output by
/// get_meta_const) and derives chain name and version packaged
/// in struct

pub fn decode_version (version_meta: Vec<u8>) -> VersionDecoded {
    let out = VersionDecoded::decode(&mut &version_meta[..]).unwrap();
    out
}

/// function takes version metadata in format '0x******' and
/// derives chain name and version packaged in struct

pub fn decode_version_from_hex (version_meta: &str) -> VersionDecoded {
    assert!(&version_meta[0..2]=="0x", "Unexpected string beginning");
    let meta_str = &(String::from(version_meta))[2..];
    let meta_work = hex::decode(meta_str).unwrap();
    decode_version(meta_work)
}

/// function to fetch the metadata as String from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_metadata(str_address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default().build(str_address).await?;
    let response: JsonValue = client.request("state_getMetadata", Params::None).await?;
    let out = match response {
        JsonValue::String(x) => x,
        _ => panic!("unexpected state metadata format"),
    };
    Ok(out)
}

/// function to get AddressBookEntry from line in address book

pub fn get_address (x: &str) -> AddressBookEntry {
    let re = Regex::new(r#"\S*: "\S*""#).unwrap();
    let blem: Vec<&str> = re.find_iter(x).map(|mat| mat.as_str()).collect();
    assert!(blem.len()==2, "Wrong formatting in address book");
    assert!(blem[0].starts_with("name: \""), "Wrong name formatting");
    assert!(blem[1].starts_with("address: \"wss://"), "Wrong address formatting");
    let out = AddressBookEntry {
        name: &blem[0][7..(&blem[0].len()-1)],
        address: &blem[1][10..(&blem[1].len()-1)],
    };
    out
}

/// function to make MetaEntry out of AddressBookEntry,
/// actually fetches stuff, is slow

pub fn make_meta_entry (x: &AddressBookEntry) -> MetaEntry {
    let out = MetaEntry {
        name: x.name.to_string(),
        meta: fetch_metadata(x.address),
    };
    out
}

/// function to sort through existing metadata and produce subsets of latest existing metadata and historical existing metadata

pub fn split_existing_metadata (old: Vec<MetaValues>) -> SplitMetaValues {
    let mut meta_map = HashMap::new();
    let mut latest_existing: Vec<MetaValues> = Vec::new();
    let mut historical_existing: Vec<MetaValues> = Vec::new();
    for x in old.iter() {
        let count = meta_map.entry(&x.name).or_insert(0);
        *count +=1;
        let mut index = None;
        let mut x_wins = false;
        for (i, y) in latest_existing.iter().enumerate() {
            if x.name == y.name {
                assert!(x.version != y.version, "Input file corrupted: chain {} has two entries of same version number or two no version entries", x.name);
                index = Some(i);
                if let Some(z) = x.version {
                    match y.version {
                        Some(p) => {
                            if z>p {x_wins = true;}
                        },
                        None => {x_wins = true;},
                    }
                }
                break;
            }
        }
        let x_for_push = MetaValues {
            name: x.name.to_owned(),
            version: x.version,
            meta: x.meta.to_owned(),
        };
        match index {
        Some(i) => {
            if x_wins {
                historical_existing.push(latest_existing.remove(i));
                latest_existing.push(x_for_push);
            }
            else {
                historical_existing.push(x_for_push);
            }
        },
        None => latest_existing.push(x_for_push)
        }
    }
    for (key, val) in meta_map.iter() {
        assert!(val<=&2, "More than two entries for chain {}", key);
    }
    let out = SplitMetaValues {
        latest: latest_existing,
        historical: historical_existing,
    };
    out
}

/// kicking a value with certain name (if found) from vector of MetaValues

fn kick (check: &MetaValues, mut hist: Vec<MetaValues>) -> Vec<MetaValues> {
    let mut num = None;
    for (i, x) in hist.iter().enumerate() {
        if x.name == check.name {
            num = Some(i);
            break;
        }
    }
    if let Some(i) = num {hist.remove(i);}
    hist
}

/// function to search vector of MetaValues for MetaValues entry with certain name
/// and update if the version is missing but is derivable

pub fn sar (name: String, mut existing: SplitMetaValues, log_file: &str) -> UpdatingMetaValues {
    let mut flag = false;
    for x in existing.latest.iter_mut() {
        if x.name == name {
            if let None = x.version {
                if let Ok(try_me) = get_meta_const(&x.meta) {
                    let v = decode_version(try_me).spec_version;
                    x.version = Some(v);
                    flag = true;
// make entry into log file
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open(log_file)
                        .unwrap();
                    if let Err(e) = writeln!(file, "{}", format!("* Updated to have version {} visibly recorded.", v)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
            break;
        }
    }
// output
    let out = UpdatingMetaValues {
    data: existing,
    flag: flag,
    };
    out
}

/// function to update SplitMetaValues with one new MetaValues datapiece,
/// writes log in logfile

pub fn update (new: MetaValues, mut existing: SplitMetaValues, log_file: &str) -> UpdatingMetaValues {
// logline, String, to be updated in all cases new.name is encountered in existing.latest
    let mut logline = None;
// flag: false if no change in existing.historical is needed
    let mut hist = false;
// index in existing.latest of new.name encounter in case the existing.latest needs to be updated
    let mut index = None;
// scanning through all existing.latest in search of new.name; break if found, because ensured that only one encounter is possible beforehand
    for (i, x) in existing.latest.iter_mut().enumerate() {
        if new.name == x.name {
// trying to decifer the version from full metadata - for networkMetadata.ts in old formatting, on_run is true if the old version was derived on the run
            let mut on_run = false;
            if let None = x.version {
                if let Ok(try_me) = get_meta_const(&x.meta) {
                    x.version = Some(decode_version(try_me).spec_version);
                    on_run = true;
                }
            }
            match x.version {
                Some(y) => {
                    match new.version {
                        Some (z) => {
                            if y == z {
                                if new.meta == x.meta {
                                    if on_run {logline = Some(format!("* Same version. Updated to have version {} visibly recorded.", z));}
                                    else {logline = Some(format!("* Version {} up to date.", z));}
                                }
                                else {
                                    logline = Some(format!("A Alarm. Same version {} has different metadata. Manual intervention needed. Older version was derived on run: {}", z, on_run));
                                }
                            }
                            else {
                                if y < z {
                                    index = Some(i);
                                    if on_run {logline = Some(format!("S Updated: new version {} was fetched. Older version {} metadata was updated to have version visibly recorded, and sent to historical record.", z, y));}
                                    else {logline = Some(format!("S Updated: new version {} was fetched. Older version {} metadata was sent to historical record.", z, y));}
                                    hist = true;
                                }
                                else {
                                    logline = Some(format!("A Alarm. Version decreased from {} to {}. Manual intervention needed. Older version was derived on run: {}", y, z, on_run));
                                }
                            }
                        },
                        None => {
                            logline = Some(format!("A Alarm. Version disappeared. Previous known version {}, was derived on run: {}. Manual intervention needed.", y, on_run));
                        },
                    }
                },
                None => {
                    match new.version {
                        Some(z) => {
                            index = Some(i);
                            logline = Some(format!("S Updated to new version {}. Old metadata without version sent to historical record.", z));
                            hist = true;
                        },
                        None => {
                            if new.meta == x.meta {
                                logline = Some(String::from("* No updates made. No version in both existing and retrieved metadata. No changes in metadata."));
                            }
                            else {
                                index = Some(i);
                                logline = Some(String::from("S Updated, only newly retrieved metadata kept. No version in both existing and retrieved metadata, and metadata changed."));
                            }
                        },
                    }
                },
            }
            break;
        }
    }
// making flag for output: true if there are some recordeable changes
    let mut flag = false;
// checking if the name was on the list
    let logline = match logline {
        Some(l) => {
// updating latest and historical
            if let Some(i) = index {
                if !hist {
                    existing.latest.remove(i);
                }
                else {
                    existing.historical = kick(&new, existing.historical);
                    existing.historical.push(existing.latest.remove(i));
                }
                existing.latest.push(new);
                flag = true;
            }
            l
        },
        None => {
            flag = true;
            let out = match new.version {
                Some(v) => format!("S Added version {} as a new entry", v),
                None => String::from("S Added as a new entry without version"),
            };
            existing.latest.push(new);
            out
        }
    };
// make entry into log file
    let mut file = OpenOptions::new()
        .append(true)
        .open(log_file)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", logline) {
        eprintln!("Couldn't write to file: {}", e);
    }
// output
    let out = UpdatingMetaValues {
    data: existing,
    flag: flag,
    };
    out
}

/// some tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn decoding_hex() {
        let piece = "0x20706f6c6b61646f743c7061726974792d706f6c6b61646f74000000001d0000000000000030df6acb689907609b0300000037e397fc7c91f5e40100000040fe3ad401f8959a04000000d2bc9897eed08f1502000000f78b278be53f454c02000000af2c0297a23e6d3d01000000ed99c5acb25eedf502000000cbca25e39f14238702000000687ad44ad37f03c201000000ab3c0572291feb8b01000000bc9d89904f5b923f0100000037c8bb1350a9a2a80100000006000000";
        let decoded = decode_version_from_hex(piece);
        assert!(decoded.specname=="polkadot", "Incorrectly parsed name [hex]");
        assert!(decoded.spec_version==29, "Incorrectly parsed version [hex]");
    }

    #[test]
    fn importing_text_cutting_text_decoding_vector() {
        let filename = "for_tests/networkMetadata_historical.ts";
        let contents = fs::read_to_string(filename).unwrap();
        let metadata_collection: Vec<MetaValues> = {
            contents
            .lines()
            .filter(|line| line.contains("export const"))
            .map(|line| split_properly(line))
            .collect()
        };
        assert!(metadata_collection.len()==8, "There used to be 8 entries when this test was made, found {}", metadata_collection.len());
        let test = &metadata_collection[7];
        let result = get_meta_const(&test.meta).unwrap();
        let decoded = decode_version(result);
        assert!(decoded.specname=="rococo", "Incorrectly parsed name [file]");
        assert!(decoded.spec_version==214, "Incorrectly parsed version [file]");
        let split1 = split_existing_metadata(metadata_collection).latest;
        assert!(split1.len()==8, "Split metadata performance changed");
    }
    
    #[test]
    fn fetching_westend() {
        let addr = "wss://westend-rpc.polkadot.io";
        let fetch_test0 = fetch_metadata(addr).unwrap();
        let result = get_meta_const(&fetch_test0).unwrap();
        let decoded = decode_version(result);
        assert!(decoded.specname=="westend", "Incorrectly parsed name [web fetch]");
        assert!(decoded.spec_version==50, "Incorrectly parsed version [web fetch]");
    }
    
    #[test]
    fn fetching_centrifuge() {
        let addr = "wss://fullnode.centrifuge.io";
        let fetch_test0 = fetch_metadata(addr).unwrap();
        let result = get_meta_const(&fetch_test0);
        assert!(result==Err("RuntimeMetadata version incompatible"), "Centrifuge used RuntimeMetadata version 11. Either that changed or can't figure that version 11 is not parseable.");
    }
    
    #[test]
    fn fetching_edgeware() {
        let addr = "wss://mainnet1.edgewa.re";
        let fetch_test0 = fetch_metadata(addr).unwrap();
        let result = get_meta_const(&fetch_test0);
        assert!(result==Err("No version found in constants"), "Edgeware used to have no edgeware version in constants at RuntimeMetadata version 12. Either that changed or can't figure that no version was found.");
    }
    
    #[test]
    fn reading_address_book() {
        let filename = "address_book";
        let contents = fs::read_to_string(filename).unwrap();
        let address_book: Vec<AddressBookEntry> = {
            contents
            .lines()
            .map(|line| get_address(line))
            .collect()
        };
        assert!(address_book.len()==8, "There used to be 8 entries when this test was made, found {}", address_book.len());
    }
    
    #[test]
    fn making_meta_entry() {
        let x = AddressBookEntry {
            name: "westend",
            address: "wss://westend-rpc.polkadot.io"
        };
        let out = make_meta_entry(&x);
        assert!(out.name=="westend", "Error making MetaEntry from westend AddressBookEntry");
    }
}

