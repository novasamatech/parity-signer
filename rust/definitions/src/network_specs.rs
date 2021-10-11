use parity_scale_codec_derive;
use parity_scale_codec::Encode;

use crate::crypto::Encryption;

//TODO: rename fields to make them more clear
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
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


#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug, Clone)]
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

impl ChainSpecs {
    pub fn show(&self, verifier: &Verifier) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, verifier.show_card())
    }
}

impl ChainSpecsToSend {
    pub fn show(&self) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\"", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.path_id, &self.secondary_color, &self.title, &self.unit)
    }
}

#[derive(Debug, parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct ChainProperties {
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

/// Verifier for both network metadata and for types information,
/// String is hexadecimal representation of verifier public key
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
pub enum Verifier {
    Ed25519(String),
    Sr25519(String),
    Ecdsa(String),
    None,
}

impl Verifier {
    pub fn show_card(&self) -> String {
        match &self {
            Verifier::Ed25519(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"ed25519\"}}", x),
            Verifier::Sr25519(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"sr25519\"}}", x),
            Verifier::Ecdsa(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"ecdsa\"}}", x),
            Verifier::None => String::from("{\"hex\":\"\",\"encryption\":\"none\"}"),
        }
    }
    pub fn show_error(&self) -> String {
        match &self {
            Verifier::Ed25519(x) => format!("public key: {}, encryption: ed25519", x),
            Verifier::Sr25519(x) => format!("public key: {}, encryption: sr25519", x),
            Verifier::Ecdsa(x) => format!("public key: {}, encryption: ecdsa", x),
            Verifier::None => String::from("none"),
        }
    }
}

/// Key for verifier tree, used to search who verifies the network on current device
pub type VerifierKey = Vec<u8>;

/// Function to generate key in verifier tree for given network
pub fn generate_verifier_key (gen_hash: &Vec<u8>) -> VerifierKey {
    gen_hash.to_vec()
}

/// Struct to prepare verifier info for the database
pub struct VerifierInfo {
    pub key: VerifierKey,
    pub verifier: Verifier,
}

/// Struct to store verifier info for particular network, used in history logging
pub struct NetworkVerifier <'a> {
    pub verifier_key: &'a str,
    pub verifier_line: String,
}

impl <'a> NetworkVerifier <'a> {
    pub fn show(&self) -> String {
        format!("\"specname\":\"{}\",\"verifier\":{}", &self.verifier_key, &self.verifier_line)
    }
}

/// Network identifier, used to search for network specs in the database
pub type NetworkKey = Vec<u8>;

/// Enum to store network identifier parts, becomes network identifier when encoded
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
pub enum NetworkKeySource {
    Ed25519(Vec<u8>),
    Sr25519(Vec<u8>),
    Ecdsa(Vec<u8>),
}

/// Generate network key from minimal amount of information
pub fn generate_network_key (gen_hash: &Vec<u8>, encryption: Encryption) -> NetworkKey {
    match encryption {
        Encryption::Ed25519 => NetworkKeySource::Ed25519(gen_hash.to_vec()),
        Encryption::Sr25519 => NetworkKeySource::Sr25519(gen_hash.to_vec()),
        Encryption::Ecdsa => NetworkKeySource::Ecdsa(gen_hash.to_vec()),
    }.encode()
}

