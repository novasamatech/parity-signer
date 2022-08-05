use std::fmt::Write as _;

use bitvec::{
    order::BitOrder,
    prelude::{BitVec, Lsb0, Msb0},
    store::BitStore,
};
use frame_metadata::v14::RuntimeMetadataV14;
use num_bigint::{BigInt, BigUint};
use parity_scale_codec::Decode;
use scale_info::{
    form::PortableForm, Field, Type, TypeDef, TypeDefBitSequence, TypeDefComposite,
    TypeDefPrimitive, TypeDefVariant,
};

use definitions::{
    error_signer::{ParserDecodingError, ParserError, ParserMetadataError},
    network_specs::ShortSpecs,
};

use crate::cards::ParserCard;
use crate::decoding_commons::{
    decode_known_length, decode_primitive_with_flags, get_compact, special_case_account_id,
    DecodedOut, OutputCard,
};
use crate::decoding_sci_ext::{special_case_era, special_case_hash, Ext, Hash, SpecialExt};

enum FoundBitOrder {
    Lsb0,
    Msb0,
}

#[derive(Clone, Copy)]
pub(crate) enum CallExpectation {
    None,
    Pallet,
    Method,
}

impl CallExpectation {
    fn add(self) -> Self {
        match self {
            CallExpectation::None => CallExpectation::Pallet,
            CallExpectation::Pallet => CallExpectation::Method,
            CallExpectation::Method => CallExpectation::Method,
        }
    }
}

/// Function to decode types that are variants of TypeDefPrimitive enum.
///
/// The function decodes only given type found_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (TypeDefPrimitive, found in the previous iteration)
/// - data (remaining Vec<u8> of data),
/// - indent used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.
fn decode_type_def_primitive(
    found_ty: &TypeDefPrimitive,
    possible_ext: &mut Option<&mut Ext>,
    compact_flag: bool,
    balance_flag: bool,
    data: &[u8],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    match found_ty {
        TypeDefPrimitive::Bool => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<bool>(data, "bool", indent)
        }
        TypeDefPrimitive::Char => {
            reject_flags(compact_flag, balance_flag)?;
            decode_char(data, indent)
        }
        TypeDefPrimitive::Str => {
            reject_flags(compact_flag, balance_flag)?;
            decode_str(data, indent)
        }
        TypeDefPrimitive::U8 => decode_primitive_with_flags::<u8>(
            data,
            possible_ext,
            compact_flag,
            balance_flag,
            "u8",
            indent,
            short_specs,
        ),
        TypeDefPrimitive::U16 => decode_primitive_with_flags::<u16>(
            data,
            possible_ext,
            compact_flag,
            balance_flag,
            "u16",
            indent,
            short_specs,
        ),
        TypeDefPrimitive::U32 => decode_primitive_with_flags::<u32>(
            data,
            possible_ext,
            compact_flag,
            balance_flag,
            "u32",
            indent,
            short_specs,
        ),
        TypeDefPrimitive::U64 => decode_primitive_with_flags::<u64>(
            data,
            possible_ext,
            compact_flag,
            balance_flag,
            "u64",
            indent,
            short_specs,
        ),
        TypeDefPrimitive::U128 => decode_primitive_with_flags::<u128>(
            data,
            possible_ext,
            compact_flag,
            balance_flag,
            "u128",
            indent,
            short_specs,
        ),
        TypeDefPrimitive::U256 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_big256(data, false, indent)
        }
        TypeDefPrimitive::I8 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i8>(data, "i8", indent)
        }
        TypeDefPrimitive::I16 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i16>(data, "i16", indent)
        }
        TypeDefPrimitive::I32 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i32>(data, "i32", indent)
        }
        TypeDefPrimitive::I64 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i64>(data, "i64", indent)
        }
        TypeDefPrimitive::I128 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i128>(data, "i128", indent)
        }
        TypeDefPrimitive::I256 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_big256(data, true, indent)
        }
    }
}

