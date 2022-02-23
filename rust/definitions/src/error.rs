use anyhow::anyhow;
use hex;
use png;
use sled;
use sp_core::crypto::SecretStringError;
use sp_runtime::MultiSigner;
use wasm_testbed;

use crate::{crypto:: Encryption, helpers::multisigner_to_public, keyring::{AddressKey, AddressBookKey, MetaKey, NetworkSpecsKey, VerifierKey}, network_specs::{ValidCurrentVerifier, Verifier, VerifierValue}, users::AddressDetails};

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

fn bad_secret_string(e: &SecretStringError) -> &'static str {
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
                let public_key = multisigner_to_public(&multisigner);
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

/// Enum-marker indicating that error originates on the Active side
#[derive(Debug)]
pub enum Active {}

impl ErrorSource for Active {
    type Error = ErrorActive;
    type NotHex = NotHexActive;
    type ExtraSpecsKeySource = ExtraSpecsKeySourceActive;
    type ExtraAddressKeySource = ExtraAddressKeySourceActive;
    type IncomingMetadataSource = IncomingMetadataSourceActive;
    type ExtraAddressGeneration = ExtraAddressGenerationActive;
    fn hex_to_error(what: Self::NotHex) -> Self::Error {
        ErrorActive::NotHex(what)
    }
    fn specs_key_to_error(network_specs_key: &NetworkSpecsKey, source: SpecsKeySource<Self>) -> Self::Error {
        match source {
            SpecsKeySource::SpecsTree => ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::NetworkSpecsKey(network_specs_key.to_owned()))),
            SpecsKeySource::AddrTree(address_key) => ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::NetworkSpecsKeyAddressDetails{address_key: address_key.to_owned(), network_specs_key: network_specs_key.to_owned()})),
            SpecsKeySource::Extra(_) => unreachable!(),
        }
    }
    fn address_key_to_error(address_key: &AddressKey, _source: AddressKeySource<Self>) -> Self::Error {
        ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::AddressKey(address_key.to_owned())))
    }
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::MetaKey(meta_key.to_owned())))
    }
    fn metadata_mismatch (name_key: String, version_key: u32, name_inside: String, version_inside: u32) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::Metadata{name_key, version_key, name_inside, version_inside}))
    }
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error {
        match source {
            MetadataSource::Database{name, version} => ErrorActive::Database(DatabaseActive::FaultyMetadata{name, version, error}),
            MetadataSource::Incoming(IncomingMetadataSourceActive::Str(IncomingMetadataSourceActiveStr::Fetch{url})) => ErrorActive::Fetch(Fetch::FaultyMetadata{url, error}),
            MetadataSource::Incoming(IncomingMetadataSourceActive::Str(IncomingMetadataSourceActiveStr::Default{filename})) => ErrorActive::DefaultLoading(DefaultLoading::FaultyMetadata{filename, error}),
            MetadataSource::Incoming(IncomingMetadataSourceActive::Wasm{filename}) => ErrorActive::Wasm(Wasm::FaultyMetadata{filename, error}),
        }
    }
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::NetworkSpecs(key)))
    }
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: Vec<u8>) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsGenesisHash{key, genesis_hash}))
    }
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsEncryption{key, encryption}))
    }
    fn address_details_decoding(key: AddressKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::AddressDetails(key)))
    }
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::AddressDetailsEncryption{key, encryption}))
    }
    fn address_details_specs_encryption_mismatch(address_key: AddressKey, network_specs_key: NetworkSpecsKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::AddressDetailsSpecsEncryption{address_key, network_specs_key}))
    }
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error {
        ErrorActive::TestAddressGeneration(AddressGeneration::Common(error))
    }
    fn transfer_content_error (transfer_content: TransferContent) -> Self::Error {
        ErrorActive::TransferContent(transfer_content)
    }
    fn db_internal (error: sled::Error) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Internal(error))
    }
    fn db_transaction (error: sled::transaction::TransactionError) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Transaction(error))
    }
    fn faulty_database_types() -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::Types))
    }
    fn types_not_found() -> Self::Error {
        ErrorActive::NotFound(NotFoundActive::Types)
    }
    fn metadata_not_found(name: String, version: u32) -> Self::Error {
        ErrorActive::NotFound(NotFoundActive::Metadata{name, version})
    }
    fn show(error: &Self::Error) -> String {
        match error {
            ErrorActive::NotHex(a) => {
                let insert = match a {
                    NotHexActive::FetchedMetadata {url} => format!("Network metadata fetched from url {}", url),
                    NotHexActive::FetchedGenesisHash {url} => format!("Network genesis hash fetched from url {}", url),
                    NotHexActive::InputSufficientCrypto => String::from("Input sufficient crypto data"),
                    NotHexActive::InputPublicKey => String::from("Input public key"),
                    NotHexActive::InputSignature => String::from("Input signature"),
                    NotHexActive::DefaultMetadata {filename} => format!("Default network metadata from file {}", filename),
                };
                format!("{} is not in hexadecimal format.", insert)
            },
            ErrorActive::TransferContent(a) => a.show(),
            ErrorActive::Database(a) => {
                let insert = match a {
                    DatabaseActive::KeyDecoding(b) => {
                        let insert = match b {
                            KeyDecodingActive::AddressBookKey(x) => format!("address book key {}", hex::encode(x.key())),
                            KeyDecodingActive::AddressKey(x) => format!("address key {}", hex::encode(x.key())),
                            KeyDecodingActive::MetaKey(x) => format!("meta key {}", hex::encode(x.key())),
                            KeyDecodingActive::NetworkSpecsKey(x) => format!("network specs key {}", hex::encode(x.key())),
                            KeyDecodingActive::NetworkSpecsKeyAddressDetails{address_key, network_specs_key} => format!("network specs key {} from network id set of address book entry with key {}", hex::encode(network_specs_key.key()), hex::encode(address_key.key())),
                        };
                        format!("Unable to parse {} from the database.", insert)
                    },
                    DatabaseActive::Internal(e) => format!("Internal error. {}", e),
                    DatabaseActive::Transaction(e) => format!("Transaction error. {}", e),
                    DatabaseActive::EntryDecoding(b) => {
                        let insert = match b {
                            EntryDecodingActive::AddressBookEntryKey(x) => format!("address book entry for key {}.", hex::encode(x.key())),
                            EntryDecodingActive::AddressBookEntryTitle{title} => format!("address book entry for title {}.", title),
                            EntryDecodingActive::AddressDetails(x) => format!("address details entry for key {}.", hex::encode(x.key())),
                            EntryDecodingActive::NetworkSpecs(x) => format!("network specs (NetworkSpecs) entry for key {}.", hex::encode(x.key())),
                            EntryDecodingActive::NetworkSpecsToSend(x) => format!("network specs (NetworkSpecsToSend) entry for key {}.", hex::encode(x.key())),
                            EntryDecodingActive::Types => String::from("types information."),
                        };
                        format!("Unable to decode {}", insert)
                    },
                    DatabaseActive::Mismatch(b) => {
                        let insert = match b {
                            MismatchActive::Metadata{name_key, version_key, name_inside, version_inside} => format!("Meta key corresponds to {}{}. Stored metadata is {}{}.", name_key, version_key, name_inside, version_inside),
                            MismatchActive::SpecsGenesisHash{key, genesis_hash} => format!("Network specs (NetworkSpecs) entry with network specs key {} has wrong genesis hash {}.", hex::encode(key.key()), hex::encode(genesis_hash)),
                            MismatchActive::SpecsEncryption{key, encryption} => format!("Network specs (NetworkSpecs) entry with network specs key {} has wrong encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchActive::SpecsToSendGenesisHash{key, genesis_hash} => format!("Network specs (NetworkSpecsToSend) entry with network specs key {} has wrong genesis hash {}.", hex::encode(key.key()), hex::encode(genesis_hash)),
                            MismatchActive::SpecsToSendEncryption{key, encryption} => format!("Network specs (NetworkSpecsToSend) entry with network specs key {} has wrong encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchActive::AddressDetailsEncryption{key, encryption} => format!("Address details entry with address key {} has not matching encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchActive::AddressDetailsSpecsEncryption{address_key, network_specs_key} => format!("Address details entry with address key {} has associated network specs key {} with wrong encryption.", hex::encode(address_key.key()), hex::encode(network_specs_key.key())),
                            MismatchActive::AddressBookSpecsName{address_book_name, specs_name} => format!("Address book name {} does not match the name in corresponding network specs {}", address_book_name, specs_name),
                        };
                        format!("Mismatch found. {}", insert)
                    },
                    DatabaseActive::FaultyMetadata{name, version, error} => format!("Bad metadata for {}{}. {}", name, version, error.show()),
                    DatabaseActive::TwoEntriesAddressEncryption{url, encryption} => format!("Hot database contains two entries for network with url {} and encryption {}.", url, encryption.show()),
                    DatabaseActive::TwoDefaultsAddress{url} => format!("Hot database contains two default entries for network with url {}.", url),
                    DatabaseActive::HotDatabaseMetadataOverTwoEntries{name} => format!("More than two entries for network {} in hot database.", name),
                    DatabaseActive::HotDatabaseMetadataSameVersionTwice{name, version} => format!("Two entries for {} version {}.", name, version),
                    DatabaseActive::NewAddressKnownGenesisHash{url, genesis_hash} => format!("Url address {} is not encountered in the hot database entries, however, fetched genesis hash {} is present in hot database entries. To change the network url, delete old entry.", url, hex::encode(genesis_hash)),
                    DatabaseActive::TwoGenesisHashVariantsForName{name} => format!("Two different genesis hash entries for network {} in address book.", name),
                    DatabaseActive::TwoUrlVariantsForName{name} => format!("Two different url entries for network {} in address book.", name),
                    DatabaseActive::TwoNamesForUrl{url} => format!("Two different network names in entries for url address {} in address book.", url),
                    DatabaseActive::TwoBase58ForName{name} => format!("Two different base58 entries for network {}.", name),
                    DatabaseActive::AddressBookEmpty => String::from("Address book is empty"),
                };
                format!("Database error. {}", insert)
            },
            ErrorActive::Fetch(a) => {
                let insert = match a {
                    Fetch::FaultyMetadata{url, error} => format!("Metadata from {} is not suitable. {}", url, error.show()),
                    Fetch::EarlierVersion{name, old_version, new_version} => format!("For {} the fetched version ({}) is lower than the latest version in the hot database ({}).", name, new_version, old_version),
                    Fetch::SameVersionDifferentMetadata{name, version} => format!("Fetched metadata for {}{} differs from the one in the hot database.", name, version),
                    Fetch::FaultySpecs{url, error} => {
                        let insert = match error {
                            SpecsError::NoBase58Prefix => String::from("No base58 prefix."),
                            SpecsError::Base58PrefixMismatch{specs, meta} => format!("Base58 prefix from fetched properties {} does not match base58 prefix in fetched metadata {}.", specs, meta),
                            SpecsError::Base58PrefixFormatNotSupported{value} => format!("Base58 prefix {} does not fit into u16.", value),
                            SpecsError::UnitNoDecimals(x) => format!("Network has units declared: {}, but no decimals.", x),
                            SpecsError::DecimalsFormatNotSupported{value} => format!("Decimals value {} does not fit into u8.", value),
                            SpecsError::DecimalsNoUnit(x) => format!("Network has decimals declared: {}, but no units.", x),
                            SpecsError::UnitFormatNotSupported{value} => format!("Units {} are not String.", value),
                            SpecsError::DecimalsArrayUnitsNot => String::from("Unexpected result for multi-token network. Decimals are fetched as an array with more than one element, whereas units are not."),
                            SpecsError::DecimalsUnitsArrayLength{decimals, unit} => format!("Unexpected result for multi-token network. Length of decimals array {} does not match the length of units array {}.", decimals, unit),
                            SpecsError::UnitsArrayDecimalsNot => String::from("Unexpected result for multi-token network. Units are fetched as an array with more than one element, whereas decimals are not."),
                            SpecsError::OverrideIgnored => String::from("Fetched single value for token decimals and unit. Token override is not possible."),
                        };
                        format!("Problem with network specs from {}. {}", url, insert)
                    },
                    Fetch::Failed{url, error} => format!("Could not make rpc call at {}. {}", url, error),
                    Fetch::ValuesChanged{url, what} => {
                        let (insert, old, new) = match what {
                            Changed::Base58Prefix{old, new} => ("base58 prefix", old.to_string(), new.to_string()),
                            Changed::GenesisHash{old, new} => ("genesis hash", hex::encode(old), hex::encode(new)),
                            Changed::Decimals{old, new} => ("decimals value", old.to_string(), new.to_string()),
                            Changed::Name{old, new} => ("name", old.to_string(), new.to_string()),
                            Changed::Unit{old, new} => ("unit", old.to_string(), new.to_string()),
                        };
                        format!("Network {} fetched from {} differs from the one in the hot database. Old: {}. New: {}.", insert, url, old, new)
                    },
                    Fetch::UnexpectedFetchedGenesisHashFormat{value} => format!("Fetched genesis hash {} has unexpected format and does not fit into [u8;32] array.", value),
                    Fetch::SpecsInDb{name, encryption} => format!("Network specs entry for {} and encryption {} is already in database.", name, encryption.show()),
                };
                format!("Fetching error. {}", insert)
            },
            ErrorActive::DefaultLoading(a) => {
                let insert = match a {
                    DefaultLoading::FaultyMetadata{filename, error} => format!("Default metadata from {} is not suitable. {}", filename, error.show()),
                    DefaultLoading::MetadataFolder(e) => format!("Error with default metadata folder. {}", e),
                    DefaultLoading::MetadataFile(e) => format!("Error with default metadata file. {}", e),
                    DefaultLoading::TypesFile(e) => format!("Error with default types information file. {}", e),
                    DefaultLoading::OrphanMetadata{name, filename} => format!("Default metadata for {} from {} does not have corresponding default network specs.", name, filename),
                };
                format!("Error on loading defaults. {}", insert)
            },
            ErrorActive::Output(e) => format!("Output error. {}", e),
            ErrorActive::NotFound(a) => {
                let insert = match a {
                    NotFoundActive::Types => String::from("types information"),
                    NotFoundActive::Metadata{name, version} => format!("metadata entry for {}{}", name, version),
                    NotFoundActive::AddressBookEntry{title} => format!("address book for title {}", title),
                    NotFoundActive::NetworkSpecsToSend(key) => format!("network specs (NetworkSpecsToSend) entry for key {}", hex::encode(key.key())),
                    NotFoundActive::AddressBookEntryWithName{name} => format!("address book entry for network name {}", name),
                    NotFoundActive::AddressBookEntryWithUrl{url} => format!("address book entry with url address {}", url),
                };
                format!("Could not find {}.", insert)
            },
            ErrorActive::TestAddressGeneration(a) => {
                let insert = match a {
                    AddressGeneration::Common(a) => a.show(),
                    AddressGeneration::Extra(_) => unreachable!(),
                };
                format!("Error generating test address. {}", insert)
            },
            ErrorActive::CommandParser(a) => {
                match a {
                    CommandParser::UnexpectedKeyArgumentSequence => String::from("Unexpected key and argument sequence."),
                    CommandParser::OnlyOneNetworkId => String::from("Only one network identifier is allowed."),
                    CommandParser::NeedKey(b) => {
                        let insert = match b {
                            CommandNeedKey::Show => "`show`",
                            CommandNeedKey::Content => "content",
                            CommandNeedKey::Crypto => "`-crypto`",
                            CommandNeedKey::Payload => "`-payload`",
                            CommandNeedKey::MsgType => "`-msgtype`",
                            CommandNeedKey::SufficientCrypto => "`-sufficient`",
                            CommandNeedKey::Signature => "`-signature`",
                            CommandNeedKey::Verifier => "`-verifier`",
                            CommandNeedKey::Remove => "`-title` or `-name`",
                            CommandNeedKey::RemoveVersion => "`-version`",
                            CommandNeedKey::DerivationsTitle => "'-title'",
                        };
                        format!("Expected {} key to be used.", insert)
                    },
                    CommandParser::DoubleKey(b) => {
                        let insert = match b {
                            CommandDoubleKey::Content => "content",
                            CommandDoubleKey::Set => "set",
                            CommandDoubleKey::CryptoOverride => "encryption override",
                            CommandDoubleKey::TokenOverride => "token override",
                            CommandDoubleKey::CryptoKey => "`-crypto`",
                            CommandDoubleKey::MsgType => "`-msgtype`",
                            CommandDoubleKey::Verifier => "`-verifier`",
                            CommandDoubleKey::Payload => "`-payload`",
                            CommandDoubleKey::Signature => "`-signature`",
                            CommandDoubleKey::Name => "`-name`",
                            CommandDoubleKey::SufficientCrypto => "`-sufficient`",
                            CommandDoubleKey::Remove => "`-remove`",
                            CommandDoubleKey::DerivationsTitle => "'-title'",
                        };
                        format!("More than one entry for {} key is not allowed.", insert)
                    },
                    CommandParser::NeedArgument(b) => {
                        let insert = match b {
                            CommandNeedArgument::TokenUnit => "`-token ***'",
                            CommandNeedArgument::TokenDecimals => "'-token'",
                            CommandNeedArgument::NetworkName => "`-n`",
                            CommandNeedArgument::NetworkUrl => "`-u`",
                            CommandNeedArgument::CryptoKey => "`-crypto`",
                            CommandNeedArgument::MsgType => "`-msgtype`",
                            CommandNeedArgument::Verifier => "`-verifier`",
                            CommandNeedArgument::VerifierHex => "`-verifier -hex`",
                            CommandNeedArgument::VerifierFile => "`-verifier -file`",
                            CommandNeedArgument::Payload => "`-payload`",
                            CommandNeedArgument::Signature => "`-signature`",
                            CommandNeedArgument::SignatureHex => "`-signature -hex`",
                            CommandNeedArgument::SignatureFile => "`-signature -file`",
                            CommandNeedArgument::Name => "`-name`",
                            CommandNeedArgument::SufficientCrypto => "`-sufficient`",
                            CommandNeedArgument::SufficientCryptoHex => "`-sufficient -hex`",
                            CommandNeedArgument::SufficientCryptoFile => "`-sufficient -file`",
                            CommandNeedArgument::Make => "'make'",
                            CommandNeedArgument::Sign => "'sign'",
                            CommandNeedArgument::RemoveTitle => "`remove -title`",
                            CommandNeedArgument::RemoveName => "`remove -name`",
                            CommandNeedArgument::RemoveVersion => "`remove -name *** -version`",
                            CommandNeedArgument::Derivations => "'derivations'",
                            CommandNeedArgument::DerivationsTitle => "'-title'",
                        };
                        format!("{} must be followed by an agrument.", insert)
                    },
                    CommandParser::BadArgument(b) => {
                        let insert = match b {
                            CommandBadArgument::CryptoKey => "`-crypto`",
                            CommandBadArgument::MsgType => "`-msgtype`",
                            CommandBadArgument::Verifier => "`-verifier`",
                            CommandBadArgument::Signature => "`-signature`",
                            CommandBadArgument::SufficientCrypto => "`-sufficient`",
                        };
                        format!("Invalid argument after {} key.", insert)
                    },
                    CommandParser::Unexpected(b) => {
                        match b {
                            CommandUnexpected::DecimalsFormat => String::from("Key `-token` should be followed by u8 decimals value."),
                            CommandUnexpected::KeyAContent => String::from("Key `-a` is used to process all, name or url was not expected."),
                            CommandUnexpected::VerifierNoCrypto => String::from("No verifier entry was expected for `-crypto none` sequence."),
                            CommandUnexpected::SignatureNoCrypto => String::from("No singature entry was expected for `-crypto none` sequence."),
                            CommandUnexpected::AliceSignature => String::from("No signature was expected for verifier Alice."),
                            CommandUnexpected::VersionFormat => String::from("Unexpected version format."),
                        }
                    },
                    CommandParser::UnknownCommand => String::from("Unknown command."),
                    CommandParser::NoCommand => String::from("No command."),
                }
            },
            ErrorActive::Input(a) => {
                match a {
                    InputActive::File(e) => format!("Error with input file. {}", e),
                    InputActive::DecodingSufficientCrypto => String::from("Unable to decode input sufficient crypto"),
                    InputActive::PublicKeyLength => String::from("Provided verifier public key has wrong length."),
                    InputActive::SignatureLength => String::from("Provided signature has wrong length."),
                    InputActive::FaultyMetadataInPayload(e) => format!("Metadata in the message to sign is not suitable. {}", e.show()),
                    InputActive::BadSignature => String::from("Bad signature."),
                    InputActive::NoValidDerivationsToExport => String::from("No valid password-free derivations found to generate ContentDerivations."),
                }
            },
            ErrorActive::Qr(e) => format!("Error generating qr code. {}", e),
            ErrorActive::NotSupported => String::from("Key combination is not supported. Please file a ticket if you need it."),
            ErrorActive::NoTokenOverrideKnownNetwork{url} => format!("Network with corresponding url {} has database records. Token override is not supported.", url),
            ErrorActive::Wasm(a) => {
                match a {
                    Wasm::WasmTestbed(e) => format!("WasmTestbed error. {}", e),
                    Wasm::FaultyMetadata{filename, error} => format!("Metadata error in .wasm file {}. {}", filename, error.show()),
                }
            },
        }
    }
}

/// Enum listing all variants of errors from the Active side
#[derive(Debug)]
pub enum ErrorActive {
    Database(DatabaseActive),
    NotHex(NotHexActive),
    TransferContent(TransferContent),
    Fetch(Fetch),
    DefaultLoading(DefaultLoading),
    Output(std::io::Error),
    NotFound(NotFoundActive),
    TestAddressGeneration(AddressGeneration<Active>),
    CommandParser(CommandParser),
    Input(InputActive),
    Qr(String),
    NotSupported,
    NoTokenOverrideKnownNetwork{url: String},
    Wasm(Wasm),
}

/// Active side errors could be displayed standardly
impl std::fmt::Display for ErrorActive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Active>::show(self))
    }
}

