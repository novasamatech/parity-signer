use regex::Regex;
use lazy_static::lazy_static;
use parity_scale_codec::{Decode, HasCompact, Compact};
use serde;
use serde_json;
use std::mem::size_of;
use sp_arithmetic::{Percent, Perbill, PerU16};
use frame_metadata::{RuntimeMetadataV12};
use bitvec::prelude::{BitVec, Lsb0};
use printing_balance::convert_balance_pretty;
use db_handling::{chainspecs::ChainSpecs, settings::{TypeEntry, Description, EnumVariant, EnumVariantType, StructField}};

use super::method::what_next;
use super::utils_base58::vec_to_base;

/// struct to store the decoded data as string and as fancy easy-to-import-to-js string,
/// remaining data as Vec<u8>, and index and indent used for fancy string printing

pub struct DecodedOut {
    pub decoded_string: String,
    pub remaining_vector: Vec<u8>,
    pub index: u32,
    pub indent: u32,
    pub fancy_out: String,
}


/// function to write pretty formatted fancy output string

pub fn fancy (index: u32, indent: u32, card_type: &str, decoded_string: &str) -> String {
    format!("{{\"index\":{},\"indent\":{},\"type\":\"{}\",\"payload\":{}}}", index, indent, card_type, decoded_string)
}


/// struct to store results of searching Vec<u8> for encoded compact:
/// consists of actual number decoded, and, if it exists, the position of first unit after the compact

pub struct CutCompact<T: HasCompact> {
    pub compact_found: T,
    pub start_next_unit: Option<usize>,
}


/// function to search Vec<u8> for shortest compact <T> by brute force

pub fn get_compact<T> (data: &Vec<u8>) -> Result<CutCompact<T>, &'static str> 
    where 
        T: HasCompact,
        Compact<T>: Decode
{
    if data.len()==0 {return Err("Empty data vector on input.");}
    let mut out = None;
    for i in 1..data.len()+1 {
        let hippo = &data[..i];
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


/// function to decode a compact, outputs DecodedOut

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


/// function to decode primitive types (fixed-width or compact form), and Percent,
/// Permill, and PerU16 structs (also fixed-width or compact form) that also have
/// stable length by std::mem::size_of() and also are serializeable

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



/// function to decode any type, including calls and vectors of calls

pub fn decode_complex (found_ty: &str, mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {

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

/// function to process single call;
/// the place to start when decoding the transaction

pub fn process_as_call (mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, mut indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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

pub fn deal_with_option (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
    if inner_ty == "bool" {
        let (decoded_string, fancy_out) = match &data[0] {
            0 => (serde_json::to_string(&(serde_json::Value::Null)).unwrap(), format!(",{}", fancy(index, indent, "none", "\"\""))),
            1 => (serde_json::to_string(&true).unwrap(), format!(",{}", fancy(index, indent, "default", "\"True\""))),
            2 => (serde_json::to_string(&false).unwrap(), format!(",{}", fancy(index, indent, "default", "\"False\""))),
            _ => {return Err("Decoding as Option: unexpected first character of data.")},
        };
        index = index + 1;
        let remaining_vector = {
            if data.len()>1 {(&data[1..]).to_vec()}
            else {Vec::new()}
        };
        Ok(DecodedOut {
            decoded_string,
            remaining_vector,
            index,
            indent,
            fancy_out,
        })
    }
    else {
        match &data[0] {
            0 => {
                let remaining_vector = {
                    if data.len()>1 {(&data[1..]).to_vec()}
                    else {Vec::new()}
                };
                let out = serde_json::Value::Null;
                let decoded_string = serde_json::to_string(&out).unwrap();
                let fancy_out = format!(",{}", fancy(index, indent, "none", "\"\""));
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
}

/// function to decode Vector<_>

pub fn deal_with_vector (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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

pub fn deal_with_array (inner_ty: &str, number_of_elements: u32, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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
                            let fancy_output_prep = format!(",{}", fancy(index, indent, "identity_field", &format!("\"{}\"", x.variant_name)));
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
            let fancy_out = format!(",{}", fancy(index, indent, "bitvec", &decoded_string));
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

pub fn special_case_account_id (data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
    if data.len() < 32 {return Err("Data shorter than expected length.")}
    let decoded_data = <[u8; 32]>::decode(&mut &data[..32]);
    match decoded_data {
        Ok(x) => {
            let decoded_string = format!("\"{}\"", vec_to_base(&(x.to_vec()), chain_specs.base58prefix));
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

pub fn special_case_balance (found_ty: &str, data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
    
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

pub fn deal_with_struct (v1: &Vec<StructField>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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

pub fn deal_with_enum (v1: &Vec<EnumVariant>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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

pub fn decode_simple (found_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, &'static str> {
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

