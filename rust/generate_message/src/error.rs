use sled;
use anyhow::anyhow;
use definitions::{crypto::Encryption};

pub enum Error {
    InternalDatabaseError(sled::Error),
    NotDecodeable(NotDecodeable),
    NotFound(NotFound),
    NotSupported,
    AddressBookEmpty,
    MetadataEmpty,
    InputOutputError(String),
    NetworkSpecsKeyMismatch(String),
    TwoEntriesAddressEncryption{address: String, encryption: Encryption},
    TwoDefaultsAddress(String),
    SpecsInDb{name: String, encryption: Encryption},
    UnexpectedGenesisHashFormat,
    FetchFailed{address: String, error: String},
    BadNetworkProperties{address: String, error: String},
    Base58Changed(String),
    DecimalsChanged(String),
    UnitChanged(String),
    NameChanged(String),
    GenesisHashChanged{address: String, old_genesis_hash: [u8; 32], new_genesis_hash: [u8; 32]},
    NoEntriesExpected(String),
    DatabaseMetadata{name: String, version: u32, error: String},
    DatabaseMetadataMismatch{name1: String, version1: u32, name2: String, version2: u32},
    DatabaseMetadataOverTwoEntries(String),
    DatabaseMetadataSameVersionTwice{name: String, version: u32},
    FetchedEarlierVersion{name: String, old_version: u32, new_version: u32},
    SameVersionDifferentMetadata{name: String, version: u32},
    StoredNameMismatch{address_book_name: String, network_specs_name: String},
    TwoGenHash(String),
    TwoAddresses(String),
    NotLoadTypes,
    NotLoadMetadata,
    DamagedMetadata,
    NotAddSpecs,
    WrongLengthPublicKey,
    WrongLengthSignature,
    BadSignature(Encryption),
    AliceKey(Encryption),
    Qr(String),
    NeedArgument(NeedArgument),
    DoubleKey(DoubleKey),
    OnlyOneNetworkId,
    BadArgument(BadArgument),
    NeedKey(NeedKey),
    UnexpectedKeyArgumentSequence,
    Unexpected(Unexpected),
    UnknownCommand,
    NoCommand,
}

pub enum NotDecodeable {
    AddressBookEntry,
    AddressBookKey,
    ChainSpecsToSend,
    FetchedMetadata{address: String, error: String},
    DatabaseVersionedName,
    SufficientCrypto,
}

pub enum NotFound {
    NetworkSpecsKey,
    AddressBookKey(String),
    Url(String),
    AddressBookNetworkName(String),
}

pub enum NeedArgument {
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
}

pub enum DoubleKey {
    Content,
    Set,
    CryptoOverride,
    CryptoKey,
    MsgType,
    Verifier,
    Payload,
    Signature,
    Name,
    SufficientCrypto,
    Remove,
}

pub enum NeedKey {
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
}

pub enum BadArgument {
    CryptoKey,
    MsgType,
    Verifier,
    Signature,
    SufficientCrypto,
}

pub enum Unexpected {
    KeyAContent,
    VerifierNoCrypto,
    SignatureNoCrypto,
    AliceSignature,
    VersionFormat,
}

