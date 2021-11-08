use parity_scale_codec_derive::{Decode, Encode};
use crate::network_specs::ChainSpecsToSend;
use crate::crypto::Encryption;

/// Struct used to store the network metadata name and version in the database
#[derive(Decode, Encode, PartialEq)]
pub struct NameVersioned {
    pub name: String,
    pub version: u32,
}

/// Struct to store the metadata values (network name, network
/// version, full metadata as Vec<u8>)
#[derive(PartialEq, Clone)]
pub struct MetaValues {
    pub name: String,
    pub version: u32,
    pub meta: Vec<u8>,
}

/// Struct to store the metadata values in history log for adding/removing metadata
/// for known networks (network name, network version, metadata hash)
pub struct MetaValuesDisplay <'a> {
    pub name: &'a str,
    pub version: u32,
    pub meta_hash: &'a str,
}

impl <'a> MetaValuesDisplay <'a> {
    pub fn show(&self) -> String {
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\"", &self.name, &self.version, &self.meta_hash)
    }
}

/// Struct to store the metadata values in history log for signing load_metadata message
/// by user (network name, network version, metadata hash, verifier)
pub struct VerifiedMetaValuesDisplay <'a> {
    pub name: &'a str,
    pub version: u32,
    pub meta_hash: &'a str,
    pub verifier_line: String, // Verifier.show_card()
}

impl <'a> VerifiedMetaValuesDisplay <'a> {
    pub fn show(&self) -> String {
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"verifier\":{}", &self.name, &self.version, &self.meta_hash, &self.verifier_line)
    }
}

/// Struct to store the metadata and network specs values in history log for adding/removing new networks
/// (network name, network version, metadata hash, network specs, network verifier) and for signing add_network
/// message by user (in which case verifier line corresponds to user account participating in signing)
pub struct NetworkDisplay <'a> {
    pub meta_values: MetaValuesDisplay <'a>,
    pub network_specs: &'a ChainSpecsToSend,
    pub verifier_line: String,
}

impl <'a> NetworkDisplay <'a> {
    pub fn show(&self) -> String {
        format!("{},{},\"verifier\":{}", self.meta_values.show(), self.network_specs.show(), self.verifier_line)
    }
}

/// Struct to decode the version vector from network metadata
#[derive(Debug, parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)]
pub struct VersionDecoded {
    pub specname: String,
    implname: String,
    auth_version: u32,
    pub spec_version: u32,
    impl_version: u32,
    apis: Vec<(u8, u32)>,
    trans_version: u32,
}

/// Struct to store network information needed for metadata and network specs fetching
#[derive(Decode, Encode)]
pub struct AddressBookEntry {
    pub name: String,
    pub genesis_hash: [u8; 32],
    pub address: String,
    pub encryption: Encryption,
    pub def: bool,
}