/// NotHex errors occuring on the Active side
#[derive(Debug)]
pub enum NotHexActive {
    FetchedMetadata {url: String},
    FetchedGenesisHash {url: String},
    InputSufficientCrypto,
    InputPublicKey,
    InputSignature,
    DefaultMetadata {filename: String},
}

/// Origin of unsuitable metadata on the Active side
#[derive(Debug)]
pub enum IncomingMetadataSourceActive {
    Str(IncomingMetadataSourceActiveStr),
    Wasm{filename: String},
}

/// Origin of unsuitable metadata on the Active side, in str form
#[derive(Debug)]
pub enum IncomingMetadataSourceActiveStr {
    Fetch{url: String},
    Default{filename: String},
}

/// Origin of unsuitable specs key on the Active side, except SpecsTree in the database.
/// Is empty.
#[derive(Debug)]
pub enum ExtraSpecsKeySourceActive {}

/// Origin of unsuitable address key on the Active side, except AddrTree in the database.
/// Is empty.
#[derive(Debug)]
pub enum ExtraAddressKeySourceActive {}

/// Source-specific address generation errors on the Active side.
/// Is empty.
#[derive(Debug)]
pub enum ExtraAddressGenerationActive {}

/// Enum listing all variants of errors related to database on Active side
#[derive(Debug)]
pub enum DatabaseActive {
    KeyDecoding(KeyDecodingActive),
    EntryDecoding(EntryDecodingActive),
    Internal(sled::Error),
    Transaction(sled::transaction::TransactionError),
    Mismatch(MismatchActive),
    FaultyMetadata{name: String, version: u32, error: MetadataError},
    TwoEntriesAddressEncryption{url: String, encryption: Encryption},
    TwoDefaultsAddress{url: String},
    HotDatabaseMetadataOverTwoEntries{name: String},
    HotDatabaseMetadataSameVersionTwice{name: String, version: u32},
    NewAddressKnownGenesisHash{url: String, genesis_hash: [u8;32]},
    TwoGenesisHashVariantsForName{name: String},
    TwoUrlVariantsForName{name: String},
    TwoNamesForUrl{url: String},
    TwoBase58ForName{name: String},
    AddressBookEmpty,
}