fn reject_flags(compact_flag: bool, balance_flag: bool) -> Result<(), ParserError> {
    if compact_flag {
        return Err(ParserError::Decoding(
            ParserDecodingError::UnexpectedCompactInsides,
        ));
    }
    if balance_flag {
        return Err(ParserError::Decoding(
            ParserDecodingError::BalanceNotDescribed,
        ));
    }
    Ok(())
}

/// Function to decode `char`.
/// `char` is always 4 byte element, so the needed length pre-determined.
///
/// The function decodes only `char` part, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - indent used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.
fn decode_char(data: &[u8], indent: u32) -> Result<DecodedOut, ParserError> {
    match data.get(0..4) {
        Some(mut slice_to_char) => match <u32>::decode(&mut slice_to_char) {
            Ok(a) => match char::from_u32(a) {
                Some(b) => {
                    let fancy_out = vec![OutputCard {
                        card: ParserCard::Default(b.to_string()),
                        indent,
                    }];
                    let remaining_vector = (data[4..]).to_vec();
                    Ok(DecodedOut {
                        remaining_vector,
                        fancy_out,
                    })
                }
                None => Err(ParserError::Decoding(
                    ParserDecodingError::PrimitiveFailure("char".to_string()),
                )),
            },
            Err(_) => Err(ParserError::Decoding(
                ParserDecodingError::PrimitiveFailure("char".to_string()),
            )),
        },
        None => Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    }
}

/// Function to decode `str`.
/// `str` is encoded as a vector of utf-converteable elements, and is therefore
/// preluded by the number of elements as compact.
///
/// The function decodes only `str` part, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - indent used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.
fn decode_str(data: &[u8], indent: u32) -> Result<DecodedOut, ParserError> {
    let pre_str = get_compact::<u32>(data)?;
    let str_length = pre_str.compact_found as usize;
    match pre_str.start_next_unit {
        Some(start) => match data.get(start..start + str_length) {
            Some(a) => {
                let text = match String::from_utf8(a.to_vec()) {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(ParserError::Decoding(
                            ParserDecodingError::PrimitiveFailure("str".to_string()),
                        ))
                    }
                };
                let fancy_out = vec![OutputCard {
                    card: ParserCard::Text(text),
                    indent,
                }];
                let remaining_vector = data[start + str_length..].to_vec();
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            }
            None => Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
        },
        None => {
            if str_length != 0 {
                Err(ParserError::Decoding(ParserDecodingError::DataTooShort))
            } else {
                let fancy_out = vec![OutputCard {
                    card: ParserCard::Text(String::new()),
                    indent,
                }];
                let remaining_vector = Vec::new();
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            }
        }
    }
}

/// Function to decode `U256` and `I256`.
/// Both `U256` and `I256` are always 32 byte element, so the needed length pre-determined.
/// Note: both types are marked as non-existing in rust in scale-info.
/// For decoding the elements, `le` functions from num_bigint crate are used.
///
/// The function decodes only `U256` or `I256` part, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining Vec<u8> of data),
/// - boolean flag if the expected value is signed (signed corresponds to I256, unsigned corresponds to U256),
/// - indent used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.
fn decode_big256(data: &[u8], signed: bool, indent: u32) -> Result<DecodedOut, ParserError> {
    match data.get(0..32) {
        Some(slice_to_big256) => {
            let fancy_out = {
                if signed {
                    vec![OutputCard {
                        card: ParserCard::Default(
                            BigInt::from_signed_bytes_le(slice_to_big256).to_string(),
                        ),
                        indent,
                    }]
                }
                // I256
                else {
                    vec![OutputCard {
                        card: ParserCard::Default(
                            BigUint::from_bytes_le(slice_to_big256).to_string(),
                        ),
                        indent,
                    }]
                } // U256
            };
            let remaining_vector = (data[32..]).to_vec();
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        }
        None => Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    }
}

