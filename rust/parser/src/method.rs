use frame_metadata::{
    decode_different::DecodeDifferent, v12::RuntimeMetadataV12, v13::RuntimeMetadataV13,
};

use crate::error::{Error, ParserDecodingError, Result};

/// Struct to store the method information
pub(crate) struct MethodOld {
    pub(crate) pallet_name: String,
    pub(crate) method_name: String,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) docs: String,
}

/// Struct to store the argument name and type
pub(crate) struct Argument {
    pub(crate) name: String,
    pub(crate) ty: String,
}

/// Struct to store current method and remaining data
pub(crate) struct NextDecodeOld {
    pub(crate) method: MethodOld,
    pub(crate) data: Vec<u8>,
}

/// Enum to transfer around older metadata (`V12` and `V13`)
pub enum OlderMeta<'a> {
    V12(&'a RuntimeMetadataV12),
    V13(&'a RuntimeMetadataV13),
}

/// Function to search through metadata version `V12` for method with given pallet index and method index,
/// in case of success outputs Method value.
/// Pallet index is explicitly recorded in network metadata as a number.
/// Method index is ordinal number in vector of calls within pallet.
fn find_method_v12(
    pallet_index: u8,
    method_index: u8,
    meta: &RuntimeMetadataV12,
) -> Result<MethodOld> {
    let mut found_pallet_name = None;
    let mut found_method_name = None;
    let mut docs = String::new();
    let mut arguments: Vec<Argument> = Vec::new();

    if let DecodeDifferent::Decoded(meta_vector) = &meta.modules {
        for y in meta_vector.iter() {
            if y.index == pallet_index {
                if let DecodeDifferent::Decoded(name) = &y.name {
                    found_pallet_name = Some(name.to_string());
                    if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                        if calls.len() <= method_index.into() {
                            return Err(Error::Decoding(ParserDecodingError::MethodNotFound {
                                method_index,
                                pallet_name: name.to_string(),
                            }));
                        }
                        if let DecodeDifferent::Decoded(nm) = &calls[method_index as usize].name {
                            found_method_name = Some(nm.to_string());
                        }
                        if let DecodeDifferent::Decoded(docs_found) =
                            &calls[method_index as usize].documentation
                        {
                            for (i, a) in docs_found.iter().enumerate() {
                                if i > 0 {
                                    docs.push('\n');
                                }
                                docs.push_str(a);
                            }
                        }
                        if let DecodeDifferent::Decoded(args) =
                            &calls[method_index as usize].arguments
                        {
                            for a in args.iter() {
                                let mut name_a = None;
                                let mut ty_a = None;
                                if let DecodeDifferent::Decoded(b) = &a.name {
                                    name_a = Some(b.to_string())
                                }
                                if let DecodeDifferent::Decoded(c) = &a.ty {
                                    ty_a = Some(c.to_string())
                                }
                                match name_a {
                                    Some(x) => match ty_a {
                                        Some(y) => {
                                            arguments.push(Argument { name: x, ty: y });
                                        }
                                        None => {
                                            return Err(Error::Decoding(
                                                ParserDecodingError::ArgumentTypeError,
                                            ))
                                        }
                                    },
                                    None => {
                                        return Err(Error::Decoding(
                                            ParserDecodingError::ArgumentNameError,
                                        ))
                                    }
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
        Some(x) => match found_method_name {
            Some(y) => {
                let out = MethodOld {
                    pallet_name: x,
                    method_name: y,
                    arguments,
                    docs,
                };
                Ok(out)
            }
            None => Err(Error::Decoding(ParserDecodingError::MethodNotFound {
                method_index,
                pallet_name: x,
            })),
        },
        None => Err(Error::Decoding(ParserDecodingError::PalletNotFound(
            pallet_index,
        ))),
    }
}

/// Function to search through metadata version `V13` for method with given pallet index and method index,
/// in case of success outputs Method value.
/// Pallet index is explicitly recorded in network metadata as a number.
/// Method index is ordinal number in vector of calls within pallet.
fn find_method_v13(
    pallet_index: u8,
    method_index: u8,
    meta: &RuntimeMetadataV13,
) -> Result<MethodOld> {
    let mut found_pallet_name = None;
    let mut found_method_name = None;
    let mut docs = String::new();
    let mut arguments: Vec<Argument> = Vec::new();

    if let DecodeDifferent::Decoded(meta_vector) = &meta.modules {
        for y in meta_vector.iter() {
            if y.index == pallet_index {
                if let DecodeDifferent::Decoded(name) = &y.name {
                    found_pallet_name = Some(name.to_string());
                    if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                        if calls.len() <= method_index.into() {
                            return Err(Error::Decoding(ParserDecodingError::MethodNotFound {
                                method_index,
                                pallet_name: name.to_string(),
                            }));
                        }
                        if let DecodeDifferent::Decoded(nm) = &calls[method_index as usize].name {
                            found_method_name = Some(nm.to_string());
                        }
                        if let DecodeDifferent::Decoded(docs_found) =
                            &calls[method_index as usize].documentation
                        {
                            for (i, a) in docs_found.iter().enumerate() {
                                if i > 0 {
                                    docs.push('\n');
                                }
                                docs.push_str(a);
                            }
                        }
                        if let DecodeDifferent::Decoded(args) =
                            &calls[method_index as usize].arguments
                        {
                            for a in args.iter() {
                                let mut name_a = None;
                                let mut ty_a = None;
                                if let DecodeDifferent::Decoded(b) = &a.name {
                                    name_a = Some(b.to_string())
                                }
                                if let DecodeDifferent::Decoded(c) = &a.ty {
                                    ty_a = Some(c.to_string())
                                }
                                match name_a {
                                    Some(x) => match ty_a {
                                        Some(y) => {
                                            arguments.push(Argument { name: x, ty: y });
                                        }
                                        None => {
                                            return Err(Error::Decoding(
                                                ParserDecodingError::ArgumentTypeError,
                                            ))
                                        }
                                    },
                                    None => {
                                        return Err(Error::Decoding(
                                            ParserDecodingError::ArgumentNameError,
                                        ))
                                    }
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
        Some(x) => match found_method_name {
            Some(y) => {
                let out = MethodOld {
                    pallet_name: x,
                    method_name: y,
                    arguments,
                    docs,
                };
                Ok(out)
            }
            None => Err(Error::Decoding(ParserDecodingError::MethodNotFound {
                method_index,
                pallet_name: x,
            })),
        },
        None => Err(Error::Decoding(ParserDecodingError::PalletNotFound(
            pallet_index,
        ))),
    }
}

/// Function to find method for current call for metadata in `V12` or `V13`
/// Outputs `NextDecode` value.
pub(crate) fn what_next_old(data: Vec<u8>, meta: &OlderMeta) -> Result<NextDecodeOld> {
    if data.len() < 2 {
        return Err(Error::Decoding(ParserDecodingError::DataTooShort));
    }
    let pallet_index = data[0];
    let method_index = data[1];
    let method = match meta {
        OlderMeta::V12(meta_v12) => find_method_v12(pallet_index, method_index, meta_v12)?,
        OlderMeta::V13(meta_v13) => find_method_v13(pallet_index, method_index, meta_v13)?,
    };
    Ok(NextDecodeOld {
        method,
        data: data[2..].to_vec(),
    })
}
