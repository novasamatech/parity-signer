//! Trait [`ErrorSource`] and error-related types shared by Signer and active
//! sides
use sp_core::{crypto::SecretStringError, H256};
use sp_runtime::MultiSigner;
#[cfg(feature = "test")]
use variant_count::VariantCount;

use crate::{
    crypto::Encryption,
    helpers::multisigner_to_public,
    keyring::{AddressKey, MetaKey, NetworkSpecsKey},
    users::AddressDetails,
};

/// Error trait for Signer and Signer-ecosystem tools
///
/// [`ErrorSource`] is implemented for:
///
/// - `Active`, errors on the active side: either hot database errors or errors
/// while preparing cold database before its moving into Signer
/// - `Signer`, errors on the Signer side
pub trait ErrorSource {
    /// Enum listing all possible errors
    type Error;

    /// Errors in transforming hexadecimal strings into `Vec<u8>`.
    ///
    /// `NotHex` errors may occur both on `Active` and on `Signer` side.
    ///
    /// On `Active` side `NotHex` errors could be related to strings fetched
    /// form url, input from command line, and processing of the default values.
    ///
    /// On `Signer` side `NotHex` errors are caused by communication errors
    /// and, since user interface should be sending valid hex strings into rust,
    /// generally are not expected to occur.
    type NotHex;

    /// Sources of the faulty [`NetworkSpecsKey`] **excluding** the database
    /// entries, that are:
    ///
    /// - key in `SPECSTREE` (cold database) and `SPECSTREEPREP` (hot database)
    /// - part of value in `ADDRTREE` (cold database), element of `network_id`
    /// set in [`AddressDetails`]
    ///
    /// On the active side enum is empty.
    ///
    /// On the Signer side the source could be only interface.
    type ExtraSpecsKeySource;

    /// Sources of the faulty [`AddressKey`] **excluding** the database
    /// entries, that are:
    ///
    /// - key in `ADDRTREE` (cold database)
    ///
    /// On the active side enum is empty.
    ///
    /// On the Signer side the source could be only interface.
    type ExtraAddressKeySource;

    /// Sources of the faulty metadata **excluding** the database entries, that
    /// are:
    ///
    /// - value stored in `METATREE` (both cold and hot databases)
    ///
    /// For active side, it could be bad fetch or damaged default file.
    ///
    /// For Signer side, it could be faulty received metadata update.
    type IncomingMetadataSource;

    /// Address generation errors, unique for given [`ErrorSource`] implementor
    type ExtraAddressGeneration;

    /// Transform `NotHex` error into `Error`
    fn hex_to_error(what: Self::NotHex) -> Self::Error;

    /// `Error` for damaged [`NetworkSpecsKey`]
    fn specs_key_to_error(
        network_specs_key: &NetworkSpecsKey,
        source: SpecsKeySource<Self>,
    ) -> Self::Error;

    /// `Error` for damaged [`AddressKey`]
    fn address_key_to_error(
        address_key: &AddressKey,
        source: AddressKeySource<Self>,
    ) -> Self::Error;

    /// `Error` for damaged [`MetaKey`]
    ///
    /// Damaged [`MetaKey`] both on active and on Signer side would mean the
    /// database corruption
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error;

    /// `Error` for mismatch of network name and/or network version between the
    /// values from [`MetaKey`] and the values from `Version` constant in
    /// `System` pallet of the metadata itself
    fn metadata_mismatch(
        name_key: String,
        version_key: u32,
        name_inside: String,
        version_inside: u32,
    ) -> Self::Error;

    /// `Error` when metadata is unsuitable for use in Signer
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error;

    /// `Error` when [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry
    /// could not be decoded
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error;

