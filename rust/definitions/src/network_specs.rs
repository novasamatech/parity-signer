use parity_scale_codec_derive::{Decode, Encode};
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;

//TODO: rename fields to make them more clear
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct ChainSpecs {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
    //TODO: add metadata signature parameters
}


#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct ChainSpecsToSend {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct ShortSpecs {
    pub base58prefix: u16,
    pub decimals: u8,
    pub genesis_hash: [u8; 32],
    pub name: String,
    pub unit: String,
}

impl ChainSpecs {
    pub fn show(&self, current_verifier: &CurrentVerifier, general_verifier: &Verifier) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"current_verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, current_verifier.show(general_verifier))
    }
    pub fn short(&self) -> ShortSpecs {
        ShortSpecs {
            base58prefix: self.base58prefix,
            decimals: self.decimals,
            genesis_hash: self.genesis_hash,
            name: self.name.to_string(),
            unit: self.unit.to_string(),
        }
    }
}

impl ChainSpecsToSend {
    pub fn show(&self) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\"", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.path_id, &self.secondary_color, &self.title, &self.unit)
    }
}

#[derive(Decode, Encode, PartialEq, Debug)]
pub struct ChainProperties {
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

/// Verifier for both network metadata and for types information
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct Verifier (pub Option<VerifierValue>);

#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum VerifierValue {
    Standard (MultiSigner),
}

impl Verifier {
    pub fn show_card(&self) -> String {
        match &self.0 {
            Some(a) => a.show_card(),
            None => String::from("{\"hex\":\"\",\"encryption\":\"none\"}"),
        }
    }
    pub fn show_error(&self) -> String {
        match &self.0 {
            Some(a) => a.show_error(),
            None => String::from("none"),
        }
    }
}

impl VerifierValue {
    pub fn show_card(&self) -> String {
        match &self {
            VerifierValue::Standard(MultiSigner::Ed25519(x)) => format!("{{\"hex\":\"{}\",\"encryption\":\"ed25519\"}}", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Sr25519(x)) => format!("{{\"hex\":\"{}\",\"encryption\":\"sr25519\"}}", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Ecdsa(x)) => format!("{{\"hex\":\"{}\",\"encryption\":\"ecdsa\"}}", hex::encode(x.0)),
        }
    }
    pub fn show_error(&self) -> String {
        match &self {
            VerifierValue::Standard(MultiSigner::Ed25519(x)) => format!("public key: {}, encryption: ed25519", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Sr25519(x)) => format!("public key: {}, encryption: sr25519", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Ecdsa(x)) => format!("public key: {}, encryption: ecdsa", hex::encode(x.0)),
        }
    }
}

/// Current network verifier.
/// Could be general verifier (by default, for networks: kusama, polkadot, westend, rococo),
/// or could be custom verifier.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum CurrentVerifier {
    General,
    Custom (Verifier),
    Dead,
}

impl CurrentVerifier {
    pub fn show(&self, general_verifier: &Verifier) -> String {
        match &self {
            CurrentVerifier::General => format!("{{\"type\":\"general\",\"details\":{}}}", general_verifier.show_card()),
            CurrentVerifier::Custom(a) => format!("{{\"type\":\"custom\",\"details\":{}}}", a.show_card()),
            CurrentVerifier::Dead => String::from("{\"type\":\"dead\",\"details\":\"\"}"),
        }
    }
}
