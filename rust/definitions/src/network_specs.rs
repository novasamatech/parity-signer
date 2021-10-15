use parity_scale_codec_derive::{Decode, Encode};

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

impl ChainSpecs {
    pub fn show(&self, current_verifier: &CurrentVerifier, general_verifier: &Verifier) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"current_verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, current_verifier.show(general_verifier))
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

/// Verifier for both network metadata and for types information,
/// String is hexadecimal representation of verifier public key
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum Verifier {
    Ed25519([u8;32]),
    Sr25519([u8;32]),
    Ecdsa([u8;33]),
    None,
}

impl Verifier {
    pub fn show_card(&self) -> String {
        match &self {
            Verifier::Ed25519(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"ed25519\"}}", hex::encode(x)),
            Verifier::Sr25519(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"sr25519\"}}", hex::encode(x)),
            Verifier::Ecdsa(x) => format!("{{\"hex\":\"{}\",\"encryption\":\"ecdsa\"}}", hex::encode(x)),
            Verifier::None => String::from("{\"hex\":\"\",\"encryption\":\"none\"}"),
        }
    }
    pub fn show_error(&self) -> String {
        match &self {
            Verifier::Ed25519(x) => format!("public key: {}, encryption: ed25519", hex::encode(x)),
            Verifier::Sr25519(x) => format!("public key: {}, encryption: sr25519", hex::encode(x)),
            Verifier::Ecdsa(x) => format!("public key: {}, encryption: ecdsa", hex::encode(x)),
            Verifier::None => String::from("none"),
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