#[derive(Debug)]
enum SpecialType {
    AccountId,
    Call,
    None,
}

fn check_special(current_type: &Type<PortableForm>) -> SpecialType {
    match current_type.path().ident() {
        Some(a) => match a.as_str() {
            "AccountId32" => SpecialType::AccountId,
            "Call" => SpecialType::Call,
            _ => SpecialType::None,
        },
        None => SpecialType::None,
    }
}

// TODO Types that should be displayed as Balance can originate not from fields, for example, from tuples.
// Typical example is (AccountId, Balance) tuple. While AccountId goes through type with "AccountId" in ident,
// and could be easily detected, Balance is immediately linked to corresponding number.
// If however, the typeName is searched for word "Balance", numerous false positives are possible.
fn field_type_name_is_balance(type_name: &str) -> bool {
    (type_name == "Balance")
        || (type_name == "T::Balance")
        || (type_name == "BalanceOf<T>")
        || (type_name == "ExtendedBalance")
        || (type_name == "BalanceOf<T, I>")
        || (type_name == "DepositBalance")
        || (type_name == "PalletBalanceOf<T>")
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn decoding_sci_complete(
    current_type: &Type<PortableForm>,
    possible_ext: &mut Option<&mut Ext>,
    compact_flag: bool,
    balance_flag: bool,
    call_expectation: &CallExpectation,
    data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    if let Some(ext) = possible_ext {
        ext.check_special(current_type)
    }
    let special_type = check_special(current_type);
    let call_expectation = {
        if let SpecialType::Call = special_type {
            call_expectation.add()
        } else {
            CallExpectation::None
        }
    };
    match special_type {
        SpecialType::AccountId => special_case_account_id(data, indent, short_specs),
        _ => {
            if let Some(ext) = possible_ext {
                if let SpecialExt::Era = ext.specialty {
                    if ext.found_ext.era.is_some() {
                        return Err(ParserError::FundamentallyBadV14Metadata(
                            ParserMetadataError::EraTwice,
                        ));
                    } else {
                        return special_case_era(data, &mut ext.found_ext, indent);
                    }
                }
            }
            match current_type.type_def() {
                TypeDef::Composite(x) => decode_type_def_composite(
                    x,
                    possible_ext,
                    compact_flag,
                    balance_flag,
                    data,
                    meta_v14,
                    indent,
                    short_specs,
                ),
                TypeDef::Variant(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    decode_type_def_variant(
                        x,
                        possible_ext,
                        &call_expectation,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                TypeDef::Sequence(x) => {
                    if compact_flag {
                        return Err(ParserError::Decoding(
                            ParserDecodingError::UnexpectedCompactInsides,
                        ));
                    }
                    let inner_type = match meta_v14.types.resolve(x.type_param().id()) {
                        // docs?
                        Some(a) => a,
                        None => {
                            return Err(ParserError::Decoding(
                                ParserDecodingError::V14TypeNotResolved,
                            ))
                        }
                    };
                    decode_type_def_sequence(
                        inner_type,
                        possible_ext,
                        balance_flag,
                        &call_expectation,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                TypeDef::Array(x) => {
                    if let Some(ext) = possible_ext {
                        if let SpecialExt::Hash(ref hash) = ext.specialty {
                            match hash {
                                Hash::GenesisHash => {
                                    if ext.found_ext.genesis_hash.is_some() {
                                        return Err(ParserError::FundamentallyBadV14Metadata(
                                            ParserMetadataError::GenesisHashTwice,
                                        ));
                                    }
                                }
                                Hash::BlockHash => {
                                    if ext.found_ext.block_hash.is_some() {
                                        return Err(ParserError::FundamentallyBadV14Metadata(
                                            ParserMetadataError::BlockHashTwice,
                                        ));
                                    }
                                }
                            }
                            return special_case_hash(
                                data,
                                &mut ext.found_ext,
                                indent,
                                short_specs,
                                hash,
                            );
                        }
                    }
                    if compact_flag {
                        return Err(ParserError::Decoding(
                            ParserDecodingError::UnexpectedCompactInsides,
                        ));
                    }
                    let inner_type = match meta_v14.types.resolve(x.type_param().id()) {
                        // docs?
                        Some(a) => a,
                        None => {
                            return Err(ParserError::Decoding(
                                ParserDecodingError::V14TypeNotResolved,
                            ))
                        }
                    };
                    decode_type_def_array(
                        inner_type,
                        x.len(),
                        possible_ext,
                        balance_flag,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                TypeDef::Tuple(x) => {
                    if compact_flag {
                        return Err(ParserError::Decoding(
                            ParserDecodingError::UnexpectedCompactInsides,
                        ));
                    }
                    let id_set = x.fields().iter().map(|a| a.id()).collect();
                    decode_type_def_tuple(
                        id_set,
                        possible_ext,
                        balance_flag,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                TypeDef::Primitive(x) => decode_type_def_primitive(
                    x,
                    possible_ext,
                    compact_flag,
                    balance_flag,
                    &data,
                    indent,
                    short_specs,
                ),
                TypeDef::Compact(x) => {
                    let inner_type = match meta_v14.types.resolve(x.type_param().id()) {
                        // docs?
                        Some(a) => a,
                        None => {
                            return Err(ParserError::Decoding(
                                ParserDecodingError::V14TypeNotResolved,
                            ))
                        }
                    };
                    let compact_flag = true;
                    decoding_sci_complete(
                        inner_type,
                        possible_ext,
                        compact_flag,
                        balance_flag,
                        &CallExpectation::None,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                TypeDef::BitSequence(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    decode_type_def_bit_sequence(x, data, meta_v14, indent)
                }
            }
        }
    }
}

pub(crate) fn decoding_sci_entry_point(
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    mut indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let pallet_index: u8 = match data.get(0) {
        Some(x) => *x,
        None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    };

    let mut found_call_type: Option<u32> = None;
    let mut found_pallet_name: Option<String> = None;
    for x in meta_v14.pallets.iter() {
        if x.index == pallet_index {
            found_pallet_name = Some(x.name.to_string());
            if let Some(a) = &x.calls {
                found_call_type = Some(a.ty.id());
            }
            break;
        }
    }
    let pallet_name = match found_pallet_name {
        Some(a) => a,
        None => {
            return Err(ParserError::Decoding(ParserDecodingError::PalletNotFound(
                pallet_index,
            )))
        }
    };
    let type_id = match found_call_type {
        Some(a) => a,
        None => {
            return Err(ParserError::Decoding(ParserDecodingError::NoCallsInPallet(
                pallet_name,
            )))
        }
    };
    let (current_type, _, _) = type_path_docs(meta_v14, type_id)?;

    let mut fancy_out = vec![OutputCard {
        card: ParserCard::Pallet(pallet_name),
        indent,
    }];
    indent += 1;
    data = data[1..].to_vec();

    let compact_flag = false;
    let balance_flag = false;
    let decoded_out = decoding_sci_complete(
        &current_type,
        &mut None,
        compact_flag,
        balance_flag,
        &CallExpectation::Pallet,
        data,
        meta_v14,
        indent,
        short_specs,
    )?;
    fancy_out.extend_from_slice(&decoded_out.fancy_out);

    Ok(DecodedOut {
        remaining_vector: decoded_out.remaining_vector,
        fancy_out,
    })
}

#[allow(clippy::too_many_arguments)]
fn decode_type_def_sequence(
    inner_type: &Type<PortableForm>,
    possible_ext: &mut Option<&mut Ext>,
    balance_flag: bool,
    call_expectation: &CallExpectation,
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let pre_vector = get_compact::<u32>(&data)?;
    let mut fancy_output_prep: Vec<OutputCard> = Vec::new();
    let elements_of_vector = pre_vector.compact_found;
    match pre_vector.start_next_unit {
        Some(start) => {
            data = data[start..].to_vec();
            for _i in 0..elements_of_vector {
                let compact_flag = false;
                let after_run = decoding_sci_complete(
                    inner_type,
                    possible_ext,
                    compact_flag,
                    balance_flag,
                    call_expectation,
                    data,
                    meta_v14,
                    indent,
                    short_specs,
                )?;
                fancy_output_prep.extend_from_slice(&after_run.fancy_out);
                data = after_run.remaining_vector;
            }
            Ok(DecodedOut {
                remaining_vector: data,
                fancy_out: fancy_output_prep,
            })
        }
        None => {
            if elements_of_vector != 0 {
                Err(ParserError::Decoding(ParserDecodingError::DataTooShort))
            } else {
                Ok(DecodedOut {
                    remaining_vector: Vec::new(),
                    fancy_out: vec![OutputCard {
                        card: ParserCard::Default(String::new()),
                        indent,
                    }],
                })
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn decode_type_def_array(
    inner_type: &Type<PortableForm>,
    len: u32,
    possible_ext: &mut Option<&mut Ext>,
    balance_flag: bool,
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for _i in 0..len {
        let compact_flag = false;
        let after_run = decoding_sci_complete(
            inner_type,
            possible_ext,
            compact_flag,
            balance_flag,
            &CallExpectation::None,
            data,
            meta_v14,
            indent,
            short_specs,
        )?;
        fancy_out.extend_from_slice(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data,
        fancy_out,
    })
}

fn decode_type_def_tuple(
    id_set: Vec<u32>,
    possible_ext: &mut Option<&mut Ext>,
    balance_flag: bool,
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for (i, type_id) in id_set.iter().enumerate() {
        let (inner_type, path, docs) = type_path_docs(meta_v14, *type_id)?;
        fancy_out.push(OutputCard {
            card: ParserCard::FieldNumber {
                number: i + 1,
                docs_field_number: String::new(),
                path_type: path,
                docs_type: docs,
            },
            indent,
        });
        let compact_flag = false;
        let after_run = decoding_sci_complete(
            &inner_type,
            possible_ext,
            compact_flag,
            balance_flag,
            &CallExpectation::None,
            data,
            meta_v14,
            indent,
            short_specs,
        )?;
        fancy_out.extend_from_slice(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data,
        fancy_out,
    })
}

struct IsOptionBool {
    is_option: bool,
    is_bool: bool,
}

fn is_option_bool(
    found_ty: &TypeDefVariant<PortableForm>,
    meta_v14: &RuntimeMetadataV14,
) -> IsOptionBool {
    let mut got_len = false;
    let mut got_none = false;
    let mut got_some = false;
    let mut is_bool = false;
    if found_ty.variants().len() == 2 {
        got_len = true;
        for x in found_ty.variants().iter() {
            if x.name().as_str() == "None" {
                got_none = true;
            }
            if x.name().as_str() == "Some" {
                got_some = true;
                let fields = x.fields();
                if fields.len() == 1 {
                    let option_type_id = fields[0].ty().id();
                    match meta_v14.types.resolve(option_type_id) {
                        Some(a) => {
                            if let TypeDef::Primitive(TypeDefPrimitive::Bool) = a.type_def() {
                                is_bool = true
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                } else {
                    break;
                }
            }
        }
    }
    IsOptionBool {
        is_option: got_len && got_none && got_some,
        is_bool,
    }
}

fn decode_type_def_variant(
    found_ty: &TypeDefVariant<PortableForm>,
    possible_ext: &mut Option<&mut Ext>,
    call_expectation: &CallExpectation,
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let enum_index = match data.get(0) {
        Some(x) => *x,
        None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    };

    let check = is_option_bool(found_ty, meta_v14);
    if check.is_option {
        if check.is_bool {
            let fancy_out = match enum_index {
                0 => vec![OutputCard {
                    card: ParserCard::None,
                    indent,
                }],
                1 => vec![OutputCard {
                    card: ParserCard::Default(String::from("True")),
                    indent,
                }],
                2 => vec![OutputCard {
                    card: ParserCard::Default(String::from("False")),
                    indent,
                }],
                _ => {
                    return Err(ParserError::Decoding(
                        ParserDecodingError::UnexpectedOptionVariant,
                    ))
                }
            };
            let remaining_vector = data[1..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        } else {
            match enum_index {
                0 => {
                    let fancy_out = vec![OutputCard {
                        card: ParserCard::None,
                        indent,
                    }];
                    let remaining_vector = data[1..].to_vec();
                    Ok(DecodedOut {
                        remaining_vector,
                        fancy_out,
                    })
                }
                1 => {
                    if data.len() == 1 {
                        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
                    }
                    data = data[1..].to_vec();
                    let found_variant = &found_ty.variants()[1];
                    let compact_flag = false;
                    let balance_flag = false;
                    process_fields(
                        found_variant.fields(),
                        possible_ext,
                        &CallExpectation::None,
                        compact_flag,
                        balance_flag,
                        data,
                        meta_v14,
                        indent,
                        short_specs,
                    )
                }
                _ => Err(ParserError::Decoding(
                    ParserDecodingError::UnexpectedOptionVariant,
                )),
            }
        }
    } else {
        let mut found_variant = None;
        for x in found_ty.variants().iter() {
            if x.index() == enum_index {
                found_variant = Some(x);
                break;
            }
        }
        let found_variant = match found_variant {
            Some(a) => a,
            None => {
                return Err(ParserError::Decoding(
                    ParserDecodingError::UnexpectedEnumVariant,
                ))
            }
        };
        let mut variant_docs = String::new();
        for (i, x) in found_variant.docs().iter().enumerate() {
            if i > 0 {
                variant_docs.push('\n');
            }
            variant_docs.push_str(x);
        }
        let mut fancy_out = match call_expectation {
            CallExpectation::None => vec![OutputCard {
                card: ParserCard::EnumVariantName {
                    name: found_variant.name().to_string(),
                    docs_enum_variant: variant_docs,
                },
                indent,
            }],
            CallExpectation::Pallet => vec![OutputCard {
                card: ParserCard::Pallet(found_variant.name().to_string()),
                indent,
            }],
            CallExpectation::Method => vec![OutputCard {
                card: ParserCard::Method {
                    method_name: found_variant.name().to_string(),
                    docs: variant_docs,
                },
                indent,
            }],
        };
        data = data[1..].to_vec();

        let compact_flag = false;
        let balance_flag = false;
        let fields_processed = process_fields(
            found_variant.fields(),
            possible_ext,
            call_expectation,
            compact_flag,
            balance_flag,
            data,
            meta_v14,
            indent + 1,
            short_specs,
        )?;
        fancy_out.extend_from_slice(&fields_processed.fancy_out);
        data = fields_processed.remaining_vector;

        Ok(DecodedOut {
            remaining_vector: data,
            fancy_out,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn process_fields(
    fields: &[Field<PortableForm>],
    possible_ext: &mut Option<&mut Ext>,
    call_expectation: &CallExpectation,
    compact_flag: bool,
    mut balance_flag: bool,
    mut data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let mut indent_skipped = false;
    let mut field_is_str = false;
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for (i, x) in fields.iter().enumerate() {
        let mut field_docs = String::new();
        for (j, y) in x.docs().iter().enumerate() {
            if j > 0 {
                field_docs.push('\n');
            }
            field_docs.push_str(y);
        }
        let (inner_type, path_type, docs_type) = type_path_docs(meta_v14, x.ty().id())?;
        match x.name() {
            Some(field_name) => {
                fancy_out.push(OutputCard {
                    card: ParserCard::FieldName {
                        name: field_name.to_string(),
                        docs_field_name: field_docs,
                        path_type,
                        docs_type,
                    },
                    indent,
                });
                if (field_name == "remark") || (field_name == "remark_with_event") {
                    field_is_str = true;
                }
            }
            None => {
                if fields.len() > 1 {
                    fancy_out.push(OutputCard {
                        card: ParserCard::FieldNumber {
                            number: i,
                            docs_field_number: field_docs,
                            path_type,
                            docs_type,
                        },
                        indent,
                    });
                } else {
                    indent_skipped = true;
                }
            }
        }
        balance_flag = match x.type_name() {
            Some(a) => field_type_name_is_balance(a),
            None => balance_flag,
        };
        let indent = {
            if indent_skipped {
                indent
            } else {
                indent + 1
            }
        };
        let after_run = {
            if field_is_str {
                decode_str(&data, indent)?
            } else {
                decoding_sci_complete(
                    &inner_type,
                    possible_ext,
                    compact_flag,
                    balance_flag,
                    call_expectation,
                    data,
                    meta_v14,
                    indent,
                    short_specs,
                )?
            }
        };
        fancy_out.extend_from_slice(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data,
        fancy_out,
    })
}

#[allow(clippy::too_many_arguments)]
fn decode_type_def_composite(
    composite_ty: &TypeDefComposite<PortableForm>,
    possible_ext: &mut Option<&mut Ext>,
    compact_flag: bool,
    balance_flag: bool,
    data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    if compact_flag && (composite_ty.fields().len() > 1) {
        return Err(ParserError::Decoding(
            ParserDecodingError::UnexpectedCompactInsides,
        ));
    }
    process_fields(
        composite_ty.fields(),
        possible_ext,
        &CallExpectation::None,
        compact_flag,
        balance_flag,
        data,
        meta_v14,
        indent,
        short_specs,
    )
}

fn decode_type_def_bit_sequence(
    bit_ty: &TypeDefBitSequence<PortableForm>,
    data: Vec<u8>,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
) -> Result<DecodedOut, ParserError> {
    let pre_bitvec = get_compact::<u32>(&data)?;
    let actual_length = match pre_bitvec.compact_found % 8 {
        0 => (pre_bitvec.compact_found / 8),
        _ => (pre_bitvec.compact_found / 8) + 1,
    };
    match pre_bitvec.start_next_unit {
        Some(start) => {
            let fin = start + (actual_length as usize);
            let into_bv_decode = match data.get(..fin) {
                Some(a) => a.to_vec(),
                None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
            };
            let bitorder_type_id = bit_ty.bit_order_type().id();
            let bitorder_type = match meta_v14.types.resolve(bitorder_type_id) {
                Some(a) => a,
                None => {
                    return Err(ParserError::Decoding(
                        ParserDecodingError::V14TypeNotResolved,
                    ))
                }
            };
            let bitorder = match bitorder_type.type_def() {
                TypeDef::Composite(_) => match bitorder_type.path().ident() {
                    Some(x) => match x.as_str() {
                        "Lsb0" => FoundBitOrder::Lsb0,
                        "Msb0" => FoundBitOrder::Msb0,
                        _ => {
                            return Err(ParserError::Decoding(ParserDecodingError::NotBitOrderType))
                        }
                    },
                    None => {
                        return Err(ParserError::Decoding(ParserDecodingError::NotBitOrderType))
                    }
                },
                _ => return Err(ParserError::Decoding(ParserDecodingError::NotBitOrderType)),
            };

            let bitstore_type_id = bit_ty.bit_store_type().id();
            let bitstore_type = match meta_v14.types.resolve(bitstore_type_id) {
                Some(a) => a,
                None => {
                    return Err(ParserError::Decoding(
                        ParserDecodingError::V14TypeNotResolved,
                    ))
                }
            };
            let card_prep = match bitstore_type.type_def() {
                TypeDef::Primitive(a) => {
                    match a {
                        TypeDefPrimitive::U8 => process_bitvec::<u8>(bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U16 => process_bitvec::<u16>(bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U32 => process_bitvec::<u32>(bitorder, into_bv_decode)?,
                        // this should not be here, but due to possible architecture limitations u64 will not compile on 32-bit architectures
                        // ideally, should be patched by `#[repr(C, align(8))]` thing similar to bitvec issue 76
                        // TypeDefPrimitive::U64 => process_bitvec::<u64> (bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U64 => match bitorder {
                            FoundBitOrder::Lsb0 => ugly_patch_u64::<Lsb0>(into_bv_decode)?,
                            FoundBitOrder::Msb0 => ugly_patch_u64::<Msb0>(into_bv_decode)?,
                        },
                        _ => {
                            return Err(ParserError::Decoding(ParserDecodingError::NotBitStoreType))
                        }
                    }
                }
                _ => return Err(ParserError::Decoding(ParserDecodingError::NotBitStoreType)),
            };

            let fancy_out = vec![OutputCard {
                card: ParserCard::BitVec(card_prep),
                indent,
            }];
            let remaining_vector = data[fin..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        }
        None => {
            if actual_length != 0 {
                return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
            }
            Ok(DecodedOut {
                remaining_vector: Vec::new(),
                fancy_out: vec![OutputCard {
                    card: ParserCard::Default(String::new()),
                    indent,
                }],
            })
        }
    }
}

fn process_bitvec<T: BitStore + Decode>(
    bitorder: FoundBitOrder,
    into_bv_decode: Vec<u8>,
) -> Result<String, ParserError> {
    match bitorder {
        FoundBitOrder::Lsb0 => match <BitVec<T, Lsb0>>::decode(&mut &into_bv_decode[..]) {
            Ok(b) => Ok(b.to_string()),
            Err(_) => Err(ParserError::Decoding(ParserDecodingError::BitVecFailure)),
        },
        FoundBitOrder::Msb0 => match <BitVec<T, Msb0>>::decode(&mut &into_bv_decode[..]) {
            Ok(b) => Ok(b.to_string()),
            Err(_) => Err(ParserError::Decoding(ParserDecodingError::BitVecFailure)),
        },
    }
}

fn ugly_patch_u64<O: BitOrder>(into_bv_decode: Vec<u8>) -> Result<String, ParserError> {
    let bitvec_decoded = match <BitVec<u32, O>>::decode(&mut &into_bv_decode[..]) {
        Ok(b) => b,
        Err(_) => return Err(ParserError::Decoding(ParserDecodingError::BitVecFailure)),
    };
    let vec = bitvec_decoded.into_vec();
    let mut out = String::from("[");
    for i in 0..vec.len() / 2 {
        if i > 0 {
            out.push_str(", ");
        }
        let print1 = BitVec::<u32, O>::from_vec(vec![vec[2 * i]]).to_string();
        let print2 = BitVec::<u32, O>::from_vec(vec![vec[2 * i + 1]]).to_string();
        let _ = write!(
            out,
            "{}{}",
            &print1[1..print1.len() - 1],
            &print2[1..print2.len() - 1]
        );
    }
    out.push(']');
    Ok(out)
}

fn type_path_docs(
    meta_v14: &RuntimeMetadataV14,
    type_id: u32,
) -> Result<(Type<PortableForm>, String, String), ParserError> {
    let current_type = match meta_v14.types.resolve(type_id) {
        Some(a) => a,
        None => {
            return Err(ParserError::Decoding(
                ParserDecodingError::V14TypeNotResolved,
            ))
        }
    };
    let mut docs = String::new();
    for (i, x) in current_type.docs().iter().enumerate() {
        if i > 0 {
            docs.push('\n');
        }
        docs.push_str(x);
    }
    let mut path = String::new();
    for (i, x) in current_type.path().segments().iter().enumerate() {
        if i > 0 {
            path.push_str(" >> ");
        }
        path.push_str(x);
    }
    Ok((current_type.to_owned(), path, docs))
}
