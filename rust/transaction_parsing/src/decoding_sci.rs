use parity_scale_codec::Decode;
use scale_info::{Field, Type, TypeDef, TypeDefPrimitive, TypeDefVariant, TypeDefComposite, TypeDefBitSequence, /*TypeDefRange,*/ form::PortableForm};
use num_bigint::{BigInt, BigUint};
use definitions::{network_specs::ChainSpecs};
use frame_metadata::v14::RuntimeMetadataV14;
use bitvec::{prelude::{BitVec, Lsb0, Msb0}, store::BitStore, order::BitOrder};
//use std::mem::size_of;

use crate::cards::Card;
use crate::decoding_commons::{DecodedOut, get_compact, decode_known_length, decode_primitive_with_flags, special_case_account_id};
use crate::error::{Error, UnableToDecode};
/*
enum SpecialDecodingCases {
    Option,
    Compact,
    Call,
    IdentityFields,
    None,
}

enum SpecialDisplayCases {
    AccountId,
    Balance,
    BitVec,
    None
}
*/
enum FoundBitOrder {
    Lsb0,
    Msb0,
}


/// Function to decode types that are variants of TypeDefPrimitive enum.
///
/// The function decodes only given type found_ty, removes already decoded part of input data Vec<u8>,
/// and returns whatever remains as DecodedOut field remaining_vector, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (TypeDefPrimitive, found in the previous iteration)
/// - data (remaining Vec<u8> of data),
/// - index and indent that are used for creating properly formatted js cards.
///
/// The function outputs the DecodedOut value in case of success.

fn decode_type_def_primitive (found_ty: &TypeDefPrimitive, compact_flag: bool, balance_flag: bool, data: &Vec<u8>, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    match found_ty {
        TypeDefPrimitive::Bool => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<bool>(data, "bool", index, indent)
        },
        TypeDefPrimitive::Char => {
            reject_flags(compact_flag, balance_flag)?;
            decode_char(data, index, indent)
        },
        TypeDefPrimitive::Str => {
            reject_flags(compact_flag, balance_flag)?;
            decode_str(data, index, indent)
        },
        TypeDefPrimitive::U8 => decode_primitive_with_flags::<u8> (data, compact_flag, balance_flag, "u8", index, indent, chain_specs),
        TypeDefPrimitive::U16 => decode_primitive_with_flags::<u16> (data, compact_flag, balance_flag, "u16", index, indent, chain_specs),
        TypeDefPrimitive::U32 => decode_primitive_with_flags::<u32> (data, compact_flag, balance_flag, "u32", index, indent, chain_specs),
        TypeDefPrimitive::U64 => decode_primitive_with_flags::<u64> (data, compact_flag, balance_flag, "u64", index, indent, chain_specs),
        TypeDefPrimitive::U128 => decode_primitive_with_flags::<u128> (data, compact_flag, balance_flag, "u128", index, indent, chain_specs),
        TypeDefPrimitive::U256 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_big256(data, false, index, indent)
        },
        TypeDefPrimitive::I8 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i8>(data, "i8", index, indent)
        },
        TypeDefPrimitive::I16 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i16>(data, "i16", index, indent)
        },
        TypeDefPrimitive::I32 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i32>(data, "i32", index, indent)
        },
        TypeDefPrimitive::I64 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i64>(data, "i64", index, indent)
        },
        TypeDefPrimitive::I128 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_known_length::<i128>(data, "i128", index, indent)
        },
        TypeDefPrimitive::I256 => {
            reject_flags(compact_flag, balance_flag)?;
            decode_big256(data, true, index, indent)
        },
    }
}


fn reject_flags (compact_flag: bool, balance_flag: bool) -> Result<(), Error> {
    if compact_flag {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides))}
    if balance_flag {return Err(Error::UnableToDecode(UnableToDecode::BalanceNotDescribed))}
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
/// - index and indent that are used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.

