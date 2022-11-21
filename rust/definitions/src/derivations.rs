use crate::crypto::Encryption;
use sp_core::H256;
use sp_runtime::MultiSigner;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SeedKeysPreview {
    /// Name of the seed.
    pub name: String,

    /// Public key of the root key.
    ///
    /// `None` if user has created no root keys.
    pub multisigner: Option<MultiSigner>,

    /// Derived keys.
    pub derived_keys: Vec<DerivedKeyPreview>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DerivedKeyPreview {
    /// Address in the network.
    ///
    /// This is either `ss58` form for substrate-based chains or
    /// h160 form for ethereum based
    /// chains
    pub address: String,

    /// The derivation path of the key if user provided one
    pub derivation_path: Option<String>,

    /// The type of encryption in the network
    pub encryption: Encryption,

    /// Genesis hash
    pub genesis_hash: H256,
}
