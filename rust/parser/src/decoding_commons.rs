//! Decoder elements common for all metadata versions
//!
use parity_scale_codec::{Compact, Decode, HasCompact};
use sp_arithmetic::PerThing;
use sp_core::crypto::AccountId32;
use std::{convert::TryInto, mem::size_of};

use definitions::{
    error_signer::{ParserDecodingError, ParserError, ParserMetadataError},
    network_specs::ShortSpecs,
};
use printing_balance::AsBalance;

use crate::cards::ParserCard;
use crate::decoding_sci_ext::{Ext, SpecialExt};

#[derive(Clone)]
pub struct OutputCard {
    pub card: ParserCard,
    pub indent: u32,
}

/// Struct to store results of searching Vec<u8> for encoded compact:
/// consists of actual number decoded, and, if it exists, the beginning position for data after the compact
pub struct CutCompact<T: HasCompact> {
    pub compact_found: T,
    pub start_next_unit: Option<usize>,
}

/// Function to search &[u8] for shortest compact <T> by brute force.
/// Outputs CutCompact value in case of success.
pub fn get_compact<T>(data: &[u8]) -> Result<CutCompact<T>, ParserError>
where
    T: HasCompact,
    Compact<T>: Decode,
{
    if data.is_empty() {
        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
    }
    let mut out = None;
    for i in 1..data.len() + 1 {
        let mut hippo = &data[..i];
        let unhippo = <Compact<T>>::decode(&mut hippo);
        if let Ok(hurray) = unhippo {
            let mut start_next_unit = None;
            if data.len() > i {
                start_next_unit = Some(i);
            }
            out = Some(CutCompact {
                compact_found: hurray.0,
                start_next_unit,
            });
            break;
        }
    }
    match out {
        Some(c) => Ok(c),
        None => Err(ParserError::Decoding(ParserDecodingError::NoCompact)),
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
/// - indent used for creating properly formatted js cards,
///
/// The function outputs the DecodedOut value in case of success.
pub(crate) fn decode_perthing<T>(
    data: &mut Vec<u8>,
    compact_flag: bool,
    found_ty: &str,
    indent: u32,
) -> Result<Vec<OutputCard>, ParserError>
where
    T: PerThing + Decode + HasCompact,
    Compact<T>: Decode,
{
    if compact_flag {
        let compact_found = get_compact::<T>(data)?;
        let out = vec![OutputCard {
            card: ParserCard::Default(compact_found.compact_found.deconstruct().into().to_string()),
            indent,
        }];
        *data = match compact_found.start_next_unit {
            Some(x) => (data[x..]).to_vec(),
            None => Vec::new(),
        };
        Ok(out)
    } else {
        let length = size_of::<T>();
        if data.len() < length {
            return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
        }
        let decoded_data = <T>::decode(&mut &data[..length]);
        match decoded_data {
            Ok(x) => {
                let out = vec![OutputCard {
                    card: ParserCard::Default(x.deconstruct().into().to_string()),
                    indent,
                }];
                *data = data[length..].to_vec();
                Ok(out)
            }
            Err(_) => Err(ParserError::Decoding(
                ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
            )),
        }
    }
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
/// - indent used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.
pub(crate) fn decode_known_length<T: Decode + std::fmt::Display>(
    data: &mut Vec<u8>,
    found_ty: &str,
    indent: u32,
) -> Result<Vec<OutputCard>, ParserError> {
    let length = size_of::<T>();
    if data.len() < length {
        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
    }
    let decoded_data = <T>::decode(&mut &data[..length]);
    match decoded_data {
        Ok(x) => {
            let out = vec![OutputCard {
                card: ParserCard::Default(x.to_string()),
                indent,
            }];
            *data = data[length..].to_vec();
            Ok(out)
        }
        Err(_) => Err(ParserError::Decoding(
            ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
        )),
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
/// - indent used for creating properly formatted js cards,
/// - ShortSpecs to format the balance properly if the balance is involved.
///
/// The function outputs the DecodedOut value in case of success.
pub(crate) fn decode_primitive_with_flags<T>(
    data: &mut Vec<u8>,
    possible_ext: &mut Option<&mut Ext>,
    compact_flag: bool,
    balance_flag: bool,
    found_ty: &str,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError>
where
    T: Decode + AsBalance + HasCompact + std::fmt::Display,
    Compact<T>: Decode,
{
    let balance_flag = {
        if let Some(ext) = possible_ext {
            if let SpecialExt::Tip = ext.specialty {
                true
            } else {
                balance_flag
            }
        } else {
            balance_flag
        }
    };
    if compact_flag {
        let compact_found = get_compact::<T>(data)?;
        let out = {
            if balance_flag {
                process_balance::<T>(
                    compact_found.compact_found,
                    possible_ext,
                    indent,
                    short_specs,
                )?
            } else {
                process_number(
                    compact_found.compact_found.to_string(),
                    possible_ext,
                    indent,
                    short_specs,
                )?
            }
        };
        *data = match compact_found.start_next_unit {
            Some(x) => (data[x..]).to_vec(),
            None => Vec::new(),
        };
        Ok(out)
    } else {
        let length = size_of::<T>();
        if data.len() < length {
            return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
        }
        let decoded_data = <T>::decode(&mut &data[..length]);
        match decoded_data {
            Ok(x) => {
                let out = {
                    if balance_flag {
                        process_balance::<T>(x, possible_ext, indent, short_specs)?
                    } else {
                        process_number(x.to_string(), possible_ext, indent, short_specs)?
                    }
                };
                *data = data[length..].to_vec();
                Ok(out)
            }
            Err(_) => Err(ParserError::Decoding(
                ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
            )),
        }
    }
}

fn process_balance<T: AsBalance>(
    balance: T,
    possible_ext: &mut Option<&mut Ext>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError> {
    let balance_output =
        <T>::convert_balance_pretty(balance, short_specs.decimals, &short_specs.unit);
    let out_balance = vec![OutputCard {
        card: ParserCard::Balance {
            number: balance_output.number.to_string(),
            units: balance_output.units.to_string(),
        },
        indent,
    }];
    let out_tip = vec![OutputCard {
        card: ParserCard::Tip {
            number: balance_output.number.to_string(),
            units: balance_output.units,
        },
        indent,
    }];
    if let Some(ext) = possible_ext {
        if let SpecialExt::Tip = ext.specialty {
            Ok(out_tip)
        } else {
            Ok(out_balance)
        }
    } else {
        Ok(out_balance)
    }
}

fn process_number(
    number: String,
    possible_ext: &mut Option<&mut Ext>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError> {
    if let Some(ext) = possible_ext {
        match ext.specialty {
            SpecialExt::Nonce => Ok(vec![OutputCard {
                card: ParserCard::Nonce(number),
                indent,
            }]),
            SpecialExt::SpecVersion => {
                ext.found_ext.network_version_printed = match ext.found_ext.network_version_printed
                {
                    Some(_) => {
                        return Err(ParserError::FundamentallyBadV14Metadata(
                            ParserMetadataError::SpecVersionTwice,
                        ))
                    }
                    None => Some(number.to_string()),
                };
                Ok(vec![OutputCard {
                    card: ParserCard::NetworkNameVersion {
                        name: short_specs.name.to_string(),
                        version: number,
                    },
                    indent,
                }])
            }
            SpecialExt::TxVersion => Ok(vec![OutputCard {
                card: ParserCard::TxVersion(number),
                indent,
            }]),
            _ => Ok(vec![OutputCard {
                card: ParserCard::Default(number),
                indent,
            }]),
        }
    } else {
        Ok(vec![OutputCard {
            card: ParserCard::Default(number),
            indent,
        }])
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
/// - indent used for creating properly formatted js cards.
/// - short_specs (taking base58 prefix from there).
///
/// The function outputs the DecodedOut value in case of success.
///
/// Resulting AccountId in base58 form is added to fancy_out on js card "Id".
pub(crate) fn special_case_account_id(
    data: &mut Vec<u8>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError> {
    match data.get(0..32) {
        Some(a) => {
            let array_decoded: [u8; 32] = a.try_into().expect("constant length, always fits");
            *data = data[32..].to_vec();
            let account_id = AccountId32::new(array_decoded);
            let out = vec![OutputCard {
                card: ParserCard::Id {
                    id: account_id,
                    base58prefix: short_specs.base58prefix,
                },
                indent,
            }];
            Ok(out)
        }
        None => Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    }
}