fn decode_char(data: &Vec<u8>, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    match data.get(0..4) {
        Some(slice_to_char) => {
            match <u32>::decode(&mut &slice_to_char[..]) {
                Ok(a) => {
                    match char::from_u32(a) {
                        Some(b) => {
                            let fancy_out = format!(",{}", (Card::Default(&b.to_string()).card(index, indent)));
                            let remaining_vector = (data[4..]).to_vec();
                            Ok(DecodedOut {
                                remaining_vector,
                                indent,
                                fancy_out,
                            })
                        },
                        None => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure("char".to_string()))),
                    }
                },
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure("char".to_string()))),
            }
        },
        None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
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
/// - index and indent that are used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.

fn decode_str(data: &Vec<u8>, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    let pre_str = get_compact::<u32>(&data)?;
    let str_length = pre_str.compact_found as usize;
    match pre_str.start_next_unit {
        Some(start) => {
            match data.get(start..start+str_length) {
                Some(a) => {
                    let text = match String::from_utf8(a.to_vec()) {
                        Ok(b) => b,
                        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::PrimitiveFailure("str".to_string()))),
                    };
                    let fancy_out = format!(",{}", (Card::Default(&text).card(index, indent)));
                    let remaining_vector = data[start+str_length..].to_vec();
                    Ok(DecodedOut {
                        remaining_vector,
                        indent,
                        fancy_out,
                    })
                },
                None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
            }
        },
        None => {
            if str_length != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            else {
                let fancy_out = format!(",{}", (Card::Default("").card(index, indent)));
                let remaining_vector = Vec::new();
                Ok(DecodedOut {
                    remaining_vector,
                    indent,
                    fancy_out,
                })
            }
        },
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
/// - index and indent that are used for creating properly formatted output cards.
///
/// The function outputs the DecodedOut value in case of success.

fn decode_big256(data: &Vec<u8>, signed: bool, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    match data.get(0..32) {
        Some(slice_to_big256) => {
            let fancy_out = {
                if signed {format!(",{}", (Card::Default(&BigInt::from_signed_bytes_le(slice_to_big256).to_string()).card(index, indent)))} // I256
                else {format!(",{}", (Card::Default(&BigUint::from_bytes_le(slice_to_big256).to_string()).card(index, indent)))} // U256
            };
            let remaining_vector = (data[32..]).to_vec();
            Ok(DecodedOut {
                remaining_vector,
                indent,
                fancy_out,
            })
        },
        None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
    }
}

fn type_is_account_id (current_type: &Type<PortableForm>) -> bool {
    match current_type.path().ident() {
        Some(a) => {
            a.as_str() == "AccountId32"
        },
        None => false,
    }
}

// TODO Types that should be displayed as Balance can originate not from fields, for example, from tuples.
// Typical example is (AccountId, Balance) tuple. While AccountId goes through type with "AccountId" in ident,
// and could be easily detected, Balance is immediately linked to corresponding number.
// If however, the typeName is searched for word "Balance", numerous false positives are possible.
fn field_type_name_is_balance (type_name: &str) -> bool {
    (type_name == "Balance")||(type_name == "T::Balance")||(type_name == "BalanceOf<T>")||(type_name == "ExtendedBalance")||(type_name == "BalanceOf<T, I>")||(type_name == "DepositBalance")
}


