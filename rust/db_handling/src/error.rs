use std::result;

use definitions::{
    crypto::Encryption,
    keyring::{AddressKey, NetworkSpecsKey, VerifierKey},
    users::AddressDetails,
};
use sp_core::H256;
use sp_runtime::MultiSigner;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error. Internal error. {0}")]
    DbError(#[from] sled::Error),

    #[error(transparent)]
    DbTransactionError(#[from] sled::transaction::TransactionError),

    #[error("stub was not found in cold storage")]
    StubNotFound,

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),

    #[error("custom verifier is general")]
    CustomVerifierIsGeneral(VerifierKey),

    #[error("verifier is dead")]
    DeadVerifier(VerifierKey),

    #[error("network {name} has no entry but the genesis {genesis_hash} exists")]
    UnexpectedGenesisHash { name: String, genesis_hash: H256 },

    #[error(transparent)]
    DefinitionsError(#[from] definitions::error::Error),

    #[error("entry not found {0}")]
    HistoryEntryNotFound(u32),

    #[error(transparent)]
    TimeFormat(#[from] time::error::Format),

    #[error("two root keys")]
    TwoRootKeys {
        seed_name: String,
        encryption: Encryption,
    },

    #[error("no networks available")]
    NoNetworksAvailable,

    #[error("seed name not matching")]
    SeedNameNotMatching {
        address_key: AddressKey,
        expected_seed_name: String,
        real_seed_name: String,
    },

    #[error("qr error {0}")]
    Qr(String),

    #[error("not found {} {}",
        hex::encode(.network_specs_key.key()),
        hex::encode(.address_key.key())
    )]
    NetworkSpecsForAddressNotFound {
        network_specs_key: NetworkSpecsKey,
        address_key: AddressKey,
    },

    #[error("empty seed")]
    EmptySeed,

    #[error("empty seed name")]
    EmptySeedName,

    #[error("secret string error")]
    SecretStringError(sp_core::crypto::SecretStringError),

    #[error("key collision batch")]
    KeyCollisionBatch {
        seed_name_existing: String,
        seed_name_new: String,
        cropped_path_existing: String,
        cropped_path_new: String,
        in_this_network: bool,
    },

    #[error("key collision")]
    KeyCollision { seed_name: String },

    #[error("derivation exists")]
    DerivationExists {
        multisigner: MultiSigner,
        address_details: AddressDetails,
        network_specs_key: NetworkSpecsKey,
    },

    #[error(transparent)]
    Bip39MnemonicType(#[from] bip39::ErrorKind),

    #[error("invalid derivation")]
    InvalidDerivation,

    #[error("lost pwd")]
    LostPwd,

    #[error("no valid derivations to export")]
    NoValidDerivationToExport,

    #[error("derivations not found")]
    DerivationsNotFound,

    #[error("sign not found")]
    Sign,

    #[error("no valid current verifier")]
    NoValidCurrentVerifier,

    #[error("names mismatch for same genesis hash: {name1}, {name2}")]
    DifferentNamesSameGenesisHash {
        name1: String,
        name2: String,
        genesis_hash: H256,
    },

    #[error("prefix mismatch for same genesis hash: {base58_1}, {base58_2}")]
    DifferentBase58Specs {
        base58_1: u16,
        base58_2: u16,
        genesis_hash: H256,
    },

    #[error("Could not find general verifier.")]
    GeneralVerifierNotFound,

    #[error("types were not found")]
    TypesNotFound,

    #[error("network specs are not found")]
    NetworkSpecsNotFound,

    #[error("address not found")]
    AddressNotFound,

    #[error("meta values not found for {network_name} version {network_version}")]
    MetaValuesNotFound {
        database_name: String,
        network_name: String,
        network_version: u32,
    },

    #[error("checksum mismatch")]
    ChecksumMismatch,

    #[error("danger status not found")]
    DangerStatusNotFound,

    #[error(transparent)]
    Defaults(#[from] defaults::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("no known seeds")]
    NoKnownSeeds,
}

pub type Result<T> = result::Result<T, Error>;
