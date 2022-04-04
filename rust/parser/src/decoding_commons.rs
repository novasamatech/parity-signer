use definitions::{
    error::{ParserDecodingError, ParserError, ParserMetadataError},
    network_specs::ShortSpecs,
};
use parity_scale_codec::{Compact, Decode, HasCompact};
use printing_balance::convert_balance_pretty;
use sp_arithmetic::PerThing;
use sp_core::crypto::AccountId32;
use std::mem::size_of;

use crate::cards::ParserCard;
use crate::decoding_sci_ext::{Ext, SpecialExt};

/// Struct to store the decoded data, used for data storage between decoding iterations.
/// decoded_string is short json-like format,
/// fancy_out is format used for js output cards (the one really going out at this point)
/// and remaining vector contains the input data not yet used after the last decoding iteration.
pub(crate) struct DecodedOut {
    pub(crate) remaining_vector: Vec<u8>,
    pub(crate) fancy_out: Vec<OutputCard>,
}

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

/// Function to search Vec<u8> for shortest compact <T> by brute force.
/// Outputs CutCompact value in case of success.
pub fn get_compact<T>(data: &Vec<u8>) -> Result<CutCompact<T>, ParserError>
where
    T: HasCompact,
    Compact<T>: Decode,
{
    if data.len() == 0 {
        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
    }
    let mut out = None;
    for i in 1..data.len() + 1 {
        let hippo = &data[..i];
        let unhippo = <Compact<T>>::decode(&mut &hippo[..]);
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
        None => return Err(ParserError::Decoding(ParserDecodingError::NoCompact)),
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
    data: &Vec<u8>,
    compact_flag: bool,
    found_ty: &str,
    indent: u32,
) -> Result<DecodedOut, ParserError>
where
    T: PerThing + Decode + HasCompact,
    Compact<T>: Decode,
{
    let (fancy_out, remaining_vector) = {
        if compact_flag {
            let compact_found = get_compact::<T>(data)?;
            let fancy_out = vec![OutputCard {
                card: ParserCard::Default(
                    compact_found.compact_found.deconstruct().into().to_string(),
                ),
                indent,
            }];
            let remaining_vector = match compact_found.start_next_unit {
                Some(x) => (data[x..]).to_vec(),
                None => Vec::new(),
            };
            (fancy_out, remaining_vector)
        } else {
            let length = size_of::<T>();
            if data.len() < length {
                return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
            }
            let decoded_data = <T>::decode(&mut &data[..length]);
            match decoded_data {
                Ok(x) => {
                    let fancy_out = vec![OutputCard {
                        card: ParserCard::Default(x.deconstruct().into().to_string()),
                        indent,
                    }];
                    let remaining_vector = data[length..].to_vec();
                    (fancy_out, remaining_vector)
                }
                Err(_) => {
                    return Err(ParserError::Decoding(
                        ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
                    ))
                }
            }
        }
    };
    Ok(DecodedOut {
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
/// - indent used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.
pub(crate) fn decode_known_length<T: Decode + std::fmt::Display>(
    data: &Vec<u8>,
    found_ty: &str,
    indent: u32,
) -> Result<DecodedOut, ParserError> {
    let length = size_of::<T>();
    if data.len() < length {
        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
    }
    let decoded_data = <T>::decode(&mut &data[..length]);
    match decoded_data {
        Ok(x) => {
            let fancy_out = vec![OutputCard {
                card: ParserCard::Default(x.to_string()),
                indent,
            }];
            let remaining_vector = data[length..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        }
        Err(_) => {
            return Err(ParserError::Decoding(
                ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
            ))
        }
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
    data: &Vec<u8>,
    possible_ext: &mut Option<&mut Ext>,
    compact_flag: bool,
    balance_flag: bool,
    found_ty: &str,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError>
where
    T: Decode + HasCompact + std::fmt::Display,
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
        let fancy_out = {
            if balance_flag {
                process_balance(
                    &compact_found.compact_found.to_string(),
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
        let remaining_vector = match compact_found.start_next_unit {
            Some(x) => (data[x..]).to_vec(),
            None => Vec::new(),
        };
        Ok(DecodedOut {
            remaining_vector,
            fancy_out,
        })
    } else {
        let length = size_of::<T>();
        if data.len() < length {
            return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
        }
        let decoded_data = <T>::decode(&mut &data[..length]);
        match decoded_data {
            Ok(x) => {
                let fancy_out = {
                    if balance_flag {
                        process_balance(&x.to_string(), possible_ext, indent, short_specs)?
                    } else {
                        process_number(x.to_string(), possible_ext, indent, short_specs)?
                    }
                };
                let remaining_vector = data[length..].to_vec();
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            }
            Err(_) => {
                return Err(ParserError::Decoding(
                    ParserDecodingError::PrimitiveFailure(found_ty.to_string()),
                ))
            }
        }
    }
}

fn process_balance(
    balance: &str,
    possible_ext: &mut Option<&mut Ext>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError> {
    let balance_output = convert_balance_pretty(balance, short_specs.decimals, &short_specs.unit);
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
            units: balance_output.units.to_string(),
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
    data: Vec<u8>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    match data.get(0..32) {
        Some(a) => match <[u8; 32]>::decode(&mut &a[..]) {
            Ok(x) => {
                let remaining_vector = data[32..].to_vec();
                let account_id = AccountId32::new(x);
                let fancy_out = vec![OutputCard {
                    card: ParserCard::Id {
                        id: account_id,
                        base58prefix: short_specs.base58prefix,
                    },
                    indent,
                }];
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            }
            Err(_) => return Err(ParserError::Decoding(ParserDecodingError::Array)),
        },
        None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    }
}
