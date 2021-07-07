use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};
use super::error::{Error, UnableToDecode};

/// Struct to store the method information
pub struct Method {
    pub pallet_name: String,
    pub method_name: String,
    pub arguments: Vec<Argument>
}

/// Struct to store the argument name and type
pub struct Argument {
    pub name: String,
    pub ty: String,
}

/// Struct to store current method and remaining data
pub struct NextDecode {
    pub method: Method,
    pub data: Vec<u8>,
}

/// Function to search through metadata for method with given pallet index and method index,
/// in case of success outputs Method value.
/// Pallet index is explicitly recorded in network metadata as a number.
/// Method index is ordinal number in vector of calls within pallet.

pub fn find_method (pallet_index: u8, method_index: u8, meta: &RuntimeMetadataV12) -> Result<Method, Error> {
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
                    if calls.len() <= method_index.into() {return Err(Error::UnableToDecode(UnableToDecode::MethodIndexTooHigh{method_index, pallet_index, total: calls.len()}));}
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
                                            None => {return Err(Error::UnableToDecode(UnableToDecode::ArgumentTypeError))},
                                        }
                                    },
                                    None => {return Err(Error::UnableToDecode(UnableToDecode::ArgumentNameError))},
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
                None => return Err(Error::UnableToDecode(UnableToDecode::MethodNotFound{method_index, pallet_name: x})),
            }
        },
        None => return Err(Error::UnableToDecode(UnableToDecode::PalletNotFound(pallet_index))),
    }
}

/// Function to find method for current call.
/// Outputs NextDecode value.

pub fn what_next (data: Vec<u8>, meta: &RuntimeMetadataV12) -> Result<NextDecode, Error> {
    if data.len() < 2 {return Err(Error::UnableToDecode(UnableToDecode::NeedPalletAndMethod));}
    let pallet_index = data[0];
    let method_index = data[1];
    let new_method = find_method(pallet_index, method_index, meta)?;
    Ok(NextDecode{
        method: new_method,
        data: data[2..].to_vec(),
    })
}

