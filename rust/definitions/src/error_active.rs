//! Errors occuring on the active side, i.e. while operating `generate_message`
//! client
//!
//! Active side deals with both preparation of cold database that would be
//! loaded in Signer on build and with hot database operations.
//!
//! All errors [`ErrorActive`] could be displayed to user as error messages in
//! `generate_message` client, and are implementing `Display` trait.
//!
//! Exact error wording will be refined eventually.
//!
//! This module gathers all possible [`ErrorActive`] errors in one place, so that
//! error management is easier.

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
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: Vec<u8>) -> Self::Error {
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
        wasm: Wasm
    },
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
        genesis_hash: [u8; 32],
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

/// Enum listing possible errors in decoding keys from the database on the Active side
#[derive(Debug)]
pub enum KeyDecodingActive {
    AddressBookKey(AddressBookKey),
    AddressKey(AddressKey),
    MetaKey(MetaKey),
    NetworkSpecsKey(NetworkSpecsKey),
    NetworkSpecsKeyAddressDetails {
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    },
}

/// Enum listing possible errors in decoding database entry content on the Active side
#[derive(Debug)]
pub enum EntryDecodingActive {
    AddressBookEntryKey(AddressBookKey),
    AddressBookEntryTitle { title: String },
    AddressDetails(AddressKey),
    NetworkSpecs(NetworkSpecsKey),
    NetworkSpecsToSend(NetworkSpecsKey),
    Types,
}

/// Enum listing possible mismatch within database on the Active side
#[derive(Debug)]
pub enum MismatchActive {
    Metadata {
        name_key: String,
        version_key: u32,
        name_inside: String,
        version_inside: u32,
    },
    SpecsGenesisHash {
        key: NetworkSpecsKey,
        genesis_hash: Vec<u8>,
    },
    SpecsEncryption {
        key: NetworkSpecsKey,
        encryption: Encryption,
    },
    SpecsToSendGenesisHash {
        key: NetworkSpecsKey,
        genesis_hash: Vec<u8>,
    },
    SpecsToSendEncryption {
        key: NetworkSpecsKey,
        encryption: Encryption,
    },
    AddressDetailsEncryption {
        key: AddressKey,
        encryption: Encryption,
    },
    AddressDetailsSpecsEncryption {
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    },
    AddressBookSpecsName {
        address_book_name: String,
        specs_name: String,
    },
}

/// Enum listing possible errors on the Active side related to fetched (or not fetched) data
#[derive(Debug)]
pub enum Fetch {
    FaultyMetadata {
        url: String,
        error: MetadataError,
    },
    EarlierVersion {
        name: String,
        old_version: u32,
        new_version: u32,
    },
    SameVersionDifferentMetadata {
        name: String,
        version: u32,
    },
    FaultySpecs {
        url: String,
        error: SpecsError,
    },
    Failed {
        url: String,
        error: String,
    },
    ValuesChanged {
        url: String,
        what: Changed,
    },
    UnexpectedFetchedGenesisHashFormat {
        value: String,
    },
    SpecsInDb {
        name: String,
        encryption: Encryption,
    },
}

#[derive(Debug)]
pub enum SpecsError {
    NoBase58Prefix,
    Base58PrefixMismatch { specs: u16, meta: u16 },
    Base58PrefixFormatNotSupported { value: String },
    UnitNoDecimals(String),
    DecimalsFormatNotSupported { value: String },
    DecimalsNoUnit(u8),
    UnitFormatNotSupported { value: String },
    DecimalsArrayUnitsNot,
    DecimalsUnitsArrayLength { decimals: String, unit: String },
    UnitsArrayDecimalsNot,
    OverrideIgnored,
}

#[derive(Debug)]
pub enum Changed {
    Base58Prefix { old: u16, new: u16 },
    GenesisHash { old: [u8; 32], new: [u8; 32] },
    Decimals { old: u8, new: u8 },
    Name { old: String, new: String },
    Unit { old: String, new: String },
}

/// Enum listing possible errors on the Active side related to loading of the defaults
#[derive(Debug)]
pub enum DefaultLoading {
    FaultyMetadata {
        filename: String,
        error: MetadataError,
    },
    MetadataFolder(std::io::Error),
    MetadataFile(std::io::Error),
    TypesFile(std::io::Error),
    OrphanMetadata {
        name: String,
        filename: String,
    },
}

/// Enum listing errors for cases when something was needed from the Active database and was not found
#[derive(Debug)]
pub enum NotFoundActive {
    Types,
    Metadata { name: String, version: u32 },
    AddressBookEntry { title: String },
    NetworkSpecsToSend(NetworkSpecsKey),
    AddressBookEntryWithName { name: String },
    AddressBookEntryWithUrl { url: String },
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
    MetaDefaultFileName,
    MetaDefaultFileVersion,
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
    MetaDefaultFileName,
    MetaDefaultFileVersion,
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
    MetaDefaultFileName,
    MetaDefaultFileVersion,
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
    Call(sc_executor_common::error::Error),
    DecodingMetadata,
    FaultyMetadata(MetadataError),
    File(std::io::Error),
    RuntimeBlob(sc_executor_common::error::WasmError),
    WasmiInstance(sc_executor_common::error::Error),
    WasmiRuntime(sc_executor_common::error::WasmError),
}
