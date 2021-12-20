use blake2_rfc::blake2b::blake2b;
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;

use crate::error::{ErrorActive, ErrorSource, TransferContent};
use crate::helpers::pic_types;
use crate::network_specs::NetworkSpecsToSend;
use crate::types::TypeEntry;

/// Struct to process the content of qr codes with load_metadata messages
pub struct ContentLoadMeta (Vec<u8>);

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct DecodedContentLoadMeta {
    meta: Vec<u8>,
    genesis_hash: [u8; 32],
}

impl ContentLoadMeta {
    /// Function to generate load_metadata content from metadata and genesis hash parts
    pub fn generate (meta: &Vec<u8>, genesis_hash: &[u8; 32]) -> Self {
        Self (
            DecodedContentLoadMeta {
                meta: meta.to_vec(),
                genesis_hash: genesis_hash.to_owned(),
            }.encode()
        )
    }
    /// Function to transform Vec<u8> into ContentLoadMeta prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get metadata from load_metadata content
    pub fn meta<T: ErrorSource>(&self) -> Result<Vec<u8>, T::Error>  {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.meta),
            Err(_) => return Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }
    /// Function to get genesis hash from load_metadata content
    pub fn genesis_hash<T: ErrorSource> (&self) -> Result<[u8; 32], T::Error> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.genesis_hash),
            Err(_) => return Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }
    /// Function to decode load_metadata message and get both metadata and network genesis hash as a tuple
    pub fn meta_genhash<T: ErrorSource> (&self) -> Result<(Vec<u8>, [u8; 32]), T::Error> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.meta, a.genesis_hash)),
            Err(_) => return Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }
    /// Function to export load_metadata content into file
    pub fn write (&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(ErrorActive::Output(e)),
        }
    }
    /// Function to prepare Vec<u8> to be signed from load_metadata information
    pub fn to_sign (&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to prepare load_metadata information for transfer as Vec<u8>
    pub fn to_transfer (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Struct to process the content of qr codes with add_specs messages
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct ContentAddSpecs (Vec<u8>);

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct DecodedContentAddSpecs {
    specs: NetworkSpecsToSend,
}

impl ContentAddSpecs {
    /// Function to generate add_specs content from network specs NetworkSpecsToSend
    pub fn generate (specs: &NetworkSpecsToSend) -> Self {
        Self (
            DecodedContentAddSpecs {
                specs: specs.to_owned(),
            }.encode()
        )
    }
    /// Function to transform Vec<u8> into ContentAddSpecs prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get network specs NetworkSpecsToSend from add_specs content
    pub fn specs<T: ErrorSource> (&self) -> Result<NetworkSpecsToSend, T::Error> {
        match <DecodedContentAddSpecs>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.specs),
            Err(_) => return Err(<T>::transfer_content_error(TransferContent::AddSpecs)),
        }
    }
    /// Function to export add_specs content into file
    pub fn write (&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(ErrorActive::Output(e)),
        }
    }
    /// Function to prepare Vec<u8> to be signed from add_specs information
    pub fn to_sign (&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to prepare add_specs information for transfer as encoded Vec<u8>
    pub fn to_transfer (&self) -> Vec<u8> {
        self.encode()
    }
}


/// Struct to process the content of qr codes with load_types messages
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct ContentLoadTypes (Vec<u8>);

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct DecodedContentLoadTypes {
    types: Vec<TypeEntry>,
}

impl ContentLoadTypes {
    /// Function to generate load_types content from vector Vec<TypeEntry>
    pub fn generate (types: &Vec<TypeEntry>) -> Self {
        Self (
            DecodedContentLoadTypes {
                types: types.to_owned(),
            }.encode()
        )
    }
    /// Function to transform Vec<u8> into ContentLoadTypes prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get vector Vec<TypeEntry> from load_types content
    pub fn types<T: ErrorSource> (&self) -> Result<Vec<TypeEntry>, T::Error> {
        match <DecodedContentLoadTypes>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.types),
            Err(_) => return Err(<T>::transfer_content_error(TransferContent::LoadTypes)),
        }
    }
    /// Function to export load_types content into file
    pub fn write (&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(ErrorActive::Output(e)),
        }
    }
    /// Function to put types information into database storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to prepare Vec<u8> to be signed from load_types information
    pub fn to_sign (&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to prepare load_types information for transfer as encoded Vec<u8>
    pub fn to_transfer (&self) -> Vec<u8> {
        self.encode()
    }
    /// Function to show encoded types hash and corresponding id pic
    pub fn show(&self) -> String {
        let types_hash = blake2b(32, &[], &self.store()).as_bytes().to_vec();
        let types_id_pic = match pic_types(&types_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"types_hash\":\"{}\",\"types_id_pic\":\"{}\"", hex::encode(types_hash), types_id_pic)
    }
}