/// Enum listing possible errors in decoding keys from the database on the Active side
#[derive(Debug)]
pub enum KeyDecodingActive {
    AddressBookKey(AddressBookKey),
    AddressKey(AddressKey),
    MetaKey(MetaKey),
    NetworkSpecsKey(NetworkSpecsKey),
    NetworkSpecsKeyAddressDetails{address_key: AddressKey, network_specs_key: NetworkSpecsKey},
}

/// Enum listing possible errors in decoding database entry content on the Active side
#[derive(Debug)]
pub enum EntryDecodingActive {
    AddressBookEntryKey(AddressBookKey),
    AddressBookEntryTitle{title: String},
    AddressDetails(AddressKey),
    NetworkSpecs(NetworkSpecsKey),
    NetworkSpecsToSend(NetworkSpecsKey),
    Types,
}

/// Enum listing possible mismatch within database on the Active side
#[derive(Debug)]
pub enum MismatchActive {
    Metadata{name_key: String, version_key: u32, name_inside: String, version_inside: u32},
    SpecsGenesisHash{key: NetworkSpecsKey, genesis_hash: Vec<u8>},
    SpecsEncryption{key: NetworkSpecsKey, encryption: Encryption},
    SpecsToSendGenesisHash{key: NetworkSpecsKey, genesis_hash: Vec<u8>},
    SpecsToSendEncryption{key: NetworkSpecsKey, encryption: Encryption},
    AddressDetailsEncryption{key: AddressKey, encryption: Encryption},
    AddressDetailsSpecsEncryption{address_key: AddressKey, network_specs_key: NetworkSpecsKey},
    AddressBookSpecsName{address_book_name: String, specs_name: String},
}

