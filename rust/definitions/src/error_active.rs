//! Errors occuring on the active side, i.e. while operating `generate_message`
//! client
//!
//! Active side deals with both preparation of cold database that would be
//! loaded in Signer on build and with hot database operations. Cold database
//! could be the *release* cold database (the actual one for Signer build) or
//! the *test* cold database (with test Alice identities, used for tests).
//!
//! All errors [`ErrorActive`] could be displayed to user as error messages in
//! `generate_message` client, and are implementing `Display` trait.
//!
//! Exact error wording will be refined eventually.
//!
//! This module gathers all possible [`ErrorActive`] errors in one place, so that
//! error management is easier.

use sp_core::H256;
use time::error::Format;

use crate::{
    crypto::Encryption,
    error::{
        AddressGeneration, AddressGenerationCommon, AddressKeySource, ErrorSource, MetadataError,
        MetadataSource, SpecsKeySource, TransferContent,
    },
    keyring::{AddressBookKey, AddressKey, MetaKey, NetworkSpecsKey},
};

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
    fn specs_key_to_error(
        network_specs_key: &NetworkSpecsKey,
        source: SpecsKeySource<Self>,
    ) -> Self::Error {
        match source {
            SpecsKeySource::SpecsTree => ErrorActive::Database(DatabaseActive::KeyDecoding(
                KeyDecodingActive::NetworkSpecsKey(network_specs_key.to_owned()),
            )),
            SpecsKeySource::AddrTree(address_key) => ErrorActive::Database(
                DatabaseActive::KeyDecoding(KeyDecodingActive::NetworkSpecsKeyAddressDetails {
                    address_key,
                    network_specs_key: network_specs_key.to_owned(),
                }),
            ),
            SpecsKeySource::Extra(_) => unreachable!(),
        }
    }
    fn address_key_to_error(
        address_key: &AddressKey,
        _source: AddressKeySource<Self>,
    ) -> Self::Error {
        ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::AddressKey(
            address_key.to_owned(),
        )))
    }
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::MetaKey(
            meta_key.to_owned(),
        )))
    }
    fn metadata_mismatch(
        name_key: String,
        version_key: u32,
        name_inside: String,
        version_inside: u32,
    ) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::Metadata {
            name_key,
            version_key,
            name_inside,
            version_inside,
        }))
    }
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error {
        match source {
            MetadataSource::Database { name, version } => {
                ErrorActive::Database(DatabaseActive::FaultyMetadata {
                    name,
                    version,
                    error,
                })
            }
            MetadataSource::Incoming(IncomingMetadataSourceActive::Str(
                IncomingMetadataSourceActiveStr::Fetch { url },
            )) => ErrorActive::Fetch(Fetch::FaultyMetadata { url, error }),
            MetadataSource::Incoming(IncomingMetadataSourceActive::Str(
                IncomingMetadataSourceActiveStr::Default { filename },
            )) => ErrorActive::DefaultLoading(DefaultLoading::FaultyMetadata { filename, error }),
            MetadataSource::Incoming(IncomingMetadataSourceActive::Wasm { filename }) => {
                ErrorActive::Wasm {
                    filename,
                    wasm: Wasm::FaultyMetadata(error),
                }
            }
        }
    }
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(
            EntryDecodingActive::NetworkSpecs(key),
        ))
    }
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: H256) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsGenesisHash {
            key,
            genesis_hash,
        }))
    }
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsEncryption {
            key,
            encryption,
        }))
    }
    fn address_details_decoding(key: AddressKey) -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(
            EntryDecodingActive::AddressDetails(key),
        ))
    }
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(
            MismatchActive::AddressDetailsEncryption { key, encryption },
        ))
    }
    fn address_details_specs_encryption_mismatch(
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    ) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Mismatch(
            MismatchActive::AddressDetailsSpecsEncryption {
                address_key,
                network_specs_key,
            },
        ))
    }
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error {
        ErrorActive::TestAddressGeneration(AddressGeneration::Common(error))
    }
    fn transfer_content_error(transfer_content: TransferContent) -> Self::Error {
        ErrorActive::TransferContent(transfer_content)
    }
    fn db_internal(error: sled::Error) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Internal(error))
    }
    fn db_transaction(error: sled::transaction::TransactionError) -> Self::Error {
        ErrorActive::Database(DatabaseActive::Transaction(error))
    }
    fn faulty_database_types() -> Self::Error {
        ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::Types))
    }
    fn types_not_found() -> Self::Error {
        ErrorActive::NotFound(NotFoundActive::Types)
    }
    fn metadata_not_found(name: String, version: u32) -> Self::Error {
        ErrorActive::NotFound(NotFoundActive::Metadata { name, version })
    }
    fn timestamp_format(error: time::error::Format) -> Self::Error {
        ErrorActive::TimeFormat(error)
    }
    fn empty_seed() -> Self::Error {
        ErrorActive::SeedPhraseEmpty
    }
    fn empty_seed_name() -> Self::Error {
        ErrorActive::SeedNameEmpty
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
                            EntryDecodingActive::AddressBookEntry{title} => format!("address book entry for title {}.", title),
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
                    Fetch::EarlierVersion{name, old_version, new_version} => format!("For {} the newly received version ({}) is lower than the latest version in the hot database ({}).", name, new_version, old_version),
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
                            CommandNeedKey::MetaDefaultFileName => "`-name`",
                            CommandNeedKey::MetaDefaultFileVersion => "`-version`",
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
                            CommandDoubleKey::MetaDefaultFileName => "`-name`",
                            CommandDoubleKey::MetaDefaultFileVersion => "`-version`",
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
                            CommandNeedArgument::MetaDefaultFileName => "`-name`",
                            CommandNeedArgument::MetaDefaultFileVersion => "`-version`",
                        };
                        format!("{} must be followed by an agrument.", insert)
                    },
                    CommandParser::BadArgument(b) => {
                        match b {
                            CommandBadArgument::CryptoKey => String::from("Invalid argument after `-crypto` key."),
                            CommandBadArgument::DecimalsFormat => String::from("Key `-token` should be followed by u8 decimals value."),
                            CommandBadArgument::MsgType => String::from("Invalid argument after `-msgtype` key."),
                            CommandBadArgument::Signature => String::from("Invalid argument after `-signature` key."),
                            CommandBadArgument::SufficientCrypto => String::from("Invalid argument after `-sufficient` key."),
                            CommandBadArgument::Verifier => String::from("Invalid argument after `-verifier` key."),
                            CommandBadArgument::VersionFormat => String::from("Unexpected version format."),
                        }
                    },
                    CommandParser::Unexpected(b) => {
                        match b {
                            CommandUnexpected::AliceSignature => String::from("No signature was expected for verifier Alice."),
                            CommandUnexpected::KeyAContent => String::from("Key `-a` is used to process all, name or url was not expected."),
                            CommandUnexpected::SignatureNoCrypto => String::from("No singature entry was expected for `-crypto none` sequence."),
                            CommandUnexpected::VerifierNoCrypto => String::from("No verifier entry was expected for `-crypto none` sequence."),
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
            ErrorActive::Wasm{filename, wasm} => {
                match wasm {
                    Wasm::Call(e) => format!("Error processing .wasm file {}. Unable to process call on wasmi instance. {}", filename, e),
                    Wasm::DecodingMetadata => format!("Error processing .wasm file {}. Unable to decode metadata.", filename),
                    Wasm::FaultyMetadata(e) => format!("Metadata error in .wasm file {}. {}", filename, e.show()),
                    Wasm::File(e) => format!("Error processing .wasm file {}. Unable to load file. {}", filename, e),
                    Wasm::RuntimeBlob(e) => format!("Error processing .wasm file {}. Unable to generate RuntimeBlob. {}", filename, e),
                    Wasm::WasmiInstance(e) => format!("Error processing .wasm file {}. Unable to generate WasmiInstance. {}", filename, e),
                    Wasm::WasmiRuntime(e) => format!("Error processing .wasm file {}. Unable to generate WasmiRuntime. {}", filename, e),
                }
            },
            ErrorActive::TimeFormat(e) => format!("Unable to produce timestamp. {}", e),
            ErrorActive::SeedPhraseEmpty => String::from("Seed phrase is empty."),
            ErrorActive::SeedNameEmpty => String::from("Seed name is empty."),
        }
    }
}