impl Error {
    pub fn show(&self) -> anyhow::Error {
        match &self {
            Error::InternalDatabaseError(e) => anyhow!("Database internal error. {}", e),
            Error::NotDecodeable(x) => {
                match x {
                    NotDecodeable::AddressBookEntry => anyhow!("Unable to decode address book entry."),
                    NotDecodeable::AddressBookKey => anyhow!("Unable to decode address book key."),
                    NotDecodeable::ChainSpecsToSend => anyhow!("Unable to decode ChainSpecsToSend entry from the database."),
                    NotDecodeable::FetchedMetadata{address, error} => anyhow!("Error decoding metadata fetched by rpc call at {}. {}", address, error),
                    NotDecodeable::DatabaseVersionedName => anyhow!("Unable to decode NameVersioned, the key from database metadata entry."),
                    NotDecodeable::SufficientCrypto => anyhow!("Unable to decode provided SufficientCrypto."),
                }
            },
            Error::NotFound(x) => {
                match x {
                    NotFound::NetworkSpecsKey => anyhow!("Network specs key missing in the database."),
                    NotFound::AddressBookKey(name) => anyhow!("Address book key {} not found in the database.", name),
                    NotFound::Url(url) => anyhow!("Networks corresponding to url {} not found in the database.", url),
                    NotFound::AddressBookNetworkName(name) => anyhow!("Network {} not found in the address book.", name),
                }
            },
            Error::NotSupported => anyhow!("Key combination is not supported. Please file a ticket."),
            Error::AddressBookEmpty => anyhow!("Address book empty."),
            Error::MetadataEmpty => anyhow!("No metadata on record."),
            Error::InputOutputError(e) => anyhow!("IO error. {}", e),
            Error::NetworkSpecsKeyMismatch(network_name) => anyhow!("Network specs key mismatch in chainspecs in the database for {}.", network_name),
            Error::TwoEntriesAddressEncryption{address, encryption} => anyhow!("Database contains two entries for network with url {} and encryption {}.", address, encryption.show()),
            Error::TwoDefaultsAddress(url) => anyhow!("Database contains two default entries for network with url {}.", url),
            Error::SpecsInDb{name, encryption} => anyhow!("Network specs entry for {} and encryption {} is already in database.", name, encryption.show()),
            Error::UnexpectedGenesisHashFormat => anyhow!("Fetched genesis hash has unexpected format."),
            Error::FetchFailed{address, error} => anyhow!("Error processing rpc call at {}. {}", address, error),
            Error::BadNetworkProperties{address, error} => anyhow!("Error interpreting network properties fetched by rpc call at {}. {}", address, error),
            Error::Base58Changed(address) => anyhow!("Base58 prefix fetched by rpc call at {} differs from the one in the database.", address),
            Error::DecimalsChanged(address) => anyhow!("Decimals fetched by rpc call at {} differ from the one in the database.", address),
            Error::UnitChanged(address) => anyhow!("Unit fetched by rpc call at {} differs from the one in the database.", address),
            Error::NameChanged(address) => anyhow!("Network name, as derived from metadata fetched by rpc call at {} differs from the one in the database.", address),
            Error::GenesisHashChanged{address, old_genesis_hash, new_genesis_hash} => anyhow!("Genesis hash fetched by rpc call at {} differs from the one in the database. Old: {}, new: {}.", address, hex::encode(old_genesis_hash), hex::encode(new_genesis_hash)),
            Error::NoEntriesExpected(address) => anyhow!("No entries for address {} found in address book, however the entries with corresponding network are found. Database needs attention.", address),
            Error::DatabaseMetadata{name, version, error} => anyhow!("Error in metadata entry {}{} from database. {}", name, version, error),
            Error::DatabaseMetadataMismatch{name1, version1, name2, version2} => anyhow!("Error in metadata entry {}{} from database. Metadata decodes into {}{}", name1, version1, name2, version2),
            Error::DatabaseMetadataOverTwoEntries(name) => anyhow!("More than two entries for network {} in database.", name),
            Error::DatabaseMetadataSameVersionTwice{name, version} => anyhow!("Two entries in the database for {} version {}.", name, version),
            Error::FetchedEarlierVersion{name, old_version, new_version} => anyhow!("For {} the fetched version {} is lower than the latest version in the database {}.", name, new_version, old_version),
            Error::SameVersionDifferentMetadata{name, version} => anyhow!("Fetched metadata for {}{} differs from the one in the database.", name, version),
            Error::StoredNameMismatch{address_book_name, network_specs_name} => anyhow!("Name mismatch found. Network name in address book: {}, network name in stored network specs: {}", address_book_name, network_specs_name),
            Error::TwoGenHash(name) => anyhow!("Two different genesis hash entries for network {} in address book.", name),
            Error::TwoAddresses(name) => anyhow!("Two different address entries for network {} in address book.", name),
            Error::NotLoadTypes => anyhow!("Provided message has no load_types content."),
            Error::NotLoadMetadata => anyhow!("Provided message has no load_metadata content."),
            Error::DamagedMetadata => anyhow!("Metadata in the message is damaged."),
            Error::NotAddSpecs => anyhow!("Provided message has no add_specs content."),
            Error::WrongLengthPublicKey => anyhow!("Provided verifier public key has wrong length."),
            Error::WrongLengthSignature => anyhow!("Provided signature has wrong length."),
            Error::BadSignature(x) => anyhow!("Bad {} signature.", x.show()),
            Error::AliceKey(x) => anyhow!("Error generating Alice key for {} encryption.", x.show()),
            Error::Qr(e) => anyhow!("Error generating apng qr code. {}", e),
            Error::NeedArgument(x) => {
                let insert = match x {
                    NeedArgument::NetworkName => "`-n`",
                    NeedArgument::NetworkUrl => "`-u`",
                    NeedArgument::CryptoKey => "`-crypto`",
                    NeedArgument::MsgType => "`-msgtype`",
                    NeedArgument::Verifier => "`-verifier`",
                    NeedArgument::VerifierHex => "`-verifier -hex`",
                    NeedArgument::VerifierFile => "`-verifier -file`",
                    NeedArgument::Payload => "`-payload`",
                    NeedArgument::Signature => "`-signature`",
                    NeedArgument::SignatureHex => "`-signature -hex`",
                    NeedArgument::SignatureFile => "`-signature -file`",
                    NeedArgument::Name => "`-name`",
                    NeedArgument::SufficientCrypto => "`-sufficient`",
                    NeedArgument::SufficientCryptoHex => "`-sufficient -hex`",
                    NeedArgument::SufficientCryptoFile => "`-sufficient -file`",
                    NeedArgument::Make => "make",
                    NeedArgument::Sign => "sign",
                    NeedArgument::RemoveTitle => "`-remove -title`",
                    NeedArgument::RemoveName => "`-remove -name`",
                    NeedArgument::RemoveVersion => "`-remove -name *** -version`"
                };
                anyhow!("{} must be followed by an agrument.", insert)
            },
            Error::DoubleKey(x) => {
                let insert = match x {
                    DoubleKey::Content => "content",
                    DoubleKey::Set => "set",
                    DoubleKey::CryptoOverride => "encryption override",
                    DoubleKey::CryptoKey => "`-crypto`",
                    DoubleKey::MsgType => "`-msgtype`",
                    DoubleKey::Verifier => "`-verifier`",
                    DoubleKey::Payload => "`-payload`",
                    DoubleKey::Signature => "`-signature`",
                    DoubleKey::Name => "`-name`",
                    DoubleKey::SufficientCrypto => "`-sufficient`",
                    DoubleKey::Remove => "`-remove`",
                };
                anyhow!("More than one entry for {} key is not allowed.", insert)
            },
            Error::NeedKey(x) => {
                let insert = match x {
                    NeedKey::Show => "`show`",
                    NeedKey::Content => "content",
                    NeedKey::Crypto => "`-crypto`",
                    NeedKey::Payload => "`-payload`",
                    NeedKey::MsgType => "`-msgtype`",
                    NeedKey::SufficientCrypto => "`-sufficient`",
                    NeedKey::Signature => "`-signature`",
                    NeedKey::Verifier => "`-verifier`",
                    NeedKey::Remove => "`-title` or `-name`",
                    NeedKey::RemoveVersion => "`-version`",
                };
                anyhow!("Expected {} key to be used.", insert)
            },
            Error::OnlyOneNetworkId => anyhow!("Only one network identifier is allowed."),
            Error::BadArgument(x) => {
                let insert = match x {
                    BadArgument::CryptoKey => "`-crypto`",
                    BadArgument::MsgType => "`-msgtype`",
                    BadArgument::Verifier => "`-verifier`",
                    BadArgument::Signature => "`-signature`",
                    BadArgument::SufficientCrypto => "`-sufficient`",
                };
                anyhow!("Invalid argument after {} key.", insert)
            },
            Error::UnexpectedKeyArgumentSequence => anyhow!("Unexpected key and argument sequence."),
            Error::Unexpected(x) => {
                match x {
                    Unexpected::KeyAContent => anyhow!("Key -a is used to process all, name or url was not expected."),
                    Unexpected::VerifierNoCrypto => anyhow!("No verifier entry was expected for `-crypto none` sequence."),
                    Unexpected::SignatureNoCrypto => anyhow!("No singature entry was expected for `-crypto none` sequence."),
                    Unexpected::AliceSignature => anyhow!("No signature was expected for verifier Alice."),
                    Unexpected::VersionFormat => anyhow!("Unexpected version format."),
                }
            },
            Error::UnknownCommand => anyhow!("Unknown command."),
            Error::NoCommand => anyhow!("No command."),
        }
    }
}


