use crate::crypto::Encryption;
use crate::navigation::SignerImage;
use parity_scale_codec::{Decode, Encode};
use sp_core::H256;

use sp_runtime::MultiSigner;

// Export data structure
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub enum ExportAddrs {
    V1(ExportAddrsV1),
}

// uniffi friendly ExportAddrs
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExportAddrsContainer {
    V1 { data: ExportAddrsV1 },
}

impl From<ExportAddrs> for ExportAddrsContainer {
    fn from(item: ExportAddrs) -> Self {
        match item {
            ExportAddrs::V1(v1) => Self::V1 { data: v1 },
        }
    }
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct ExportAddrsV1 {
    pub addrs: Vec<SeedInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct SeedInfo {
    /// Name of the seed.
    pub name: String,

    /// Public key of the root key.
    pub multisigner: MultiSigner,

    /// Derived keys.
    pub derived_keys: Vec<AddrInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct AddrInfo {
    /// Address in the network.
    ///
    /// This is either `ss58` form for substrate-based chains or
    /// h160 form for ethereum based chains
    pub address: String,

    /// The derivation path of the key if user provided one
    pub derivation_path: Option<String>,

    /// The type of encryption in the network
    pub encryption: Encryption,

    /// Genesis hash
    pub genesis_hash: H256,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SeedKeysPreviewSummary {
    pub seed_keys: Vec<SeedKeysPreview>,
    pub already_existing_key_count: u32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SeedKeysPreview {
    /// Name of the seed.
    pub name: String,

    /// Public key of the root key.
    pub multisigner: MultiSigner,

    pub importable_keys: Vec<DerivedKeyPreview>,

    pub already_existing_keys: Vec<DerivedKeyPreview>,

    // Those with bad format or network missing
    pub non_importable_keys: Vec<DerivedKeyInfo>,

    pub is_key_set_missing: bool,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DerivedKeyInfo {
    /// The derivation path of the key
    pub derivation_path: String,

    pub key_error: DerivedKeyError,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DerivedKeyPreview {
    /// Address in the network.
    ///
    /// This is either `ss58` form for substrate-based chains or
    /// h160 form for ethereum based chains
    pub address: String,

    /// The derivation path of the key
    pub derivation_path: String,

    /// The type of encryption in the network
    pub encryption: Encryption,

    /// Genesis hash
    pub genesis_hash: H256,

    pub identicon: SignerImage,

    pub has_pwd: bool,

    pub network_title: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DerivedKeyError {
    /// Network specs were not imported into the Signer
    NetworkMissing,

    /// Bad format of the derivation path
    BadFormat,
}