/// All possible errors that could occur on the active side
#[derive(Debug)]
pub enum ErrorActive {
    /// Errors within database.
    ///
    /// Associated data is [`DatabaseActive`] with more details.
    Database(DatabaseActive),

    /// Expected to get a hexadecimal string, got something different.
    ///
    /// Associated data is [`NotHexActive`] with more details.
    NotHex(NotHexActive),

    /// Damaged update payload.
    ///
    /// Associated data is [`TransferContent`] with more details.
    TransferContent(TransferContent),

    /// Error fetching network information through rpc call.
    ///
    /// Associated data is [`Fetch`] with more details.
    Fetch(Fetch),

    /// Error loading default information while generating the database.
    ///
    /// Associated data is [`DefaultLoading`] with more details.
    DefaultLoading(DefaultLoading),

    /// Error generating output file, associated data is `std::io::Error`.
    Output(std::io::Error),

    /// Something was expected to be found in the hot database, but was not.
    ///
    /// Associated data is [`NotFoundActive`] with more details.
    NotFound(NotFoundActive),

    /// Test address generation in cold database has failed.
    ///
    /// Associated data is [`AddressGeneration`] with more details.
    TestAddressGeneration(AddressGeneration<Active>),

    /// Error parsing commands from `generate_message` client command line.
    ///
    /// Associated data is [`CommandParser`] with more details.
    CommandParser(CommandParser),

    /// Input is invalid.
    ///
    /// Associated data is [`InputActive`] with more details.
    Input(InputActive),

    /// Error generating QR code, static or fountain.
    ///
    /// `generate_message` can produce QR codes with updates data for:
    ///
    /// - network specs: `add_specs` message, usually static `png`
    /// - network metadata: `load_metadata` message, usually fountain `apng`
    /// - types information: `load_types` message, usually fountain `apng`
    ///
    /// These QR codes could be scanned into Signer through air-gap to update
    /// Signer database.
    ///
    /// Associated data is text of the error produced by the QR generator.
    Qr(String),

    /// Requested command from `generate_message` client command line is not
    /// supported.
    NotSupported,

    /// It is not allowed to override token in known network, if the fetching
    /// uses or updates the hot database.
    ///
    /// Associated data is url address used for the fetching.
    NoTokenOverrideKnownNetwork { url: String },

    /// Error extracting network metadata from `wasm` file.
    Wasm {
        /// `wasm` file name
        filename: String,

        /// error details
        wasm: Wasm,
    },

    /// Time formatting error
    TimeFormat(Format),

    /// Got empty seed phrase
    SeedPhraseEmpty,

    /// Got empty seed name
    SeedNameEmpty,
}

impl std::fmt::Display for ErrorActive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Active>::show(self))
    }
}

/// NotHex errors occuring on the Active side
///
/// Expected to receive data in hexadecimal format, got something different.
/// [`NotHexActive`] specifies what was expected.
#[derive(Debug)]
pub enum NotHexActive {
    /// Network metadata, fetched through rpc call.
    ///
    /// Associated data is the url address used for the fetching.
    FetchedMetadata { url: String },

    /// Network genesis hash, fetched through rpc call.
    ///
    /// Associated data is the url address used for the fetching.
    FetchedGenesisHash { url: String },

    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) received in
    /// command line in `generate_message` client.
    ///
    /// Could be encountered when generating signed update with `sign` command.
    InputSufficientCrypto,

    /// Public key received in command line in `generate_message` client.
    ///
    /// Could be encountered when generating signed update with `make` command.
    InputPublicKey,

    /// Signature received in command line in `generate_message` client.
    ///
    /// Could be encountered when generating signed update with `make` command.
    InputSignature,

    /// Default network metadata, used to generate cold database, with filename
    /// as an associated data.
    DefaultMetadata { filename: String },
}

/// Source of unsuitable metadata on the Active side
#[derive(Debug)]
pub enum IncomingMetadataSourceActive {
    /// Bad metadata was in a hexadecimal string
    Str(IncomingMetadataSourceActiveStr),

    /// Bad metadata was contained in `wasm` file, associated data is the filename.
    Wasm { filename: String },
}

/// Source of unsuitable hexadecimal string metadata on the Active side
#[derive(Debug)]
pub enum IncomingMetadataSourceActiveStr {
    /// Metadata was fetched, associated data is url used for rpc call.
    Fetch { url: String },

    /// Metadata is the default one, associated data is the filename.
    Default { filename: String },
}

/// Source of damaged [`NetworkSpecsKey`], exclusive for the active side.
/// Is empty.
#[derive(Debug)]
pub enum ExtraSpecsKeySourceActive {}

