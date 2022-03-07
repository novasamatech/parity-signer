use sp_core::crypto::SecretStringError;
use sp_runtime::MultiSigner;

use crate::{crypto:: Encryption, helpers::multisigner_to_public, keyring::{AddressKey, MetaKey, NetworkSpecsKey}, users::AddressDetails};

/// Trait describing the origin of errors.
/// ErrorSource is implemented for Active (errors on the active side -
/// either hot database errors or errors while preparing cold database
/// before its moving into Signer)
/// and for Signer (errors on the Signer side)
pub trait ErrorSource {
///
/// Type specifies errors occuring on either Active or Signer side.
    type Error;
///
/// NotHex refers to errors in transforming hexadecimal strings into Vec<u8>.
/// NotHex errors may occur both on Active and on Signer side.
/// On Active side NotHex errors could be related to strings fetched form url,
/// input from command line, and processing of the default values.
/// On Signer side NotHex errors are caused by communication errors 
/// and, since user interface should be sending valid hex strings into rust,
/// generally should not be occuring.
    type NotHex;
///
/// Describes the source of the faulty NetworkSpecsKey
/// aside from the database itself (key in SpecsTree and value part in AddrTree).
/// On the Active side is empty.
/// On the Signer side the source could be only interface.
    type ExtraSpecsKeySource;
///
/// Describes the source of the faulty AddressSpecsKey
/// aside from the database itself.
/// On the active side is empty.
/// On the Signer side the source could be only interface.
    type ExtraAddressKeySource;
///
/// Incoming metadata source describes possible origins of the faulty metadata
/// excluding the database.
/// For Active side, it could be bad fetch or damaged default file.
/// For Signer side, it could be faulty received metadata.
    type IncomingMetadataSource;
///
/// Source-specific errors during address generation:
/// test address generation on Active side
/// and actual address generation on the Signer side
    type ExtraAddressGeneration;
///
/// Function to transform NotHex error into Error
    fn hex_to_error(what: Self::NotHex) -> Self::Error;
///
/// Function to create Error for bad network specs key
    fn specs_key_to_error(network_specs_key: &NetworkSpecsKey, source: SpecsKeySource<Self>) -> Self::Error;
///
/// Function to create Error for bad address key
    fn address_key_to_error(address_key: &AddressKey, source: AddressKeySource<Self>) -> Self::Error;
///
/// Function to create Error for bad meta key (database only both on the Active side
/// and on the Signer side)
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error;
///
/// Function to create errors for metadata name and/or version mismatch within the database
    fn metadata_mismatch (name_key: String, version_key: u32, name_inside: String, version_inside: u32) -> Self::Error;
///
/// Function to create errors related to metadata being unsuitable
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error;
///
/// Functions to create errors related to network specs NetworkSpecs 
/// (is used on Signer side, but also on Active side, for example, during metadata transfer in test cold database)
/// Function to generate error in the event that NetworkSpecs entry could not be decoded...
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error;
/// ... or has mismatch in genesis hash between key and stored value,
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: Vec<u8>) -> Self::Error;
/// ... or has mismatch in encryption between key and stored value,
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error;
///
/// Functions to create errors related to address details AddressDetails
/// (intended for Signer side, but could appear on the Active side during test identities generation)
/// Function to generate error in the event that AddressDetails entry could not be decoded...
    fn address_details_decoding(key: AddressKey) -> Self::Error;
/// ... or has mismatch in encryption between key and stored value,
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error;
/// ... or has mismatch between encryption within address details and encryption
/// of an associated network
    fn address_details_specs_encryption_mismatch(address_key: AddressKey, network_specs_key: NetworkSpecsKey) -> Self::Error;
///
/// Function to generate error in case of a common address generation error
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error;
///
/// Function to create error corresponding to faulty transfer content
    fn transfer_content_error (transfer_content: TransferContent) -> Self::Error;
