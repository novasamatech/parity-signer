use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;

use crate::{network_specs::ChainSpecsToSend, types::TypeEntry};

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
    pub fn meta (&self) -> Result<Vec<u8>, &'static str>  {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.meta),
            Err(_) => return Err("load_metadata content could not be decoded")
        }
    }
    /// Function to get genesis hash from load_metadata content
    pub fn genesis_hash (&self) -> Result<[u8; 32], &'static str> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.genesis_hash),
            Err(_) => return Err("load_metadata content could not be decoded")
        }
    }
    /// Function to decode load_metadata message and get both metadata and network genesis hash as a tuple
    pub fn meta_genhash (&self) -> Result<(Vec<u8>, [u8; 32]), &'static str> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.meta, a.genesis_hash)),
            Err(_) => return Err("load_metadata content could not be decoded")
        }
    }
    /// Function to export load_metadata content into file
    pub fn write (&self, filename: &str) -> Result<(), String> {
        match std::fs::write(&filename, &self.0) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }
    /// Function to put load_metadata information into storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Struct to process the content of qr codes with add_specs messages
pub struct ContentAddSpecs (Vec<u8>);

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct DecodedContentAddSpecs {
    specs: ChainSpecsToSend,
}

impl ContentAddSpecs {
    /// Function to generate add_specs content from network specs ChainSpecsToSend
    pub fn generate (specs: &ChainSpecsToSend) -> Self {
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
    /// Function to get network specs ChainSpecsToSend from add_specs content
    pub fn specs (&self) -> Result<ChainSpecsToSend, &'static str> {
        match <DecodedContentAddSpecs>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.specs),
            Err(_) => return Err("add_specs content could not be decoded")
        }
    }
    /// Function to export add_specs content into file
    pub fn write (&self, filename: &str) -> Result<(), String> {
        match std::fs::write(&filename, &self.0) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }
    /// Function to put add_specs information into storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}


/// Struct to process the content of qr codes with load_types messages
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
    pub fn types (&self) -> Result<Vec<TypeEntry>, &'static str> {
        match <DecodedContentLoadTypes>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.types),
            Err(_) => return Err("load_types content could not be decoded")
        }
    }
    /// Function to export load_types content into file
    pub fn write (&self, filename: &str) -> Result<(), String> {
        match std::fs::write(&filename, &self.0) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }
    /// Function to put types information into storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

