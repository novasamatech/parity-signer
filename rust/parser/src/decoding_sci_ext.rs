use definitions::{
    error::{ParserDecodingError, ParserError},
    network_specs::ShortSpecs,
};
use frame_metadata::v14::RuntimeMetadataV14;
use parity_scale_codec::Decode;
use scale_info::{form::PortableForm, Type};
use sp_runtime::generic::Era;

use crate::cards::ParserCard;
use crate::decoding_commons::{DecodedOut, OutputCard};
use crate::decoding_sci::{decoding_sci_complete, CallExpectation};

pub(crate) fn decode_ext_attempt(
    data: &Vec<u8>,
    ext: &mut Ext,
    meta_v14: &RuntimeMetadataV14,
    indent: u32,
    short_specs: &ShortSpecs,
) -> Result<DecodedOut, ParserError> {
    let mut data = data.to_vec();
    let mut fancy_out: Vec<OutputCard> = Vec::new();
    for x in meta_v14.extrinsic.signed_extensions.iter() {
        ext.identifier = x.identifier.to_string();
        let current_type = match meta_v14.types.resolve(x.ty.id()) {
            Some(a) => a,
            None => {
                return Err(ParserError::Decoding(
                    ParserDecodingError::V14TypeNotResolved,
                ))
            }
        };
        let decoded_out = decoding_sci_complete(
            current_type,
            &mut Some(ext),
            false,
            false,
            &CallExpectation::None,
            data,
            meta_v14,
            indent,
            short_specs,
        )?;
        fancy_out.extend_from_slice(&decoded_out.fancy_out);
        data = decoded_out.remaining_vector;
    }
    for x in meta_v14.extrinsic.signed_extensions.iter() {
        ext.identifier = x.identifier.to_string();
        let current_type = match meta_v14.types.resolve(x.additional_signed.id()) {
            Some(a) => a,
            None => {
                return Err(ParserError::Decoding(
                    ParserDecodingError::V14TypeNotResolved,
                ))
            }
        };
        let decoded_out = decoding_sci_complete(
            current_type,
            &mut Some(ext),
            false,
            false,
            &CallExpectation::None,
            data,
            meta_v14,
            indent,
            short_specs,
        )?;
        fancy_out.extend_from_slice(&decoded_out.fancy_out);
        data = decoded_out.remaining_vector;
    }
    Ok(DecodedOut {
        remaining_vector: data.to_vec(),
        fancy_out,
    })
}

pub(crate) struct Ext {
    pub(crate) identifier: String,
    pub(crate) specialty: SpecialExt,
    pub(crate) found_ext: FoundExt,
}

impl Ext {
    pub(crate) fn init() -> Self {
        Self {
            identifier: String::new(),
            specialty: SpecialExt::None,
            found_ext: FoundExt::init(),
        }
    }
    pub(crate) fn check_special(&mut self, current_type: &Type<PortableForm>) {
        self.specialty = match current_type.path().ident() {
            Some(a) => match a.as_str() {
                "Era" => SpecialExt::Era,
                "CheckNonce" => SpecialExt::Nonce,
                "ChargeTransactionPayment" => SpecialExt::Tip,
                _ => SpecialExt::None,
            },
            None => SpecialExt::None,
        };
        if let SpecialExt::None = self.specialty {
            self.specialty = match self.identifier.as_str() {
                "CheckSpecVersion" => SpecialExt::SpecVersion,
                "CheckTxVersion" => SpecialExt::TxVersion,
                "CheckGenesis" => SpecialExt::Hash(Hash::GenesisHash),
                "CheckMortality" => SpecialExt::Hash(Hash::BlockHash),
                "CheckNonce" => SpecialExt::Nonce,
                "ChargeTransactionPayment" => SpecialExt::Tip,
                _ => SpecialExt::None,
            };
        }
    }
}

pub(crate) struct FoundExt {
    pub(crate) era: Option<Era>,
    pub(crate) genesis_hash: Option<[u8; 32]>,
    pub(crate) block_hash: Option<[u8; 32]>,
    pub(crate) network_version_printed: Option<String>,
}

impl FoundExt {
    pub(crate) fn init() -> Self {
        Self {
            era: None,
            genesis_hash: None,
            block_hash: None,
            network_version_printed: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Hash {
    GenesisHash,
    BlockHash,
}

#[derive(Debug)]
pub(crate) enum SpecialExt {
    Era,
    Nonce,
    Tip,
    SpecVersion,
    TxVersion,
    Hash(Hash),
    None,
}

pub(crate) fn special_case_hash(
    data: Vec<u8>,
    found_ext: &mut FoundExt,
    indent: u32,
    short_specs: &ShortSpecs,
    hash: &Hash,
) -> Result<DecodedOut, ParserError> {
    match data.get(0..32) {
        Some(a) => match <[u8; 32]>::decode(&mut &a[..]) {
            Ok(x) => {
                let remaining_vector = data[32..].to_vec();
                let fancy_out = match hash {
                    Hash::GenesisHash => {
                        found_ext.genesis_hash = Some(x);
                        if x != short_specs.genesis_hash {
                            return Err(ParserError::Decoding(
                                ParserDecodingError::GenesisHashMismatch,
                            ));
                        }
                        Vec::new()
                    }
                    Hash::BlockHash => {
                        found_ext.block_hash = Some(x);
                        vec![OutputCard {
                            card: ParserCard::BlockHash(x),
                            indent,
                        }]
                    }
                };
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

pub(crate) fn special_case_era(
    data: Vec<u8>,
    found_ext: &mut FoundExt,
    indent: u32,
) -> Result<DecodedOut, ParserError> {
    let (era_data, remaining_vector) = match data.get(0) {
        Some(0) => (data[0..1].to_vec(), data[1..].to_vec()),
        Some(_) => match data.get(0..2) {
            Some(a) => (a.to_vec(), data[2..].to_vec()),
            None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
        },
        None => return Err(ParserError::Decoding(ParserDecodingError::DataTooShort)),
    };
    match Era::decode(&mut &era_data[..]) {
        Ok(a) => {
            found_ext.era = Some(a);
            Ok(DecodedOut {
                remaining_vector,
                fancy_out: vec![OutputCard {
                    card: ParserCard::Era(a),
                    indent,
                }],
            })
        }
        Err(_) => return Err(ParserError::Decoding(ParserDecodingError::Era)),
    }
}
