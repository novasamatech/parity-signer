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
use definitions::{network_specs::ChainSpecs, types::{TypeEntry, Description, EnumVariant, EnumVariantType, StructField}};

use super::method::what_next;
use super::utils_base58::vec_to_base;
use super::cards::Card;
use super::error::{Error, UnableToDecode, SystemError};


/// Struct to store the decoded data, used for data storage between decoding iterations.
/// decoded_string is short json-like format,
/// fancy_out is format used for js output cards (the one really going out at this point)
/// index and indent are parameters for js output cards, already modified for the next decoding iteration, 
/// and remaining vector contains the input data not yet used after the last decoding iteration.

pub struct DecodedOut {
    pub remaining_vector: Vec<u8>,
    pub index: u32,
    pub indent: u32,
    pub fancy_out: String,
}


/// Struct to store results of searching Vec<u8> for encoded compact:
/// consists of actual number decoded, and, if it exists, the beginning position for data after the compact

pub struct CutCompact<T: HasCompact> {
    pub compact_found: T,
    pub start_next_unit: Option<usize>,
}


/// Function to search Vec<u8> for shortest compact <T> by brute force.
/// Outputs CutCompact value in case of success.

pub fn get_compact<T> (data: &Vec<u8>) -> Result<CutCompact<T>, Error> 
    where 
        T: HasCompact,
        Compact<T>: Decode
{
    if data.len()==0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
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
        None => return Err(Error::UnableToDecode(UnableToDecode::NoCompact)),
    }
}


/// Function to decode a serializeable type of known length (i.e. length stable with respect to mem::size_of).
///
/// The function decodes only this type, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - found_ty: name of the type found,
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.

