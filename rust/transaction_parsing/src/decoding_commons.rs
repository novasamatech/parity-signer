use parity_scale_codec::{Decode, HasCompact, Compact};
use sp_arithmetic::{PerThing};
use std::mem::size_of;
use definitions::network_specs::ChainSpecs;
use printing_balance::convert_balance_pretty;
use sp_core::crypto::{Ss58Codec, Ss58AddressFormat, AccountId32};

use crate::cards::Card;
use crate::error::{Error, SystemError, UnableToDecode};

/// Struct to store the decoded data, used for data storage between decoding iterations.
/// decoded_string is short json-like format,
/// fancy_out is format used for js output cards (the one really going out at this point)
/// and remaining vector contains the input data not yet used after the last decoding iteration.
pub struct DecodedOut {
    pub remaining_vector: Vec<u8>,
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


/// Function to decode types with trait PerThing (Percent, Permill, Perbill etc).
/// Decoding type T either as compact or as fixed length type.
/// Used only in decoding_older module, without serde.
///
/// The function decodes only this element, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - compact flag to initiate compact decoding,
/// - &str name of type to be displayed in case of error,
/// - index and indent that are used for creating properly formatted js cards,
///
/// The function outputs the DecodedOut value in case of success.
pub fn decode_perthing<T> (data: &Vec<u8>, compact_flag: bool, found_ty: &str, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> 
    where 
        T: PerThing + Decode + HasCompact,
        Compact<T>: Decode
{
    let (fancy_out, remaining_vector) = {
        if compact_flag {
            let compact_found = get_compact::<T>(data)?;
            let fancy_out = format!(",{}", (Card::Default(&compact_found.compact_found.deconstruct().into().to_string())).card(index, indent));
            let remaining_vector = match compact_found.start_next_unit {
                Some(x) => (data[x..]).to_vec(),
                None => Vec::new(),
            };
            (fancy_out, remaining_vector)
        }
        else {
            let length = size_of::<T>();
            if data.len() < length {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            let decoded_data = <T>::decode(&mut &data[..length]);
            match decoded_data {
                Ok(x) => {
                    let fancy_out = format!(",{}", (Card::Default(&x.deconstruct().into().to_string())).card(index, indent));
                    let remaining_vector = data[length..].to_vec();
                    (fancy_out, remaining_vector)
                },
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(found_ty.to_string()))),
            }
        }
    };
    Ok(DecodedOut{
        remaining_vector,
        fancy_out,
    })
}

/// Function to decode a displayable type of known length (i.e. length stable with respect to mem::size_of).
/// Used in both decoding_older and decoding_sci, for types not compatible with compact or balance printing
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
pub fn decode_known_length<T: Decode + std::fmt::Display>(data: &Vec<u8>, found_ty: &str, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    let length = size_of::<T>();
    if data.len() < length {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
    let decoded_data = <T>::decode(&mut &data[..length]);
    match decoded_data {
        Ok(x) => {
            let fancy_out = format!(",{}", (Card::Default(&x.to_string())).card(index, indent));
            let remaining_vector = data[length..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        },
        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(found_ty.to_string()))),
    }
}


/// Function to decode a displayable type compatible with compact and balance printing.
/// Used in both decoding_older and decoding_sci.
/// Decoding type T either as compact or as fixed length type, possibly as a balance.
///
/// The function decodes only this element, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - compact flag and balance flag to choose decoding variant,
/// - &str name of type to be displayed in case of error,
/// - index and indent that are used for creating properly formatted js cards,
/// - ChainSpecs to format the balance properly if the balance is involved.
///
/// The function outputs the DecodedOut value in case of success.
pub fn decode_primitive_with_flags <T> (data: &Vec<u8>, compact_flag: bool, balance_flag: bool, found_ty: &str, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> 
    where 
        T: Decode + HasCompact + std::fmt::Display,
        Compact<T>: Decode
{
    if compact_flag {
        let compact_found = get_compact::<T>(data)?;
        let fancy_out = {
            if balance_flag {
                let balance_output = match convert_balance_pretty (&compact_found.compact_found.to_string(), chain_specs.decimals, &chain_specs.unit) {
                    Ok(x) => x,
                    Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
                };
                format!(",{}", (Card::Balance{number: &balance_output.number, units: &balance_output.units}).card(index, indent))
            }
            else {format!(",{}", (Card::Default(&compact_found.compact_found.to_string())).card(index, indent))}
        };
        let remaining_vector = match compact_found.start_next_unit {
            Some(x) => (data[x..]).to_vec(),
            None => Vec::new(),
        };
        Ok(DecodedOut{
            remaining_vector,
            fancy_out,
        })
    }
    else {
        let length = size_of::<T>();
        if data.len() < length {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
        let decoded_data = <T>::decode(&mut &data[..length]);
        match decoded_data {
            Ok(x) => {
                let fancy_out = {
                    if balance_flag {
                        let balance_output = match convert_balance_pretty (&x.to_string(), chain_specs.decimals, &chain_specs.unit) {
                            Ok(x) => x,
                            Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
                        };
                        format!(",{}", (Card::Balance{number: &balance_output.number, units: &balance_output.units}).card(index, indent))
                    }
                    else {format!(",{}", (Card::Default(&x.to_string())).card(index, indent))}
                };
                let remaining_vector = data[length..].to_vec();
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            },
            Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(found_ty.to_string()))),
        }
    }
}



/// Function to decode of AccountId special case and transform the result into base58 format.
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
pub fn special_case_account_id (data: Vec<u8>, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    match data.get(0..32) {
        Some(a) => {
            match <[u8; 32]>::decode(&mut &a[..]) {
                Ok(x) => {
                    let remaining_vector = data[32..].to_vec();
                    let account_id = AccountId32::new(x);
                    let base58print = account_id.to_ss58check_with_version(Ss58AddressFormat::Custom(chain_specs.base58prefix));
                    let fancy_out = format!(",{}", (Card::Id(&base58print)).card(index, indent));
                    Ok(DecodedOut {
                        remaining_vector,
                        fancy_out,
                    })
                },
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::Array)),
            }
        },
        None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
    }
}


