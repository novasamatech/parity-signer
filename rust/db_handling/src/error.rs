use definitions::crypto::Encryption;
use sled;
use anyhow::anyhow;
use hex;

#[derive(PartialEq)]
pub enum Error {
    InternalDatabaseError(sled::Error),
    DatabaseTransactionError(sled::transaction::TransactionError),
    ChecksumMismatch,
    NotHex(NotHex),
    NotFound(NotFound),
    NotDecodeable(NotDecodeable),
    GenesisHashMismatch,
    NetworkSpecsKeyMismatch,
    BadTypesFile(String),
    BadDefaultMetadata(String),
    MetadataNameMismatch,
    MetadataVersionMismatch,
    MetadataDefaultFile(String),
    RegexVersion,
    Base58(String),
    CreateAddress(CreateAddress),
    AddressKeyCollision {name: String, seed_name: String},
    AddressAlreadyExists{public: Vec<u8>, network: Vec<u8>},
    IdentityExists,
    InvalidDerivation,
    UnknownEncryption,
    AddressKey(String),
    EncryptionMismatchId,
    EncryptionMismatchNetwork,
    DeadVerifier,
    PageOutOfRange{given_page_number: u32, total_page_number: u32},
    SpecsHistoricalDecoding{network_name: String, encryption: Encryption},
}

#[derive(PartialEq)]
pub enum NotHex {
    GenesisHash,
    DefaultMeta,
    PublicKey,
    Signature,
    NetworkSpecsKey,
    SufficientCrypto,
}

#[derive(PartialEq)]
pub enum NotFound {
    NetworkSpecsKey,
    NameVersioned{name: String, version: u32},
    Types,
    NetworkSpecs(String),
    MetaFromName(String),
    Address,
    GeneralVerifier,
    CurrentVerifier,
    Stub,
    Sign,
    DangerStatus,
    Order(u32),
}

#[derive(PartialEq)]
pub enum NotDecodeable {
    ChainSpecs,
    AddressDetails,
    AddressKey,
    Types,
    Metadata,
    Version,
    NameVersioned,
    EntryOrder,
    Entry,
    NetworkSpecsKey,
    GeneralVerifier,
    CurrentVerifier,
    Stub,
    Sign,
    DangerStatus,
}

#[derive(PartialEq)]
pub enum CreateAddress {
    NetworkNotFound,
    Ed25519,
    Sr25519,
    Ecdsa,
    EncryptionMismatch,
}


