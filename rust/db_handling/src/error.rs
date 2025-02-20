use std::result;

use definitions::{
    crypto::Encryption,
    helpers::multisigner_to_public,
    keyring::{AddressKey, NetworkSpecsKey, VerifierKey},
    users::AddressDetails,
};
use sp_core::H256;
use sp_runtime::MultiSigner;

/// DB handling error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No networks available")]
    NoNetworksAvailable,

    #[error(transparent)]
    DbTransactionError(#[from] sled::transaction::TransactionError),

    #[error(transparent)]
    Defaults(#[from] defaults::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),

    #[error(transparent)]
    TimeFormat(#[from] time::error::Format),

    #[error(transparent)]
    DefinitionsError(#[from] definitions::error::Error),

    #[error(transparent)]
    Bip39MnemonicType(#[from] bip39::ErrorKind),

    #[error("Database error. Internal error. {0}")]
    DbError(#[from] sled::Error),

    #[error("Database schema version mismatch. Expected v{expected}, found v{found}.")]
    DbSchemaMismatch { expected: u32, found: u32 },

    /// Temporary database entry in `TRANSACTION` tree of the Vault database
    /// under the key `STUB`, used to store the update data awaiting for the
    /// user approval.
    ///
    /// Missing `Stub` when it is expected always indicates the database
    /// corruption.
    #[error(
        "Unable to decode temporary entry with information \
        needed for accepting approved information."
    )]
    Stub,

    /// Network [`CurrentVerifier`](definitions::network_specs::CurrentVerifier) is
    /// `ValidCurrentVerifier::Custom(_)`, but the custom verifier value
    /// coincides with the general verifier.
    ///
    /// Associated data is [`VerifierKey`] corresponding to faulty entry.
    #[error(
        "Network with genesis hash {} verifier is set as a custom one. \
        This custom verifier coinsides the database general verifier \
        and not None. This should not have happened and likely \
        indicates database corruption.",
        hex::encode(.0.key()),
    )]
    CustomVerifierIsGeneral(VerifierKey),

    /// Network has no entry for
    /// [`CurrentVerifier`](definitions::network_specs::CurrentVerifier) under
    /// `verifier_key` in `VERIFIERS` tree of the database, however, the
    /// corresponding genesis hash is encountered in a
    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry under
    /// `network_specs_key` in `SPECSTREE` tree of the database.
    /// No network specs record can get into database without the verifier
    /// entry, and the verifier could not be removed while network specs still
    /// remain, so this indicates the database got corrupted.
    #[error(
        "Network {name} with genesis hash {} has some network specs entries, \
        while there is no verifier entry.",
        hex::encode(genesis_hash)
    )]
    UnexpectedGenesisHash { name: String, genesis_hash: H256 },

    /// History log [`Entry`](definitions::history::Entry) stored in `HISTORY` tree
    /// of the Vault database under the key [`Order`]
    /// could not be decoded.
    ///
    /// Associated data is the corresponding [`Order`].
    ///
    /// [`Order`]: definitions::keyring::Order
    #[error("Unable to decode history entry for order {0}")]
    HistoryEntryNotFound(u32),

    /// Database has two seed addresses (i.e. with empty derivation path and no
    /// password) for same seed name and [`Encryption`]
    ///
    /// This indicates the database corruption, since the encryption method,
    /// seed name and derivation path strictly determine the public key.
    #[error(
        "More than one seed key (i.e. with empty path and without password)
        found for seed name {seed_name} and encryption {}.",
        encryption.show(),
    )]
    TwoRootKeys {
        /// seed name
        seed_name: String,

        /// encryption algorithm for which two seed keys were found
        encryption: Encryption,
    },

    /// To generate QR code with public address information export, Vault
    /// receives both seed name and
    /// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// from the navigation state `Navstate`.
    /// `MultiSigner` gets transformed into [`AddressKey`] and corresponds to
    /// [`AddressDetails`](definitions::users::AddressDetails) that are exported.
    /// `AddressDetails` also contain `seed_name` field, that must coincide
    /// with the one received directly from the navigator.
    /// This error appears if the seed names are different.
    #[error(
        "Expected seed name {expected_seed_name} for address key {}. \
        Address details in database have {real_seed_name} name.",
        hex::encode(address_key.key()),
    )]
    SeedNameNotMatching {
        /// address key for which the export is done
        address_key: AddressKey,

        /// seed name, from the navigator
        expected_seed_name: String,

        /// seed name, from the `AddressDetails`
        real_seed_name: String,
    },

    /// QR handling error.
    #[error("QR error {0}.")]
    Qr(String),

    /// [`NetworkSpecsKey`] of a network in `network_id` field of the
    /// [`AddressDetails`](definitions::users::AddressDetails) corresponding to
    /// [`AddressKey`].
    ///
    /// This happens when the derivation is created in some other network(s), but
    /// not in the given network. This way the `AddressKey` is in the database,
    /// but the address in the network is not.
    #[error(
        "Could not find network specs key {} in address details with key {}.",
        hex::encode(.network_specs_key.key()),
        hex::encode(.address_key.key())
    )]
    NetworkSpecsKeyForAddressNotFound {
        network_specs_key: NetworkSpecsKey,
        address_key: AddressKey,
    },

    /// Got empty seed phrase.
    #[error("Seed phrase empty.")]
    EmptySeed,

    /// Got empty seed name.
    #[error("Seed name is empty.")]
    EmptySeedName,

    /// Error in [`SecretString`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/type.SecretString.html).
    ///
    /// `SecretString` consists of combined seed phrase and derivation.
    ///
    /// Associated error content is
    /// [`SecretStringError`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html).
    #[error("Secret string error: {}", format!("{:?}", .0))]
    SecretStringError(sp_core::crypto::SecretStringError),

    /// Same public key was produced for a different seed phrase and/or
    /// derivation path, during database transaction preparation (not yet in
    /// the database).
    #[error(
        "Tried to create colliding addresses within {}.\
        Address for seed name {seed_name_new} and path {cropped_path_new} \
        has same public key as address for seed name {seed_name_existing} \
        and path {cropped_path_existing}.",
        if *.in_this_network { "same network" } else { "different networks" }
    )]
    KeyCollisionBatch {
        seed_name_existing: String,
        seed_name_new: String,
        cropped_path_existing: String,
        cropped_path_new: String,
        in_this_network: bool,
    },

    /// Same public key was produced for a different seed phrase and/or
    /// derivation path, as already existing in the database.
    ///
    /// Address is generated within a network using seed phrase and derivation
    /// path.
    ///
    /// Address is defined by public key and [`NetworkSpecsKey`]. Public key
    /// is created from seed phrase and derivation with encryption algorithm
    /// supported by the network.
    ///
    /// If two networks are using the same encryption algorithm, generating
    /// public key in both with same seed phrase and derivation path would
    /// result in two identical public keys. This is normal and expected
    /// behavior, and this is the reason why [`AddressDetails`] contain a set
    /// of allowed networks in `network_id` field.
    ///
    /// It is, however, possible, that when generating public key for
    /// **different** seed names and/or **different** derivation
    /// paths, resulting public keys accidentally coincide.
    ///
    /// This is called here `KeyCollision`, and results in error.
    #[error("Address key collision for seed name {seed_name}")]
    KeyCollision { seed_name: String },

    /// Derivation that user tried to create already exists.
    ///
    /// Associated error content is:
    /// - [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    ///   of the already existing address
    /// - [`AddressDetails`] for already existing address
    /// - [`NetworkSpecsKey`] of the associated network
    #[error(
        "Seed {} already has derivation {}{} \
        for network specs key {}, public key {}.",
        .address_details.seed_name,
        .address_details.path,
        if .address_details.has_pwd { "///<password>" } else { "" },
        hex::encode(network_specs_key.key()),
        hex::encode(multisigner_to_public(.multisigner)),
    )]
    DerivationExists {
        multisigner: MultiSigner,
        address_details: Box<AddressDetails>,
        network_specs_key: NetworkSpecsKey,
    },
    /// Received `derivations` update payload contains an invalid derivation.
    ///
    /// Associated data is the derivation that could not be used as a `String`.
    #[error("Derivation {0} has invalid format.")]
    InvalidDerivation(String),

    /// User was creating the derivation with password, and thus moved into
    /// `PasswordConfirm` modal, however, the password was not found when
    /// cutting password from the path.
    #[error("Derivation had password, then lost it.")]
    LostPwd,

    /// Temporary database entry in `TRANSACTION` tree of the Vault database
    /// under the key `DRV`, used to store the derivation import data.
    ///
    /// Missing `Derivations` when it is expected always indicates the database
    /// corruption.
    #[error("Derivations not found.")]
    DerivationsNotFound,

    /// Temporary database entry in `TRANSACTION` tree of the Vault database
    /// under the key `SIGN`, used to store the signable transaction data
    /// awaiting for the user approval.
    ///
    /// Missing `Sign` when it is expected always indicates the database
    /// corruption.
    #[error(
        "Could not find database temporary entry with information \
        needed for signing approved transaction."
    )]
    Sign,

    #[error("Unable to decode current verifier entry for key {}.", hex::encode(.0.key()))]
    NoValidCurrentVerifier(VerifierKey),

    /// While searching for all networks with same genesis hash, found that
    /// there are networks with same genesis hash, but different names.
    #[error(
        "Different network names ({name1}, {name2}) in database for same genesis hash {}.",
        hex::encode(genesis_hash)
    )]
    DifferentNamesSameGenesisHash {
        name1: String,
        name2: String,
        genesis_hash: H256,
    },

    /// Network specs entries have same genesis hash, but different base58 prefix
    #[error(
        "Prefix mismatch for same genesis hash {}: {base58_1}, {base58_2}",
        hex::encode(genesis_hash.as_ref()),
    )]
    DifferentBase58Specs {
        base58_1: u16,
        base58_2: u16,
        genesis_hash: H256,
    },

    /// General verifier [`Verifier`](definitions::network_specs::Verifier) information
    /// stored in `SETTREE` tree of the database under key `GENERALVERIFIER`.
    ///
    /// Missing general verifier always indicates the database corruption.
    #[error("Could not find general verifier.")]
    GeneralVerifierNotFound,

    /// [`DangerRecord`](definitions::danger::DangerRecord) information
    /// stored in `SETTREE` tree of the database under key `DANGER`.
    ///
    /// Missing `DangerRecord` always indicates the database corruption.
    #[error("Types not found.")]
    TypesNotFound,

    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) for a network
    /// in `SPECSTREE` tree of the Vault database, searched by
    /// [`NetworkSpecsKey`].
    ///
    /// Associated data is the `NetworkSpecsKey` used for the search.
    #[error("Could not find network specs for network specs key {}.", hex::encode(.0.key()))]
    NetworkSpecsNotFound(NetworkSpecsKey),

    /// [`AddressDetails`](definitions::users::AddressDetails) for [`AddressKey`] in
    /// `ADDRTREE` tree of the Vault database.
    ///
    /// Associated data is the `AddressKey` used for search.
    #[error(
        "Could not find address details for address key {}.", hex::encode(.0.key())
    )]
    AddressNotFound(AddressKey),

    /// Network metadata in `METATREE` tree of the Vault database, for network
    /// name and version combination.
    #[error("Meta values not found for {name} version {version}")]
    MetaValuesNotFound {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// Database checksum does not match the expected value.
    #[error("Checksum mismatch.")]
    ChecksumMismatch,

    /// [`DangerRecord`](definitions::danger::DangerRecord) information
    /// stored in `SETTREE` tree of the database under key `DANGER`.
    ///
    /// Missing `DangerRecord` always indicates the database corruption.
    #[error("Danger status not found.")]
    DangerStatusNotFound,

    #[error("There are no seeds. Please create a seed first.")]
    NoKnownSeeds,

    /// Found `secret_exposed` flag mismatch in the database: address is not
    /// marked as potentially exposed when it must have been.
    #[error(
    "Address details entry with public key {} (seed {}, path {}{}) is not marked as potentially exposed, \
        when it should be.",
    hex::encode(multisigner_to_public(.multisigner)),
    .address_details.seed_name,
    .address_details.path,
    if .address_details.has_pwd { "///<password>" } else { "" },
    )]
    SecretExposedMismatch {
        multisigner: MultiSigner,
        address_details: AddressDetails,
    },

    /// User has entered a wrong password for a passworded address.
    #[error("Wrong password.")]
    WrongPassword,

    #[error("Missing information about whether the path {0} is passworded.")]
    MissingPasswordInfo(String),

    #[error("Key pair with public key {} can't be expressed as a direct derivation from a seed",
    hex::encode(multisigner_to_public(.multisigner)),
    )]
    NoSeedForKeyPair { multisigner: MultiSigner },

    #[error("No seed is found for {} root key",
    hex::encode(multisigner_to_public(.multisigner)),
    )]
    NoSeedFound { multisigner: MultiSigner },

    #[error("No root derivation for seed {0}")]
    NoRootKeyForSeed(String),

    #[error("Data packing error: {0}")]
    DataPacking(String),
}

/// DB handling result.
pub type Result<T> = result::Result<T, Error>;