/// Source of damaged [`AddressKey`], exclusive for the active side.
/// Is empty.
#[derive(Debug)]
pub enum ExtraAddressKeySourceActive {}

/// Errors in generating address, exclusive for the active side.
/// Is empty.
#[derive(Debug)]
pub enum ExtraAddressGenerationActive {}

/// Errors in the database content on the active side
///
/// Describes errors with already existing database content (e.g. damaged keys,
/// damaged values, various mismatches, data that could not have been added to
/// the database in the first place etc).
///
/// Note that [`NotFoundActive`] is a separate set of errors. Things **not
/// found** are kept separately here from things **damaged**.
#[derive(Debug)]
pub enum DatabaseActive {
    /// Key used in one of the database trees has invalid content, and could
    /// not be decoded.
    ///
    /// Associated data is [`KeyDecodingActive`] with more details.
    KeyDecoding(KeyDecodingActive),

    /// Value found in one of the database trees has invalid content, and could
    /// not be decoded.
    ///
    /// Associated data is [`EntryDecodingActive`] with more details.
    EntryDecoding(EntryDecodingActive),

    /// Database [`Error`](https://docs.rs/sled/0.34.6/sled/enum.Error.html).
    ///
    /// Could happen, for example, when opening the database, loading trees,
    /// reading values etc.
    Internal(sled::Error),

    /// Database
    /// [`TransactionError`](https://docs.rs/sled/0.34.6/sled/transaction/enum.TransactionError.html).
    ///
    /// Could happen when making transactions in multiple trees simultaneously.
    Transaction(sled::transaction::TransactionError),

    /// Data retrieved from the database contains some internal contradictions,
    /// could not have been written in the database this way, and is therefore
    /// likely indicating the database corruption.
    ///
    /// Associated data is [`MismatchActive`] with more details.
    Mismatch(MismatchActive),

    /// Network metadata that already is in the database, is damaged.
    ///
    /// Unsuitable metadata could not be put in the database in the first place,
    /// finding one would mean the database got corrupted.
    FaultyMetadata {
        /// network name, from [`MetaKey`]
        name: String,

        /// network version, from [`MetaKey`]
        version: u32,

        /// what exactly is wrong with the metadata
        error: MetadataError,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains more than one
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) with same
    /// `address` and `encryption` fields values.
    TwoEntriesAddressEncryption {
        /// url address of the entries
        url: String,

        /// [`Encryption`] of the entries
        encryption: Encryption,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains more than one
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) with same
    /// `address` field value, for which `def` field is `true`, i.e. two
    /// entries for the same network are default ones.
    TwoDefaultsAddress {
        /// url address of the entries
        url: String,
    },

    /// `METATREE` of the hot database shoud contain at most two latest
    /// metadata versions for each network, with the older entries being
    /// deleted as the new ones appear.
    ///
    /// This error appears if during the processing more than two metadata
    /// entries for the network are found.
    HotDatabaseMetadataOverTwoEntries {
        /// network name
        name: String,
    },

    /// `METATREE` of the hot database has two entries for a network with the
    /// same metadata version.
    ///
    /// Note: at this moment should be unreachable, since the entries are
    /// getting checked for consistency with [`MetaKey`].
    HotDatabaseMetadataSameVersionTwice {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// Fetched through rpc call network genesis hash is known to the hot
    /// database, although the url address used for rpc call is not.
    ///
    /// Hot database does not allow to store more than one trusted url address
    /// for rpc calls for same network.
    ///
    /// Alternative url address could be used if the database is not updated
    /// (`-d` key is used).
    ///
    /// To update the address in the database in case the old one is no longer
    /// acceptable, one should remove old entry, and only then add the new one.
    NewAddressKnownGenesisHash {
        /// new for database url address used for rpc call, for which a known
        /// genesis hash was retrieved
        url: String,

        /// network genesis hash that was fetched through rpc call and found in
        /// the database
        genesis_hash: H256,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) entries with same
    /// `name` field and different `genesis_hash` values.
    ///
    /// This is not allowed, as it would cause uncertainty in `load_metadata`
    /// message generation, which is build using metadata and genesis hash.
    TwoGenesisHashVariantsForName {
        /// network name
        name: String,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) entries with same
    /// `name` field and different `address` values.
    ///
    /// Hot database does not allow to store more than one trusted url address
    /// for rpc calls for same network.
    ///
    /// Alternative url address could be used if the database is not updated
    /// (`-d` key is used).
    ///
    /// To update the address in the database in case the old one is no longer
    /// acceptable, one should remove old entry, and only then add the new one.
    TwoUrlVariantsForName {
        /// network name
        name: String,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) entries with same
    /// `address` field and different `name` fields.
    ///
    /// Name in address book is taken from the metadata, metadata is fetched
    /// using rpc call, so one url address can correspond to only one network
    /// name.
    TwoNamesForUrl {
        /// url address, for which two condlicting names were found
        url: String,
    },

    /// `SPECSTREEPREP` tree of the hot database contains
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) entries
    /// with same network name, but different base58 prefix.
    TwoBase58ForName {
        /// network name
        name: String,
    },

    /// `ADDRESS_BOOK` tree of the hot database has no entries to process.
    AddressBookEmpty,
}

/// Errors decoding database keys on the active side
///
/// `IVec` value of the database key could be unfallably transformed into the
/// contents of the corresponding key from the [`keyring`](crate::keyring)
/// module. All these keys were, however, generated using certain information,
/// and if this information could not be extracted from the key, it indicates
/// that the database is damaged and results in [`KeyDecodingActive`] error.
#[derive(Debug)]
pub enum KeyDecodingActive {
    /// [`AddressBookKey`] from the hot database could not be processed.
    ///
    /// Associated data is the damaged `AddressBookKey`.
    AddressBookKey(AddressBookKey),

    /// [`AddressKey`] from the test cold database could not be processed.
    ///
    /// Associated data is the damaged `AddressKey`.
    AddressKey(AddressKey),

    /// [`MetaKey`] from the hot database could not be processed.
    ///
    /// Associated data is the damaged `MetaKey`.
    MetaKey(MetaKey),

    /// [`NetworkSpecsKey`] from the database (hot or cold one) could not
    /// be processed.
    ///
    /// Associated data is the damaged `NetworkSpecsKey`.
    NetworkSpecsKey(NetworkSpecsKey),

