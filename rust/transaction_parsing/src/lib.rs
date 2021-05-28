use regex::Regex;
use lazy_static::lazy_static;
use hex;
use parity_scale_codec::{Decode, HasCompact, Compact};
use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};
use base58::{ToBase58, FromBase58};
use blake2_rfc::blake2b::{blake2b};
use meta_reading::*;
use printing_balance::*;
use std::convert::TryInto;
use std::collections::HashMap;
use serde_json;
use std::mem::size_of;
use bitvec::prelude::*;
use sp_arithmetic::{Percent, Perbill, PerU16};
use sp_runtime::generic::Era;

const PREFIX: &[u8] = b"SS58PRE";

/// struct to store the chain name and genesis hash
pub struct ChainSpecEntry {
    pub name: String,
    pub genesis_hash: String,
    pub base58prefix: u8,
    pub decimals: u8,
    pub unit: String,
}

/// struct to store the argument name and type for method
pub struct Argument {
    pub name: String,
    pub ty: String,
}

#[derive(serde::Serialize)]
/// struct to store decoded arguments
pub struct ArgumentFilled {
    pub name: String,
    pub value: String,
}

/// struct to store the method information
pub struct Method {
    pub pallet_name: String,
    pub method_name: String,
    pub arguments: Vec<Argument>
}

#[derive(serde::Serialize)]
/// struct to store decoded method information
pub struct MethodFilled {
    pub pallet_name: String,
    pub method_name: String,
    pub arguments: Vec<ArgumentFilled>
}

/// function to convert [u8; 32] array into base58 address,
/// needs u8 prefix (defauls used BASE58PREFIX)
pub fn arr_to_base (array: [u8; 32], prefix: u8) -> String {
    let mut fin = vec![prefix];
    fin.extend_from_slice(&array.to_vec());
    let hash = blake2b(64, &[], &[PREFIX, &fin].concat());
    fin.extend_from_slice(&hash.as_bytes()[0..2]);
    fin.to_base58()
}

/// function to convert 64 symbol hex string into base58
/// address, needs u8 prefix (default used BASE58PREFIX)

pub fn hex_to_base (hex_part: &str, prefix: u8) -> String {
    assert!(hex_part.len()==64, "Wrong hex part length");
    let mut fin = vec![prefix];
    let part_unhex = hex::decode(&hex_part).unwrap();
    fin.extend_from_slice(&part_unhex);
    let hash = blake2b(64, &[], &[PREFIX, &fin].concat());
    fin.extend_from_slice(&hash.as_bytes()[0..2]);
    fin.to_base58()
}

/// function to convert base58 address into [u8; 32] array

pub fn base_to_arr (address: &str) -> [u8; 32] {
    let address_unbase = address.from_base58().unwrap();
// cut off the prefix [0] and the hash [2 last symbols]
    let part = &address_unbase[1..(address_unbase.len()-2)];
    let hash_part = &address_unbase[(address_unbase.len()-2)..];
    let hash = blake2b(64, &[], &[PREFIX, &address_unbase[..(address_unbase.len()-2)]].concat());
// check hash
    assert!(&hash.as_bytes()[0..2] == hash_part, "Hash error?");
    part.try_into().unwrap()
}

/// function to convert a base58 address into 64 symbol hex string

pub fn base_to_hex (address: &str) -> String {
    let address_unbase = address.from_base58().unwrap();
// cut off the prefix [0] and the hash [2 last symbols]
    let part = &address_unbase[1..(address_unbase.len()-2)];
    let hash_part = &address_unbase[(address_unbase.len()-2)..];
    let hash = blake2b(64, &[], &[PREFIX, &address_unbase[..(address_unbase.len()-2)]].concat());
// check hash
    assert!(&hash.as_bytes()[0..2] == hash_part, "Hash error?");
    hex::encode(part)
}

/// function vector of ChainGenHash values using genesis hash database

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

/// function to find the chain name in database based on genesis hash

pub fn specs_from_genesis_hash (chain_spec_database: &str, genesis_hash_found: &str) -> Result<ChainSpecEntry, &'static str> {

// checking the input
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

struct MetaValuesVersioned {
    name: String,
    version: u32,
    meta: String,
}

/// function to fetch full metadata of latest version of chain with known name from file
/// at this point looks for correct chain name and, if chain is versioned in database, correct chain version in latest;
/// if database entry has no chain version, metadata is accepted
pub fn find_meta(chain_name: &str, version: u32, metadata_contents: &str) -> Result<(RuntimeMetadataV12, Option<u32>), &'static str> {
   
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

/// function to fetch and decifer metadata and search through it for
/// pallet number and method number, outputs Method