pub fn decoding_sci_complete (type_id: u32, compact_flag: bool, balance_flag: bool, data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, mut indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let current_type = match meta_v14.types.resolve(type_id) {
        Some(a) => a,
        None => return Err(Error::UnableToDecode(UnableToDecode::V14TypeNotResolved)),
    };
/*
// checking if the entry is actually a bundled call - this will slightly change output formatting
    if let Some(x) = current_type.path().ident() {
        if x.as_str() == "Call" {
            
            indent = indent + 1;
        }
    }
*/
    let mut docs = String::new();
    for (i, x) in current_type.docs().iter().enumerate() {
        if i>0 {docs.push_str("\n");}
        docs.push_str(x);
    }
    
    let mut path = String::from("[");
    for (i, x) in current_type.path().segments().iter().enumerate() {
        if i>0 {path.push_str(",");}
        path.push_str(&format!("\"{}\"", x));
    }
    path.push_str("]");
    
    let mut fancy_out = {
        if (docs.len()==0)&&(path.len()==2) {String::new()}
        else {
            let out = format!(",{}", (Card::PathDocs{path: &path, docs: &docs}.card(index, indent)));
            indent = indent + 1;
            out
        }
    };
    let after_run = {
        if type_is_account_id(current_type) {special_case_account_id(data, index, indent, chain_specs)?}
        else {    
            match current_type.type_def() {
                TypeDef::Composite(x) =>  {
                    if balance_flag {return Err(Error::UnableToDecode(UnableToDecode::BalanceNotDescribed))}
                    decode_type_def_composite (x, compact_flag, data, &meta_v14, index, indent, chain_specs)?
                },
                TypeDef::Variant(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    decode_type_def_variant (x, data, &meta_v14, index, indent, chain_specs)?
                },
                TypeDef::Sequence(x) => {
                    if compact_flag {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides))}
                    decode_type_def_sequence (x.type_param().id(), balance_flag, data, &meta_v14, index, indent, chain_specs)?
                },
                TypeDef::Array(x) => {
                    if compact_flag {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides))}
                    decode_type_def_array(x.type_param().id(), x.len(), balance_flag, data, &meta_v14, index, indent, chain_specs)?
                },
                TypeDef::Tuple(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    let id_set = x.fields().iter().map(|a| a.id()).collect();
                    decode_type_def_tuple(id_set, data, &meta_v14, index, indent, chain_specs)?
                },
                TypeDef::Primitive(x) => decode_type_def_primitive (x, compact_flag, balance_flag, &data, index, indent, chain_specs)?,
                TypeDef::Compact(x) => {
                    let inner_type_id = x.type_param().id();
                    let compact_flag = true;
                    decoding_sci_complete(inner_type_id, compact_flag, balance_flag, data, meta_v14, index, indent, chain_specs)?
                },
                TypeDef::BitSequence(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    decode_type_def_bit_sequence (x, data, &meta_v14, index, indent)?
                },
/*
                TypeDef::Range(x) => {
                    reject_flags(compact_flag, balance_flag)?;
                    decode_type_def_range (x, data, &meta_v14, index, indent)?
                },
*/
            }
        }
    };
    fancy_out.push_str(&after_run.fancy_out);
    Ok(DecodedOut{
        remaining_vector: after_run.remaining_vector,
        indent,
        fancy_out,
    })
}



pub fn decoding_sci_entry_point (mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, mut indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let pallet_index: u8 = match data.get(0) {
        Some(x) => *x,
        None => return Err(Error::UnableToDecode(UnableToDecode::NeedPallet)),
    };
    
    let mut found_call_type: Option<u32> = None;
    let mut found_pallet_name: Option<String> = None;
    for x in meta_v14.pallets.iter() {
        if x.index == pallet_index {
            found_pallet_name = Some(x.name.to_string());
            if let Some(a) = &x.calls {found_call_type = Some(a.ty.id());}
            break;
        }
    }
    let pallet_name = match found_pallet_name {
        Some(a) => a,
        None => return Err(Error::UnableToDecode(UnableToDecode::PalletNotFound(pallet_index))),
    };
    let type_id = match found_call_type {
        Some(a) => a,
        None => return Err(Error::UnableToDecode(UnableToDecode::NoCallsInPallet(pallet_name))),
    };
    let mut fancy_out = format!("{}", (Card::Pallet(&pallet_name)).card(index, indent));
    indent = indent + 1;
    data = data[1..].to_vec();
    
    let compact_flag = false;
    let balance_flag = false;
    let decoded_out = decoding_sci_complete(type_id, compact_flag, balance_flag, data, meta_v14, index, indent, chain_specs)?;
    fancy_out.push_str(&decoded_out.fancy_out);
    
    Ok(DecodedOut{
        remaining_vector: decoded_out.remaining_vector,
        indent,
        fancy_out,
    })
}


