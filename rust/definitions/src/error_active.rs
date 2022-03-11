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
    NoTokenOverrideKnownNetwork { url: String },
    Wasm { filename: String, wasm: Wasm },
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
    FetchedMetadata { url: String },
    FetchedGenesisHash { url: String },
    InputSufficientCrypto,
    InputPublicKey,
    InputSignature,
    DefaultMetadata { filename: String },
}

/// Origin of unsuitable metadata on the Active side
#[derive(Debug)]
pub enum IncomingMetadataSourceActive {
    Str(IncomingMetadataSourceActiveStr),
    Wasm { filename: String },
}

/// Origin of unsuitable metadata on the Active side, in str form
#[derive(Debug)]
pub enum IncomingMetadataSourceActiveStr {
    Fetch { url: String },
    Default { filename: String },
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
    FaultyMetadata {
        name: String,
        version: u32,
        error: MetadataError,
    },
    TwoEntriesAddressEncryption {
        url: String,
        encryption: Encryption,
    },
    TwoDefaultsAddress {
        url: String,
    },
    HotDatabaseMetadataOverTwoEntries {
        name: String,
    },
    HotDatabaseMetadataSameVersionTwice {
        name: String,
        version: u32,
    },
    NewAddressKnownGenesisHash {
        url: String,
        genesis_hash: [u8; 32],
    },
    TwoGenesisHashVariantsForName {
        name: String,
    },
    TwoUrlVariantsForName {
        name: String,
    },
    TwoNamesForUrl {
        url: String,
    },
    TwoBase58ForName {
        name: String,
    },
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
    Call(sc_executor_common::error::Error),
    DecodingMetadata,
    FaultyMetadata(MetadataError),
    File(std::io::Error),
    RuntimeBlob(sc_executor_common::error::WasmError),
    WasmiInstance(sc_executor_common::error::Error),
    WasmiRuntime(sc_executor_common::error::WasmError),
}