pub fn find_method (pallet_index: u8, method_index: u8, meta: &RuntimeMetadataV12) -> Result<Method, &'static str> {
    let mut found_pallet_name = None;
    let mut found_method_name = None;
    let mut arguments: Vec<Argument> = Vec::new();
    
    if let DecodeDifferent::Decoded(meta_vector) = &meta.modules {
        for y in meta_vector.iter() {
            if y.index == pallet_index {
                if let DecodeDifferent::Decoded(name) = &y.name {
                    found_pallet_name = Some(name.to_string());
                }
                if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                    if calls.len() <= method_index.into() {return Err("Method not found, index too high");}
                    else {
                        if let DecodeDifferent::Decoded(nm) = &calls[method_index as usize].name {
                            found_method_name = Some(nm.to_string());
                        }
                        if let DecodeDifferent::Decoded(args) = &calls[method_index as usize].arguments {
                            for a in args.iter() {
                                let mut name_a = None;
                                let mut ty_a = None;
                                if let DecodeDifferent::Decoded(b) = &a.name {name_a = Some(b.to_string())}
                                if let DecodeDifferent::Decoded(c) = &a.ty {ty_a = Some(c.to_string())}
                                match name_a {
                                    Some(x) => {
                                        match ty_a {
                                            Some(y) => {arguments.push(Argument{name: x, ty: y});},
                                            None => {return Err("Arguments type error.")},
                                        }
                                    },
                                    None => {return Err("Arguments name error.")},
                                }
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    match found_pallet_name {
        Some(x) => {
            match found_method_name {
                Some(y) => {
                    let out = Method {
                        pallet_name: x,
                        method_name: y,
                        arguments: arguments,
                    };
                    Ok(out)
                },
                None => return Err("Method not found"),
            }
        },
        None => return Err("Pallet not found"),
    }
}

/// function to make a hashmap of all types encountered in chain methods

pub fn map_types (meta: &RuntimeMetadataV12) -> HashMap<String, u32> {
    
    let mut types_map = HashMap::new();
    
    if let DecodeDifferent::Decoded(meta_vector) = &meta.modules {
        for y in meta_vector.iter() {
            if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                for z in calls {
                    if let DecodeDifferent::Decoded(args) = &z.arguments {
                        for a in args.iter() {
                            if let DecodeDifferent::Decoded(c) = &a.ty {
                                let count = types_map.entry(c.to_string()).or_insert(0);
                                *count +=1;
                            }
                        }
                    }
                }
            }
        }
    }
    types_map
}

/// function to make a hashmap of all types in all chains for file

pub fn map_types_all (metadata_contents: &str) -> HashMap<String, u32> {
    
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
    
    let mut types_map = HashMap::new();
    
    for x in metadata.iter() {
        let meta_unhex = hex::decode(&x.meta[2..]).unwrap();
        if let Ok(data_back) = RuntimeMetadataV12::decode(&mut &meta_unhex[..]) {
            if let DecodeDifferent::Decoded(meta_vector) = data_back.modules {
                for y in meta_vector.iter() {
                    if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                        for z in calls {
                            if let DecodeDifferent::Decoded(args) = &z.arguments {
                                for a in args.iter() {
                                    if let DecodeDifferent::Decoded(c) = &a.ty {
                                        let count = types_map.entry(c.to_string()).or_insert(0);
                                        *count +=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    types_map
}

#[derive(Debug)]
pub enum EnumVariantType {
    None,
    Type(String),
    Struct(Vec<StructField>),
}

#[derive(Debug)]
pub struct EnumVariant {
    pub variant_name: String,
    pub variant_type: EnumVariantType,
}

#[derive(Debug)]
pub struct StructField {
    pub field_name: Option<String>,
    pub field_type: String,
}

#[derive(Debug)]
pub enum Description {
    Type(String),
    Enum(Vec<EnumVariant>),
    Struct(Vec<StructField>)
}

#[derive(Debug)]
pub struct TypeEntry {
    pub name: String,
    pub description: Description,
}

// Making statics for regex parsing of type database

lazy_static! {
    static ref REG_STRUCTS_WITH_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\{(?P<description>(\n +(pub )?\w+: .*(,)?)*\n)\}"#).unwrap();
    static ref REG_STRUCTS_NO_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\((pub )?(?P<description>.*)\)"#).unwrap();
    static ref REG_STRUCT_FIELDS: Regex = Regex::new(r#"(?m)^ +(pub )?(?P<field_name>[\w]+): (?P<field_type>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?$"#).unwrap();
    static ref REG_ENUM: Regex = Regex::new(r#"(pub )?enum (?P<name>.*?)( )?\{(?P<description>(\n +\w+( )?(\(.*\))?(\{[^}]*\})?(,)?)*)\n\}"#).unwrap();
    static ref REG_ENUM_VARIANTS: Regex = Regex::new(r#"(?m)^ +(?P<variant_name>\w+)(( )?(?P<variant_type>(\(.*\))|(\{[^}]*\})))?(,)?$"#).unwrap();
    static ref REG_ENUM_SIMPLE: Regex = Regex::new(r#"(?m)^(?P<simple_type>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))$"#).unwrap();
    static ref REG_ENUM_TUPLE: Regex = Regex::new(r#"(?P<type_in_tuple>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))(, )?"#).unwrap();
    static ref REG_ENUM_STRUCT: Regex = Regex::new(r#"(\n)?( +)?(?P<struct_field_name>\w+): (?P<struct_field_type>)(?P<fieldtype>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?"#).unwrap();
    static ref REG_TYPES: Regex = Regex::new(r#"(?m)(pub )?type (?P<name>.*) = (?P<description>.*);$"#).unwrap();
}

/// function to process the external file with types description (structs, enums, custom types) and generate a vector of entries

pub fn generate_type_database (type_info: &str) -> Vec<TypeEntry> {
    
// output preparation
    let mut output_prep: Vec<TypeEntry> = Vec::new();

    for caps1 in REG_STRUCTS_WITH_NAMES.captures_iter(type_info) {
        let struct_name = (&caps1["name"]).to_string();
        let struct_description = (&caps1["description"]).to_string();
        let mut struct_fields: Vec<StructField> = Vec::new();
        for caps2 in REG_STRUCT_FIELDS.captures_iter(&struct_description) {
            let new = StructField {
                field_name: Some((&caps2["field_name"]).to_string()),
                field_type: (&caps2["field_type"]).to_string(),
            };
            struct_fields.push(new);
        }
        let new_entry = TypeEntry {
            name: struct_name,
            description: Description::Struct(struct_fields),
        };
        output_prep.push(new_entry);
    }
    for caps in REG_STRUCTS_NO_NAMES.captures_iter(type_info) {
        let only_field = StructField {
            field_name: None,
            field_type: (&caps["description"]).to_string()
        };
        let new_entry = TypeEntry {
            name: (&caps["name"]).to_string(),
            description: Description::Struct(vec![only_field]),
        };
        output_prep.push(new_entry);
    }
    for caps1 in REG_ENUM.captures_iter(type_info) {
        let enum_name = (&caps1["name"]).to_string();
        let enum_description = (&caps1["description"]).to_string();
        let enum_variants = enum_description
            .lines()
            .filter(|line| REG_ENUM_VARIANTS.is_match(line))
            .map(|line| {
                let caps2 = REG_ENUM_VARIANTS.captures(&line).unwrap();
                let variant_name = caps2.name("variant_name").unwrap().as_str().to_string();
                let variant_type = match caps2.name("variant_type") {
                    None => EnumVariantType::None,
                    Some(a) => {
                        let x = a.as_str().to_string();
                        if x.starts_with("(") {
                            // either a single type or a tuple
                            match REG_ENUM_SIMPLE.captures(&x[1..x.len()-1]){
                                // single type
                                Some(b) => EnumVariantType::Type(b.name("simple_type").unwrap().as_str().to_string()),
                                // tuple
                                None => EnumVariantType::Type(x),
                            }
                        }
                        else {
                            // struct with struct-like fields
                            let mut type_is_struct: Vec<StructField> = Vec::new();
                            for caps3 in REG_ENUM_STRUCT.captures_iter(&x) {
                                let new = StructField {
                                    field_name: Some((&caps3["field_name"]).to_string()),
                                    field_type: (&caps3["field_type"]).to_string(),
                                };
                                type_is_struct.push(new);
                            }
                            EnumVariantType::Struct(type_is_struct)
                        }
                    },
                };
                EnumVariant {
                    variant_name,
                    variant_type,
                }
            })
            .collect();
        let new_entry = TypeEntry {
            name: enum_name,
            description: Description::Enum(enum_variants),
        };
        output_prep.push(new_entry);
    }
    for caps in REG_TYPES.captures_iter(type_info) {
        let new_entry = TypeEntry {
            name: (&caps["name"]).to_string(),
            description: Description::Type((&caps["description"]).to_string()),
        };
        output_prep.push(new_entry);
    }
    output_prep
}

pub struct CutCompact<T: HasCompact> {
    pub compact_found: T,
    pub start_next_unit: Option<usize>,
}

/// function to search Vec<u8> for shortest compact <T> starting from certain position;
/// could be used also in decoding vectors (since vectors are encoded by lumping
/// all vector elements one after another and adding the compact encoded
/// length at the very start)

pub fn get_compact<T> (data: &Vec<u8>) -> Result<CutCompact<T>, &'static str> 
    where 
        T: HasCompact,
        Compact<T>: Decode
{
    if data.len()==0 {return Err("Empty data vector on input.");}
    let mut out = None;
    for i in 1..data.len()+1 {
        let hippo = &data[..i];
//        println!("current hippo: {:?}", hippo);
        let unhippo = <Compact<T>>::decode(&mut &hippo[..]);
        if let Ok(hurray) = unhippo {
            let mut start_next_unit = None;
            if data.len()>i {start_next_unit = Some(i);}
            out = Some(CutCompact{
                compact_found: hurray.0,
                start_next_unit,
            });
            break;
        }
    }
    match out {
        Some(c) => Ok(c),
        None => return Err("No compact found"),
    }
}

/// struct to store current method and data

pub struct NextDecode {
    pub method: Method,
    pub data: Vec<u8>,
}

/// function to find method for current call

pub fn what_next (data: Vec<u8>, meta: &RuntimeMetadataV12) -> Result<NextDecode, &'static str> {
//    let mut finished_segment = false;
    if data.len() < 2 {return Err("Data vector too short");}
    let pallet_index = data[0];
    let method_index = data[1];
    let new_method = find_method(pallet_index, method_index, meta)?;
//    println!("Found method {} in pallet {}", new_method.method_name, new_method.pallet_name);
    Ok(NextDecode{
        method: new_method,
        data: data[2..].to_vec(),
    })
}

/// struct to store the decoded data as string and remaining data as Vec<u8>

pub struct DecodedOut {
    pub decoded_string: String,
    pub remaining_vector: Vec<u8>,
    pub index: u32,
    pub indent: u32,
    pub fancy_out: String,
}

/// function to write pretty formatted fancy out string
fn fancy (index: u32, indent: u32, card_type: &str, decoded_string: &str) -> String {
    format!("{{\"index\":{},\"indent\":{},\"type\":\"{}\",\"payload\":{}}}", index, indent, card_type, decoded_string)
}

/// function to decode types with known length, outputs DecodedOut

pub fn decode_known_length<T: Decode + serde::ser::Serialize>(data: &Vec<u8>, mut index: u32, indent: u32) -> Result<DecodedOut, &'static str> {
    let length = size_of::<T>();
    if data.len() < length {return Err("Data shorter than expected length.")}
    let decoded_data = <T>::decode(&mut &data[..length]);
    match decoded_data {
        Ok(x) => {
            let decoded_string = format!("\"{}\"", serde_json::to_string(&x).expect("Type should have been checked."));
            let fancy_out = format!(",{}", fancy(index, indent, "default", &decoded_string));
            index = index + 1;
            let remaining_vector = {
                if data.len()>length {(data[length..]).to_vec()}
                else {Vec::new()}
            };
            Ok(DecodedOut {
                decoded_string,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        Err(_) => return Err("Failed to decode."),
    }
}

/// function to decode a compact: search data from beginning for compact, outputs DecodedOut

pub fn decode_as_compact<T> (data: &Vec<u8>, mut index: u32, indent: u32) -> Result<DecodedOut, &'static str> 
    where 
        T: HasCompact + serde::ser::Serialize,
        Compact<T>: Decode
{
    if data.len()==0 {return Err("Data is empty.");}
    let compact_found = get_compact::<T>(data)?;
    let decoded_string = format!("\"{}\"", serde_json::to_string(&compact_found.compact_found).expect("Type should have been checked."));
    let fancy_out = format!(",{}", fancy(index, indent, "default", &decoded_string));
    index = index + 1;
    let remaining_vector = match compact_found.start_next_unit {
        Some(x) => (data[x..]).to_vec(),
        None => Vec::new(),
    };
    Ok(DecodedOut{
        decoded_string,
        remaining_vector,
        index,
        indent,
        fancy_out,
    })
}

/// function to parse the type in database of simplest types
/// and decode accordingly

pub fn decode_primitive (found_ty: &str, data: &Vec<u8>, index: u32, indent: u32) -> Result<DecodedOut, &'static str> {
    match found_ty {
        "bool" => decode_known_length::<bool>(data, index, indent),
        "u8" => decode_known_length::<u8>(data, index, indent),
        "u16" => decode_known_length::<u16>(data, index, indent),
        "u32" => decode_known_length::<u32>(data, index, indent),
        "u64" => decode_known_length::<u64>(data, index, indent),
        "u128" => decode_known_length::<u128>(data, index, indent),
        "Percent" => decode_known_length::<Percent>(data, index, indent),
        "Perbill" => decode_known_length::<Perbill>(data, index, indent),
        "PerU16" => decode_known_length::<PerU16>(data, index, indent),
        "Compact<u8>" => decode_as_compact::<u8>(data, index, indent),
        "Compact<u16>" => decode_as_compact::<u16>(data, index, indent),
        "Compact<u32>" => decode_as_compact::<u32>(data, index, indent),
        "Compact<u64>" => decode_as_compact::<u64>(data, index, indent),
        "Compact<u128>" => decode_as_compact::<u128>(data, index, indent),
        "Compact<Percent>" => decode_as_compact::<Percent>(data, index, indent),
        "Compact<Perbill>" => decode_as_compact::<Perbill>(data, index, indent),
        "Compact<PerU16>" => decode_as_compact::<PerU16>(data, index, indent),
        _ => return Err("Not a primitive type"),
    }
}

/// function to decode any type

pub fn decode_complex (found_ty: &str, mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {

    match found_ty {
        "Box<<T as Config<I>>::Proposal>" | "Box<<T as Config>::Call>" | "Box<<T as Config>::Proposal>" => {
            process_as_call(data, meta, type_database, index, indent, chain_specs)
        },
        "Vec<<T as Config>::Call>" => {
            let pre_vector = get_compact::<u32>(&data)?;
            let number_of_calls = pre_vector.compact_found;
            let mut output_prep = String::from("[");
            let mut fancy_output_prep = String::new();
            match pre_vector.start_next_unit {
                Some(start) => {
                    if data.len() < start + 2*(number_of_calls as usize) {return Err("Expected vector of calls. Following data is shorter than expected.")}
                    data = data[start..].to_vec();
                    for i in 1..number_of_calls+1 {
                        let after_run = process_as_call(data, meta, type_database, index, indent, chain_specs)?;
                        index = after_run.index;
                        if i>1 {output_prep.push(',')}
                        output_prep.push_str(&after_run.decoded_string);
                        fancy_output_prep.push_str(&after_run.fancy_out);
                        data = after_run.remaining_vector;
                    }
                },
                None => {
                    if number_of_calls != 0 {return Err("Expected vector of calls. Found no data after vector declaration.");}
                },
            }
            output_prep.push(']');
            Ok(DecodedOut{
                decoded_string: output_prep,
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        _ => decode_simple (found_ty, data, type_database, index, indent, chain_specs),
    }
}

/// function to process single call

pub fn process_as_call (mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, mut indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    let call_in_processing = what_next (data, meta)?;
    data = call_in_processing.data;
    
    let mut fancy_out = format!(",{}", fancy(index, indent, "call", &format!("{{\"method\":\"{}\",\"pallet\":\"{}\"}}", call_in_processing.method.method_name, call_in_processing.method.pallet_name)));
    index = index + 1;
    indent = indent + 1;
    
    let mut results = String::from("{");
    
    for (i, x) in call_in_processing.method.arguments.iter().enumerate() {
        let add_to_fancy_out = format!(",{}", fancy(index, indent, "varname", &format!("\"{}\"", x.name)));
        fancy_out.push_str(&add_to_fancy_out);
        index = index + 1;
        
        let decoded_out = decode_complex(&x.ty, data, meta, type_database, index, indent+1, chain_specs)?;
        index = decoded_out.index;
        data = decoded_out.remaining_vector;
        let new = format!("\"{}\":{}", x.name, decoded_out.decoded_string);
        if i>0 {results.push(',');}
        results.push_str(&new);
        fancy_out.push_str(&decoded_out.fancy_out);
    }
    
    results.push_str("}");
    
    let out = format!("\"method\":{{\"pallet\":\"{}\",\"method\":\"{}\"}},\"args\":{}", call_in_processing.method.pallet_name, call_in_processing.method.method_name, results);
    
    Ok(DecodedOut{
    decoded_string: out,
    remaining_vector: data.to_vec(),
    index,
    indent,
    fancy_out,
    })
}

// Making regex for type parsing

lazy_static! {
    static ref REGOPTION: Regex = Regex::new(r#"(?m)^Option<(?P<arg>.*)>"#).unwrap();
    static ref REGVECTOR: Regex = Regex::new(r#"(?m)^Vec<(?P<arg>.*)>"#).unwrap();
    static ref REGTUPLE: Regex = Regex::new(r#"(?m)^\((?P<arg1>([^( ]*(\([^)]*\)[^) ]*))|[^(),]*), (?P<arg2>([^( ]*\([^)]*\)[^) ]*)|[^(),]*)(, (?P<arg3>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?(, (?P<arg4>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?\)"#).unwrap();
    static ref REGARRAY: Regex = Regex::new(r#"(?i)(?m)^\[(?P<arg>[a-z0-9]+); (?P<num>[0-9]+)\]"#).unwrap();
    static ref REGCOMPACT: Regex = Regex::new(r#"(?m)^Compact<(?P<arg>.*)>"#).unwrap();
}

/// function to decode Option<_>

pub fn deal_with_option (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    match &data[0] {
        0 => {
            let remaining_vector = {
                if data.len()>1 {(&data[1..]).to_vec()}
                else {Vec::new()}
            };
            let out = serde_json::Value::Null;
            let decoded_string = serde_json::to_string(&out).unwrap();
            let fancy_out = format!(",{}", fancy(index, indent, "None", "\"\""));
            index = index + 1;
            Ok(DecodedOut {
                decoded_string,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        1 => {
            if data.len()==1 {return Err("Decoding as Option: data too short.")}
            data = data[1..].to_vec();
            decode_simple(inner_ty, data, type_database, index, indent, chain_specs)
        },
        _ => {return Err("Decoding as Option: unexpected first character of data.")},
    }
}

/// function to decode Vector<_>

pub fn deal_with_vector (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    let pre_vector = get_compact::<u32>(&data)?;
    let mut output_prep = String::from("[");
    let mut fancy_output_prep = String::new();
    let elements_of_vector = pre_vector.compact_found;
    match pre_vector.start_next_unit {
        Some(start) => {
            data = data[start..].to_vec();
            for i in 1..elements_of_vector+1 {
                let after_run = decode_simple(inner_ty, data, type_database, index, indent, chain_specs)?;
                index = after_run.index;
                fancy_output_prep.push_str(&after_run.fancy_out);
                output_prep.push_str(&after_run.decoded_string);
                if i<elements_of_vector {output_prep.push(',')}
                else {output_prep.push(']')}
                data = after_run.remaining_vector;
            }
            Ok(DecodedOut {
                decoded_string: output_prep,
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        None => {
            if elements_of_vector != 0 {return Err("Decoding vector: found no data after vector length declaration.");}
            else {
                Ok(DecodedOut {
                    decoded_string: String::from("[]"),
                    remaining_vector: Vec::new(),
                    index,
                    indent,
                    fancy_out: String::new(),
                })
            }
        },
    }
}

/// function to decode an array

pub fn deal_with_array (inner_ty: &str, number_of_elements: u32, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    let mut output_prep = String::from("[");
    let mut fancy_output_prep = String::new();
    for i in 1..number_of_elements+1 {
        let after_run = decode_simple(inner_ty, data, type_database, index, indent, chain_specs)?;
        index = after_run.index;
        fancy_output_prep.push_str(&after_run.fancy_out);
        output_prep.push_str(&after_run.decoded_string);
        if i<number_of_elements {output_prep.push(',')}
        else {output_prep.push(']')}
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut{
        decoded_string: output_prep,
        remaining_vector: data,
        index,
        indent,
        fancy_out: fancy_output_prep,
    })
}

/// function to decode of IdentityFields special case

pub fn special_case_identity_fields (data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32) -> Result<DecodedOut, &'static str> {
    // at the moment, the length is known: 8 units in Vec<u8>
    if data.len() < 8 {return Err("Decoding as IdentityFields: data too short");}
    let remaining_vector = {
        if data.len() > 8 {data[8..].to_vec()}
        else {Vec::new()}
    };
    let into_bv = data[..8].to_vec();
    // make correct Bitvec
    let bv: BitVec<Lsb0, u8> = BitVec::from_vec(into_bv);
    let mut found = false;
    let mut output_prep = String::from("(");
    let mut fancy_out = String::new();
    for x in type_database.iter() {
        if x.name == "IdentityField" {
            found = true;
            match &x.description {
                Description::Enum(v1) => {
                    for (i, x) in v1.iter().enumerate() {
                        if bv[i] {
                            if output_prep.len()!=1 {output_prep.push(',')}
                            let new = format!("IdentityField::{}", x.variant_name);
                            output_prep.push_str(&new);
                            let fancy_output_prep = format!(",{}", fancy(index, indent, "IdentityField", &format!("\"{}\"", x.variant_name)));
                            fancy_out.push_str(&fancy_output_prep);
                            index = index + 1;
                        };
                    }
                    output_prep.push(')');
                },
                _ => return Err("Decoding as IdentityFields: IdentityField no longer enum.")
            }
            break;
        }
    }
    if !found {return Err("Decoding as IdentityFields: enum IdentityField not found.");}
    Ok(DecodedOut{
        decoded_string: output_prep,
        remaining_vector,
        index,
        indent,
        fancy_out,
    })
}

/// function to decode BitVec special case

pub fn special_case_bitvec (data: Vec<u8>, mut index: u32, indent: u32) -> Result<DecodedOut, &'static str> {
    // the data is preluded by compact indicating the number of BitVec elements - info from js documentation, decode not implemented for BitVec as is
    let pre_bitvec = get_compact::<u32>(&data)?;
    let actual_length = match pre_bitvec.compact_found % 8 {
        0 => (pre_bitvec.compact_found / 8),
        _ => (pre_bitvec.compact_found / 8) +1,
    };
    match pre_bitvec.start_next_unit {
        Some(start) => {
            let fin = start + (actual_length as usize);
            if data.len() < fin {return Err("Decoding as BitVec: data too short");}
            let into_bv = data[start..fin].to_vec();
            let bv: BitVec<Lsb0, u8> = BitVec::from_vec(into_bv);
            let decoded_string = format!("\"{}\"", bv);
            let fancy_out = format!(",{}", fancy(index, indent, "BitVec", &decoded_string));
            index = index + 1;
            let remaining_vector = {
                if data.len() > fin {data[fin..].to_vec()}
                else {Vec::new()}
            };
            Ok(DecodedOut {
                decoded_string,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        None => {
            if actual_length != 0 {return Err("Decoding as BitVec: no actual data after length declaration.");}
            Ok(DecodedOut {
                decoded_string: String::from("[]"),
                remaining_vector: Vec::new(),
                index,
                indent,
                fancy_out: String::new(),
            })
        }
    }
}

/// function to decode of AccounId special case (with transformation into base58 format)

pub fn special_case_account_id (data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    if data.len() < 32 {return Err("Data shorter than expected length.")}
    let decoded_data = <[u8; 32]>::decode(&mut &data[..32]);
    match decoded_data {
        Ok(x) => {
            let decoded_string = format!("\"{}\"", arr_to_base(x, chain_specs.base58prefix));
            let remaining_vector = {
                if data.len()>32 {(&data[32..]).to_vec()}
                else {Vec::new()}
            };
            let fancy_out = format!(",{}", fancy(index, indent, "Id", &decoded_string));
            index = index + 1;
            Ok(DecodedOut {
                decoded_string,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        Err(_) => return Err("Decoding as base58 address failed."),
    }
}

/// function to determine if special case balance is applicable
fn goto_balance(found_ty: &str) -> bool {
    (found_ty == "Balance")||(found_ty == "T::Balance")||(found_ty == "BalanceOf<T>")||(found_ty == "BalanceOf<T, I>")||(found_ty == "Compact<Balance>")||(found_ty == "Compact<T::Balance>")||(found_ty == "Compact<BalanceOf<T>>")||(found_ty == "Compact<BalanceOf<T, I>>")
}

/// function to decode and represent properly any special case related to balances
/// (both compacts and non-compacts)

pub fn special_case_balance (found_ty: &str, data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    
    match found_ty {
        "Balance" | "T::Balance" | "BalanceOf<T>" | "BalanceOf<T, I>" => {
            let length = size_of::<u128>();
            if data.len() < length {return Err("Data shorter than expected length.")}
            let decoded_data = <u128>::decode(&mut &data[..length]);
            match decoded_data {
                Ok(x) => {
                    let balance_output = convert_balance_pretty (x, chain_specs.decimals, &chain_specs.unit)?;
                    let decoded_string = format!("\"{}\",\"units\":\"{}\"", balance_output.number, balance_output.units);
                    let fancy_out = format!(",{}", fancy(index, indent, "balance", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", balance_output.number, balance_output.units)));
                    index = index + 1;
                    let remaining_vector = {
                        if data.len()>length {(data[length..]).to_vec()}
                        else {Vec::new()}
                    };
                    Ok(DecodedOut {
                        decoded_string,
                        remaining_vector,
                        index,
                        indent,
                        fancy_out,
                    })
                },
                Err(_) => return Err("Failed to decode balance."),
            }
        },
        "Compact<Balance>" | "Compact<T::Balance>" | "Compact<BalanceOf<T>>" | "Compact<BalanceOf<T, I>>" => {
            if data.len()==0 {return Err("Data is empty.");}
            let compact_found = get_compact::<u128>(&data)?;
            let balance_output = convert_balance_pretty (compact_found.compact_found, chain_specs.decimals, &chain_specs.unit)?;
            let decoded_string = format!("\"{}\",\"units\":\"{}\"", balance_output.number, balance_output.units);
            let fancy_out = format!(",{}", fancy(index, indent, "balance", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", balance_output.number, balance_output.units)));
            index = index + 1;
            let remaining_vector = match compact_found.start_next_unit {
                Some(x) => (data[x..]).to_vec(),
                None => Vec::new(),
            };
            Ok(DecodedOut{
                decoded_string,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        _ => return Err("Balance type not described.")
    }
}

/// function to decode structs

pub fn deal_with_struct (v1: &Vec<StructField>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    let mut fancy_out = String::new();
    let mut output_prep = String::from("{{");
    for (i, y) in v1.iter().enumerate() {
        let fancy_output_prep = match &y.field_name {
            Some(z) => format!(",{}", fancy(index, indent, "field_name", &format!("\"{}\"", z))),
            None => format!(",{}", fancy(index, indent, "field_number", &format!("\"{}\"", i))),
        };
        fancy_out.push_str(&fancy_output_prep);
        index = index + 1;
        let after_run = decode_simple(&y.field_type, data, type_database, index, indent+1, chain_specs)?;
        data = after_run.remaining_vector;
        index = after_run.index;
        fancy_out.push_str(&after_run.fancy_out);
        match &y.field_name {
            Some(z) => {
                let line = format!("\"{}\":\"{}\"", z, after_run.decoded_string);
                output_prep.push_str(&line);
            },
            None => {
                output_prep.push_str("\"");
                output_prep.push_str(&after_run.decoded_string);
                output_prep.push_str("\"");
            },
        }
        if i < v1.len() {output_prep.push(',')}
        else {output_prep.push_str("}}")}
    }
    Ok(DecodedOut {
        decoded_string: output_prep,
        remaining_vector: data,
        index,
        indent,
        fancy_out,
    })
}

/// function to decode enums

pub fn deal_with_enum (v1: &Vec<EnumVariant>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
    let enum_index = data[0] as usize;
    if enum_index >= v1.len() {return Err("While decoding Enum, encountered unexpected variant.")}
    let found_variant = &v1[enum_index];
    match &found_variant.variant_type {
        EnumVariantType::None => {
            let remaining_vector = {
                if data.len()>1 {(&data[1..]).to_vec()}
                else {Vec::new()}
            };
            let out = format!("\"{}\"", found_variant.variant_name);
            let fancy_out = format!(",{}", fancy(index, indent, "enum_variant_name", &format!("\"{}\"", found_variant.variant_name)));
            index = index + 1;
            Ok(DecodedOut {
                decoded_string: out,
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        EnumVariantType::Type(inner_ty) => {
            if data.len()==1 {return Err("While decoding Enum, expected declared variant to be followed by some associated data, that data was not found.")}
            data=data[1..].to_vec();
            let mut fancy_output_prep = format!(",{}", fancy(index, indent, "enum_variant_name",  &format!("\"{}\"", found_variant.variant_name)));
            index = index + 1;
            let after_run = decode_simple(&inner_ty, data, type_database, index, indent+1, chain_specs)?;
            index = after_run.index;
            fancy_output_prep.push_str(&after_run.fancy_out);
            let output_prep = format!("{{\"{}\":\"{}\"}}", found_variant.variant_name, after_run.decoded_string);
            data = after_run.remaining_vector;
            Ok(DecodedOut {
                decoded_string: output_prep,
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        EnumVariantType::Struct(v2) => {
            if data.len()==1 {return Err("While decoding Enum, expected declared variant to be followed by some associated data, that data was not found.")}
            data=data[1..].to_vec();
            let mut fancy_out = String::new();
            let mut output_prep = format!("{{\"{}\":{{", found_variant.variant_name);
            for (i, y) in v2.iter().enumerate() {
                let fancy_output_prep = match &y.field_name {
                    Some(z) => format!(",{}", fancy(index, indent, "field_name", &format!("\"{}\"", z))),
                    None => format!(",{}", fancy(index, indent, "field_number", &format!("\"{}\"", i))),
                };
                fancy_out.push_str(&fancy_output_prep);
                index = index + 1;
                let after_run = decode_simple(&y.field_type, data, type_database, index, indent+1, chain_specs)?;
                data = after_run.remaining_vector;
                index = after_run.index;
                fancy_out.push_str(&after_run.fancy_out);
                match &y.field_name {
                    Some(z) => {
                        let line = format!("\"{}\":\"{}\"", z, after_run.decoded_string);
                        output_prep.push_str(&line);
                    },
                    None => {
                        output_prep.push_str("\"");
                        output_prep.push_str(&after_run.decoded_string);
                        output_prep.push_str("\"");
                    },
                }
                if i < v2.len() {output_prep.push(',')}
                else {output_prep.push_str("}}}}")}
            }
            Ok(DecodedOut {
                decoded_string: output_prep,
                remaining_vector: data,
                index,
                indent,
                fancy_out,
            })
        },
    }
}


/// function to decode anything except calls

pub fn decode_simple (found_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecEntry) -> Result<DecodedOut, &'static str> {
//    println!("Dealing with type: {}", found_ty);
    if data.len()==0 {return Err("Data is empty.");}
    match decode_primitive(&found_ty, &data, index, indent) {
        Ok(a) => Ok(a),
        Err(_) => {
            // check for option
            match REGOPTION.captures(&found_ty) {
                Some(caps) => {
                    let inner_ty = caps.name("arg").unwrap().as_str();
                    deal_with_option(inner_ty, data, type_database, index, indent, chain_specs)
                },
                None => {
                    // check for vector
                    match REGVECTOR.captures(&found_ty) {
                        Some(caps) => {
                            let inner_ty = caps.name("arg").unwrap().as_str();
                            deal_with_vector(inner_ty, data, type_database, index, indent, chain_specs)
                        },
                        None => {
                            // check for tuples
                            match REGTUPLE.captures(&found_ty) {
                                Some(caps) => {
                                    let mut output_prep = String::from("(");
                                    let mut fancy_out = String::new();
                                    let mut i=1;
                                    loop {
                                        let capture_name = format!("arg{}", i);
                                        match caps.name(&capture_name) {
                                            Some(x) => {
                                                let fancy_output_prep = format!(",{}", fancy(index, indent, "field_number", &format!("\"{}\"", i)));
                                                fancy_out.push_str(&fancy_output_prep);
                                                index = index + 1;
                                                let inner_ty = x.as_str();
                                                let after_run = decode_simple(inner_ty, data, type_database, index, indent+1, chain_specs)?;
                                                index = after_run.index;
                                                fancy_out.push_str(&after_run.fancy_out);
                                                if i>1 {output_prep.push(',');}
                                                output_prep.push_str("\"");
                                                output_prep.push_str(&after_run.decoded_string);
                                                output_prep.push_str("\"");
                                                data = after_run.remaining_vector;
                                            }
                                            None => {
                                                output_prep.push(')');
                                                break;
                                            }
                                        }
                                        i = i+1;
                                    }
                                    Ok(DecodedOut{
                                        decoded_string: output_prep,
                                        remaining_vector: data,
                                        index,
                                        indent,
                                        fancy_out,
                                    })
                                },
                                None => {
                                    // check for array
                                    match REGARRAY.captures(&found_ty) {
                                        Some(caps) => {
                                            let inner_ty = &caps["arg"];
                                            let number_of_elements: u32 = caps["num"].parse().expect("Should have captured a number.");
                                            deal_with_array(inner_ty, number_of_elements, data, type_database, index, indent, chain_specs)
                                        },
                                        None => {
                                            // check for special case of Balance as is or in Compact form
                                            if goto_balance(found_ty) {
                                                special_case_balance(found_ty, data, index, indent, chain_specs)
                                            }
                                            else {
                                            // check for compact and find the alias for compact
                                                match REGCOMPACT.captures(&found_ty) {
                                                    Some(caps) => {
                                                        let inner_ty = &caps["arg"];
                                                        let mut new_inner_ty = None;
                                                        for x in type_database.iter() {
                                                            if x.name == inner_ty {
                                                                new_inner_ty = match &x.description {
                                                                    Description::Type(a) => Some(a),
                                                                    _ => return Err("Decoding compact: unexpected compact insides."),
                                                                };
                                                                break;
                                                            }
                                                        }
                                                        match new_inner_ty {
                                                            Some(a) => {
                                                                let new_ty = found_ty.replace(inner_ty, a);
                                                                decode_simple(&new_ty, data, type_database, index, indent, chain_specs)
                                                            },
                                                            None => return Err("Decoding compact: type in compact not transforming into primitive.")
                                                        }
                                                    },
                                                    None => {
                                                        // special case of IdentityFields type (from pallet_identity): uses BitFlags, private fields
                                                        if found_ty == "IdentityFields" {
                                                            special_case_identity_fields (data, type_database, index, indent)
                                                        }
                                                        else {
                                                            // special case of BitVec type
                                                            if found_ty == "BitVec" {special_case_bitvec(data, index, indent)}
                                                            else {
                                                                // special case of AccountId type
                                                                if (found_ty == "AccountId")||(found_ty == "T::AccountId") {special_case_account_id(data, index, indent, chain_specs)}
                                                                else {
                                                                    // do a search through the type database
                                                                    let mut found_solution = None;
                                                                    for x in type_database.iter() {
                                                                        if x.name == found_ty {
                                                                            let wrap_me = match &x.description {
                                                                                Description::Type(inner_ty) => {decode_simple(&inner_ty, data, type_database, index, indent, chain_specs)?},
                                                                                Description::Enum(v1) => {deal_with_enum(v1, data, type_database, index, indent, chain_specs)?},
                                                                                Description::Struct(v1) => {deal_with_struct(v1, data, type_database, index, indent, chain_specs)?},
                                                                            };
                                                                            found_solution = Some(wrap_me);
                                                                            break;
                                                                        }
                                                                    }
                                                                    match found_solution {
                                                                        Some(x) => Ok(x),
                                                                        None => return Err("Could not interpret the type."),
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                }
                                            }
                                        },
                                    }
                                },
                            }
                        },
                    }
                },
            }
        }
    }
} 

/// function to print extrinsics in fancy format
fn print_fancy_extrinsics (index: u32, indent: u32, tip_output: PrettyOutput, short: ExtrinsicValues, chain_name: &str) -> String {
    match short.era {
        Era::Immortal => format!("{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", short.nonce)), fancy(index+1, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", tip_output.number, tip_output.units)), fancy(index+2, indent, "tx_spec", &format!("{{\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", chain_name, short.metadata_version, short.tx_version))),
        Era::Mortal(period, phase) => format!("{},{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, short.nonce)), fancy(index+1, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", tip_output.number, tip_output.units)), fancy(index+2, indent, "block_hash", &format!("\"{}\"", hex::encode(short.block_hash))), fancy(index+3, indent, "tx_spec", &format!("{{\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", chain_name, short.metadata_version, short.tx_version))),
    }
}

/// struct to store three important file names: chain_spec_database, metadata, and types_description_database

pub struct DataFiles<'a> {
    pub chain_spec_database: &'a str,
    pub metadata_contents: &'a str,
    pub types_info: &'a str,
}

/// struct to separate prelude, address, actual method, and extrinsics in transaction string
#[derive(Debug, Decode)]
pub struct TransactionParts {
    pub prelude: [u8; 3],
    pub author: [u8; 32],
    pub method: Vec<u8>,
    pub extrinsics: ExtrinsicValues,
    pub genesis_hash: [u8; 32],
}

/// struct to decode extrinsics
#[derive(Debug, Decode)]
pub struct ExtrinsicValues {
    pub era: Era,
#[codec(compact)]
    pub nonce: u64,
#[codec(compact)]
    pub tip: u128,
    pub metadata_version: u32,
    pub tx_version: u32,
    pub genesis_hash: [u8; 32],
    pub block_hash: [u8; 32],
}

/// struct to store the output of decoding: "normal" format and fancy easy-into-js format

pub struct DecodingResult {
    pub normal: String,
    pub js: String,
}

/// function to parse full transaction
/// transaction format corresponds to what we get from qr code:
/// i.e. it starts with 53****, followed by address, followed by actual transaction piece,
/// followed by extrinsics, concluded with chain genesis hash

pub fn full_run (transaction: &str, datafiles: DataFiles) -> Result<DecodingResult, &'static str> {
    let data_hex = match transaction.starts_with("0x") {
        true => &transaction[2..],
        false => &transaction,
    };
    
    let data = match hex::decode(data_hex) {
        Ok(a) => a,
        Err(_) => return Err("Wrong format of input transaction string."),
    };
    
    let transaction_decoded = match <TransactionParts>::decode(&mut &data[..]) {
        Ok(a) => a,
        Err(_) => return Err("Error separating prelude, author address, method, and extrinsics"),
    };
    
    let short = transaction_decoded.extrinsics;
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

// try to get chain specs from genesis hash
    if transaction_decoded.genesis_hash != short.genesis_hash {return Err("Two different genesis hashes are found.")}
    if let Era::Immortal = short.era {if short.genesis_hash != short.block_hash {return Err("Block hash found to not be equal to genesis hash in immortal transaction.")}}
    
    let genesis_hash = hex::encode(&transaction_decoded.genesis_hash);
    
    match specs_from_genesis_hash(&datafiles.chain_spec_database, &genesis_hash) {
        Ok(chain_specs) => {
            let chain_name = &chain_specs.name;
            let chain_prefix = chain_specs.base58prefix;

        // transform author
            let author = arr_to_base(transaction_decoded.author, chain_prefix);
            let mut to_normal = format!("\"author\":{{\"base58\":\"{}\"}}", author);
            let mut to_fancy = fancy(index, indent, "author", &format!("{{\"base58\":\"{}\"}}", author));
            index = index + 1;
            
        // update tip output
            let tip_output = convert_balance_pretty (short.tip, chain_specs.decimals, &chain_specs.unit)?;
            
        // transform extrinsics information for normal output
            let extrinsics_to_normal = match short.era {
                Era::Immortal => format!("\"extrinsics\":{{\"era\":\"Immortal\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version),
                Era::Mortal(period, phase) => format!("\"extrinsics\":{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\",\"block_hash\":\"{}\"}}", phase, period, short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version, hex::encode(short.block_hash)),
            };
            
        // fetch chain metadata in RuntimeMetadataV12 format
            match find_meta(&chain_name, short.metadata_version, &datafiles.metadata_contents) {
                Ok((meta, ver)) => {
                    if let Some(x) = ver {
                        let warn_normal = format!(",\"warning\":{{\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"}}", short.metadata_version, x);
                        to_normal.push_str(&warn_normal);
                        let warn_fancy = format!("],\"warning\":[{}", fancy(index, indent, "warning", &format!("\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"", short.metadata_version, x)));
                        index = index + 1;
                        to_fancy.push_str(&warn_fancy);
                    }
                    
                // generate type database to be used in decoding
                    let type_database = generate_type_database (&datafiles.types_info);
                    
                // transaction parsing
                    let transaction_parsed = process_as_call (transaction_decoded.method, &meta, &type_database, index, indent, &chain_specs)?;
                    let index = transaction_parsed.index;
                    
                    if transaction_parsed.remaining_vector.len() != 0 {return Err("After transaction parsing, some data in transaction vector remained unused.")}

                    let normal = format!("{},{},{}", to_normal, transaction_parsed.decoded_string, extrinsics_to_normal);
                //transform extrinsics information for fancy output
                    let extrinsics_to_js = print_fancy_extrinsics (index, indent, tip_output, short, chain_name);
                    let js = format!("{{\"author\":[{}],\"method\":[{}],\"extrinsics\":[{}]}}", to_fancy, &transaction_parsed.fancy_out[1..], extrinsics_to_js);
                    Ok(DecodingResult{
                        normal,
                        js,
                    })
                },
                Err(e) => {
                    let error_normal = format!("\"error\":{{\"{}\"}}", e);
                    let error_fancy = fancy(index, indent, "error", &format!("\"{}\"", e));
                    index = index + 1;
                    let normal = format!("{},{},{}", to_normal, error_normal, extrinsics_to_normal);
                //transform extrinsics information for fancy output
                    let extrinsics_to_js = print_fancy_extrinsics (index, indent, tip_output, short, chain_name);
                    let js = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", to_fancy, error_fancy, extrinsics_to_js);
                    Ok(DecodingResult{
                        normal,
                        js,
                    })
                },
            }
        },
        Err("No matching genesis hash found.") => {
            let error_normal = String::from("\"error\":{{\"Network not found. Please add the network.\"}}");
            let error_fancy = fancy(index, indent, "error", "\"Network not found. Please add the network.\"");
            index = index + 1;
            // extrinsics information
            let extrinsics_to_normal = match short.era {
                Era::Immortal => format!("\"extrinsics\":{{\"era\":\"Immortal\",\"nonce\":\"{}\",\"tip\":\"{}\",\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", short.nonce, short.tip, hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version),
                Era::Mortal(period, phase) => format!("\"extrinsics\":{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\",\"tip\":\"{}\",\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\",\"block_hash\":\"{}\"}}", phase, period, short.nonce, short.tip, hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version, hex::encode(short.block_hash)),
            };
            let extrinsics_to_js = match short.era {
                Era::Immortal => format!("{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", short.nonce)), fancy(index+1, indent, "tip_plain", &format!("\"{}\"", short.tip)), fancy(index+2, indent, "tx_spec_plain", &format!("{{\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version))),
                Era::Mortal(period, phase) => format!("{},{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, short.nonce)), fancy(index+1, indent, "tip_plain", &format!("\"{}\"", short.tip)), fancy(index+2, indent, "block_hash", &format!("\"{}\"", hex::encode(short.block_hash))), fancy(index+3, indent, "tx_spec_plain", &format!("{{\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version))),
            };
            let normal = format!("{},{}", error_normal, extrinsics_to_normal);
            let js = format!("{{\"error\":[{}],\"extrinsics\":[{}]}}", error_fancy, extrinsics_to_js);
            Ok(DecodingResult{
                normal,
                js,
            })
        },
        Err(e) => return Err(e),
    }
}

/// some tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const BASE58PREFIX: u8 = 42;

    #[test]
    fn alice_and_bob_hex() {
        // addresses in base58
        let alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        // 64 symbol pieces extracted from transaction strings in polkadot - difference while changing transfer receiver
        let to_bob = "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";
        let to_alice = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        assert!(base_to_hex(alice) == to_alice, "Base to hex did not work for Alice! {} vs {}", base_to_hex(alice), to_alice);
        assert!(base_to_hex(bob) == to_bob, "Base to hex did not work for Bob! {} vs {}", base_to_hex(bob), to_bob);
        assert!(hex_to_base(to_alice, BASE58PREFIX) == alice, "Hex to base did not work for Alice! {} vs {}", hex_to_base(to_alice, BASE58PREFIX), alice);
        assert!(hex_to_base(to_bob, BASE58PREFIX) == bob, "Hex to base did not work for Bob! {} vs {}", hex_to_base(to_bob, BASE58PREFIX), bob);
    }
    
    #[test]
    fn bob_array() {
        let bob_array = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
        let bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        assert!(arr_to_base(bob_array, BASE58PREFIX) == bob, "Bob array into base58 not converting right.");
        assert!(base_to_arr(bob) == bob_array, "Bob base58 into array not converting right.");
    }
    
    #[test]
    fn read_hash_book() {
        let chain_spec_database = fs::read_to_string("database_output").unwrap();
        let hash_book = get_genesis_hash(&chain_spec_database);
        assert!(hash_book.len()==9, "Used to be 9. Found: {}", hash_book.len());
    }
}