impl Error {
    pub fn show (&self) -> anyhow::Error {
        match &self {
            Error::InternalDatabaseError(e) => anyhow!("Database internal error. {}", e),
            Error::DatabaseTransactionError(e) => anyhow!("Database transaction error. {}", e),
            Error::ChecksumMismatch => anyhow!("Database checksum mismatch."),
            Error::NotHex(a) => {
                let ins = match a {
                    NotHex::GenesisHash => "Genesis hash",
                    NotHex::DefaultMeta => "Default metadata string",
                    NotHex::PublicKey => "Public key",
                    NotHex::Signature => "Signature",
                    NotHex::NetworkSpecsKey => "Network key",
                    NotHex::SufficientCrypto => "Sufficient crypto",
                };
                anyhow!("{} could not be decoded as hex.", ins)
            },
            Error::NotFound(e) => {
                match e {
                    NotFound::NetworkSpecsKey => anyhow!("Network not found."),
                    NotFound::NameVersioned{name, version} => anyhow!("Metadata for {} version {} not in the database.", name, version),
                    NotFound::Types => anyhow!("Types not found."),
                    NotFound::NetworkSpecs(name) => anyhow!("No network specs found in the database for {}", name),
                    NotFound::MetaFromName(name) => anyhow!("No metadata entries found in the database for {}", name),
                    NotFound::Address => anyhow!("This address does not exist in the database"),
                    NotFound::GeneralVerifier => anyhow!("General verifier not found"),
                    NotFound::CurrentVerifier => anyhow!("Current network verifier not found"),
                    NotFound::Stub => anyhow!("No database transaction stub in the transaction tree"),
                    NotFound::Sign => anyhow!("No sign preparation stored in the transaction tree"),
                    NotFound::DangerStatus => anyhow!("Danger status not found in the database"),
                    NotFound::Order(x) => anyhow!("Entry with order {} not found in history record.", x),
                }
            },
            Error::NotDecodeable(e) => {
                match e {
                    NotDecodeable::ChainSpecs => anyhow!("Network specs are damaged and could not be decoded."),
                    NotDecodeable::AddressDetails => anyhow!("Address details were damaged and not decodeable."),
                    NotDecodeable::AddressKey => anyhow!("Address key could not be decoded."),
                    NotDecodeable::Types => anyhow!("Types information from the database could not be decoded."),
                    NotDecodeable::Metadata => anyhow!("Version vector of metadata from the database could not be retrieved."),
                    NotDecodeable::Version => anyhow!("Version vector of metadata from the database could not be decoded."),
                    NotDecodeable::NameVersioned => anyhow!("Versioned name (the key for metadata) could not be decoded."),
                    NotDecodeable::EntryOrder => anyhow!("History entry order (storage key) from the database could not be decoded."),
                    NotDecodeable::Entry => anyhow!("History entry from the database could not be decoded."),
                    NotDecodeable::NetworkSpecsKey => anyhow!("Network key could not be decoded."),
                    NotDecodeable::GeneralVerifier => anyhow!("General verifier could not be decoded."),
                    NotDecodeable::CurrentVerifier => anyhow!("Current network verifier could not be decoded."),
                    NotDecodeable::Stub => anyhow!("Database transaction stub from the transaction tree could not be decoded"),
                    NotDecodeable::Sign => anyhow!("Sign preparation unit from the transaction tree could not be decoded"),
                    NotDecodeable::DangerStatus => anyhow!("Danger status could not be decoded"),
                }
            },
            Error::GenesisHashMismatch => anyhow!("Genesis hash mismatch."),
            Error::NetworkSpecsKeyMismatch => anyhow!("Network key does not match genesis hash and encryption algorithm."),
            Error::BadTypesFile(e) => anyhow!("Error loading default types. {}", e),
            Error::BadDefaultMetadata(e) => anyhow!("Error loading default metadata. {}", e),
            Error::MetadataNameMismatch => anyhow!("Database records damaged. Name decoded from version constant does not match the name from database key."),
            Error::MetadataVersionMismatch => anyhow!("Database records damaged. Metadata version decoded from version constant does not match the version from database key."),
            Error::MetadataDefaultFile(e) => anyhow!("Error loading default metadata. {}", e),
            Error::RegexVersion => anyhow!("Error while loading default metadata. Network version does not fit in u32."),
            Error::Base58(e) => anyhow!("Error in database. Unable to convert public key into base58. {}", e),
            Error::CreateAddress(e) => {
                match e {
                    CreateAddress::NetworkNotFound => anyhow!("Error creating address. Network not found."),
                    CreateAddress::Ed25519 => anyhow!("Error generating ed25519 address"),
                    CreateAddress::Sr25519 => anyhow!("Error generating sr25519 address"),
                    CreateAddress::Ecdsa => anyhow!("Error generating ecdsa address"),
                    CreateAddress::EncryptionMismatch => anyhow!("Error creating address. Network encryption does not match seed object encryption."),
                }
            },
            Error::AddressKeyCollision {name, seed_name} => anyhow!("Address key collision with existing identity {} of seed {}", name, seed_name),
            Error::AddressAlreadyExists {public, network} => anyhow!("Address with public key {} already exists for network key {}", hex::encode(public), hex::encode(network)),
            Error::IdentityExists => anyhow!("Identity already exists"),
            Error::InvalidDerivation => anyhow!("Invalid derivation format"),
            Error::UnknownEncryption => anyhow!("System error: unknown encryption algorithm"),
            Error::AddressKey(x) => anyhow!("Error generating address key. {}", x),
            Error::EncryptionMismatchId => anyhow!("Identity encryption algorithm not matching network encryption algorithm"),
            Error::EncryptionMismatchNetwork => anyhow!("Encryption algorithm from network specs not matching the one from network key"),
            Error::DeadVerifier => anyhow!("Network is locked. Wipe clean and reset Signer to be able to re-install this network"),
            Error::PageOutOfRange{given_page_number, total_page_number} => anyhow!("Requested history page {} does not exist. Total number of pages {}.", given_page_number, total_page_number),
            Error::SpecsHistoricalDecoding{network_name, encryption} => anyhow!("Historical transaction could not be decoded. Missing network specs for {}, encryption {}.", network_name, encryption.show()),
        }
    }
}