fn decode_type_def_sequence (type_id: u32, balance_flag: bool, mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let pre_vector = get_compact::<u32>(&data)?;
    let mut fancy_output_prep = String::new();
    let elements_of_vector = pre_vector.compact_found;
    match pre_vector.start_next_unit {
        Some(start) => {
            data = data[start..].to_vec();
            for _i in 0..elements_of_vector {
                let compact_flag = false;
                let after_run = decoding_sci_complete(type_id, compact_flag, balance_flag, data, meta_v14, index, indent, chain_specs)?;
                fancy_output_prep.push_str(&after_run.fancy_out);
                data = after_run.remaining_vector;
            }
            Ok(DecodedOut {
                remaining_vector: data,
                indent,
                fancy_out: fancy_output_prep,
            })
        },
        None => {
            if elements_of_vector != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            else {
                Ok(DecodedOut {
                    remaining_vector: Vec::new(),
                    indent,
                    fancy_out: Card::Default("").card(index, indent),
                })
            }
        },
    }
}


fn decode_type_def_array (type_id: u32, len: u32, balance_flag: bool, mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let mut fancy_output_prep = String::new();
    for _i in 0..len {
        let compact_flag = false;
        let after_run = decoding_sci_complete(type_id, compact_flag, balance_flag, data, meta_v14, index, indent, chain_specs)?;
        fancy_output_prep.push_str(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut{
        remaining_vector: data,
        indent,
        fancy_out: fancy_output_prep,
    })
}


fn decode_type_def_tuple (id_set: Vec<u32>, mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let mut fancy_out = String::new();
    for (i, type_id) in id_set.iter().enumerate() {
        let fancy_output_prep = format!(",{}", (Card::FieldNumber{number: i+1, docs: ""}).card(index, indent));
        fancy_out.push_str(&fancy_output_prep);
        let compact_flag = false;
        let balance_flag = false;
        let after_run = decoding_sci_complete(*type_id, compact_flag, balance_flag, data, meta_v14, index, indent, chain_specs)?;
        fancy_out.push_str(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut{
        remaining_vector: data,
        indent,
        fancy_out,
    })
}

struct IsOptionBool {
    is_option: bool,
    is_bool: bool,
}

fn is_option_bool (found_ty: &TypeDefVariant<PortableForm>, meta_v14: &RuntimeMetadataV14) -> IsOptionBool {
    let mut got_len = false;
    let mut got_none = false;
    let mut got_some = false;
    let mut is_bool = false;
    if found_ty.variants().len() == 2 {
        got_len = true;
        for x in found_ty.variants().iter() {
            if x.name().as_str() == "None" {got_none = true;}
            if x.name().as_str() == "Some" {
                got_some = true;
                let fields = x.fields();
                if fields.len() == 1 {
                    let option_type_id = fields[0].ty().id();
                    match meta_v14.types.resolve(option_type_id) {
                        Some(a) => {
                            if let TypeDef::Primitive(TypeDefPrimitive::Bool) = a.type_def() {is_bool = true}
                            else {break;}
                        },
                        None => break,
                    }
                }
                else {break;}
            }
        }
    }
    IsOptionBool {
        is_option: got_len&&got_none&&got_some,
        is_bool,
    }
}

/*
fn is_option (found_ty: &TypeDefVariant<PortableForm>) -> bool {
    let mut got_none = false;
    let mut got_some = false;
    let mut out = false;
    if found_ty.variants().len() == 2 {
        for x in found_ty.variants().iter() {
            if x.name().as_str() == "None" {got_none = true;}
            if x.name().as_str() == "Some" {got_some = true;}
        }
        if got_none&&got_some {out = true}
    }
    out
}
*/

fn decode_type_def_variant (found_ty: &TypeDefVariant<PortableForm>, mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    
    let enum_index = match data.get(0) {
        Some(x) => *x,
        None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
    } as usize;
    
    let check = is_option_bool(found_ty, meta_v14);
    if check.is_option {
        if check.is_bool {
            let fancy_out = match enum_index {
                0 => format!(",{}", (Card::None).card(index, indent)),
                1 => format!(",{}", (Card::Default("True")).card(index, indent)),
                2 => format!(",{}", (Card::Default("False")).card(index, indent)),
                _ => {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedOptionVariant))},
            };
            let remaining_vector = data[1..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                indent,
                fancy_out,
            })
        }
        else {
            match enum_index {
                0 => {
                    let fancy_out = format!(",{}", (Card::None).card(index, indent));
                    let remaining_vector = data[1..].to_vec();
                    Ok(DecodedOut {
                        remaining_vector,
                        indent,
                        fancy_out,
                    })
                },
                1 => {
                    if data.len()==1 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
                    data = data[1..].to_vec();
                    let found_variant = &found_ty.variants()[1];
                    let compact_flag = false;
                    process_fields(found_variant.fields(), compact_flag, data, meta_v14, index, indent, chain_specs)
                },
                _ => {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedOptionVariant))},
            }
        }
    }
    else {
        let found_variant = match found_ty.variants().get(enum_index) {
            Some(a) => a,
            None => return Err(Error::UnableToDecode(UnableToDecode::UnexpectedEnumVariant)),
        };
        let mut variant_docs = String::new();
        for (i, x) in found_variant.docs().iter().enumerate() {
            if i>0 {variant_docs.push_str("\n");}
            variant_docs.push_str(x);
        }
        let mut fancy_out = format!(",{}", (Card::EnumVariantName{name: &found_variant.name(), docs: &variant_docs}).card(index, indent));
        data = data[1..].to_vec();
        
        let compact_flag = false;
        let fields_processed = process_fields(found_variant.fields(), compact_flag, data, meta_v14, index, indent+1, chain_specs)?;
        fancy_out.push_str(&fields_processed.fancy_out);
        data = fields_processed.remaining_vector;

        Ok(DecodedOut {
            remaining_vector: data,
            indent,
            fancy_out,
        })
    }
}


