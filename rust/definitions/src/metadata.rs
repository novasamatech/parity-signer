use parity_scale_codec_derive::{Decode, Encode};

/// Struct used to store the network metadata name and version in the database
#[derive(Decode, Encode)]
pub struct NameVersioned {
    pub name: String,
    pub version: u32,
}

/// Struct to store the metadata values (network name, network
/// version, full metadata as Vec<u8>)
pub struct MetaValues {
    pub name: String,
    pub version: u32,
    pub meta: Vec<u8>,
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

/// Struct to store newtork information needed for metadata and network specs fetching
#[derive(Decode, Encode)]
pub struct AddressBookEntry {
    pub name: String,
    pub genesis_hash: [u8; 32],
    pub address: String,
}