pub fn decode_known_length<T: Decode + serde::ser::Serialize>(data: &Vec<u8>, found_ty: &str, mut index: u32, indent: u32) -> Result<DecodedOut, Error> {
    let length = size_of::<T>();
    if data.len() < length {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
    let decoded_data = <T>::decode(&mut &data[..length]);
    match decoded_data {
        Ok(x) => {
            let fancy_out = format!(",{}", (Card::Default(&serde_json::to_string(&x).expect("Type should have been checked."))).card(index, indent));
            index = index + 1;
            let remaining_vector = {
                if data.len()>length {(data[length..]).to_vec()}
                else {Vec::new()}
            };
            Ok(DecodedOut {
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(found_ty.to_string()))),
    }
}


/// Function to decode type T (compactable and serializeable) as compact.
///
/// The function decodes only this compact, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.

pub fn decode_as_compact<T> (data: &Vec<u8>, mut index: u32, indent: u32) -> Result<DecodedOut, Error> 
    where 
        T: HasCompact + serde::ser::Serialize,
        Compact<T>: Decode
{
    let compact_found = get_compact::<T>(data)?;
    let fancy_out = format!(",{}", (Card::Default(&serde_json::to_string(&compact_found.compact_found).expect("Type should have been checked."))).card(index, indent));
    index = index + 1;
    let remaining_vector = match compact_found.start_next_unit {
        Some(x) => (data[x..]).to_vec(),
        None => Vec::new(),
    };
    Ok(DecodedOut{
        remaining_vector,
        index,
        indent,
        fancy_out,
    })
}


/// Function to decode primitive types (fixed-width or compact form), and Percent,
/// Permill, and PerU16 structs (also fixed-width or compact form).
/// All those types have stable length by std::mem::size_of() and also are serializeable.
///
/// The function decodes only found_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (type of the argument found in the previous iteration, to be interpreted on run)
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.

pub fn decode_primitive (found_ty: &str, data: &Vec<u8>, index: u32, indent: u32) -> Result<DecodedOut, Error> {
    match found_ty {
        "bool" => decode_known_length::<bool>(data, found_ty, index, indent),
        "u8" => decode_known_length::<u8>(data, found_ty, index, indent),
        "u16" => decode_known_length::<u16>(data, found_ty, index, indent),
        "u32" => decode_known_length::<u32>(data, found_ty, index, indent),
        "u64" => decode_known_length::<u64>(data, found_ty, index, indent),
        "u128" => decode_known_length::<u128>(data, found_ty, index, indent),
        "Percent" => decode_known_length::<Percent>(data, found_ty, index, indent),
        "Perbill" => decode_known_length::<Perbill>(data, found_ty, index, indent),
        "PerU16" => decode_known_length::<PerU16>(data, found_ty, index, indent),
        "Compact<u8>" => decode_as_compact::<u8>(data, index, indent),
        "Compact<u16>" => decode_as_compact::<u16>(data, index, indent),
        "Compact<u32>" => decode_as_compact::<u32>(data, index, indent),
        "Compact<u64>" => decode_as_compact::<u64>(data, index, indent),
        "Compact<u128>" => decode_as_compact::<u128>(data, index, indent),
        "Compact<Percent>" => decode_as_compact::<Percent>(data, index, indent),
        "Compact<Perbill>" => decode_as_compact::<Perbill>(data, index, indent),
        "Compact<PerU16>" => decode_as_compact::<PerU16>(data, index, indent),
        _ => return Err(Error::UnableToDecode(UnableToDecode::NotPrimitive(found_ty.to_string()))),
    }
}


/// Function to decode any type, including calls and vectors of calls.
/// Here starts the decoding of argument with type found_ty.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only found_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (type of the argument found in the previous iteration, to be interpreted on run)
/// - data (remaining Vec<u8> of data),
/// - meta (metadata for the network used),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.
///
/// Calls and vectors of calls are treated separately here.
/// All simpler types are processed through decode_simple function.

pub fn decode_complex (found_ty: &str, mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {

    match found_ty {
        "Box<<T as Config<I>>::Proposal>" | "Box<<T as Config>::Call>" | "Box<<T as Config>::Proposal>" => {
            process_as_call(data, meta, type_database, index, indent, chain_specs)
        },
        "Vec<<T as Config>::Call>" => {
            let pre_vector = get_compact::<u32>(&data)?;
            let number_of_calls = pre_vector.compact_found;
            let mut fancy_output_prep = String::new();
            match pre_vector.start_next_unit {
                Some(start) => {
                    if data.len() < start + 2*(number_of_calls as usize) {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
                    data = data[start..].to_vec();
                    for _i in 0..number_of_calls {
                        let after_run = process_as_call(data, meta, type_database, index, indent, chain_specs)?;
                        index = after_run.index;
                        fancy_output_prep.push_str(&after_run.fancy_out);
                        data = after_run.remaining_vector;
                    }
                },
                None => {
                    if number_of_calls != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
                },
            }
            Ok(DecodedOut{
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        _ => decode_simple (found_ty, data, type_database, index, indent, chain_specs),
    }
}


/// Function to process a single call (essentially, Vec<u8>).
/// This is the place to start when decoding the transaction.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single call, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (call itself),
/// - meta (metadata for the network used),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.
///
/// In each encoded call the first two elements denote the numbers of the pallet and the method
/// in corresponding network metadata, indicating what arguments the call has.
/// The card "call" containing pallet name and method name is added to fancy_out.
/// Each argument is then processed in a sequence, the name of the argument
/// and the type of the argument are found in the network metadata during the run.
/// For each argument the card "varname" with argument name is added to fancy_out,
/// followed by card(s) of actual decoded agrument values.

pub fn process_as_call (mut data: Vec<u8>, meta: &RuntimeMetadataV12, type_database: &Vec<TypeEntry>, mut index: u32, mut indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let call_in_processing = what_next (data, meta)?;
    data = call_in_processing.data;
    
    let mut fancy_out = format!(",{}", (Card::Call{method: &call_in_processing.method.method_name, pallet: &call_in_processing.method.pallet_name}).card(index, indent));
    index = index + 1;
    indent = indent + 1;
    
    for x in call_in_processing.method.arguments.iter() {
        let add_to_fancy_out = format!(",{}", (Card::Varname(&x.name)).card(index, indent));
        fancy_out.push_str(&add_to_fancy_out);
        index = index + 1;
        
        let decoded_out = decode_complex(&x.ty, data, meta, type_database, index, indent+1, chain_specs)?;
        index = decoded_out.index;
        data = decoded_out.remaining_vector;
        fancy_out.push_str(&decoded_out.fancy_out);
    }
    
    Ok(DecodedOut{
        remaining_vector: data.to_vec(),
        index,
        indent,
        fancy_out,
    })
}


// Making regular expressions for analysing any type encountered except calls and Vec<calls>.
// The type to be parsed is found in network metadata on the run.
// In some cases the type could be Option<arg>, Vec<arg>, tuple such as (arg1, arg2, arg3)
// (currently among types are found tuples of up to 4 elements, could change at any point),
// array such as [arg; num], and compact such as Compact<arg>.
// To reduce the number of types in type_database and to simplify and
// at least a bit generalize the decoding process,
// the options, vectors, tuples, arrays, and compacts are currently treated specially
// and first are "husked" to interpret the inner type.
// This regular expressions are used for just that: find the inner type.
// All those are constructed statically, and pre-tested.

lazy_static! {
    static ref REGOPTION: Regex = Regex::new(r#"(?m)^Option<(?P<arg>.*)>"#).expect("constructed from checked static value");
    static ref REGVECTOR: Regex = Regex::new(r#"(?m)^Vec<(?P<arg>.*)>"#).expect("constructed from checked static value");
    static ref REGTUPLE: Regex = Regex::new(r#"(?m)^\((?P<arg1>([^( ]*(\([^)]*\)[^) ]*))|[^(),]*), (?P<arg2>([^( ]*\([^)]*\)[^) ]*)|[^(),]*)(, (?P<arg3>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?(, (?P<arg4>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?\)"#).expect("constructed from checked static value");
    static ref REGARRAY: Regex = Regex::new(r#"(?i)(?m)^\[(?P<arg>[a-z0-9]+); (?P<num>[0-9]+)\]"#).expect("constructed from checked static value");
    static ref REGCOMPACT: Regex = Regex::new(r#"(?m)^Compact<(?P<arg>.*)>"#).expect("constructed from checked static value");
}


/// Function to decode Option<inner_ty>.
/// Special case of Option<bool> is considered separately.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single option, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - inner_ty (type inside Option, found using regular expressions)
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.
///
/// Js cards are of type "none" if the Option<_> is None.
/// At this moment no special js card for Some(x) is presented, only the card of x itself.

pub fn deal_with_option (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    if inner_ty == "bool" {
    
        let fancy_out = match &data[0] {
            0 => format!(",{}", (Card::None).card(index, indent)),
            1 => format!(",{}", (Card::Default("True")).card(index, indent)),
            2 => format!(",{}", (Card::Default("False")).card(index, indent)),
            _ => {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedOptionVariant))},
        };
        index = index + 1;
        let remaining_vector = {
            if data.len()>1 {(&data[1..]).to_vec()}
            else {Vec::new()}
        };
        Ok(DecodedOut {
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
                let fancy_out = format!(",{}", (Card::None).card(index, indent));
                index = index + 1;
                Ok(DecodedOut {
                    remaining_vector,
                    index,
                    indent,
                    fancy_out,
                })
            },
            1 => {
                if data.len()==1 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
                data = data[1..].to_vec();
                decode_simple(inner_ty, data, type_database, index, indent, chain_specs)
            },
            _ => {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedOptionVariant))},
        }
    }
}

/// Function to decode Vector<inner_ty>.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single vector of type inner_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// Encoded vectors are preluded by the number of vector elements as compact,
/// followed by concatenated individually encoded elements.
/// So, the function first searches for compact to determine the number of elements
/// and then proceeds to decode each element of the vector.
///
/// The function takes as arguments
/// - inner_ty (type inside Vec, found using regular expressions)
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.

pub fn deal_with_vector (inner_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let pre_vector = get_compact::<u32>(&data)?;
    let mut fancy_output_prep = String::new();
    let elements_of_vector = pre_vector.compact_found;
    match pre_vector.start_next_unit {
        Some(start) => {
            data = data[start..].to_vec();
            for _i in 0..elements_of_vector {
                let after_run = decode_simple(inner_ty, data, type_database, index, indent, chain_specs)?;
                index = after_run.index;
                fancy_output_prep.push_str(&after_run.fancy_out);
                data = after_run.remaining_vector;
            }
            Ok(DecodedOut {
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        None => {
            if elements_of_vector != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            else {
                Ok(DecodedOut {
                    remaining_vector: Vec::new(),
                    index,
                    indent,
                    fancy_out: String::new(),
                })
            }
        },
    }
}


/// Function to decode an array such as [inner_ty; number_of_elements].
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single array of type inner_ty and length number_of_elements,
/// removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - inner_ty (type of array elements, found using regular expressions)
/// - number_of_elements (number of array elements, found using regular expressions)
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.

pub fn deal_with_array (inner_ty: &str, number_of_elements: u32, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let mut fancy_output_prep = String::new();
    for _i in 0..number_of_elements {
        let after_run = decode_simple(inner_ty, data, type_database, index, indent, chain_specs)?;
        index = after_run.index;
        fancy_output_prep.push_str(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut{
        remaining_vector: data,
        index,
        indent,
        fancy_out: fancy_output_prep,
    })
}

/// Function to decode IdentityFields special case.
/// IdentityFields is a struct from pallet_identity::IdentityFields,
/// which is wrapper type for BitFlags<IdentityField>.
/// To avoid output complications arising from private non-printable fields
/// the type was re-implemented here using BitVec and enum IdentityField from type_database.
///
/// The function decodes only a single IdentityFields type entry,
/// removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.
///
/// For each identity field an individual js card "identity_field" is added to fancy_out.

pub fn special_case_identity_fields (data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32) -> Result<DecodedOut, Error> {
    // at the moment, the length is known: 8 units in Vec<u8>
    if data.len() < 8 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
    let remaining_vector = {
        if data.len() > 8 {data[8..].to_vec()}
        else {Vec::new()}
    };
    let into_bv = data[..8].to_vec();
    // make correct Bitvec
    let bv: BitVec<Lsb0, u8> = BitVec::from_vec(into_bv);
    let mut found = false;
    let mut fancy_out = String::new();
    for x in type_database.iter() {
        if x.name == "IdentityField" {
            if let Description::Enum(v1) = &x.description {
                found = true;
                for (i, x) in v1.iter().enumerate() {
                    if bv[i] {
                        let fancy_output_prep = format!(",{}", (Card::IdentityField(&x.variant_name)).card(index, indent));
                        fancy_out.push_str(&fancy_output_prep);
                        index = index + 1;
                    };
                }
            }
            break;
        }
    }
    if !found {return Err(Error::UnableToDecode(UnableToDecode::IdFields))}
    Ok(DecodedOut{
        remaining_vector,
        index,
        indent,
        fancy_out,
    })
}

/// Function to decode BitVec special case.
///
/// The function decodes only a single BitVec type entry,
/// removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// Trait Decode is not implemented for BitVec type.
/// Existing signer documentation in js suggests that the encoded BitVec is preluded by the number
/// of BitVec elements as compact, and each 8 of those form an u8 element in input data.
/// So, the function first searches for compact to determine the number of BitVec elements
/// then calculates how many actual u8 elements are used to store those, takes needed length of data
/// and gets BitVec from it.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.
///
/// Resulting BitVec is added to fancy_out on js card "bitvec".

pub fn special_case_bitvec (data: Vec<u8>, mut index: u32, indent: u32) -> Result<DecodedOut, Error> {
    // the data is preluded by compact indicating the number of BitVec elements - info from js documentation, decode not implemented for BitVec as is
    let pre_bitvec = get_compact::<u32>(&data)?;
    let actual_length = match pre_bitvec.compact_found % 8 {
        0 => (pre_bitvec.compact_found / 8),
        _ => (pre_bitvec.compact_found / 8) +1,
    };
    match pre_bitvec.start_next_unit {
        Some(start) => {
            let fin = start + (actual_length as usize);
            if data.len() < fin {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            let into_bv = data[start..fin].to_vec();
            let bv: BitVec<Lsb0, u8> = BitVec::from_vec(into_bv);
            let fancy_out = format!(",{}", (Card::BitVec(bv)).card(index, indent));
            index = index + 1;
            let remaining_vector = {
                if data.len() > fin {data[fin..].to_vec()}
                else {Vec::new()}
            };
            Ok(DecodedOut {
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        None => {
            if actual_length != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            Ok(DecodedOut {
                remaining_vector: Vec::new(),
                index,
                indent,
                fancy_out: String::new(),
            })
        }
    }
}

/// Function to decode of AccountId special case and transform the result into base58 format.
/// TODO At this moment AccountId is assumed to be [u8; 32], based on js signer files.
/// It is possible that just like the public key it could also be [u8; 33], depending on encryption used.
/// This should be clarified and corrected if needed.
///
/// The function decodes only a single AccountId type entry,
/// removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
/// - chain_specs (taking base58 prefix from there).
///
/// The function outputs the DecodedOut value in case of success.
///
/// Resulting AccountId in base58 form is added to fancy_out on js card "Id".

pub fn special_case_account_id (data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    if data.len() < 32 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
    let decoded_data = <[u8; 32]>::decode(&mut &data[..32]);
    match decoded_data {
        Ok(x) => {
            let remaining_vector = {
                if data.len()>32 {(&data[32..]).to_vec()}
                else {Vec::new()}
            };
            let fancy_out = format!(",{}", (Card::Id(&vec_to_base(&(x.to_vec()), chain_specs.base58prefix))).card(index, indent));
            index = index + 1;
            Ok(DecodedOut {
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::Array)),
    }
}


/// Function to determine if special case balance is applicable
fn goto_balance(found_ty: &str) -> bool {
    (found_ty == "Balance")||(found_ty == "T::Balance")||(found_ty == "BalanceOf<T>")||(found_ty == "BalanceOf<T, I>")||(found_ty == "Compact<Balance>")||(found_ty == "Compact<T::Balance>")||(found_ty == "Compact<BalanceOf<T>>")||(found_ty == "Compact<BalanceOf<T, I>>")
}


/// Function to decode and represent properly any special case related to balances
/// (both compacts and non-compacts).
/// 
/// The function decodes only a single balance-related type entry,
/// removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
/// - chain_specs (taking currency units and decimals from there).
///
/// The function outputs the DecodedOut value in case of success.
///
/// Resulting balance is added to fancy_out on js card "balance".

pub fn special_case_balance (found_ty: &str, data: Vec<u8>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    
    match found_ty {
        "Balance" | "T::Balance" | "BalanceOf<T>" | "BalanceOf<T, I>" => {
            let length = size_of::<u128>();
            if data.len() < length {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            let decoded_data = <u128>::decode(&mut &data[..length]);
            match decoded_data {
                Ok(x) => {
                    let balance_output = match convert_balance_pretty (x, chain_specs.decimals, &chain_specs.unit) {
                        Ok(x) => x,
                        Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
                    };
                    let fancy_out = format!(",{}", (Card::Balance{number: &balance_output.number, units: &balance_output.units}).card(index, indent));
                    index = index + 1;
                    let remaining_vector = {
                        if data.len()>length {(data[length..]).to_vec()}
                        else {Vec::new()}
                    };
                    Ok(DecodedOut {
                        remaining_vector,
                        index,
                        indent,
                        fancy_out,
                    })
                },
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(String::from("<u128>")))),
            }
        },
        "Compact<Balance>" | "Compact<T::Balance>" | "Compact<BalanceOf<T>>" | "Compact<BalanceOf<T, I>>" => {
            let compact_found = get_compact::<u128>(&data)?;
            let balance_output = match convert_balance_pretty (compact_found.compact_found, chain_specs.decimals, &chain_specs.unit) {
                Ok(x) => x,
                Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
            };
            let fancy_out = format!(",{}", (Card::Balance{number: &balance_output.number, units: &balance_output.units}).card(index, indent));
            index = index + 1;
            let remaining_vector = match compact_found.start_next_unit {
                Some(x) => (data[x..]).to_vec(),
                None => Vec::new(),
            };
            Ok(DecodedOut{
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        _ => return Err(Error::UnableToDecode(UnableToDecode::BalanceNotDescribed(found_ty.to_string())))
    }
}


/// Function to decode structs described in type_database.
/// 
/// Function iterates over struct fields, for each field it creates js card "field_name"
/// or "field_number" (if there are no field names),
/// and card(s) for corresponding content in fancy_out.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
///
/// The function takes as arguments
/// - vector of StructField of currently processed type, as found in type_database
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.

pub fn deal_with_struct (v1: &Vec<StructField>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let mut fancy_out = String::new();
    for (i, y) in v1.iter().enumerate() {
        let fancy_output_prep = match &y.field_name {
            Some(z) => format!(",{}", (Card::FieldName(&z)).card(index, indent)),
            None => format!(",{}", (Card::FieldNumber(i)).card(index, indent)),
        };
        fancy_out.push_str(&fancy_output_prep);
        index = index + 1;
        let after_run = decode_simple(&y.field_type, data, type_database, index, indent+1, chain_specs)?;
        data = after_run.remaining_vector;
        index = after_run.index;
        fancy_out.push_str(&after_run.fancy_out);
    }
    Ok(DecodedOut {
        remaining_vector: data,
        index,
        indent,
        fancy_out,
    })
}


/// Function to decode enums described in type_database.
/// 
/// Function determines which enum variant is used, and adds to fancy_out card "enum_variant_name",
/// and, if there is content associated with enum variant, card(s) for this content.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
///
/// The function takes as arguments
/// - vector of EnumVariant of currently processed type, as found in type_database
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.

pub fn deal_with_enum (v1: &Vec<EnumVariant>, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let enum_index = data[0] as usize;
    if enum_index >= v1.len() {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedEnumVariant))}
    let found_variant = &v1[enum_index];
    match &found_variant.variant_type {
        EnumVariantType::None => {
            let remaining_vector = {
                if data.len()>1 {(&data[1..]).to_vec()}
                else {Vec::new()}
            };
            let fancy_out = format!(",{}", (Card::EnumVariantName(&found_variant.variant_name)).card(index, indent));
            index = index + 1;
            Ok(DecodedOut {
                remaining_vector,
                index,
                indent,
                fancy_out,
            })
        },
        EnumVariantType::Type(inner_ty) => {
            if data.len()==1 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            data=data[1..].to_vec();
            let mut fancy_output_prep = format!(",{}", (Card::EnumVariantName(&found_variant.variant_name)).card(index, indent));
            index = index + 1;
            let after_run = decode_simple(&inner_ty, data, type_database, index, indent+1, chain_specs)?;
            index = after_run.index;
            fancy_output_prep.push_str(&after_run.fancy_out);
            data = after_run.remaining_vector;
            Ok(DecodedOut {
                remaining_vector: data,
                index,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        EnumVariantType::Struct(v2) => {
            if data.len()==1 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            data=data[1..].to_vec();
            let mut fancy_out = String::new();
            for (i, y) in v2.iter().enumerate() {
                let fancy_output_prep = match &y.field_name {
                    Some(z) => format!(",{}", (Card::FieldName(&z)).card(index, indent)),
                    None => format!(",{}", (Card::FieldNumber(i)).card(index, indent)),
                };
                fancy_out.push_str(&fancy_output_prep);
                index = index + 1;
                let after_run = decode_simple(&y.field_type, data, type_database, index, indent+1, chain_specs)?;
                data = after_run.remaining_vector;
                index = after_run.index;
                fancy_out.push_str(&after_run.fancy_out);
            }
            Ok(DecodedOut {
                remaining_vector: data,
                index,
                indent,
                fancy_out,
            })
        },
    }
}


/// Function to decode any type, except calls.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only found_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (type of the argument found in the previous iteration, to be interpreted on run)
/// - data (remaining Vec<u8> of data),
/// - type_database (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by decode_primitive function), this database
/// currently is retrieved and decoded from the database on device used),
/// - index and indent that are used for creating properly formatted js cards,
/// - chain_specs (network parameters, such as base58 prefix, currency units and decimals,
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the DecodedOut value in case of success.

pub fn decode_simple (found_ty: &str, mut data: Vec<u8>, type_database: &Vec<TypeEntry>, mut index: u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {

    if data.len()==0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
    match decode_primitive(&found_ty, &data, index, indent) {
        Ok(a) => Ok(a),
        Err(_) => {
            // check for option
            match REGOPTION.captures(&found_ty) {
                Some(caps) => {
                    let inner_ty = match caps.name("arg") {
                        Some(c) => c.as_str(),
                        None => return Err(Error::SystemError(SystemError::RegexError)),
                    };
                    deal_with_option(inner_ty, data, type_database, index, indent, chain_specs)
                },
                None => {
                    // check for vector
                    match REGVECTOR.captures(&found_ty) {
                        Some(caps) => {
                            let inner_ty = match caps.name("arg") {
                                Some(c) => c.as_str(),
                                None => return Err(Error::SystemError(SystemError::RegexError)),
                            };
                            deal_with_vector(inner_ty, data, type_database, index, indent, chain_specs)
                        },
                        None => {
                            // check for tuples
                            match REGTUPLE.captures(&found_ty) {
                                Some(caps) => {
                                    let mut fancy_out = String::new();
                                    let mut i=1;
                                    loop {
                                        let capture_name = format!("arg{}", i);
                                        match caps.name(&capture_name) {
                                            Some(x) => {
                                                let fancy_output_prep = format!(",{}", (Card::FieldNumber(i)).card(index, indent));
                                                fancy_out.push_str(&fancy_output_prep);
                                                index = index + 1;
                                                let inner_ty = x.as_str();
                                                let after_run = decode_simple(inner_ty, data, type_database, index, indent+1, chain_specs)?;
                                                index = after_run.index;
                                                fancy_out.push_str(&after_run.fancy_out);
                                                data = after_run.remaining_vector;
                                            },
                                            None => break,
                                        }
                                        i = i+1;
                                    }
                                    Ok(DecodedOut{
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
                                                                    _ => return Err(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides)),
                                                                };
                                                                break;
                                                            }
                                                        }
                                                        match new_inner_ty {
                                                            Some(a) => {
                                                                let new_ty = found_ty.replace(inner_ty, a);
                                                                decode_simple(&new_ty, data, type_database, index, indent, chain_specs)
                                                            },
                                                            None => return Err(Error::UnableToDecode(UnableToDecode::CompactNotPrimitive)),
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
                                                                        None => return Err(Error::UnableToDecode(UnableToDecode::UnknownType(found_ty.to_string()))),
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