/// Enum listing possible errors on the Active side related to fetched (or not fetched) data
#[derive(Debug)]
pub enum Fetch {
    FaultyMetadata{url: String, error: MetadataError},
    EarlierVersion{name: String, old_version: u32, new_version: u32},
    SameVersionDifferentMetadata{name: String, version: u32},
    FaultySpecs{url: String, error: SpecsError},
    Failed{url: String, error: String},
    ValuesChanged{url: String, what: Changed},
    UnexpectedFetchedGenesisHashFormat{value: String},
    SpecsInDb{name: String, encryption: Encryption},
}

#[derive(Debug)]
pub enum SpecsError {
    NoBase58Prefix,
    Base58PrefixMismatch{specs: u16, meta: u16},
    Base58PrefixFormatNotSupported{value: String},
    UnitNoDecimals(String),
    DecimalsFormatNotSupported{value: String},
    DecimalsNoUnit(u8),
    UnitFormatNotSupported{value: String},
    DecimalsArrayUnitsNot,
    DecimalsUnitsArrayLength{decimals: String, unit: String},
    UnitsArrayDecimalsNot,
    OverrideIgnored,
}

#[derive(Debug)]
pub enum Changed {
    Base58Prefix{old: u16, new: u16},
    GenesisHash{old: [u8;32], new: [u8;32]},
    Decimals{old: u8, new: u8},
    Name{old: String, new: String},
    Unit{old: String, new: String},
}

/// Enum listing possible errors on the Active side related to loading of the defaults
#[derive(Debug)]
pub enum DefaultLoading {
    FaultyMetadata{filename: String, error: MetadataError},
    MetadataFolder(std::io::Error),
    MetadataFile(std::io::Error),
    TypesFile(std::io::Error),
    OrphanMetadata{name: String, filename: String},
}

/// Enum listing errors for cases when something was needed from the Active database and was not found
#[derive(Debug)]
pub enum NotFoundActive {
    Types,
    Metadata{name: String, version: u32},
    AddressBookEntry{title: String},
    NetworkSpecsToSend(NetworkSpecsKey),
    AddressBookEntryWithName{name: String},
    AddressBookEntryWithUrl{url: String},
}

/// Enum listing errors with command line parser from the `generate_message` crate
#[derive(Debug)]
pub enum CommandParser {
    UnexpectedKeyArgumentSequence,
    OnlyOneNetworkId,
    NeedKey(CommandNeedKey),
    DoubleKey(CommandDoubleKey),
    NeedArgument(CommandNeedArgument),
    BadArgument(CommandBadArgument),
    Unexpected(CommandUnexpected),
    UnknownCommand,
    NoCommand,
}

/// Enum listing command line parser errors conserning missing key
#[derive(Debug)]
pub enum CommandNeedKey {
    Show,
    Content,
    Crypto,
    Payload,
    MsgType,
    SufficientCrypto,
    Signature,
    Verifier,
    Remove,
    RemoveVersion,
    DerivationsTitle,
}

/// Enum listing command line parser errors conserning key encountered twice
#[derive(Debug)]
pub enum CommandDoubleKey {
    Content,
    Set,
    CryptoOverride,
    TokenOverride,
    CryptoKey,
    MsgType,
    Verifier,
    Payload,
    Signature,
    Name,
    SufficientCrypto,
    Remove,
    DerivationsTitle,
}

/// Enum listing command line parser errors conserning missing key argument
#[derive(Debug)]
pub enum CommandNeedArgument {
    TokenUnit,
    TokenDecimals,
    NetworkName,
    NetworkUrl,
    CryptoKey,
    Verifier,
    VerifierHex,
    VerifierFile,
    Payload,
    MsgType,
    Signature,
    SignatureHex,
    SignatureFile,
    Name,
    SufficientCrypto,
    SufficientCryptoHex,
    SufficientCryptoFile,
    Make,
    Sign,
    RemoveTitle,
    RemoveName,
    RemoveVersion,
    Derivations,
    DerivationsTitle,
}

/// Enum listing command line parser errors conserning an unsuitable key argument
#[derive(Debug)]
pub enum CommandBadArgument {
    CryptoKey,
    MsgType,
    Verifier,
    Signature,
    SufficientCrypto,
}

/// Enum listing command line parser errors conserning unexpected command content
#[derive(Debug)]
pub enum CommandUnexpected {
    DecimalsFormat,
    KeyAContent,
    VerifierNoCrypto,
    SignatureNoCrypto,
    AliceSignature,
    VersionFormat,
}

/// Enum listing possible input errors
#[derive(Debug)]
pub enum InputActive {
    File(std::io::Error),
    DecodingSufficientCrypto,
    PublicKeyLength,
    SignatureLength,
    FaultyMetadataInPayload(MetadataError),
    BadSignature,
    NoValidDerivationsToExport,
}

/// Enum listing possible errors with .wasm files processing
#[derive(Debug)]
pub enum Wasm {
    WasmTestbed(wasm_testbed::WasmTestbedError),
    FaultyMetadata{filename: String, error: MetadataError},
}

/// Enum-marker indicating that error originates on the Signer side
#[derive(Debug)]
pub enum Signer {}