fn process_fields (fields: &[Field<PortableForm>], compact_flag: bool, mut data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    let mut fancy_out = String::new();
    for (i, x) in fields.iter().enumerate() {
        let mut balance_flag = false;
        let mut field_docs = String::new();
        for (j, y) in x.docs().iter().enumerate() {
            if j>0 {field_docs.push_str("\n");}
            field_docs.push_str(y);
        }
        match x.name() {
            Some(field_name) => {
                let fancy_out_prep = format!(",{}", (Card::FieldName{name: &field_name, docs: &field_docs}).card(index, indent));
                fancy_out.push_str(&fancy_out_prep);
                balance_flag = match x.type_name() {
                    Some(a) => field_type_name_is_balance(&a),
                    None => false,
                };
            },
            None => {
                if fields.len()>1 {
                    let fancy_out_prep = format!(",{}", (Card::FieldNumber{number: i, docs: &field_docs}).card(index, indent));
                    fancy_out.push_str(&fancy_out_prep);
                }
            },
        }
        let after_run = decoding_sci_complete(x.ty().id(), compact_flag, balance_flag, data, meta_v14, index, indent+1, chain_specs)?;
        fancy_out.push_str(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data,
        indent,
        fancy_out,
    })
}

fn decode_type_def_composite (composite_ty: &TypeDefComposite<PortableForm>, compact_flag: bool, data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32, chain_specs: &ChainSpecs) -> Result<DecodedOut, Error> {
    if compact_flag && (composite_ty.fields().len()>1) {return Err(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides))}
    process_fields (composite_ty.fields(), compact_flag, data, meta_v14, index, indent, chain_specs)
}

