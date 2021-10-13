use parity_scale_codec_derive::{Decode, Encode};

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

impl Encryption {
    pub fn show(&self) -> String {
        match &self {
            Encryption::Ed25519 => String::from("ed25519"),
            Encryption::Sr25519 => String::from("sr25519"),
            Encryption::Ecdsa => String::from("ecdsa"),
        }
    }
}

/// Struct to store `sufficient crypto` information
#[derive(Decode, Encode, PartialEq, Debug)]
pub enum SufficientCrypto {
    Ed25519 {public_key: [u8; 32], signature: [u8; 64]},
    Sr25519 {public_key: [u8; 32], signature: [u8; 64]},
    Ecdsa {public_key: [u8; 33], signature: [u8; 65]},
}