///
/// Functions to create error corresponding to database internal error...
    fn db_internal (error: sled::Error) -> Self::Error;
/// ... and database transaction error
    fn db_transaction (error: sled::transaction::TransactionError) -> Self::Error;
///
/// Functions to create error corresponding to the situation 
/// when types information from the database that could not be decoded...
    fn faulty_database_types() -> Self::Error;
/// ... or could not be found at all
    fn types_not_found() -> Self::Error;
///
/// Function to create error when metadata for the network with given name and version was not found
    fn metadata_not_found(name: String, version: u32) -> Self::Error;
///
/// Function to print the Error, is used in cards, for anyhow or std::fmt::Display output
    fn show(error: &Self::Error) -> String;
}

pub enum MetadataSource <T: ErrorSource + ?Sized> {
    Database {name: String, version: u32},
    Incoming(T::IncomingMetadataSource),
}

pub enum SpecsKeySource <T: ErrorSource + ?Sized> {
    SpecsTree,
    AddrTree(AddressKey),
    Extra(T::ExtraSpecsKeySource),
}

pub enum AddressKeySource <T: ErrorSource + ?Sized> {
    AddrTree,
    Extra(T::ExtraAddressKeySource),
}

#[derive(Debug)]
pub enum AddressGeneration <T: ErrorSource + ?Sized> {
    Common(AddressGenerationCommon),
    Extra(T::ExtraAddressGeneration),
}

#[derive(Debug)]
pub enum AddressGenerationCommon {
    EncryptionMismatch{network_encryption: Encryption, seed_object_encryption: Encryption},
    KeyCollision{seed_name: String},
    SecretString(SecretStringError),
    DerivationExists(MultiSigner, AddressDetails, NetworkSpecsKey),
}

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
    pub fn show(&self) -> String {
        match &self {
            AddressGenerationCommon::EncryptionMismatch{network_encryption, seed_object_encryption} => format!("Network encryption {} is different from seed object encryption {}.", network_encryption.show(), seed_object_encryption.show()),
            AddressGenerationCommon::KeyCollision{seed_name} => format!("Address key collision for seed name {}", seed_name),
            AddressGenerationCommon::SecretString(e) => format!("Bad secret string: {}.", bad_secret_string(e)),
            AddressGenerationCommon::DerivationExists(multisigner, address_details, network_specs_key) => {
                let public_key = multisigner_to_public(multisigner);
                if address_details.has_pwd {
                    format!("Seed {} already has derivation {}///<password> for network specs key {}, public key {}.", address_details.seed_name, address_details.path, hex::encode(network_specs_key.key()), hex::encode(public_key))
                }
                else {
                    format!("Seed {} already has derivation {} for network specs key {}, public key {}.", address_details.seed_name, address_details.path, hex::encode(network_specs_key.key()), hex::encode(public_key))
                }
            },
        }
    }
}

/// Enum to specify errors occuring with decoding transfer content,
/// all of its variants could be encountered both on the Active side
/// (when checking the message content while signing it)
/// and on the cold side (when processing the received messages)
#[derive(Debug)]
pub enum TransferContent {
    AddSpecs,
    LoadMeta,
    LoadTypes,
}

impl TransferContent {
    pub fn show(&self) -> String {
        let insert = match &self {
            TransferContent::AddSpecs => "`add_specs`",
            TransferContent::LoadMeta => "`load_meta`",
            TransferContent::LoadTypes => "`load_types`",
        };
        format!("Payload could not be decoded as {}.", insert)
    }
}

/// Enum to describe intrinsic problems of the metadata
#[derive(Debug)]
pub enum MetadataError {
    VersionIncompatible,
    NoSystemPallet,
    NoVersionInConstants,
    RuntimeVersionNotDecodeable,
    Base58PrefixNotDecodeable,
    Base58PrefixSpecsMismatch{specs: u16, meta: u16},
    NotMeta,
    UnableToDecode,
}

impl MetadataError {
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