fn decode_type_def_bit_sequence (bit_ty: &TypeDefBitSequence<PortableForm>, data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    
    let pre_bitvec = get_compact::<u32>(&data)?;
    let actual_length = match pre_bitvec.compact_found % 8 {
        0 => (pre_bitvec.compact_found / 8),
        _ => (pre_bitvec.compact_found / 8) +1,
    };
    match pre_bitvec.start_next_unit {
        Some(start) => {
            let fin = start + (actual_length as usize);
            let into_bv_decode = match data.get(..fin) {
                Some(a) => a.to_vec(),
                None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
            };
            let bitorder_type_id = bit_ty.bit_order_type().id();
            let bitorder_type = match meta_v14.types.resolve(bitorder_type_id) {
                Some(a) => a,
                None => return Err(Error::UnableToDecode(UnableToDecode::V14TypeNotResolved)),
            };
            let bitorder = match bitorder_type.type_def() {
                TypeDef::Composite(_) => match bitorder_type.path().ident() {
                    Some(x) => {
                        match x.as_str() {
                            "Lsb0" => FoundBitOrder::Lsb0,
                            "Msb0" => FoundBitOrder::Msb0,
                            _ => return Err(Error::UnableToDecode(UnableToDecode::NotBitOrderType)),
                        }
                    },
                    None => return Err(Error::UnableToDecode(UnableToDecode::NotBitOrderType)),
                },
                _ => return Err(Error::UnableToDecode(UnableToDecode::NotBitOrderType)),
            };
            
            let bitstore_type_id = bit_ty.bit_store_type().id();
            let bitstore_type = match meta_v14.types.resolve(bitstore_type_id) {
                Some(a) => a,
                None => return Err(Error::UnableToDecode(UnableToDecode::V14TypeNotResolved)),
            };
            let card_prep = match bitstore_type.type_def() {
                TypeDef::Primitive(a) => {
                    match a {
                        TypeDefPrimitive::U8 => process_bitvec::<u8> (bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U16 => process_bitvec::<u16> (bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U32 => process_bitvec::<u32> (bitorder, into_bv_decode)?,
                    // this should not be here, but due to possible architecture limitations u64 will not compile on 32-bit architectures
                    // ideally, should be patched by `#[repr(C, align(8))]` thing similar to bitvec issue 76
                        // TypeDefPrimitive::U64 => process_bitvec::<u64> (bitorder, into_bv_decode)?,
                        TypeDefPrimitive::U64 => match bitorder {
                            FoundBitOrder::Lsb0 => ugly_patch_u64::<Lsb0> (into_bv_decode)?,
                            FoundBitOrder::Msb0 => ugly_patch_u64::<Msb0> (into_bv_decode)?,
                        },
                        _ => return Err(Error::UnableToDecode(UnableToDecode::NotBitStoreType)),
                    }
                },
                _ => return Err(Error::UnableToDecode(UnableToDecode::NotBitStoreType)),
            };
            
            let fancy_out = format!(",{}", (Card::BitVec(card_prep)).card(index, indent));
            let remaining_vector = data[fin..].to_vec();
            Ok(DecodedOut {
                remaining_vector,
                indent,
                fancy_out,
            })
        },
        None => {
            if actual_length != 0 {return Err(Error::UnableToDecode(UnableToDecode::DataTooShort))}
            Ok(DecodedOut {
                remaining_vector: Vec::new(),
                indent,
                fancy_out: Card::Default("").card(index, indent),
            })
        }
    }
}


fn process_bitvec<T: BitStore + Decode> (bitorder: FoundBitOrder, into_bv_decode: Vec<u8>) -> Result<String, Error> {
    match bitorder {
        FoundBitOrder::Lsb0 => {
            match <BitVec<Lsb0, T>>::decode(&mut &into_bv_decode[..]) {
                Ok(b) => Ok(b.to_string()),
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::BitVecFailure)),
            }
        },
        FoundBitOrder::Msb0 => {
            match <BitVec<Msb0, T>>::decode(&mut &into_bv_decode[..]) {
                Ok(b) => Ok(b.to_string()),
                Err(_) => return Err(Error::UnableToDecode(UnableToDecode::BitVecFailure)),
            }
        },
    }
}