    /// [`NetworkSpecsKey`] encountered as one of the entries in `network_id`
    /// field of the [`AddressDetails`](crate::users::AddressDetails) in test
    /// cold database could not be processed.
    NetworkSpecsKeyAddressDetails {
        /// [`AddressKey`] corresponding to `AddressDetails` that contain the
        /// damaged `NetworkSpecsKey`
        address_key: AddressKey,

        /// damaged `NetworkSpecsKey`
        network_specs_key: NetworkSpecsKey,
    },
}

/// Errors decoding database entry content on the active side
///
/// Database stores most of the values SCALE-encoded, and to be used they must
/// be decoded. If the decoding fails, it indicates that the database is
/// damaged.
#[derive(Debug)]
pub enum EntryDecodingActive {
    /// Hot database entry could not be decoded as
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry).
    ///
    /// Associated data is the corresponding address book title.
    AddressBookEntry { title: String },

    /// Test cold database entry could not be decoded as
    /// [`AddressDetails`](crate::users::AddressDetails).
    ///
    /// Associated data is the corresponding [`AddressKey`].
    AddressDetails(AddressKey),

    /// Cold database entry could not be decoded as
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs).
    ///
    /// Associated data is the corresponding [`NetworkSpecsKey`].
    NetworkSpecs(NetworkSpecsKey),

    /// Hot database entry could not be decoded as
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend).
    ///
    /// Associated data is the corresponding [`NetworkSpecsKey`].
    NetworkSpecsToSend(NetworkSpecsKey),

    /// Types information from hot database, i.e. encoded `Vec<TypeEntry>`
    /// stored in `SETTREE` under the key `TYPES`, could not be decoded.
    Types,
}

/// Mismatch errors within database on active side
///
/// Data could be recorded in both hot database and cold database only
/// in ordered fasion, i.e. with keys corresponding to the data stored in the
/// encoded values etc.
///
/// If the data retrieved from the database contains some internal
/// contradictions, it indicates the database corruption.
#[derive(Debug)]
pub enum MismatchActive {
    /// Network name and/or network version in [`MetaKey`] do not match the
    /// network name and network version from `Version` constant, `System`
    /// pallet of the metadata stored under this `MetaKey`.
    ///
    /// Error could be encountered only in the hot database.
    Metadata {
        /// network name as it is in the key
        name_key: String,

        /// network version as it is in the key
        version_key: u32,

        /// network name as it is in the metadata
        name_inside: String,

        /// network version as it is in the metadata
        version_inside: u32,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry stored under
    /// this `NetworkSpecsKey` in `SPECSTREE` tree of the cold database
    /// contains `genesis_hash` field with a different genesis hash.
    SpecsGenesisHash {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// genesis hash as it is in the `NetworkSpecs`
        genesis_hash: H256,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry stored under
    /// this `NetworkSpecsKey` in `SPECSTREE` tree of the cold database
    /// contains `encryption` field with a different [`Encryption`].
    SpecsEncryption {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// [`Encryption`] as it is in the `NetworkSpecs`
        encryption: Encryption,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) entry
    /// stored under this `NetworkSpecsKey` in `SPECSTREEPREP` tree of the hot
    /// database contains `genesis_hash` field with a different genesis hash.
    SpecsToSendGenesisHash {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// genesis hash as it is in the `NetworkSpecsToSend`
        genesis_hash: H256,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) entry
    /// stored under this `NetworkSpecsKey` in `SPECSTREEPREP` tree of the hot
    /// database contains `encryption` field with a different [`Encryption`].
    SpecsToSendEncryption {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// [`Encryption`] as it is in the `NetworkSpecsToSend`
        encryption: Encryption,
    },

    /// [`AddressKey`] has an associated [`Encryption`].
    /// [`AddressDetails`](crate::users::AddressDetails) entry stored under
    /// this `AddressKey` contains `encryption` field with a different
    /// [`Encryption`].
    ///
    /// Error could be encountered only in the test cold database.
    AddressDetailsEncryption {
        /// [`AddressKey`] corresponding to mismatching data
        key: AddressKey,

        /// [`Encryption`] as it is in the `AddressDetails`
        encryption: Encryption,
    },

    /// [`AddressKey`] has an associated [`Encryption`].
    /// [`AddressDetails`](crate::users::AddressDetails) entry stored under
    /// this `AddressKey` contains `network_id` field with a set of
    /// [`NetworkSpecsKey`] values corresponding to networks in which this
    /// address exists. [`NetworkSpecsKey`] is built using network genesis hash
    /// and `Encryption`.
    ///
    /// If the `Encryption` value from one of `NetworkSpecsKey` values is
    /// different from `Encryption` assocaited with `AddressKey`, this error
    /// appears.
    ///
    /// Error could be encountered only in the test cold database.
    AddressDetailsSpecsEncryption {
        /// [`AddressKey`] corresponding to mismatching data
        address_key: AddressKey,

        /// [`NetworkSpecsKey`] having `Encryption` different from the one
        /// associated with `AddressKey`
        network_specs_key: NetworkSpecsKey,
    },

    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) in hot database
    /// contains `encryption` and `genesis_hash` fields, from which the
    /// corresponding [`NetworkSpecsKey`] could be built.
    ///
    /// `NetworkSpecsKey` has an associated
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) value
    /// stored in `SPECSTREEPREP` tree of the hot database. `NetworkSpecsToSend`
    /// has field `name` with network name.
    ///
    /// This error appears if the `name` from `NetworkSpecsToSend` differs from
    /// the `name` in `AddressBookEntry`.
    AddressBookSpecsName {
        /// name in [`AddressBookEntry`](crate::metadata::AddressBookEntry)
        address_book_name: String,

        /// name in [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
        specs_name: String,
    },
}

/// Errors on the active side related to data fetched (or not fetched) through
/// rpc calls
#[derive(Debug)]
pub enum Fetch {
    /// Fetched metadata is not suitable for use in Signer.
    FaultyMetadata {
        /// url address used for rpc call
        url: String,

        /// what exactly is wrong with the metadata
        error: MetadataError,
    },

    /// Fetched network metadata version is lower than the one already in the
    /// hot database.
    EarlierVersion {
        /// network name
        name: String,

        /// network version in hot database, higher than the one just fetched
        old_version: u32,

        /// network version just fetched
        new_version: u32,
    },

