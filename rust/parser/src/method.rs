//! Get method information from metadata [`RuntimeMetadataV12`] or
//! [`RuntimeMetadataV13`]
//!
//!

use frame_metadata::{
    decode_different::DecodeDifferent, v12::RuntimeMetadataV12, v13::RuntimeMetadataV13,
};

use definitions::error_signer::{ParserDecodingError, ParserError};

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

/// Enum to transfer around older metadata (V12 and V13)
pub enum OlderMeta<'a> {
    V12(&'a RuntimeMetadataV12),
    V13(&'a RuntimeMetadataV13),
}

/// Find method specified by pallet index and method index, for old metadata.
trait FindMethod {
    /// Search through metadata for pallet with given pallet index and then
    /// through the pallet for the method with given method index.
    ///
    /// Pallet index is explicitly recorded in network metadata as a number.
    ///
    /// Method index is ordinal number in vector of calls within pallet.
    ///
    /// Output [`MethodOld`] if successful.
    fn find_method(
        meta: &Self,
        pallet_index: u8,
        method_index: u8,
    ) -> Result<MethodOld, ParserError>;
}

/// Implement [`FindMethod`] for old metadata.
macro_rules! impl_find_method {
    ($($runtime_metadata: ty), *) => {
        $(
            impl FindMethod for $runtime_metadata {
                fn find_method(meta: &Self, pallet_index: u8, method_index: u8) -> Result<MethodOld, ParserError> {
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
                                            return Err(ParserError::Decoding(
                                                ParserDecodingError::MethodNotFound {
                                                    method_index,
                                                    pallet_name: name.to_string(),
                                                },
                                            ));
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
                                                            return Err(ParserError::Decoding(
                                                                ParserDecodingError::ArgumentTypeError,
                                                            ))
                                                        }
                                                    },
                                                    None => {
                                                        return Err(ParserError::Decoding(
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
                            None => Err(ParserError::Decoding(ParserDecodingError::MethodNotFound {
                                method_index,
                                pallet_name: x,
                            })),
                        },
                        None => Err(ParserError::Decoding(ParserDecodingError::PalletNotFound(
                            pallet_index,
                        ))),
                    }
                }
            }
        )*
    }
}

impl_find_method!(RuntimeMetadataV12, RuntimeMetadataV13);

/// Function to find method for current call for metadata in v12 or v13
/// Outputs NextDecode value.

pub(crate) fn what_next_old(
    data: &mut Vec<u8>,
    meta: &OlderMeta,
) -> Result<MethodOld, ParserError> {
    if data.len() < 2 {
        return Err(ParserError::Decoding(ParserDecodingError::DataTooShort));
    }
    let pallet_index = data[0];
    let method_index = data[1];
    *data = data[2..].to_vec();
    match meta {
        OlderMeta::V12(meta_v12) => {
            <RuntimeMetadataV12>::find_method(meta_v12, pallet_index, method_index)
        }
        OlderMeta::V13(meta_v13) => {
            <RuntimeMetadataV13>::find_method(meta_v13, pallet_index, method_index)
        }
    }
}
