use parity_scale_codec_derive::{Decode, Encode};

use super::network_specs::NetworkKey;


/// Type of encryption; only allow supported types here - compile-time check for that is happening
/// here.
//TODO: check if it is redundant
//Could not be replaced by sp_core::...::CRYPTO_ID as that doesn't do anything at compile time
#[derive(Clone, Copy, PartialEq, Debug, Decode, Encode)]
pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

/// Struct associated with public address that has secret key available
#[derive(Decode, Encode, Debug)]
pub struct AddressDetails {
    pub seed_name: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_id: Vec<NetworkKey>,
    pub encryption: Encryption,
}

/// Struct to move seed around
/// TODO: zeroize somehow
#[derive(PartialEq, Debug)]
pub struct SeedObject {
    pub seed_name: String,
    pub seed_phrase: String,
    pub encryption: Encryption,
}


/// Network identifier, used to search for network specs in the database
/// At this moment, vector made from public key
pub type AddressKey = Vec<u8>;

/// Generate address key from minimal amount of information
pub fn generate_address_key (public: &Vec<u8>) -> AddressKey {
    public.to_vec()
}
