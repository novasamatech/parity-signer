
//TODO: rename fields to make them more clear
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
pub struct ChainSpecs {
    pub base58prefix: u8,
    pub color: String,
    pub decimals: u8,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
    pub verifier: Verifier,
    //TODO: add metadata signature parameters
}


#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
pub struct ChainSpecsToSend {
    pub base58prefix: u8,
    pub color: String,
    pub decimals: u8,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}


#[derive(Debug, parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)]
pub struct ChainProperties {
    pub base58prefix: u8,
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
            Verifier::None => String::from("\"none\""),
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

/// Network identifier, used to search for network specs in the database
/// At this moment, vector made from genesis hash
pub type NetworkKey = Vec<u8>;

/// Generate network key from minimal amount of information
pub fn generate_network_key (gen_hash: &Vec<u8>) -> NetworkKey {
    gen_hash.to_vec()
}