    /// Fetched network metadata is different from the one already in the hot
    /// database, with the same network name and network version.
    SameVersionDifferentMetadata {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// Fetched network specs are not suitable for use in Signer.
    FaultySpecs {
        /// url address used for rpc cal
        url: String,

        /// what exactly is wrong with the network specs
        error: SpecsError,
    },

    /// Rpc call failed.
    Failed {
        /// url address used for rpc call
        url: String,

        /// received error message
        error: String,
    },

    /// Fetched data is different from the one already in the hot database.
    ValuesChanged {
        /// url address used for rpc call
        url: String,

        /// what exactly has changed
        what: Changed,
    },

    /// Fetched genesis hash could not be transformed in expected [u8; 32] value.
    UnexpectedFetchedGenesisHashFormat {
        /// genesis hash value as received through rpc call
        value: String,
    },

    /// Network specs are already in the database
    SpecsInDb {
        /// network name
        name: String,

        /// network supported encryption
        encryption: Encryption,
    },
}

/// Errors on the active side with network specs received through rpc call
#[derive(Debug)]
pub enum SpecsError {
    /// Network base58 prefix information is not found neither in results of
    /// the `system_properties` rpc call, nor in `System` pallet of the metadata
    /// fetched with `state_getMetadata` rpc call.
    NoBase58Prefix,

    /// Network base58 prefix information found through `system_properties` rpc
    /// call differs from the one from `System` pallet of the metadata fetched
    /// with "state_getMetadata" rpc call.
    ///
    /// Associated data is corresponding base58 prefixes.
    Base58PrefixMismatch { specs: u16, meta: u16 },

    /// Network base58 prefix information received through `system_properties`
    /// rpc call could not be transformed into expected `u16` prefix.
    ///
    /// Associated data is base58 prefix as received.
    Base58PrefixFormatNotSupported { value: String },

    /// Network decimals information **is not found** in the results if the
    /// `system_properties` rpc call, but the unit information **is found**.
    ///
    /// Associated data is the fetched unit value.
    UnitNoDecimals(String),

    /// Network decimals information received through `system_properties`
    /// rpc call could not be transformed into expected `u8` value.
    ///
    /// Associated data is decimals information as received.
    DecimalsFormatNotSupported { value: String },

    /// Network unit information **is not found** in the results if the
    /// `system_properties` rpc call, but the decimals information **is found**.
    ///
    /// Associated data is the fetched decimals value.
    DecimalsNoUnit(u8),

    /// Network unit information received through `system_properties`
    /// rpc call could not be transformed into expected `String` value.
    ///
    /// Associated data is unit information as received.
    UnitFormatNotSupported { value: String },

    /// An array with more than one element is received for network decimals
    /// through `system_properties` rpc call. Received units are not an array.
    DecimalsArrayUnitsNot,

    /// Both the network decimals and network units are received as arrays,
    /// but the array length is different, i.e. something not straightforward
    /// is going on with the network.
    ///
    /// Associated data are the printed sets as they are received through the
    /// `system_properties` rpc call.
    DecimalsUnitsArrayLength { decimals: String, unit: String },

    /// An array with more than one element is received for network units
    /// through `system_properties` rpc call. Received decimals are not an array.
    UnitsArrayDecimalsNot,

    /// Unit and decimal override is not allowed.
    ///
    /// The only case when the decimals and unit override is permitted is when
    /// the network has a matching set of decimals and units, and user has to
    /// select the needed set element manually.
    ///
    /// If the network has a single decimals value and a single unit value, i.e.
    /// the values that would be suitable on their own, and user attempts to
    /// override it, this error appears.
    OverrideIgnored,
}

/// Data received through rpc call is different from the data in hot database
#[derive(Debug)]
pub enum Changed {
    /// Network base58 prefix in hot database (consistent between the metadata
    /// in `METATREE` and network specs in `SPECSPREPTREE`) is different from
    /// the one received through new rpc calls (also consistent).
    ///
    /// Associated data is the base58 prefix values in question.
    Base58Prefix { old: u16, new: u16 },

    /// Network genesis hash in hot database is different from the one fetched
    /// through a new rpc call.
    ///
    /// Network genesis hash is encountered in `SPECSTREEPREP` and
    /// `ADDRESS_BOOK`.
    ///
    /// It is possible for a network to change genesis hash, some, especially
    /// experimental ones, are doing it quite regularly.
    ///
    /// If the network has changed the genesis hash, it would be best to remove
    /// the old entry from the database, and then load the new one. If the
    /// network is one of the default ones (currently Polkadot, Kusama,
    /// Westend), the `defaults` crate must be updated as well.
    ///
    /// Associated data is the genesis hash values in question.
    GenesisHash { old: H256, new: H256 },

    /// Network decimals value in
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// stored in `SPECSTREEPREP` tree of the hot database is different from
    /// the one fetched through a new rpc call.
    ///
    /// Network decimals value is expected to be permanent.
    Decimals { old: u8, new: u8 },

    /// Network name is stored in multiple places in the hot database:
    ///
    /// - in `name` field of network specs
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) stored
    /// in `SPECSTREEPREP` tree
    /// - in `name` field of address book entry
    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) stored in
    /// `ADDRESS_BOOK` tree
    /// - encoded as a part of [`MetaKey`] and inside the network metadata
    /// stored in `METATREE` tree
    ///
    /// All those entries eventually are produced from network name that is
    /// part of `Version` constant in `System` pallet of the network metadata.
    ///
    /// Network name is expected to be permanent. This error appears if the
    /// name derived from metadata fetched through a new rpc call is different.
    Name { old: String, new: String },

    /// Network unit value in
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// stored in `SPECSTREEPREP` tree of the hot database is different from
    /// the one fetched through a new rpc call.
    ///
    /// Network unit value is expected to be permanent.
    Unit { old: String, new: String },
}

/// Error loading of the defaults
///
/// Could happen during both hot and cold detabase generation.
#[derive(Debug)]
pub enum DefaultLoading {
    /// Default metadata is damaged.
    ///
    /// This is relevant only for cold database.
    ///
    /// Hot database has no default metadata entries.
    FaultyMetadata {
        /// filename, in which the faulty metadata was found
        filename: String,

        /// what exactly is wrong with the metadata
        error: MetadataError,
    },

    /// Unable to read directory with default metadata
    MetadataFolder(std::io::Error),

    /// Unable to read file with default metadata
    MetadataFile(std::io::Error),

    /// Unable to read file with defalt types information
    TypesFile(std::io::Error),

    /// Default metadata set contains metadata files that have no corresponding
    /// default network specs and address book entries.
    OrphanMetadata {
        /// name of the network that has default metadata, but no default specs
        name: String,

        /// filename of the file with orphan metadata
        filename: String,
    },
}

/// Errors when something was needed from the hot database and was not found
#[derive(Debug)]
pub enum NotFoundActive {
    /// Types information stored in `SETTREE` tree of the hot database under key
    /// `TYPES`.
    ///
    /// Types information is added to the database when generating it and could
    /// not be deleted from the client.
    Types,

