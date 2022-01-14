use parity_scale_codec_derive::{Decode, Encode};
use sp_core;
use sp_runtime::{MultiSigner, MultiSignature};

use crate::network_specs::VerifierValue;

/// Type of encryption; only allow supported types here - compile-time check for that is happening
/// here.
//TODO: check if it is redundant
//Could not be replaced by sp_core::...::CRYPTO_ID as that doesn't do anything at compile time
#[derive(Clone, PartialEq, Debug, Decode, Encode)]
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
    Ed25519 {public: sp_core::ed25519::Public, signature: sp_core::ed25519::Signature},
    Sr25519 {public: sp_core::sr25519::Public, signature: sp_core::sr25519::Signature},
    Ecdsa {public: sp_core::ecdsa::Public, signature: sp_core::ecdsa::Signature},
}

impl SufficientCrypto {
    pub fn get_verifier_value(&self) -> VerifierValue {
        match &self {
            SufficientCrypto::Ed25519 {public, signature: _} => VerifierValue::Standard(MultiSigner::Ed25519(public.to_owned())),
            SufficientCrypto::Sr25519 {public, signature:_} => VerifierValue::Standard(MultiSigner::Sr25519(public.to_owned())),
            SufficientCrypto::Ecdsa {public, signature: _} => VerifierValue::Standard(MultiSigner::Ecdsa(public.to_owned())),
        }
    }
    pub fn get_multi_signature(&self) -> MultiSignature {
        match &self {
            SufficientCrypto::Ed25519 {public: _, signature} => MultiSignature::Ed25519(signature.to_owned()),
            SufficientCrypto::Sr25519 {public: _, signature} => MultiSignature::Sr25519(signature.to_owned()),
            SufficientCrypto::Ecdsa {public: _, signature} => MultiSignature::Ecdsa(signature.to_owned()),
        }
    }
}
