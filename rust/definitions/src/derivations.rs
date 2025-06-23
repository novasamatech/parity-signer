use crate::crypto::Encryption;
use crate::navigation::Identicon;
use sp_core::H256;

use sp_runtime::MultiSigner;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SeedKeysPreview {
    /// Name of the seed.
    pub name: String,

    /// Public key of the root key.
    pub multisigner: MultiSigner,

    /// Derived keys.
    pub derived_keys: Vec<DerivedKeyPreview>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DerivedKeyPreview {
    /// Address in the network.
    ///
    /// This is either `ss58` form for substrate-based chains or
    /// hex public key form for ethereum based chains (ecdsa)
    pub address: String,

    /// The derivation path of the key if user provided one
    pub derivation_path: Option<String>,

    /// The type of encryption in the network
    pub encryption: Encryption,

    /// Genesis hash
    pub genesis_hash: H256,

    pub identicon: Identicon,

    /// Has to be calculated using `inject_derivations_has_pwd`. Otherwise, `None`
    pub has_pwd: Option<bool>,

    /// Might be `None` if network specs were not imported into the Vault
    pub network_title: Option<String>,

    pub status: DerivedKeyStatus,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DerivedKeyStatus {
    /// Key can be imported into the Vault
    Importable,

    /// Key is already into the Vault. Unable to determine for a key with password
    AlreadyExists,

    Invalid {
        errors: Vec<DerivedKeyError>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DerivedKeyError {
    /// Network specs were not imported into the Vault
    NetworkMissing,

    /// Seed is not in the Vault
    KeySetMissing,

    /// Bad format of the derivation path
    BadFormat,
}