    /// Network metadata for given network name and version, searched in hot
    /// database `METATREE` tree.
    Metadata {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) searched in
    /// `ADDRESS_BOOK` tree of the hot database by network address book title
    /// (i.e. with a specific [`AddressBookKey`]).
    ///
    /// Associated data is network address book title used for searching.
    AddressBookEntry { title: String },

    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend) searched
    /// by [`NetworkSpecsKey`] in `SPECSTREEPREP` tree of the hot database.
    ///
    /// Associated data is [`NetworkSpecsKey`] used for searching.
    NetworkSpecsToSend(NetworkSpecsKey),

    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) searched in
    /// `ADDRESS_BOOK` tree of the hot database by matching the `name` field.
    ///
    /// Associated data is the network name used for searching.
    AddressBookEntryWithName { name: String },

    /// [`AddressBookEntry`](crate::metadata::AddressBookEntry) searched in
    /// `ADDRESS_BOOK` tree of the hot database by matching the `address` field.
    ///
    /// Associated data is the url address used for searching.
    AddressBookEntryWithUrl { url: String },
}

/// Command line parser errors from the `generate_message` crate
///
/// Client of `generate_message` crate supports following commands:
///
/// - `add_specs`, to add entries into hot database `SPECSTREEPREP` and
/// `ADDRESS_BOOK` and to generate signable `add_specs` message content
/// - `derivations`, to generate derivations import messages
/// - `load_metadata`, to add entries into hot database `METATREE` tree and
/// to generate signable `load_metadata` message content
/// - `load_types`, to generate signable `load_types` message content
/// - `make` to generate qr images or text files with update messages, unsigned,
/// signed by test verifier (Alice) or signed with user-provided public key
/// and signature
/// - `make_cold_release` to generate *release* cold database as a part of
/// Signer build process
/// - `make_cold_with_identities` to generate *test* cold database with Alice
/// identities pre-generated, for tests
/// - `meta_default_file` to produce a file with hexadecimal network metadata
/// from an entry in `METATREE` of the hot database; such files are used in
/// `defaults` crate
/// - `remove` to remove a metadata entry or a network from the hot database
/// - `restore_defaults` to generate default hot database in place of the
/// current one
/// - `show`, to show current hot database entries in `METATREE` or
/// `ADDRESS_BOOK` trees
/// - `sign` to generate qr images or text files with update messages using
/// Signer-generated [`SufficientCrypto`](crate::crypto::SufficientCrypto) data
/// - `transfer_meta_to_cold` to transfer metadata from hot database into *test*
/// cold database, only for networks with network specs entries in the cold
/// database
/// - `transfer_meta_to_cold_release` to transfer metadata from hot database
/// into *release* cold database, only for networks with network specs entries
/// in the cold database
/// - `unwasm`, to generate signable `load_metadata` message content from `wasm`
/// file, for networks before release
///
/// Commands may have keys (starting with `-`), keys may be followed by the
/// arguments.
#[derive(Debug)]
pub enum CommandParser {
    /// Agrument sequence could not be processed.
    UnexpectedKeyArgumentSequence,

    /// Received more than one network identifier.
    ///
    /// Commands `add_specs` and `load_metadata` could be called for:
    ///
    /// - networks specified by network name (as it is recorded in entries in
    /// `ADDRESS_BOOK`), using key `-n`
    /// - url address to use for rpc call, using key `-u`.
    ///
    /// Both these keys must be used for an individual network, i.e. only one
    /// network name or one network url address must be provided.
    ///
    /// This error appears if the parser has recognized more than one command
    /// element as a network identifier.
    OnlyOneNetworkId,

    /// A key needed to run the command was not provided.
    ///
    /// Associated data is [`CommandNeedKey`] with more details.
    NeedKey(CommandNeedKey),

    /// Same key was encountered more than once.
    ///
    /// Associated data is [`CommandDoubleKey`] with more details.
    DoubleKey(CommandDoubleKey),

    /// A key must have been followed by some argument, but was not.
    ///
    /// Associated data is [`CommandNeedArgument`] with more details.
    NeedArgument(CommandNeedArgument),

    /// An argument following the key is unsuitable.
    ///
    /// Associated data is [`CommandBadArgument`] with more details.
    BadArgument(CommandBadArgument),

    /// Unexpected excessive entry in the command.
    ///
    /// Associated data is [`CommandUnexpected`] with more details.
    Unexpected(CommandUnexpected),

    /// Command is not known.
    UnknownCommand,

    /// No command provided.
    NoCommand,
}

/// Missing key in `generate_message` command
#[derive(Debug)]
pub enum CommandNeedKey {
    /// Command `show` needs key:
    ///
    /// - `-address_book` to show the contents of the hot database `ADDRESS_BOOK`
    /// tree
    /// - `-database` to show the contents of the hot database `METATREE` tree
    Show,

    /// Commands `add_specs` and `load_metadata` need key specifying content:
    ///
    /// - `-a` to process all networks with entries in the `ADDRESS_BOOK` tree
    /// - `-n` to process network by provided network name (in case of
    /// `load_metadata`) or network address book title (in case of `add_specs`)
    /// - `-u` to process network by provided url address
    Content,

    /// Command `make` requires `-crypto` key, followed by the encryption to be
    /// used in generating update transaction. Possible variants are:
    ///
    /// - `none` for unsigned updates
    /// - `ed25519`
    /// - `sr25519`
    /// - `ecdsa`
    ///
    /// Entered encryption must match entered verifier and signature data.
    Crypto,

    /// Commands `derivations`, `make`, `sign`, `unwasm` require `-payload` key
    /// to be used, followed by the name of the file to process.
    Payload,

    /// Commands `make` and `sign` require `-msgtype` key, followed by what is
    /// contained in the payload: `add_specs`, `load_metadata` or `load_types`.
    MsgType,

    /// Command `sign` requires `-sufficient` key, followed by
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input, matching
    /// the payload content.
    SufficientCrypto,

    /// Command `make` requires signature if the message is to be signed by
    /// real verifier.
    Signature,

    /// Command `make` requires public key of the verifier if the message is to
    /// be signed.
    ///
    /// Verifier could be:
    /// - custom (real user, with own public key), in this case a real signature
    /// would be required
    /// - test verifier (Alice), no signature
    Verifier,