    /// `Error` when there is genesis hash mismatch between stored
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) contents and the
    /// [`NetworkSpecsKey`]
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: H256) -> Self::Error;

    /// `Error` when there is encryption mismatch between stored
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) contents and the
    /// [`NetworkSpecsKey`]
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error;

    /// `Error` when [`AddressDetails`] entry could not be decoded
    fn address_details_decoding(key: AddressKey) -> Self::Error;

    /// `Error` when there is encryption mismatch between `encryption` field
    /// of [`AddressDetails`] and the [`AddressKey`]
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error;

    /// `Error` when there is encryption mismatch between one of
    /// [`NetworkSpecsKey`] in `network_id` field of [`AddressDetails`] and the
    /// `encryption` field of [`AddressDetails`]
    fn address_details_specs_encryption_mismatch(
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    ) -> Self::Error;

    /// `Error` corresponding to one of [`AddressGenerationCommon`]
    /// variants
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error;

    /// `Error` corresponding to faulty [`TransferContent`]
    fn transfer_content_error(transfer_content: TransferContent) -> Self::Error;

    /// `Error` corresponding to database internal error
    fn db_internal(error: sled::Error) -> Self::Error;

    /// `Error` corresponding to database transaction error
    fn db_transaction(error: sled::transaction::TransactionError) -> Self::Error;

    /// `Error` when types information from the database could not be decoded
    fn faulty_database_types() -> Self::Error;

    /// `Error` when types information was expected to be in the database, but
    /// was not found
    fn types_not_found() -> Self::Error;

    /// `Error` when metadata for the network with given name and version was
    /// expected to be in the database, but was not found
    fn metadata_not_found(name: String, version: u32) -> Self::Error;

    /// `Error` when time could not be formatted for history record
    fn timestamp_format(error: time::error::Format) -> Self::Error;

    /// `Error` when unexpectedly got empty seed phrase: if fed in upstream,
    /// empty seed phrase is interpreted as Alice seed phrase automatically.
    fn empty_seed() -> Self::Error;

    /// `Error` when unexpectedly got empty seed name: already forbidden on the
    /// interface, unlikely to happen in general.
    fn empty_seed_name() -> Self::Error;

    /// Print `Error` as a `String`
    ///
    /// Generated string is used in parsing cards, in Signer side anyhow
    /// errors, or in active side errors std::fmt::Display implementation.
    fn show(error: &Self::Error) -> String;
}

/// Source of the error-causing network metadata
#[derive(Debug)]
pub enum MetadataSource<T: ErrorSource + ?Sized> {
    /// Faulty metadata in the database
    ///
    /// Associated `name` and `version` are the ones found from the
    /// corresponding [`MetaKey`]
    Database { name: String, version: u32 },

    /// Faulty metadata was received from the outside
    Incoming(T::IncomingMetadataSource),
}

/// Source of the error-causing [`NetworkSpecsKey`]
#[derive(Debug)]
pub enum SpecsKeySource<T: ErrorSource + ?Sized> {
    /// Faulty [`NetworkSpecsKey`] is a key in the tree `SPECSTREE` (cold
    /// database) or `SPECSTREEPREP` (hot database)
    SpecsTree,

    /// Faulty [`NetworkSpecsKey`] is encountered in `network_id` set in
    /// [`AddressDetails`]
    ///
    /// Associated [`AddressKey`] is the one under which the [`AddressDetails`]
    /// are stored in the tree `ADDRTREE` (cold database)
    AddrTree(AddressKey),

    /// Faulty [`NetworkSpecsKey`] is not from the database
    Extra(T::ExtraSpecsKeySource),
}

/// Source of the error-causing [`AddressKey`]
#[derive(Debug)]
pub enum AddressKeySource<T: ErrorSource + ?Sized> {
    /// Faulty [`AddressKey`] is a key in the tree `ADDRTREE` (cold
    /// database)
    AddrTree,

    /// Faulty [`AddressKey`] is not from the database
    Extra(T::ExtraAddressKeySource),
}

/// Errors in the address generation
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum AddressGeneration<T: ErrorSource + ?Sized> {
    /// Address generation errors common for active and Signer sides
    Common(AddressGenerationCommon),

    /// Address generation errors, unique for given [`ErrorSource`] implementor
    Extra(T::ExtraAddressGeneration),
}

/// Address generation errors common for active and Signer sides
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum AddressGenerationCommon {
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
    KeyCollision { seed_name: String },

    /// Same public key was produced for a different seed phrase and/or
    /// derivation path, during database transaction preparation (not yet in
    /// the database).
    KeyCollisionBatch {
        seed_name_existing: String,
        seed_name_new: String,
        cropped_path_existing: String,
        cropped_path_new: String,
        in_this_network: bool,
    },

    /// Error in [`SecretString`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/type.SecretString.html).
    ///
    /// SecretString consists of combined seed phrase and derivation.
    ///
    /// Associated error content is
    /// [`SecretStringError`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html).
    SecretString(SecretStringError),

    /// Derivaion that user tried to create already exists.
    ///
    /// Associated error content is:
    /// - [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// of the already existing address
    /// - [`AddressDetails`] for already existing address
    /// - [`NetworkSpecsKey`] of the associated network
    DerivationExists(MultiSigner, AddressDetails, NetworkSpecsKey),
}

/// Display
/// [`SecretStringError`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html)
/// as `&str`.
pub(crate) fn bad_secret_string(e: &SecretStringError) -> &'static str {
    match e {
        SecretStringError::InvalidFormat => "invalid overall format",
        SecretStringError::InvalidPhrase => "invalid bip39 phrase",
        SecretStringError::InvalidPassword => "invalid password",
        SecretStringError::InvalidSeed => "invalid seed",
        SecretStringError::InvalidSeedLength => "invalid seed length",
        SecretStringError::InvalidPath => "invalid path",
    }
}

