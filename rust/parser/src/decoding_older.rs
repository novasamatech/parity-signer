use bitvec::prelude::{BitVec, Lsb0};
use lazy_static::lazy_static;
use regex::Regex;
use sp_arithmetic::{PerU16, Perbill, Percent};

use definitions::{
    network_specs::ShortSpecs,
    types::{Description, EnumVariant, EnumVariantType, StructField, TypeEntry},
};

use crate::cards::ParserCard;
use crate::decoding_commons::{
    decode_known_length, decode_perthing, decode_primitive_with_flags, get_compact,
    special_case_account_id, DecodedOut, OutputCard,
};
use crate::error::{Error, ParserDecodingError, Result};
use crate::method::{what_next_old, OlderMeta};

/// Function to decode primitive types (fixed-width or compact form), and `Percent`,
/// `Permill`, and `PerU16` structs (also fixed-width or compact form).
/// All those types have stable length by `std::mem::size_of()` and also are serializeable.
///
/// The function decodes only found_ty, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - `found_ty` (type of the argument found in the previous iteration, to be interpreted on run)
/// - `data` (remaining `Vec<u8>` of data),
/// - `indent` used for creating properly formatted js cards.
///
/// The function outputs the `DecodedOut` value in case of success.
fn decode_primitive(
    found_ty: &str,
    data: &[u8],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Option<DecodedOut> {
    match found_ty {
        "bool" => decode_known_length::<bool>(data, found_ty, indent).ok(),
        "u8" => decode_primitive_with_flags::<u8>(
            data,
            &mut None,
            false,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "u16" => decode_primitive_with_flags::<u16>(
            data,
            &mut None,
            false,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "u32" => decode_primitive_with_flags::<u32>(
            data,
            &mut None,
            false,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "u64" => decode_primitive_with_flags::<u64>(
            data,
            &mut None,
            false,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "u128" => decode_primitive_with_flags::<u128>(
            data,
            &mut None,
            false,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Percent" => {
            decode_perthing::<Percent>(data, false, found_ty, indent).ok()
        }
        "Perbill" => {
            decode_perthing::<Perbill>(data, false, found_ty, indent).ok()
        }
        "PerU16" => {
            decode_perthing::<PerU16>(data, false, found_ty, indent).ok()
        }
        "Compact<u8>" => decode_primitive_with_flags::<u8>(
            data,
            &mut None,
            true,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Compact<u16>" => decode_primitive_with_flags::<u16>(
            data,
            &mut None,
            true,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Compact<u32>" => decode_primitive_with_flags::<u32>(
            data,
            &mut None,
            true,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Compact<u64>" => decode_primitive_with_flags::<u64>(
            data,
            &mut None,
            true,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Compact<u128>" => decode_primitive_with_flags::<u128>(
            data,
            &mut None,
            true,
            false,
            found_ty,
            indent,
            short_specs,
        ).ok(),
        "Compact<Percent>" => {
            decode_perthing::<Percent>(data, true, found_ty, indent).ok()
        }
        "Compact<Perbill>" => {
            decode_perthing::<Perbill>(data, true, found_ty, indent).ok()
        }
        "Compact<PerU16>" => {
            decode_perthing::<PerU16>(data, true, found_ty, indent).ok()
        }
        _ => None,
    }
}

/// Function to decode any type, including calls and vectors of calls.
/// Here starts the decoding of argument with type found_ty.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only found_ty, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - `found_ty` (type of the argument found in the previous iteration, to be interpreted on run)
/// - `data` (remaining `Vec<u8>` of data),
/// - `meta` (metadata for the network used),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - `indent` used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// Calls and vectors of calls are treated separately here.
/// All simpler types are processed through `decode_simple` function.
fn decode_complex(
    found_ty: &str,
    mut data: Vec<u8>,
    meta: &OlderMeta,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    match found_ty {
        "Box<<T as Config<I>>::Proposal>"
        | "Box<<T as Config>::Call>"
        | "Box<<T as Config>::Proposal>" => {
            process_as_call(data, meta, type_database, indent, short_specs)
        }
        "Vec<<T as Config>::Call>" => {
            let pre_vector = get_compact::<u32>(&data)?;
            let number_of_calls = pre_vector.compact_found;
            let mut fancy_output_prep: Vec<OutputCard> = Vec::new();
            match pre_vector.start_next_unit {
                Some(start) => {
                    if data.len() < start + 2 * (number_of_calls as usize) {
                        return Err(Error::Decoding(ParserDecodingError::DataTooShort));
                    }
                    data = data[start..].to_vec();
                    for _i in 0..number_of_calls {
                        let after_run =
                            process_as_call(data, meta, type_database, indent, short_specs)?;
                        fancy_output_prep.extend_from_slice(&after_run.fancy_out);
                        data = after_run.remaining_vector;
                    }
                }
                None => {
                    if number_of_calls != 0 {
                        return Err(Error::Decoding(ParserDecodingError::DataTooShort));
                    }
                }
            }
            Ok(DecodedOut {
                remaining_vector: data,
                fancy_out: fancy_output_prep,
            })
        }
        _ => decode_simple(found_ty, data, type_database, indent, short_specs),
    }
}

/// Function to process a single call (essentially, `Vec<u8>`).
/// This is the place to start when decoding the transaction.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single call, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - `data` (call itself),
/// - `meta` (metadata for the network used),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - `indent` used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// In each encoded call the first two elements denote the numbers of the pallet and the method
/// in corresponding network metadata, indicating what arguments the call has.
/// The card "call" containing pallet name and method name is added to `fancy_out`.
/// Each argument is then processed in a sequence, the name of the argument
/// and the type of the argument are found in the network metadata during the run.
/// For each argument the card "varname" with argument name is added to `fancy_out`,
/// followed by card(s) of actual decoded argument values.
pub(crate) fn process_as_call(
    mut data: Vec<u8>,
    meta: &OlderMeta,
    type_database: &[TypeEntry],
    mut indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    let call_in_processing = what_next_old(data, meta)?;
    data = call_in_processing.data;

    let mut fancy_out = vec![
        OutputCard {
            card: ParserCard::Pallet(call_in_processing.method.pallet_name),
            indent,
        },
        OutputCard {
            card: ParserCard::Method {
                method_name: call_in_processing.method.method_name,
                docs: call_in_processing.method.docs,
            },
            indent: indent + 1,
        },
    ];
    indent += 2;

    for x in call_in_processing.method.arguments.iter() {
        fancy_out.push(OutputCard {
            card: ParserCard::Varname(x.name.to_string()),
            indent,
        });

        let decoded_out =
            decode_complex(&x.ty, data, meta, type_database, indent + 1, short_specs)?;
        data = decoded_out.remaining_vector;
        fancy_out.extend_from_slice(&decoded_out.fancy_out);
    }

    Ok(DecodedOut {
        remaining_vector: data.to_vec(),
        fancy_out,
    })
}

// Making regular expressions for analysing any type encountered except calls and Vec<calls>.
// The type to be parsed is found in network metadata on the run.
// In some cases the type could be `Option<arg>`, `Vec<arg>`, tuple such as (arg1, arg2, arg3)
// (currently among types are found tuples of up to 4 elements, could change at any point),
// array such as [arg; num], and compact such as Compact<arg>.
// To reduce the number of types in `type_database` and to simplify and
// at least a bit generalize the decoding process,
// the options, vectors, tuples, arrays, and compacts are currently treated specially
// and first are "husked" to interpret the inner type.
// This regular expressions are used for just that: find the inner type.
// All those are constructed statically, and pre-tested.
lazy_static! {
    static ref REGOPTION: Regex = Regex::new(r#"(?m)^Option<(?P<arg>.*)>"#).expect("constructed from checked static value");
    static ref REGVECTOR: Regex = Regex::new(r#"(?m)^Vec<(?P<arg>.*)>"#).expect("constructed from checked static value");
    static ref REGTUPLE: Regex = Regex::new(r"(?m)^\((?P<arg1>([^( ]*(\([^)]*\)[^) ]*))|[^(),]*), (?P<arg2>([^( ]*\([^)]*\)[^) ]*)|[^(),]*)(, (?P<arg3>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?(, (?P<arg4>([^( ]*\([^)]*\)[^) ]*)|[^(),]*))?\)").expect("constructed from checked static value");
    static ref REGARRAY: Regex = Regex::new(r"(?i)(?m)^\[(?P<arg>[a-z0-9]+); (?P<num>[0-9]+)\]").expect("constructed from checked static value");
    static ref REGCOMPACT: Regex = Regex::new(r#"(?m)^Compact<(?P<arg>.*)>"#).expect("constructed from checked static value");
}

/// Function to decode `Option<inner_ty>`.
/// Special case of Option<bool> is considered separately.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single option, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - `inner_ty` (type inside Option, found using regular expressions)
/// - `data` (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - `indent` used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// Js cards are of type "none" if the Option<_> is None.
/// At this moment no special js card for Some(x) is presented, only the card of x itself.
fn deal_with_option(
    inner_ty: &str,
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    if inner_ty == "bool" {
        let fancy_out = match &data[0] {
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
                return Err(Error::Decoding(
                    ParserDecodingError::UnexpectedOptionVariant,
                ))
            }
        };
        let remaining_vector = {
            if data.len() > 1 {
                data[1..].to_vec()
            } else {
                Vec::new()
            }
        };
        Ok(DecodedOut {
            remaining_vector,
            fancy_out,
        })
    } else {
        match &data[0] {
            0 => {
                let remaining_vector = {
                    if data.len() > 1 {
                        data[1..].to_vec()
                    } else {
                        Vec::new()
                    }
                };
                let fancy_out = vec![OutputCard {
                    card: ParserCard::None,
                    indent,
                }];
                Ok(DecodedOut {
                    remaining_vector,
                    fancy_out,
                })
            }
            1 => {
                if data.len() == 1 {
                    return Err(Error::Decoding(ParserDecodingError::DataTooShort));
                }
                data = data[1..].to_vec();
                decode_simple(inner_ty, data, type_database, indent, short_specs)
            }
            _ => Err(Error::Decoding(
                ParserDecodingError::UnexpectedOptionVariant,
            )),
        }
    }
}

/// Function to decode `Vector<inner_ty>`.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single vector of type `inner_ty`, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// Encoded vectors are preluded by the number of vector elements as compact,
/// followed by concatenated individually encoded elements.
/// So, the function first searches for compact to determine the number of elements
/// and then proceeds to decode each element of the vector.
///
/// The function takes as arguments
/// - `inner_ty` (type inside `Vec`, found using regular expressions)
/// - `data` (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - `indent` used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
fn deal_with_vector(
    inner_ty: &str,
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    let pre_vector = get_compact::<u32>(&data)?;
    let mut fancy_output_prep: Vec<OutputCard> = Vec::new();
    let elements_of_vector = pre_vector.compact_found;
    match pre_vector.start_next_unit {
        Some(start) => {
            data = data[start..].to_vec();
            for _i in 0..elements_of_vector {
                let after_run = decode_simple(inner_ty, data, type_database, indent, short_specs)?;
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
                Err(Error::Decoding(ParserDecodingError::DataTooShort))
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

/// Function to decode an array such as `[inner_ty; number_of_elements]`.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only a single array of type `inner_ty` and length `number_of_elements`,
/// removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - `inner_ty` (type of array elements, found using regular expressions)
/// - `number_of_elements` (number of array elements, found using regular expressions)
/// - `data` (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - `indent` used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
fn deal_with_array(
    inner_ty: &str,
    number_of_elements: u32,
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    let mut fancy_output_prep: Vec<OutputCard> = Vec::new();
    for _i in 0..number_of_elements {
        let after_run = decode_simple(inner_ty, data, type_database, indent, short_specs)?;
        fancy_output_prep.extend_from_slice(&after_run.fancy_out);
        data = after_run.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data,
        fancy_out: fancy_output_prep,
    })
}

/// Function to decode IdentityFields special case.
/// `IdentityFields` is a struct from `pallet_identity::IdentityFields`,
/// which is wrapper type for `BitFlags<IdentityField>`.
/// To avoid output complications arising from private non-printable fields
/// the type was re-implemented here using `BitVec` and enum `IdentityField` from `type_database`.
///
/// The function decodes only a single `IdentityFields` type entry,
/// removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - indent used for creating properly formatted js cards.
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// For each identity field an individual js card `identity_field` is added to `fancy_out`.
fn special_case_identity_fields(
    data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
) -> Result<DecodedOut> {
    // at the moment, the length is known: 8 units in `Vec<u8>`
    if data.len() < 8 {
        return Err(Error::Decoding(ParserDecodingError::DataTooShort));
    }
    let remaining_vector = {
        if data.len() > 8 {
            data[8..].to_vec()
        } else {
            Vec::new()
        }
    };
    let into_bv = data[..8].to_vec();
    // make correct `BitVec`
    let bv: BitVec<u8, Lsb0> = BitVec::from_vec(into_bv);
    let mut found = false;
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for x in type_database.iter() {
        if x.name == "IdentityField" {
            if let Description::Enum(v1) = &x.description {
                found = true;
                for (i, x) in v1.iter().enumerate() {
                    if bv[i] {
                        fancy_out.push(OutputCard {
                            card: ParserCard::IdentityField(x.variant_name.to_string()),
                            indent,
                        });
                    };
                }
            }
            break;
        }
    }
    if !found {
        return Err(Error::Decoding(ParserDecodingError::IdFields));
    }
    Ok(DecodedOut {
        remaining_vector,
        fancy_out,
    })
}

/// Function to decode `BitVec` special case.
///
/// The function decodes only a single `BitVec` type entry,
/// removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// Trait Decode is not implemented for `BitVec` type.
/// Existing signer documentation in js suggests that the encoded `BitVec` is preluded by the number
/// of `BitVec` elements as compact, and each 8 of those form an `u8` element in input data.
/// So, the function first searches for compact to determine the number of `BitVec` elements
/// then calculates how many actual `u8` elements are used to store those, takes needed length of data
/// and gets `BitVec` from it.
///
/// The function takes as arguments
/// - data (remaining `Vec<u8>` of data),
/// - indent used for creating properly formatted js cards.
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// Resulting `BitVec` is added to `fancy_out` on js card `bitvec`.
fn special_case_bitvec(data: Vec<u8>, indent: u32) -> Result<DecodedOut> {
    // the data is preluded by compact indicating the number of `BitVec` elements - info from js documentation, decode not implemented for `BitVec` as is
    let pre_bitvec = get_compact::<u32>(&data)?;
    let actual_length = match pre_bitvec.compact_found % 8 {
        0 => pre_bitvec.compact_found / 8,
        _ => (pre_bitvec.compact_found / 8) + 1,
    };
    match pre_bitvec.start_next_unit {
        Some(start) => {
            let fin = start + (actual_length as usize);
            if data.len() < fin {
                return Err(Error::Decoding(ParserDecodingError::DataTooShort));
            }
            let into_bv = data[start..fin].to_vec();
            let bv: BitVec<u8, Lsb0> = BitVec::from_vec(into_bv);
            let fancy_out = vec![OutputCard {
                card: ParserCard::BitVec(bv.to_string()),
                indent,
            }];
            let remaining_vector = {
                if data.len() > fin {
                    data[fin..].to_vec()
                } else {
                    Vec::new()
                }
            };
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        }
        None => {
            if actual_length != 0 {
                return Err(Error::Decoding(ParserDecodingError::DataTooShort));
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

/// Function to determine if special case balance is applicable
fn goto_balance(found_ty: &str) -> bool {
    (found_ty == "Balance")
        || (found_ty == "T::Balance")
        || (found_ty == "BalanceOf<T>")
        || (found_ty == "BalanceOf<T, I>")
        || (found_ty == "Compact<Balance>")
        || (found_ty == "Compact<T::Balance>")
        || (found_ty == "Compact<BalanceOf<T>>")
        || (found_ty == "Compact<BalanceOf<T, I>>")
}

/// Function to decode and represent properly any special case related to balances
/// (both compacts and non-compacts).
///
/// The function decodes only a single balance-related type entry,
/// removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - data (remaining `Vec<u8>` of data),
/// - indent used for creating properly formatted js cards.
/// - `short_specs` (taking currency units and decimals from there).
///
/// The function outputs the `DecodedOut` value in case of success.
///
/// Resulting balance is added to `fancy_out` on js card "balance".
fn special_case_balance(
    found_ty: &str,
    data: Vec<u8>,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    match found_ty {
        "Balance" | "T::Balance" | "BalanceOf<T>" | "BalanceOf<T, I>" => {
            decode_primitive_with_flags::<u128>(
                &data,
                &mut None,
                false,
                true,
                "u128",
                indent,
                short_specs,
            )
        }
        "Compact<Balance>"
        | "Compact<T::Balance>"
        | "Compact<BalanceOf<T>>"
        | "Compact<BalanceOf<T, I>>" => decode_primitive_with_flags::<u128>(
            &data,
            &mut None,
            true,
            true,
            "u128",
            indent,
            short_specs,
        ),
        _ => Err(Error::Decoding(ParserDecodingError::BalanceNotDescribed)),
    }
}

/// Function to decode structs described in `type_database`.
///
/// Function iterates over struct fields, for each field it creates js card `field_name`
/// or `field_number` (if there are no field names),
/// and card(s) for corresponding content in `fancy_out`.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
///
/// The function takes as arguments
/// - vector of `StructField` of currently processed type, as found in `type_database`
/// - data (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - indent used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
fn deal_with_struct(
    v1: &[StructField],
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for (i, y) in v1.iter().enumerate() {
        let fancy_output_prep = match &y.field_name {
            Some(z) => OutputCard {
                card: ParserCard::FieldName {
                    name: z.to_string(),
                    docs_field_name: String::new(),
                    path_type: String::new(),
                    docs_type: String::new(),
                },
                indent,
            },
            None => OutputCard {
                card: ParserCard::FieldNumber {
                    number: i,
                    docs_field_number: String::new(),
                    path_type: String::new(),
                    docs_type: String::new(),
                },
                indent,
            },
        };
        fancy_out.push(fancy_output_prep);
        let after_run = decode_simple(&y.field_type, data, type_database, indent + 1, short_specs)?;
        data = after_run.remaining_vector;
        fancy_out.extend_from_slice(&after_run.fancy_out);
    }
    Ok(DecodedOut {
        remaining_vector: data,
        fancy_out,
    })
}

/// Function to decode enums described in `type_database`.
///
/// Function determines which enum variant is used, and adds to `fancy_out` card `enum_variant_name`,
/// and, if there is content associated with enum variant, card(s) for this content.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
///
/// The function takes as arguments
/// - vector of `EnumVariant` of currently processed type, as found in `type_database`
/// - data (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - indent used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
fn deal_with_enum(
    v1: &[EnumVariant],
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    let enum_index = data[0] as usize;
    if enum_index >= v1.len() {
        return Err(Error::Decoding(ParserDecodingError::UnexpectedEnumVariant));
    }
    let found_variant = &v1[enum_index];
    match &found_variant.variant_type {
        EnumVariantType::None => {
            let remaining_vector = {
                if data.len() > 1 {
                    data[1..].to_vec()
                } else {
                    Vec::new()
                }
            };
            let fancy_out = vec![OutputCard {
                card: ParserCard::EnumVariantName {
                    name: found_variant.variant_name.to_string(),
                    docs_enum_variant: String::new(),
                },
                indent,
            }];
            Ok(DecodedOut {
                remaining_vector,
                fancy_out,
            })
        }
        EnumVariantType::Type(inner_ty) => {
            if data.len() == 1 {
                return Err(Error::Decoding(ParserDecodingError::DataTooShort));
            }
            data = data[1..].to_vec();
            let mut fancy_output_prep = vec![OutputCard {
                card: ParserCard::EnumVariantName {
                    name: found_variant.variant_name.to_string(),
                    docs_enum_variant: String::new(),
                },
                indent,
            }];
            let after_run = decode_simple(inner_ty, data, type_database, indent + 1, short_specs)?;
            fancy_output_prep.extend_from_slice(&after_run.fancy_out);
            data = after_run.remaining_vector;
            Ok(DecodedOut {
                remaining_vector: data,
                fancy_out: fancy_output_prep,
            })
        }
        EnumVariantType::Struct(v2) => {
            if data.len() == 1 {
                return Err(Error::Decoding(ParserDecodingError::DataTooShort));
            }
            data = data[1..].to_vec();
            let mut fancy_out: Vec<OutputCard> = Vec::new();
            for (i, y) in v2.iter().enumerate() {
                let fancy_output_prep = match &y.field_name {
                    Some(z) => OutputCard {
                        card: ParserCard::FieldName {
                            name: z.to_string(),
                            docs_field_name: String::new(),
                            path_type: String::new(),
                            docs_type: String::new(),
                        },
                        indent,
                    },
                    None => OutputCard {
                        card: ParserCard::FieldNumber {
                            number: i,
                            docs_field_number: String::new(),
                            path_type: String::new(),
                            docs_type: String::new(),
                        },
                        indent,
                    },
                };
                fancy_out.push(fancy_output_prep);
                let after_run =
                    decode_simple(&y.field_type, data, type_database, indent + 1, short_specs)?;
                data = after_run.remaining_vector;
                fancy_out.extend_from_slice(&after_run.fancy_out);
            }
            Ok(DecodedOut {
                remaining_vector: data,
                fancy_out,
            })
        }
    }
}

/// Function to decode any type, except calls.
///
/// This function is recursive, i.e. it could call itself later if needed with changed input data.
/// The function decodes only found_ty, removes already decoded part of input data `Vec<u8>`,
/// and returns whatever remains as `DecodedOut` field `remaining_vector`, which is processed later separately.
///
/// The function takes as arguments
/// - found_ty (type of the argument found in the previous iteration, to be interpreted on run)
/// - data (remaining `Vec<u8>` of data),
/// - `type_database` (it describes all fundamental types that could be encountered in known networks
/// and are not primitive types (i.e. types decoded by `decode_primitive` function), this database
/// currently is retrieved and decoded from the database on device used),
/// - indent used for creating properly formatted js cards,
/// - `short_specs` (network parameters, such as base58 prefix, currency units and decimals),
/// all those are used in some cases for proper output formatting).
///
/// The function outputs the `DecodedOut` value in case of success.
fn decode_simple(
    found_ty: &str,
    mut data: Vec<u8>,
    type_database: &[TypeEntry],
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut> {
    if data.is_empty() {
        return Err(Error::Decoding(ParserDecodingError::DataTooShort));
    }
    match decode_primitive(found_ty, &data, indent, short_specs) {
        Some(a) => Ok(a),
        None => {
            // check for option
            match REGOPTION.captures(found_ty) {
                Some(caps) => {
                    deal_with_option(&caps["arg"], data, type_database, indent, short_specs)
                }
                None => {
                    // check for vector
                    match REGVECTOR.captures(found_ty) {
                        Some(caps) => {
                            deal_with_vector(&caps["arg"], data, type_database, indent, short_specs)
                        }
                        None => {
                            // check for tuples
                            match REGTUPLE.captures(found_ty) {
                                Some(caps) => {
                                    let mut fancy_out: Vec<OutputCard> = Vec::new();
                                    let mut i = 1;
                                    loop {
                                        let capture_name = format!("arg{i}");
                                        match caps.name(&capture_name) {
                                            Some(x) => {
                                                fancy_out.push(OutputCard {
                                                    card: ParserCard::FieldNumber {
                                                        number: i,
                                                        docs_field_number: String::new(),
                                                        path_type: String::new(),
                                                        docs_type: String::new(),
                                                    },
                                                    indent,
                                                });
                                                let inner_ty = x.as_str();
                                                let after_run = decode_simple(
                                                    inner_ty,
                                                    data,
                                                    type_database,
                                                    indent + 1,
                                                    short_specs,
                                                )?;
                                                fancy_out.extend_from_slice(&after_run.fancy_out);
                                                data = after_run.remaining_vector;
                                            }
                                            None => break,
                                        }
                                        i += 1;
                                    }
                                    Ok(DecodedOut {
                                        remaining_vector: data,
                                        fancy_out,
                                    })
                                }
                                None => {
                                    // check for array
                                    match REGARRAY.captures(found_ty) {
                                        Some(caps) => {
                                            let inner_ty = &caps["arg"];
                                            let number_of_elements: u32 = caps["num"]
                                                .parse()
                                                .expect("Should have captured a number.");
                                            deal_with_array(
                                                inner_ty,
                                                number_of_elements,
                                                data,
                                                type_database,
                                                indent,
                                                short_specs,
                                            )
                                        }
                                        None => {
                                            // check for special case of Balance as is or in Compact form
                                            if goto_balance(found_ty) {
                                                special_case_balance(
                                                    found_ty,
                                                    data,
                                                    indent,
                                                    short_specs,
                                                )
                                            } else {
                                                // check for compact and find the alias for compact
                                                match REGCOMPACT.captures(found_ty) {
                                                    Some(caps) => {
                                                        let inner_ty = &caps["arg"];
                                                        let mut new_inner_ty = None;
                                                        for x in type_database.iter() {
                                                            if x.name == inner_ty {
                                                                new_inner_ty = match &x.description {
                                                                    Description::Type(a) => Some(a),
                                                                    _ => return Err(Error::Decoding(ParserDecodingError::UnexpectedCompactInsides)),
                                                                };
                                                                break;
                                                            }
                                                        }
                                                        match new_inner_ty {
                                                            Some(a) => {
                                                                let new_ty = found_ty.replace(inner_ty, a);
                                                                decode_simple(&new_ty, data, type_database, indent, short_specs)
                                                            },
                                                            None => Err(Error::Decoding(ParserDecodingError::UnexpectedCompactInsides)),
                                                        }
                                                    }
                                                    None => {
                                                        // special case of IdentityFields type (from pallet_identity): uses BitFlags, private fields
                                                        if found_ty == "IdentityFields" {
                                                            special_case_identity_fields(
                                                                data,
                                                                type_database,
                                                                indent,
                                                            )
                                                        } else {
                                                            // special case of `BitVec` type
                                                            if found_ty == "BitVec" {
                                                                special_case_bitvec(data, indent)
                                                            } else {
                                                                // special case of AccountId type
                                                                if (found_ty == "AccountId")
                                                                    || (found_ty == "T::AccountId")
                                                                {
                                                                    special_case_account_id(
                                                                        data,
                                                                        indent,
                                                                        short_specs,
                                                                    )
                                                                } else {
                                                                    // do a search through the type database
                                                                    let mut found_solution = None;
                                                                    for x in type_database.iter() {
                                                                        if x.name == found_ty {
                                                                            let wrap_me = match &x.description {
                                                                                Description::Type(inner_ty) => {decode_simple(inner_ty, data, type_database, indent, short_specs)?},
                                                                                Description::Enum(v1) => {deal_with_enum(v1, data, type_database, indent, short_specs)?},
                                                                                Description::Struct(v1) => {deal_with_struct(v1, data, type_database, indent, short_specs)?},
                                                                            };
                                                                            found_solution =
                                                                                Some(wrap_me);
                                                                            break;
                                                                        }
                                                                    }
                                                                    match found_solution {
                                                                        Some(x) => Ok(x),
                                                                        None => Err(Error::Decoding(ParserDecodingError::UnknownType(found_ty.to_string()))),
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