    /// Command `remove` needs one of these keys:
    ///
    /// - `-title`, followed by network address book title, to remove
    /// `ADDRESS_BOOK` entry for the network, and metadata in case there are no
    /// more networks that use this metadata
    /// - `-name`, followed by network name argument, key `-version`, and
    /// network version argument, to remove specific metadata entry from
    /// `METATREE` by name and version
    Remove,

    /// If command `remove` is processed with the key `-name`, it requires also
    /// a key `-version` followed by the metadata version, to specify the
    /// version to be deleted.
    RemoveVersion,

    /// Transaction with derivation import is generated for a specific network,
    /// this network is addressed by `-title` key followed by network address
    /// book title.
    DerivationsTitle,

    /// Command `meta_default_file` must have `-name` key followed by the
    /// network name to specify the metadata being exported.
    MetaDefaultFileName,

    /// Command `meta_default_file` must have `-version` key followed by the
    /// network metadata version to specify the metadata being exported.
    MetaDefaultFileVersion,
}

/// Key in `generate_message` command encountered twice
#[derive(Debug)]
pub enum CommandDoubleKey {
    /// Commands `add_specs` and `load_metadata` allow only one content key:
    ///
    /// - `-a` to process all networks with entries in the `ADDRESS_BOOK` tree
    /// - `-n` to process network by provided network name (in case of
    /// `load_metadata`) or network address book title (in case of `add_specs`)
    /// - `-u` to process network by provided url address
    Content,

    /// Commands `add_specs` and `load_metadata` allow at most one set key,
    ///
    /// - `-f` to use the data from the database, without rpc calls, and save
    /// files for signing
    /// - `-d` to make rpc calls without updating the database, and save files
    /// for signing
    /// - `-k` to make rpc calls, update the database, and save for signing only
    /// the files with the data **new** to the database
    /// - `-p` to make rpc calls, update the database, and make no files
    /// - `-t` (the default one, is done when the set key is not specified as
    /// well), to make rpc calls, update the database, and save all files for
    /// signing
    Set,

    /// Command `add_specs` may use encryption override key to specify the
    /// encryption supported by the network. Encryption override must be used
    /// for networks without an entry in `ADDRESS_BOOK` tree. Encryption
    /// override could be also used for networks already recorded in
    /// `ADDRESS_BOOK` if the network supports more than one [`Encryption`].
    ///
    /// Encryption override key (`-ed25519`, `-sr25519`, `-ecdsa`) could not be
    /// used more than once.
    CryptoOverride,

    /// Command `add_specs`, when used for networks without network specs
    /// entries in `SPECSTREEPREP` and with mora than one token supported,
    /// could use token override. For this, key `-token` followed by `u8`
    /// decimals and `String` unit arguments is used.
    ///
    /// Token override key may be used only once.
    TokenOverride,

    /// Command `make` must have exactly one `-crypto` key, followed by the
    /// encryption argument.
    CryptoKey,

    /// Commands `make` and `sign` must have exactly one `-msgtype` key,
    /// followed by the message type argument.
    MsgType,

    /// Command `make` can have at most one `-verifier` key.
    Verifier,

    /// Commands `derivations`, `make`, `sign`, `unwasm` must have exactly one
    /// `-payload` key, followed by the name of the file to process.
    Payload,

    /// Command `make` can have at most one `-signeture` key.
    Signature,

    /// Commands `make` and `sign` can have at most one `-name` key,
    /// followed by the custom name of the export file in `../files/signed/`
    /// folder.
    Name,

    /// Command `sign` must have exactly one `-sufficient` key, followed by the
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input, matching
    /// the payload content.
    SufficientCrypto,

    /// Command `remove` must have exactly one content key:
    ///
    /// - `-title`, followed by network address book title, to remove
    /// `ADDRESS_BOOK` entry for the network, and metadata in case there are no
    /// more networks that use this metadata
    /// - `-name`, followed by network name argument, key `-version`, and
    /// network version argument, to remove specific metadata entry from
    /// `METATREE` by name and version
    Remove,

    /// Command `derivations` must have exactly one `-title` key followed by
    /// network address book title for network in which the derivation export
    /// is generated.
    DerivationsTitle,

    /// Command `meta_default_file` must have exactly one `-name` key followed
    /// by the network name.
    MetaDefaultFileName,

    /// Command `meta_default_file` must have exactly one `-version` key followed
    /// by the network version.
    MetaDefaultFileVersion,
}

/// Missing argument for the key in `generate_message` command
#[derive(Debug)]
pub enum CommandNeedArgument {
    /// Key `-token` in `add_specs` command was supposed to be followed by `u8`
    /// decimals and `String` unit agruments.
    ///
    /// Unit argument was not provided.
    ///
    /// Note: this error can occur only when `-token` is the last key in the
    /// sequence.
    ///
    /// On the other hand, if sequence used is, for example,
    /// `$ cargo run add_specs -d -u wss://my_super_network.io -token 10 -sr25519`
    /// parser will interpret `-sr25519` part as an attempted unit entry (indeed,
    /// units could be whatever, so no obvious criteria to recognize them),
    /// and will complain that no encryption is provided for an unknown network.
    TokenUnit,

    /// Key `-token` in `add_specs` command was supposed to be followed by `u8`
    /// decimals and `String` unit agruments.
    ///
    /// Decimals argument was not provided.
    ///
    /// Note: this error can occur only when `-token` is the last key in the
    /// sequence. Otherwise the parser will try to interpret as `u8` decimals
    /// the next key and complain that it is not `u8`.
    TokenDecimals,

    /// Commands `add_specs` and `load_metadata` with content key `-n` require
    /// network identifier: network address book title for `add_specs` and
    /// network name for `load_metadata`
    NetworkName,

    /// Commands `add_specs` and `load_metadata` with content key `-u` require
    /// url address input for rpc calls
    NetworkUrl,

    /// Key `-crypto` in `make` command was supposed to be followed by an
    /// agrument:
    ///
    /// - `ed25519`
    /// - `sr25519`
    /// - `ecdsa`
    /// - `none`
    CryptoKey,

    /// Key `-verifier` in `make` command was supposed to be followed by an
    /// argument or additional key:
    ///
    /// - argument `Alice`
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    Verifier,

    /// Key sequence `-verifier -hex` in `make` command must be followed by a
    /// hexadecimal verifier public key
    VerifierHex,

    /// Key sequence `-verifier -file` in `make` command must be followed by a
    /// filename of the file in `../files/for_signing/` with verifier public
    /// key as `&[u8]`.
    VerifierFile,

