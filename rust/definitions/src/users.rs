use parity_scale_codec_derive::{Decode, Encode};
use zeroize::Zeroize;

use crate::crypto::Encryption;
use crate::keyring::NetworkSpecsKey;

/// Struct associated with public address that has secret key available
#[derive(Decode, Encode, Debug, Clone)]
pub struct AddressDetails {
    pub seed_name: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_id: Vec<NetworkSpecsKey>,
    pub encryption: Encryption,
}

/// Struct to move seed around
#[derive(PartialEq, Debug, Zeroize)]
#[zeroize(drop)]
pub struct SeedObject {
    pub seed_name: String,
    pub seed_phrase: String,
    pub encryption: Encryption,
}

