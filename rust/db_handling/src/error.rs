use sled;
use definitions::metadata::NameVersioned;
use anyhow::anyhow;

#[derive(PartialEq)]
pub enum Error {
    InternalDatabaseError(sled::Error),
    NotHex(NotHex),
    NotFound(NotFound),
    NotDecodeable(NotDecodeable),
    GenesisHashMismatch,
    NetworkKeyMismatch,
    BadTypesFile(String),
    MetadataNameMismatch,
    MetadataVersionMismatch,
    MetadataDefaultFile(String),
    RegexVersion,
    Base58(String),
    CreateAddress(CreateAddress),
    AddressKeyCollision {name: String, seed_name: String},
    IdentityExists,
    InvalidDerivation,
    UnknownEncryption,
    AddressKey(String),
    EncryptionMismatchId,
    EncryptionMismatchNetwork,
}

#[derive(PartialEq)]
pub enum NotHex {
    GenesisHash,
    DefaultMeta,
    PublicKey,
    Signature,
    NetworkKey,
    SufficientCrypto,
}

#[derive(PartialEq)]
pub enum NotFound {
    NetworkKey,
    NameVersioned(NameVersioned),
    Types,
    NetworkSpecs(String),
    MetaFromName(String),
    Address,
    Verifier,
}

#[derive(PartialEq)]
pub enum NotDecodeable {
    ChainSpecs,
    AddressDetailsDel,
    AddressDetails,
    AddressKey,
    Types,
    Metadata,
    Version,
    NameVersioned,
    EntryOrder,
    Entry,
    NetworkKey,
    Verifier,
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
            Error::NotHex(a) => {
                let ins = match a {
                    NotHex::GenesisHash => "Genesis hash",
                    NotHex::DefaultMeta => "Default metadata string",
                    NotHex::PublicKey => "Public key",
                    NotHex::Signature => "Signature",
                    NotHex::NetworkKey => "Network key",
                    NotHex::SufficientCrypto => "Sufficient crypto",
                };
                anyhow!("{} could not be decoded as hex.", ins)
            },
            Error::NotFound(e) => {
                match e {
                    NotFound::NetworkKey => anyhow!("Network not found."),
                    NotFound::NameVersioned(x) => anyhow!("Metadata for {} version {} not in the database.", x.name, x.version),
                    NotFound::Types => anyhow!("Types not found."),
                    NotFound::NetworkSpecs(name) => anyhow!("No network specs found in the database for {}", name),
                    NotFound::MetaFromName(name) => anyhow!("No metadata entries found in the database for {}", name),
                    NotFound::Address => anyhow!("This address does not exist in the database"),
                    NotFound::Verifier => anyhow!("Network verifier not found"),
                }
            },
            Error::NotDecodeable(e) => {
                match e {
                    NotDecodeable::ChainSpecs => anyhow!("Network specs are damaged and could not be decoded."),
                    NotDecodeable::AddressDetailsDel => anyhow!("Address details were damaged and not decodeable. Removed the record from identities tree."),
                    NotDecodeable::AddressDetails => anyhow!("Address details were damaged and not decodeable."),
                    NotDecodeable::AddressKey => anyhow!("Address key could not be decoded."),
                    NotDecodeable::Types => anyhow!("Types information from the database could not be decoded."),
                    NotDecodeable::Metadata => anyhow!("Version vector of metadata from the database could not be retrieved."),
                    NotDecodeable::Version => anyhow!("Version vector of metadata from the database could not be decoded."),
                    NotDecodeable::NameVersioned => anyhow!("Versioned name (the key for metadata) could not be decoded."),
                    NotDecodeable::EntryOrder => anyhow!("History entry order (storage key) from the database could not be decoded."),
                    NotDecodeable::Entry => anyhow!("History entry from the database could not be decoded."),
                    NotDecodeable::NetworkKey => anyhow!("Network key could not be decoded."),
                    NotDecodeable::Verifier => anyhow!("Network verifier could not be decoded."),
                }
            },
            Error::GenesisHashMismatch => anyhow!("Genesis hash mismatch."),
            Error::NetworkKeyMismatch => anyhow!("Network key does not match genesis hash and encryption algorithm."),
            Error::BadTypesFile(e) => anyhow!("Error loading default types. {}", e),
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
            Error::IdentityExists => anyhow!("Identity with this name already exists"),
            Error::InvalidDerivation => anyhow!("Invalid derivation format"),
            Error::UnknownEncryption => anyhow!("System error: unknown encryption algorithm"),
            Error::AddressKey(x) => anyhow!("Error generating address key. {}", x),
            Error::EncryptionMismatchId => anyhow!("Identity encryption algorithm not matching network encryption algorithm"),
            Error::EncryptionMismatchNetwork => anyhow!("Encryption algorithm from network specs not matching the one from network key"),
        }
    }
}