fn ugly_patch_u64<O: BitOrder> (into_bv_decode: Vec<u8>) -> Result<String, Error> {
    let bitvec_decoded = match <BitVec<O, u32>>::decode(&mut &into_bv_decode[..]) {
        Ok(b) => b,
        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::BitVecFailure)),
    };
    let vec = bitvec_decoded.into_vec();
    let mut out = String::from("[");
    for i in 0..vec.len()/2 {
        if i>0 {out.push_str(", ");}
        let print1 = BitVec::<O, u32>::from_vec(vec![vec[2*i]]).to_string();
        let print2 = BitVec::<O, u32>::from_vec(vec![vec[2*i+1]]).to_string();
        out.push_str(&format!("{}{}", &print1[1..print1.len()-1], &print2[1..print2.len()-1]));
    }
    out.push_str("]");
    Ok(out)
}

/*
fn decode_type_def_range (found_ty: &TypeDefRange<PortableForm>, data: Vec<u8>, meta_v14: &RuntimeMetadataV14, index: &mut u32, indent: u32) -> Result<DecodedOut, Error> {
    let inclusive_flag = found_ty.inclusive();
    let index_type_id = found_ty.index_type().id();
    let index_type = match meta_v14.types.resolve(index_type_id) {
        Some(a) => a,
        None => return Err(Error::UnableToDecode(UnableToDecode::V14TypeNotResolved)),
    };
    match index_type.type_def() {
         TypeDef::Primitive(a) => {
             match a {
                 TypeDefPrimitive::U8 => process_range::<u8> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::U16 => process_range::<u16> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::U32 => process_range::<u32> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::U64 => process_range::<u64> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::U128 => process_range::<u128> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::I8 => process_range::<i8> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::I16 => process_range::<i16> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::I32 => process_range::<i32> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::I64 => process_range::<i64> (data, index, indent, inclusive_flag),
                 TypeDefPrimitive::I128 => process_range::<i128> (data, index, indent, inclusive_flag),
                  _ => return Err(Error::UnableToDecode(UnableToDecode::NotRangeIndex)),
             }
         },
         _ => return Err(Error::UnableToDecode(UnableToDecode::NotRangeIndex)),
     }
}

fn process_range <T: PartialOrd + std::fmt::Display + Decode> (data: Vec<u8>, index: &mut u32, indent: u32, inclusive_flag: bool) -> Result<DecodedOut, Error> {
    let length = 2 * size_of::<T>();
    match data.get(..length) {
        Some(into_range) => {
            let remaining_vector = data[length..].to_vec();
            let fancy_out = {
                if inclusive_flag {
                    match <std::ops::RangeInclusive<T>>::decode(&mut &into_range[..]) {
                        Ok(range) => {format!(",{}", (Card::Range{start: range.start().to_string(), end: range.end().to_string(), inclusive: inclusive_flag}).card(index, indent))},
                        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::RangeFailure)),
                    }
                }
                else {
                    match <std::ops::Range<T>>::decode(&mut &into_range[..]) {
                        Ok(range) => {format!(",{}", (Card::Range{start: range.start.to_string(), end: range.end.to_string(), inclusive: inclusive_flag}).card(index, indent))},
                        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::RangeFailure)),
                    }
                }
            };
            Ok(DecodedOut{
                remaining_vector,
                indent,
                fancy_out,
            })
        },
        None => return Err(Error::UnableToDecode(UnableToDecode::DataTooShort)),
    }
}
*/