impl AddressGenerationCommon {
    /// Display [`AddressGenerationCommon`] in readable form
    pub fn show(&self) -> String {
        match &self {
            AddressGenerationCommon::KeyCollision { seed_name } => {
                format!("Address key collision for seed name {}", seed_name)
            }
            AddressGenerationCommon::KeyCollisionBatch {
                seed_name_existing,
                seed_name_new,
                cropped_path_existing,
                cropped_path_new,
                in_this_network,
            } => {
                if *in_this_network {
                    format!("Tried to create colliding addresses within same network. Address for seed name {} and path {} has same public key as address for seed name {} and path {}.", seed_name_new, cropped_path_new, seed_name_existing, cropped_path_existing)
                } else {
                    format!("Tried to create colliding addresses within different networks. Address for seed name {} and path {} has same public key as address for seed name {} and path {}.", seed_name_new, cropped_path_new, seed_name_existing, cropped_path_existing)
                }
            }
            AddressGenerationCommon::SecretString(e) => {
                format!("Bad secret string: {}.", bad_secret_string(e))
            }
            AddressGenerationCommon::DerivationExists(
                multisigner,
                address_details,
                network_specs_key,
            ) => {
                let public_key = multisigner_to_public(multisigner);
                if address_details.has_pwd {
                    format!("Seed {} already has derivation {}///<password> for network specs key {}, public key {}.", address_details.seed_name, address_details.path, hex::encode(network_specs_key.key()), hex::encode(public_key))
                } else {
                    format!("Seed {} already has derivation {} for network specs key {}, public key {}.", address_details.seed_name, address_details.path, hex::encode(network_specs_key.key()), hex::encode(public_key))
                }
            }
        }
    }
}

/// Error decoding transfer content
///
/// All variants could be encountered both on the active side
/// (when checking the message content while signing it)
/// and on the Signer side (when processing the received messages)
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum TransferContent {
    /// `add_specs` message content
    AddSpecs,

    /// `load_metadata` message content
    LoadMeta,

    /// `load_types` message content
    LoadTypes,
}

impl TransferContent {
    /// Display [`TransferContent`] in readable form
    pub fn show(&self) -> String {
        let insert = match &self {
            TransferContent::AddSpecs => "`add_specs`",
            TransferContent::LoadMeta => "`load_meta`",
            TransferContent::LoadTypes => "`load_types`",
        };
        format!("Payload could not be decoded as {}.", insert)
    }
}

/// Intrinsic problems of the metadata making it unsuitable for Signer use
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum MetadataError {
    /// Supported are V12, V13, and V14 versions of
    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html).
    ///
    /// Any other version results in error.
    VersionIncompatible,

    /// Metadata does not have `System` pallet, i.e. there is no place to look
    /// for network
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    NoSystemPallet,

    /// Metadata does not have `Version` constant in `System` pallet, i.e.
    /// there is no place to look for network
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    NoVersionInConstants,

    /// `Vec<u8>` retrieved from `Version` constant in `System` pallet could
    /// not be decoded as
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    RuntimeVersionNotDecodeable,

    /// Metadata has `SS58Prefix` constant in `System` pallet, but its content
    /// could not be decoded as valid base58 prefix, i.e. as `u16` or `u8`
    /// number
    Base58PrefixNotDecodeable,

    /// Base58 prefix from metadata (`meta`) does not match base58 prefix in specs (`specs`)
    Base58PrefixSpecsMismatch { specs: u16, meta: u16 },

    /// Metadata first 4 bytes are not expected `b"meta"` prelude
    NotMeta,

    /// Metadata body (after `b"meta"` prelude) could not be decoded as
    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
    UnableToDecode,
}

impl MetadataError {
    /// Display [`MetadataError`] in readable form
    pub fn show(&self) -> String {
        match &self {
            MetadataError::VersionIncompatible => String::from("Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."),
            MetadataError::NoSystemPallet => String::from("No system pallet in runtime metadata."),
            MetadataError::NoVersionInConstants => String::from("No runtime version in system pallet constants."),
            MetadataError::RuntimeVersionNotDecodeable => String::from("Runtime version from system pallet constants could not be decoded."),
            MetadataError::Base58PrefixNotDecodeable => String::from("Base58 prefix is found in system pallet constants, but could not be decoded."),
            MetadataError::Base58PrefixSpecsMismatch{specs, meta} => format!("Base58 prefix {} from system pallet constants does not match the base58 prefix from network specs {}.", meta, specs),
            MetadataError::NotMeta => String::from("Metadata vector does not start with 0x6d657461."),
            MetadataError::UnableToDecode => String::from("Runtime metadata could not be decoded."),
        }
    }
}