impl ErrorSource for Signer {
    type Error = ErrorSigner;
    type NotHex = NotHexSigner;
    type ExtraSpecsKeySource = ExtraSpecsKeySourceSigner;
    type ExtraAddressKeySource = ExtraAddressKeySourceSigner;
    type IncomingMetadataSource = IncomingMetadataSourceSigner;
    type ExtraAddressGeneration = ExtraAddressGenerationSigner;
    fn hex_to_error(what: Self::NotHex) -> Self::Error {
        ErrorSigner::Interface(InterfaceSigner::NotHex(what))
    }
    fn specs_key_to_error(network_specs_key: &NetworkSpecsKey, source: SpecsKeySource<Self>) -> Self::Error {
        match source {
            SpecsKeySource::SpecsTree => ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::NetworkSpecsKey(network_specs_key.to_owned()))),
            SpecsKeySource::AddrTree(address_key) => ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails{address_key: address_key.to_owned(), network_specs_key: network_specs_key.to_owned()})),
            SpecsKeySource::Extra(ExtraSpecsKeySourceSigner::Interface) => ErrorSigner::Interface(InterfaceSigner::KeyDecoding(KeyDecodingSignerInterface::NetworkSpecsKey(network_specs_key.to_owned()))),
        }
        
    }
    fn address_key_to_error(address_key: &AddressKey, source: AddressKeySource<Self>) -> Self::Error {
        match source {
            AddressKeySource::AddrTree => ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::AddressKey(address_key.to_owned()))),
            AddressKeySource::Extra(ExtraAddressKeySourceSigner::Interface) => ErrorSigner::Interface(InterfaceSigner::KeyDecoding(KeyDecodingSignerInterface::AddressKey(address_key.to_owned()))),
        }
    }
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::MetaKey(meta_key.to_owned())))
    }
    fn metadata_mismatch (name_key: String, version_key: u32, name_inside: String, version_inside: u32) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::Metadata{name_key, version_key, name_inside, version_inside}))
    }
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error {
        match source {
            MetadataSource::Database{name, version} => ErrorSigner::Database(DatabaseSigner::FaultyMetadata{name, version, error}),
            MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData) => ErrorSigner::Input(InputSigner::FaultyMetadata(error)),
        }
    }
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::NetworkSpecs(key)))
    }
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: Vec<u8>) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsGenesisHash{key, genesis_hash}))
    }
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsEncryption{key, encryption}))
    }
    fn address_details_decoding(key: AddressKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::AddressDetails(key)))
    }
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::AddressDetailsEncryption{key, encryption}))
    }
    fn address_details_specs_encryption_mismatch(address_key: AddressKey, network_specs_key: NetworkSpecsKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::AddressDetailsSpecsEncryption{address_key, network_specs_key}))
    }
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error {
        ErrorSigner::AddressGeneration(AddressGeneration::Common(error))
    }
    fn transfer_content_error (transfer_content: TransferContent) -> Self::Error {
        ErrorSigner::Input(InputSigner::TransferContent(transfer_content))
    }
    fn db_internal (error: sled::Error) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Internal(error))
    }
    fn db_transaction (error: sled::transaction::TransactionError) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Transaction(error))
    }
    fn faulty_database_types() -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Types))
    }
    fn types_not_found() -> Self::Error {
        ErrorSigner::NotFound(NotFoundSigner::Types)
    }
    fn metadata_not_found(name: String, version: u32) -> Self::Error {
        ErrorSigner::NotFound(NotFoundSigner::Metadata{name, version})
    }
    fn show(error: &Self::Error) -> String {
        match error {
            ErrorSigner::Interface(a) => {
                let insert = match a {
                    InterfaceSigner::NotHex(b) => {
                        let insert = match b {
                            NotHexSigner::NetworkSpecsKey {input} => format!("Network specs key {}", input),
                            NotHexSigner::InputContent => String::from("Input content"),
                            NotHexSigner::AddressKey {input} => format!("Address key {}", input),
                        };
                        format!("{} is not in hexadecimal format.", insert)
                    },
                    InterfaceSigner::KeyDecoding(b) => {
                        let (insert, key) = match b {
                            KeyDecodingSignerInterface::AddressKey(x) => ("address", x.key()),
                            KeyDecodingSignerInterface::NetworkSpecsKey(x) => ("network specs", x.key()),
                        };
                        format!("Unable to parse {} key {} passed through the interface.", insert, hex::encode(key))
                    },
                    InterfaceSigner::PublicKeyLength => String::from("Public key length does not match the encryption."),
                    InterfaceSigner::HistoryPageOutOfRange{page_number, total_pages} => format!("Requested history page {} does not exist. Total number of pages {}.", page_number, total_pages),
                    InterfaceSigner::SeedNameNotMatching{address_key, expected_seed_name, real_seed_name} => format!("Expected seed name {} for address key {}. Address details in database have {} name.", expected_seed_name, hex::encode(address_key.key()), real_seed_name),
                    InterfaceSigner::LostPwd => String::from("Derivation had password, then lost it."),
                    InterfaceSigner::VersionNotU32(x) => format!("Version {} could not be converted into u32.", x),
                    InterfaceSigner::IncNotU32(x) => format!("Increment {} could not be converted into u32.", x),
                    InterfaceSigner::OrderNotU32(x) => format!("Order {} could not be converted into u32", x),
                    InterfaceSigner::FlagNotBool(x) => format!("Flag {} could not be converted into bool.", x),
                };
                format!("Error on the interface. {}", insert)
            },
            ErrorSigner::Database(a) => {
                let insert = match a {
                    DatabaseSigner::KeyDecoding(b) => {
                        let insert = match b {
                            KeyDecodingSignerDb::AddressKey(x) => format!("address key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::EntryOrder(x) => format!("history entry order {}", hex::encode(x)),
                            KeyDecodingSignerDb::MetaKey(x) => format!("meta key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::NetworkSpecsKey(x) => format!("network specs key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails{address_key, network_specs_key} => format!("network specs key {} from network id set of address book entry with key {}", hex::encode(network_specs_key.key()), hex::encode(address_key.key())),
                        };
                        format!("Unable to parse {} from the database.", insert)
                    },
                    DatabaseSigner::Internal(e) => format!("Internal error. {}", e),
                    DatabaseSigner::Transaction(e) => format!("Transaction error. {}", e),
                    DatabaseSigner::ChecksumMismatch => String::from("Checksum mismatch."),
                    DatabaseSigner::EntryDecoding(b) => {
                        let insert = match b {
                            EntryDecodingSigner::AddressDetails(x) => format!("address details entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::CurrentVerifier(x) => format!("current verifier entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::DangerStatus => String::from("danger status entry."),
                            EntryDecodingSigner::Derivations => String::from("temporary entry with information needed to import derivations."),
                            EntryDecodingSigner::GeneralVerifier => String::from("general verifier entry."),
                            EntryDecodingSigner::HistoryEntry(x) => format!("history entry for order {}.", x),
                            EntryDecodingSigner::NetworkSpecs(x) => format!("network specs (NetworkSpecs) entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::Sign => String::from("temporary entry with information needed for signing approved transaction."),
                            EntryDecodingSigner::Stub => String::from("temporary entry with information needed for accepting approved information."),
                            EntryDecodingSigner::Types => String::from("types information."),
                        };
                        format!("Unable to decode {}", insert)
                    },
                    DatabaseSigner::Mismatch(b) => {
                        let insert = match b {
                            MismatchSigner::Metadata{name_key, version_key, name_inside, version_inside} => format!("Meta key corresponds to {}{}. Stored metadata is {}{}.", name_key, version_key, name_inside, version_inside),
                            MismatchSigner::SpecsGenesisHash{key, genesis_hash} => format!("Network specs (NetworkSpecs) entry with network specs key {} has not matching genesis hash {}.", hex::encode(key.key()), hex::encode(genesis_hash)),
                            MismatchSigner::SpecsEncryption{key, encryption} => format!("Network specs (NetworkSpecs) entry with network specs key {} has not matching encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchSigner::AddressDetailsEncryption{key, encryption} => format!("Address details entry with address key {} has not matching encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchSigner::AddressDetailsSpecsEncryption{address_key, network_specs_key} => format!("Address details entry with address key {} has associated network specs key {} with wrong encryption.", hex::encode(address_key.key()), hex::encode(network_specs_key.key())),
                        };
                        format!("Mismatch found. {}", insert)
                    },
                    DatabaseSigner::FaultyMetadata{name, version, error} => format!("Bad metadata for {}{}. {}", name, version, error.show()),
                    DatabaseSigner::UnexpectedGenesisHash{verifier_key, network_specs_key} => format!("No verifier information found for network with genesis hash {}, however genesis hash is encountered in network specs entry with key {}.", hex::encode(verifier_key.genesis_hash()), hex::encode(network_specs_key.key())),
                    DatabaseSigner::SpecsCollision{name, encryption} => format!("More than one entry for network specs with name {} and encryption {}.", name, encryption.show()),
                    DatabaseSigner::DifferentNamesSameGenesisHash{name1, name2, genesis_hash} => format!("Different network names ({}, {}) in database for same genesis hash {}.", name1, name2, hex::encode(genesis_hash)),
                    DatabaseSigner::TwoTransactionsInEntry(x) => format!("Entry with order {} contains more than one transaction-related event. This should not be possible in current Signer and likely indicates database corruption.", x),
                    DatabaseSigner::CustomVerifierIsGeneral(key) => format!("Network with genesis hash {} verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption.", hex::encode(key.genesis_hash())),
                    DatabaseSigner::TwoRootKeys{seed_name, encryption} => format!("More than one root key (i.e. with empty path and without password) found for seed name {} and encryption {}.", seed_name, encryption.show()),
                    DatabaseSigner::DifferentBase58Specs{genesis_hash, base58_1, base58_2} => format!("More than one base58 prefix in network specs database entries for network with genesis hash {}: {} and {}.", hex::encode(genesis_hash), base58_1, base58_2),
                };
                format!("Database error. {}", insert)
            },
            ErrorSigner::Input(a) => {
                let insert = match a {
                    InputSigner::TransferContent(a) => a.show(),
                    InputSigner::TransferDerivations => String::from("Payload could not be decoded as derivations transfer."),
                    InputSigner::FaultyMetadata(error) => format!("Received metadata is unsuitable. {}", error.show()),
                    InputSigner::TooShort => String::from("Input is too short."),
                    InputSigner::NotSubstrate(code) => format!("Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x{}.", code),
                    InputSigner::PayloadNotSupported(code) => format!("Payload type with code 0x{} is not supported.", code),
                    InputSigner::SameNameVersionDifferentMeta{name, version} => format!("Metadata for {}{} is already in the database and is different from the one in received payload.", name, version),
                    InputSigner::MetadataKnown{name, version} => format!("Metadata for {}{} is already in the database.", name, version),
                    InputSigner::ImportantSpecsChanged(key) => format!("Similar network specs are already stored in the database under key {}. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit).", hex::encode(key.key())),
                    InputSigner::DifferentBase58{genesis_hash, base58_database, base58_input} => format!("Network with genesis hash {} already has entries in the database with base58 prefix {}. Received network specs have different base 58 prefix {}.", hex::encode(genesis_hash), base58_database, base58_input),
                    InputSigner::EncryptionNotSupported(code) => format!("Payload with encryption 0x{} is not supported.", code),
                    InputSigner::BadSignature => String::from("Received payload has bad signature."),
                    InputSigner::LoadMetaUnknownNetwork{name} => format!("Network {} is not in the database. Add network specs before loading the metadata.", name),
                    InputSigner::LoadMetaNoSpecs{name, valid_current_verifier, general_verifier} => {
                        let insert = match valid_current_verifier {
                            ValidCurrentVerifier::General => format!("{} (general verifier)", general_verifier.show_error()),
                            ValidCurrentVerifier::Custom(a) => format!("{} (custom verifier)", a.show_error()),
                        };
                        format!("Network {} was previously known to the database with verifier {}. However, no network specs are in the database at the moment. Add network specs before loading the metadata.", name, insert)
                    },
                    InputSigner::NeedVerifier{name, verifier_value} => format!("Saved network {} information was signed by verifier {}. Received information is not signed.", name, verifier_value.show_error()),
                    InputSigner::NeedGeneralVerifier{content, verifier_value} => {
                        let insert = match content {
                            GeneralVerifierForContent::Network{name} => format!("{} network information", name),
                            GeneralVerifierForContent::Types => String::from("types information"),
                        };
                        format!("General verifier in the database is {}. Received unsigned {} could be accepted only if signed by the general verifier.", verifier_value.show_error(), insert)
                    },
                    InputSigner::LoadMetaSetVerifier{name, new_verifier_value} => format!("Network {} currently has no verifier set up. Received load_metadata message is verified by {}. In order to accept verified metadata, first download properly verified network specs.", name, new_verifier_value.show_error()),
                    InputSigner::LoadMetaVerifierChanged{name, old_verifier_value, new_verifier_value} => format!("Network {} current verifier is {}. Received load_metadata message is verified by {}. Changing verifier for the network would require wipe and reset of Signer.", name, old_verifier_value.show_error(), new_verifier_value.show_error()),
                    InputSigner::LoadMetaSetGeneralVerifier{name, new_general_verifier_value} => format!("Network {} is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by {}. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.", name, new_general_verifier_value.show_error()),
                    InputSigner::LoadMetaGeneralVerifierChanged{name, old_general_verifier_value, new_general_verifier_value} => format!("Network {} is verified by the general verifier which currently is {}. Received load_metadata message is verified by {}. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.", name, old_general_verifier_value.show_error(), new_general_verifier_value.show_error()),
                    InputSigner::GeneralVerifierChanged{content, old_general_verifier_value, new_general_verifier_value} => {
                        let insert = match content {
                            GeneralVerifierForContent::Network{name} => format!("network {} specs", name),
                            GeneralVerifierForContent::Types => String::from("types information"),
                        };
                        format!("General verifier in the database is {}. Received {} could be accepted only if verified by the same general verifier. Current message is verified by {}.", old_general_verifier_value.show_error(), insert, new_general_verifier_value.show_error())
                    },
                    InputSigner::TypesKnown => String::from("Exactly same types information is already in the database."),
                    InputSigner::MessageNotReadable => String::from("Received message could not be read."),
                    InputSigner::UnknownNetwork{genesis_hash, encryption} => format!("Input generated within unknown network and could not be processed. Add network with genesis hash {} and encryption {}.", hex::encode(genesis_hash), encryption.show()),
                    InputSigner::NoMetadata{name} => format!("Input transaction is generated in network {}. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata.", name),
                    InputSigner::SpecsKnown{name, encryption} => format!("Exactly same network specs for network {} with encryption {} are already in the database.", name, encryption.show()),
                    InputSigner::AddSpecsVerifierChanged {name, old_verifier_value, new_verifier_value} => format!("Network {} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.", name, old_verifier_value.show_error(), new_verifier_value.show_error()),
                    InputSigner::InvalidDerivation(x) => format!("Derivation {} has invalid format.", x),
                    InputSigner::OnlyNoPwdDerivations => String::from("Only derivations without passwords are allowed in bulk import."),
                    InputSigner::SeedNameExists(x) => format!("Seed name {} already exists.", x),
                };
                format!("Bad input data. {}", insert)
            },
            ErrorSigner::NotFound(a) => {
                match a {
                    NotFoundSigner::CurrentVerifier(verifier_key) => format!("Could not find current verifier for network with genesis hash {}.", hex::encode(verifier_key.genesis_hash())),
                    NotFoundSigner::GeneralVerifier => String::from("Could not find general verifier."),
                    NotFoundSigner::Types => String::from("Could not find types information."),
                    NotFoundSigner::NetworkSpecs(network_specs_key) => format!("Could not find network specs for network specs key {}.", hex::encode(network_specs_key.key())),
                    NotFoundSigner::NetworkSpecsForName(name) => format!("Could not find network specs for {}.", name),
                    NotFoundSigner::NetworkSpecsKeyForAddress{network_specs_key, address_key} => format!("Could not find network specs key {} in address details with key {}.", hex::encode(network_specs_key.key()), hex::encode(address_key.key())),
                    NotFoundSigner::AddressDetails(address_key) => format!("Could not find address details for address key {}.", hex::encode(address_key.key())),
                    NotFoundSigner::Metadata{name, version} => format!("Could not find metadata entry for {}{}.", name, version),
                    NotFoundSigner::DangerStatus => String::from("Could not find danger status information."),
                    NotFoundSigner::Stub => String::from("Could not find database temporary entry with information needed for accepting approved information."),
                    NotFoundSigner::Sign => String::from("Could not find database temporary entry with information needed for signing approved transaction."),
                    NotFoundSigner::Derivations => String::from("Could not find database temporary entry with information needed for importing derivations set."),
                    NotFoundSigner::HistoryEntry(x) => format!("Could not find history entry with order {}.", x),
                    NotFoundSigner::HistoryNetworkSpecs{name, encryption} => format!("Could not find network specs for {} with encryption {} needed to decode historical transaction.", name, encryption.show()),
                    NotFoundSigner::TransactionEvent(x) => format!("Entry with order {} contains no transaction-related events.", x),
                    NotFoundSigner::HistoricalMetadata{name} => format!("Historical transaction was generated in network {} and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata.", name),
                    NotFoundSigner::NetworkForDerivationsImport{genesis_hash, encryption} => format!("Unable to import derivations for network with genesis hash {} and encryption {}. Network is unknown. Please add corresponding network specs.", hex::encode(genesis_hash), encryption.show()),
                }
            },
            ErrorSigner::DeadVerifier(key) => format!("Network with genesis hash {} is disabled. It could be enabled again only after complete wipe and re-installation of Signer.", hex::encode(key.genesis_hash())),
            ErrorSigner::AddressGeneration(a) => {
                let insert = match a {
                    AddressGeneration::Common(a) => a.show(),
                    AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseGeneration(e)) => format!("Could not create random phrase. {}", e),
                    AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseValidation(e)) => format!("Proposed random phrase is invalid. {}", e),
                    AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation) =>  String::from("Invalid derivation format."),
                };
                format!("Error generating address. {}", insert)
            },
            ErrorSigner::Qr(e) => format!("Error generating qr code. {}", e),
            ErrorSigner::Parser(a) => format!("Error parsing incoming transaction content. {}", a.show()),
            ErrorSigner::AllExtensionsParsingFailed{network_name, errors} => {
                let mut insert = String::new();
                for (i,(version, parser_error)) in errors.iter().enumerate() {
                    if i>0 {insert.push_str(" ")}
                    insert.push_str(&format!("Parsing with {}{} metadata: {}", network_name, version, parser_error.show()));
                }
                format!("Failed to decode extensions. Try updating metadata for {} network. {}", network_name, insert)
            },
            ErrorSigner::AddressUse(e) => format!("Error with secret string of existing address: {}.", bad_secret_string(e)),
            ErrorSigner::WrongPassword => String::from("Wrong password."),
            ErrorSigner::WrongPasswordNewChecksum(_) => String::from("Wrong password."),
            ErrorSigner::PngGeneration(e) => format!("Error generating png. {}", e),
            ErrorSigner::NoNetworksAvailable => String::from("No networks available. Please load networks information to proceed."),
        }
    }
}

/// Enum listing all variants of errors from the Signer side
#[derive(Debug)]
pub enum ErrorSigner {
    Interface(InterfaceSigner),
    Database(DatabaseSigner),
    Input(InputSigner),
    NotFound(NotFoundSigner),
    DeadVerifier(VerifierKey),
    AddressGeneration(AddressGeneration<Signer>),
    Qr(String),
    Parser(ParserError),
    AllExtensionsParsingFailed{network_name: String, errors: Vec<(u32, ParserError)>},
    AddressUse(SecretStringError),
    WrongPassword,
    WrongPasswordNewChecksum (u32),
    PngGeneration(png::EncodingError),
    NoNetworksAvailable,
}

/// Signer side errors could be exported into native interface,
/// before that they are transformed into anyhow errors
impl ErrorSigner {
    pub fn anyhow(&self) -> anyhow::Error {
        anyhow!(<Signer>::show(self))
    }
}

/// Enum listing all variants of errors on the interface between native and Rust parts,
/// on Signer side
#[derive(Debug)]
pub enum InterfaceSigner {
    NotHex(NotHexSigner),
    KeyDecoding(KeyDecodingSignerInterface),
    PublicKeyLength,
    HistoryPageOutOfRange{page_number: u32, total_pages: u32},
    SeedNameNotMatching{address_key: AddressKey, expected_seed_name: String, real_seed_name: String},
    LostPwd,
    VersionNotU32(String),
    IncNotU32(String),
    OrderNotU32(String),
    FlagNotBool(String),
}

/// NotHex errors occuring on the Signer side
#[derive(Debug)]
pub enum NotHexSigner {
    NetworkSpecsKey {input: String},
    InputContent,
    AddressKey {input: String},
}

/// Source of bad network specs keys on the Signer side
#[derive(Debug)]
pub enum ExtraSpecsKeySourceSigner {
    Interface,
}

/// Source of bad address keys
pub enum ExtraAddressKeySourceSigner {
    Interface,
}

/// Source of unsuitable metadata on the Signer side
#[derive(Debug)]
pub enum IncomingMetadataSourceSigner {
    ReceivedData,
}

/// Enum listing possible errors in decoding keys from the interface on the Signer side
#[derive(Debug)]
pub enum KeyDecodingSignerInterface {
    AddressKey(AddressKey),
    NetworkSpecsKey(NetworkSpecsKey),
}

/// Enum listing all variants of errors in the database on Signer side
#[derive(Debug)]
pub enum DatabaseSigner {
    KeyDecoding(KeyDecodingSignerDb),
    Internal(sled::Error),
    Transaction(sled::transaction::TransactionError),
    ChecksumMismatch,
    EntryDecoding(EntryDecodingSigner),
    Mismatch(MismatchSigner),
    FaultyMetadata{name: String, version: u32, error: MetadataError},
    UnexpectedGenesisHash{verifier_key: VerifierKey, network_specs_key: NetworkSpecsKey},
    SpecsCollision{name: String, encryption: Encryption},
    DifferentNamesSameGenesisHash{name1: String, name2: String, genesis_hash: Vec<u8>},
    TwoTransactionsInEntry(u32),
    CustomVerifierIsGeneral(VerifierKey),
    TwoRootKeys{seed_name: String, encryption: Encryption},
    DifferentBase58Specs{genesis_hash: [u8;32], base58_1: u16, base58_2: u16},
}

/// Enum listing possible errors in decoding keys from the database on the Signer side
#[derive(Debug)]
pub enum KeyDecodingSignerDb {
    AddressKey(AddressKey),
    EntryOrder(Vec<u8>),
    MetaKey(MetaKey),
    NetworkSpecsKey(NetworkSpecsKey),
    NetworkSpecsKeyAddressDetails{address_key: AddressKey, network_specs_key: NetworkSpecsKey},
}

/// Enum listing possible errors in decoding database entry content on the Signer side
#[derive(Debug)]
pub enum EntryDecodingSigner {
    AddressDetails(AddressKey),
    CurrentVerifier(VerifierKey),
    DangerStatus,
    Derivations,
    GeneralVerifier,
    HistoryEntry(u32),
    NetworkSpecs(NetworkSpecsKey),
    Sign,
    Stub,
    Types,
}

#[derive(Debug)]
/// Enum listing possible mismatch within database on the Signer side
pub enum MismatchSigner {
    Metadata{name_key: String, version_key: u32, name_inside: String, version_inside: u32},
    SpecsGenesisHash{key: NetworkSpecsKey, genesis_hash: Vec<u8>},
    SpecsEncryption{key: NetworkSpecsKey, encryption: Encryption},
    AddressDetailsEncryption{key: AddressKey, encryption: Encryption},
    AddressDetailsSpecsEncryption{address_key: AddressKey, network_specs_key: NetworkSpecsKey},
}

/// Enum listing errors with input received by the Signer
#[derive(Debug)]
pub enum InputSigner {
    TransferContent(TransferContent),
    TransferDerivations,
    FaultyMetadata(MetadataError),
    TooShort,
    NotSubstrate(String),
    PayloadNotSupported(String),
    SameNameVersionDifferentMeta{name: String, version: u32},
    MetadataKnown{name: String, version: u32},
    ImportantSpecsChanged(NetworkSpecsKey),
    DifferentBase58{genesis_hash: [u8;32], base58_database: u16, base58_input: u16},
    EncryptionNotSupported(String),
    BadSignature,
    LoadMetaUnknownNetwork{name: String},
    LoadMetaNoSpecs{name: String, valid_current_verifier: ValidCurrentVerifier, general_verifier: Verifier},
    NeedVerifier{name: String, verifier_value: VerifierValue},
    NeedGeneralVerifier{content: GeneralVerifierForContent, verifier_value: VerifierValue},
    LoadMetaSetVerifier{name: String, new_verifier_value: VerifierValue},
    LoadMetaVerifierChanged{name: String, old_verifier_value: VerifierValue, new_verifier_value: VerifierValue},
    LoadMetaSetGeneralVerifier{name: String, new_general_verifier_value: VerifierValue},
    LoadMetaGeneralVerifierChanged{name: String, old_general_verifier_value: VerifierValue, new_general_verifier_value: VerifierValue},
    GeneralVerifierChanged{content: GeneralVerifierForContent, old_general_verifier_value: VerifierValue, new_general_verifier_value: VerifierValue},
    TypesKnown,
    MessageNotReadable,
    UnknownNetwork{genesis_hash: Vec<u8>, encryption: Encryption},
    NoMetadata{name: String},
    SpecsKnown{name: String, encryption: Encryption},
    AddSpecsVerifierChanged {name: String, old_verifier_value: VerifierValue, new_verifier_value: VerifierValue},
    InvalidDerivation(String),
    OnlyNoPwdDerivations,
    SeedNameExists(String),
}

#[derive(Debug)]
pub enum GeneralVerifierForContent {
    Network{name: String},
    Types,
}

/// Enum listing errors for cases when something was needed from the Signer database
/// and was not found.
/// Not necessarily the database failure, could be just not updated Signer as well
#[derive(Debug)]
pub enum NotFoundSigner {
    CurrentVerifier(VerifierKey),
    GeneralVerifier,
    Types,
    NetworkSpecs(NetworkSpecsKey),
    NetworkSpecsForName(String),
    NetworkSpecsKeyForAddress{network_specs_key: NetworkSpecsKey, address_key: AddressKey},
    AddressDetails(AddressKey),
    Metadata{name: String, version: u32},
    DangerStatus,
    Stub,
    Sign,
    Derivations,
    HistoryEntry(u32),
    HistoryNetworkSpecs{name: String, encryption: Encryption},
    TransactionEvent(u32),
    HistoricalMetadata{name: String},
    NetworkForDerivationsImport{genesis_hash: [u8;32], encryption: Encryption},
}

/// Enum listing errors that can happen when generating address only on the Signer side
#[derive(Debug)]
pub enum ExtraAddressGenerationSigner {
    RandomPhraseGeneration(anyhow::Error),
    RandomPhraseValidation(anyhow::Error),
    InvalidDerivation,
}

/// Enum listing errors that occur during the transaction parsing
#[derive(Debug)]
pub enum ParserError {
    SeparateMethodExtensions, // can not separate method from extensions, bad transaction
    Decoding(ParserDecodingError), // errors occuring during the decoding procedure
    FundamentallyBadV14Metadata(ParserMetadataError), // errors occuring because the metadata is legit, but not acceptable in existing safety paradigm, for example, in V14 has no mention of network spec version in extrinsics
    WrongNetworkVersion {as_decoded: String, in_metadata: u32},
}

/// Enum listing errors directly related to transaction parsing
#[derive(Debug)]
pub enum ParserDecodingError {
    UnexpectedImmortality,
    UnexpectedMortality,
    GenesisHashMismatch,
    ImmortalHashMismatch,
    ExtensionsOlder,
    MethodNotFound{method_index: u8, pallet_name: String},
    PalletNotFound(u8),
    MethodIndexTooHigh{method_index: u8, pallet_index: u8, total: usize},
    NoCallsInPallet(String),
    V14TypeNotResolved,
    ArgumentTypeError,
    ArgumentNameError,
    NotPrimitive(String),
    NoCompact,
    DataTooShort,
    PrimitiveFailure(String),
    UnexpectedOptionVariant,
    IdFields,
    Array,
    BalanceNotDescribed,
    UnexpectedEnumVariant,
    UnexpectedCompactInsides,
    CompactNotPrimitive,
    UnknownType(String),
    NotBitStoreType,
    NotBitOrderType,
    BitVecFailure,
    Era,
    SomeDataNotUsedMethod,
    SomeDataNotUsedExtensions,
}

/// Not every V14 metadata are suitable for transaction parsing.
/// At the very least the extensions must include Era (once), BlockHash (once),
/// Version (once) and at most once the network genesis hash.
/// If the metadata does not follow those criteria, transactons could not be read,
/// and therefore, could not be signed.
#[derive(Debug)]
pub enum ParserMetadataError {
    NoEra,
    NoBlockHash,
    NoVersionExt,
    EraTwice,
    GenesisHashTwice,
    BlockHashTwice,
    SpecVersionTwice,
}

impl ParserError {
    pub fn show (&self) -> String {
        match &self {
            ParserError::SeparateMethodExtensions => String::from("Unable to separate transaction method and extensions."),
            ParserError::Decoding(x) => {
                match x {
                    ParserDecodingError::UnexpectedImmortality => String::from("Expected mortal transaction due to prelude format. Found immortal transaction."),
                    ParserDecodingError::UnexpectedMortality => String::from("Expected immortal transaction due to prelude format. Found mortal transaction."),
                    ParserDecodingError::GenesisHashMismatch => String::from("Genesis hash values from decoded extensions and from used network specs do not match."),
                    ParserDecodingError::ImmortalHashMismatch => String::from("Block hash for immortal transaction not matching genesis hash for the network."),
                    ParserDecodingError::ExtensionsOlder => String::from("Unable to decode extensions for V12/V13 metadata using standard extensions set."),
                    ParserDecodingError::MethodNotFound{method_index, pallet_name} => format!("Method number {} not found in pallet {}.", method_index, pallet_name),
                    ParserDecodingError::PalletNotFound(i) => format!("Pallet with index {} not found.", i),
                    ParserDecodingError::MethodIndexTooHigh{method_index, pallet_index, total} => format!("Method number {} too high for pallet number {}. Only {} indices available.", method_index, pallet_index, total),
                    ParserDecodingError::NoCallsInPallet(x) => format!("No calls found in pallet {}.", x),
                    ParserDecodingError::V14TypeNotResolved => String::from("Referenced type could not be resolved in v14 metadata."),
                    ParserDecodingError::ArgumentTypeError => String::from("Argument type error."),
                    ParserDecodingError::ArgumentNameError => String::from("Argument name error."),
                    ParserDecodingError::NotPrimitive(x) => format!("Expected primitive type. Found {}.", x),
                    ParserDecodingError::NoCompact => String::from("Expected compact. Not found it."),
                    ParserDecodingError::DataTooShort => String::from("Data too short for expected content."),
                    ParserDecodingError::PrimitiveFailure(x) => format!("Unable to decode part of data as {}.", x),
                    ParserDecodingError::UnexpectedOptionVariant => String::from("Encountered unexpected Option<_> variant."),
                    ParserDecodingError::IdFields => String::from("IdentityField description error."),
                    ParserDecodingError::Array => String::from("Unable to decode part of data as an array."),
                    ParserDecodingError::BalanceNotDescribed => String::from("Unexpected type encountered for Balance"),
                    ParserDecodingError::UnexpectedEnumVariant => String::from("Encountered unexpected enum variant."),
                    ParserDecodingError::UnexpectedCompactInsides => String::from("Unexpected type inside compact."),
                    ParserDecodingError::CompactNotPrimitive => String::from("Type claimed inside compact is not compactable."),
                    ParserDecodingError::UnknownType(x) => format!("No description found for type {}.", x),
                    ParserDecodingError::NotBitStoreType => String::from("Declared type is not suitable BitStore type for BitVec."),
                    ParserDecodingError::NotBitOrderType => String::from("Declared type is not suitable BitOrder type for BitVec."),
                    ParserDecodingError::BitVecFailure => String::from("Could not decode BitVec."),
                    ParserDecodingError::Era => String::from("Could not decode Era."),
                    ParserDecodingError::SomeDataNotUsedMethod => String::from("After decoding the method some data remained unused."),
                    ParserDecodingError::SomeDataNotUsedExtensions => String::from("After decoding the extensions some data remained unused."),
                }
            },
            ParserError::FundamentallyBadV14Metadata(x) => {
                let insert = match x {
                    ParserMetadataError::NoEra => String::from("Era information is missing."),
                    ParserMetadataError::NoBlockHash => String::from("Block hash information is missing."),
                    ParserMetadataError::NoVersionExt => String::from("Metadata spec version information is missing."),
                    ParserMetadataError::EraTwice => String::from("Era information is encountered mora than once."),
                    ParserMetadataError::GenesisHashTwice => String::from("Genesis hash is encountered more than once."),
                    ParserMetadataError::BlockHashTwice => String::from("Block hash is encountered more than once."),
                    ParserMetadataError::SpecVersionTwice => String::from("Metadata spec version is encountered more than once."),
                };
                format!("Metadata signed extensions are not compatible with Signer (v14 metadata). {}", insert)
            },
            ParserError::WrongNetworkVersion{as_decoded, in_metadata} => format!("Network spec version decoded from extensions ({}) differs from the version in metadata ({}).", as_decoded, in_metadata),
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
    fn show(&self) -> String {
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