    /// Key `-payload` must be followed by a filename of the file:
    /// - in `../files/for_signing/` for `make` and `sign` commands
    /// - in `../generate_message/` for `derivations` and `unwasm` commands
    Payload,

    /// Key `-msgtype` in `make` and `sign` must be followed by a valid message
    /// type argument:
    ///
    /// - `add_specs`
    /// - `load_metadata`
    /// - `load_types`
    MsgType,

    /// Key `-signature` in `make` command was supposed to be followed by an
    /// additional key:
    ///
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    Signature,

    /// Key sequence `-signature -hex` in `make` command must be followed by a
    /// hexadecimal signature.
    SignatureHex,

    /// Key sequence `-signature -file` in `make` command must be followed by a
    /// filename of the file in `../files/for_signing/` with signature
    /// as `&[u8]`.
    SignatureFile,

    /// Key `-name` in `make` and `sign` commands, if used, must be followed by
    /// a filename of target file in `../files/signed`.
    Name,

    /// Key `-sufficient` in `sign` command was supposed to be followed by an
    /// additional key:
    ///
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    SufficientCrypto,

    /// Key sequence `-sufficient -hex` in `sign` command must be followed by a
    /// hexadecimal SCALE-encoded `SufficientCrypto` string.
    SufficientCryptoHex,

    /// Key sequence `-sufficient -file` in `sign` command must be followed by a
    /// filename of the file in `../files/for_signing/` with SCALE-encoded
    /// `SufficientCrypto` as `&[u8]`.
    SufficientCryptoFile,

    /// Command `make` must be followed by additional keys.
    Make,

    /// Command `sign` must be followed by additional keys.
    Sign,

    /// Key `-title` in `remove` command must be followed by network address
    /// book title.
    RemoveTitle,

    /// Key `-name` in `remove` command must be followed by network name.
    RemoveName,

    /// Key-argument sequence `remove -name <***> -version` in `remove` command
    /// must be followed by network version.
    RemoveVersion,

    /// Command `derivations` must be followed by additional keys.
    Derivations,

    /// Key `-title` in `derivations` command must be followed by network
    /// address book title.
    DerivationsTitle,

    /// Key `-name` in `meta_default_file` command must be followed by network
    /// name.
    MetaDefaultFileName,

    /// Key `-version` in `meta_Default_file` command must be followed by
    /// network version.
    MetaDefaultFileVersion,
}

/// Unsuitable argument for the key in `generate_message` command
#[derive(Debug)]
pub enum CommandBadArgument {
    /// The valid arguments for key `-crypto` are:
    ///
    /// - `ed25519`
    /// - `sr25519`
    /// - `ecdsa`
    /// - `none`
    CryptoKey,

    /// Key `-token` must be followed by `u8` decimals and `String` unit values.
    ///
    /// This error appears if the value immediately after `-token` could not be
    /// parsed as `u8`.
    DecimalsFormat,

    /// The valid arguments for key `-msgtype` are:
    ///
    /// - `add_specs`
    /// - `load_metadata`
    /// - `load_types`
    MsgType,

    /// Signature may be entered from a file or as a hexadecimal string.
    /// Key `-signature` may be followed by:
    ///
    /// `-file` followed by the name of the file in `../files/for_signing/` with
    /// signature as `&[u8]`
    /// `-hex` followed by hexadecimal signature
    Signature,

    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) may be entered
    /// from a file or as a hexadecimal string.
    /// Key `-sufficient` may be followed by:
    ///
    /// `-file` followed by the name of the file in `../files/for_signing/` with
    /// SCALE-encoded `SufficientCrypto` as `&[u8]`
    /// `-hex` followed by hexadecimal SCALE-encoded `SufficientCrypto` string
    SufficientCrypto,

    /// Verifier entered after key `-verifier` may be:
    ///
    /// `-file` followed by name of the file in `../files/for_signing/` with
    /// verifier public key as `&[u8]`
    /// `-hex` followed by hexadecimal verifier public key
    /// `Alice`
    Verifier,

    /// Commands `remove` and `meta_default_file` require network version to be
    /// specified after key `-version`.
    ///
    /// This error appears if the value immediately after `-version` could not be
    /// parsed as `u32`.
    VersionFormat,
}

/// Unexpected content in `generate_message` command
#[derive(Debug)]
pub enum CommandUnexpected {
    /// Command `make` with `-verifier Alice` can not accept the signature.
    AliceSignature,

    /// Commands `add_specs` and `load_metadata` can not accept name or url
    /// address if `-a` (process all) content key is used.
    KeyAContent,

    /// Command `make` with `-crypto none` can not accept the signature.
    SignatureNoCrypto,

    /// Command `make` with `-crypto none` can not accept the verifier public key.
    VerifierNoCrypto,
}

/// Errors in `generate_message` input
#[derive(Debug)]
pub enum InputActive {
    /// Unable to read the file with input.
    File(std::io::Error),

    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) could not be
    /// decoded.
    DecodingSufficientCrypto,

    /// The length of public key does not match the selected encryption
    /// algorithm.
    PublicKeyLength,

    /// The length of data signature does not match the selected encryption
    /// algorithm.
    SignatureLength,

    /// Tried to apply signature (i.e. used command `make` or `sign`) to
    /// metadata that is not suitable for use in Signer
    FaultyMetadataInPayload(MetadataError),

    /// Provided data signature (entered separately or as a part of
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input) is invalid
    /// for given public key and data.
    BadSignature,

    /// Provided file contains no valid password-free derivations that could be
    /// exported
    NoValidDerivationsToExport,
}

/// Errors with `wasm` files processing
// TODO add links to external errors and definitions, when the `sc-...` crates
// are published
#[derive(Debug)]
pub enum Wasm {
    /// Failed to make "Metadata_metadata" call on data extracted from `wasm`
    /// file.
    Call(sc_executor_common::error::Error),

    /// Metadata extracted from `wasm` file could not be decoded.
    DecodingMetadata,

    /// Metadata extracted from `wasm` file is not suitable to be used in
    /// Signer.
    ///
    /// Associated data is [`MetadataError`] specifying what exactly is wrong
    /// with the metadata.
    FaultyMetadata(MetadataError),

    /// Error reading `wasm` file.
    File(std::io::Error),

    /// Error generating `RuntimeBlob`.
    RuntimeBlob(sc_executor_common::error::WasmError),

    /// Error generating `WasmiInstance`.
    WasmiInstance(sc_executor_common::error::Error),

    /// Error generating `WasmiRuntime`.
    WasmiRuntime(sc_executor_common::error::WasmError),
}
