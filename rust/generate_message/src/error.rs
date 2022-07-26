use definitions::{
    crypto::Encryption,
    error::MetadataError,
    error_active::{Changed, CommandNeedKey, CommandParser, InputActive, SpecsError},
    keyring::NetworkSpecsKey,
    metadata::AddressBookEntry,
};
use sp_core::H256;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] db_handling::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Definitions(#[from] definitions::error::Error),

    #[error(transparent)]
    Sled(#[from] sled::Error),

    #[error(transparent)]
    JsonRPC(#[from] jsonrpsee::core::error::Error),

    #[error("qr error {0}")]
    Qr(Box<dyn std::error::Error>),

    /// Network specs are already in the database
    #[error("specs in db")]
    SpecsInDb {
        /// network name
        name: String,

        /// network supported encryption
        encryption: Encryption,
    },

    #[error("not found {0}")]
    NotFound(String),

    #[error("not supported")]
    NotSupported,

    #[error("two genesis hash variants for name")]
    TwoGenesisHashVariantsForName {
        /// network name
        name: String,
    },

    #[error("two url variants for name")]
    TwoUrlVariantsForName {
        /// network name
        name: String,
    },

    #[error("two base58 for name")]
    TwoBase58ForName {
        /// network name
        name: String,
    },

    #[error("address book empty")]
    AddressBookEmpty,

    /// Tried to fetch with `-u` key using address already known to the database
    #[error("url in db")]
    UKeyUrlInDb {
        /// network address book title
        title: String,

        /// url address
        url: String,
    },

    /// Tried to fetch with `-u` key using address not known to the database,
    /// but got genesis hash that is already known.
    ///
    /// Likely tried to fetch with different address when one already is in the
    /// database.
    ///
    /// Hot database does not allow to store more than one trusted url address
    /// for rpc calls for same network.
    ///
    /// Alternative url address could be used if the database is not updated
    /// (`-d` key is used).
    ///
    /// To update the address in the database in case the old one is no longer
    /// acceptable, one should remove old entry, and only then add the new one.
    #[error("hash in db")]
    UKeyHashInDb {
        /// address book entry with exactly matching genesis hash
        address_book_entry: AddressBookEntry,

        /// url address used for fetch
        url: String,
    },

    #[error("unexpected metadata format")]
    UnexpectedMetadataFormat,

    #[error("unexpected genesis hash format")]
    UnexpectedGenesisHashFormat,

    #[error("unexpected system properties format")]
    UnexpectedSystemPropertiesFormat,

    #[error("unexpected block hash format")]
    UnexpectedBlockHashFormat,

    #[error("address book specs name")]
    AddressBookSpecsName {
        /// name in [`AddressBookEntry`](definitions::metadata::AddressBookEntry)
        address_book_name: String,

        /// name in [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
        specs_name: String,
    },

    #[error("network specs to send")]
    NetworkSpecsToSend(NetworkSpecsKey),

    #[error("two names for url")]
    TwoNamesForUrl {
        /// url address, for which two condlicting names were found
        url: String,
    },

    #[error("hot database metadata over two entries")]
    HotDatabaseMetadataOverTwoEntries {
        /// network name
        name: String,
    },

    #[error("a")]
    HotDatabaseMetadataSameVersionTwice {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// Fetched network metadata version is lower than the one already in the
    /// hot database.
    #[error("earlier version")]
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
    #[error("same version different metadata")]
    SameVersionDifferentMetadata {
        /// network name
        name: String,

        /// network version
        version: u32,

        /// optionally recorded block hash for which the metadata was fetched
        ///when recorded in the database
        block_hash_in_db: Option<H256>,

        /// block hash for which the metadata is fetched now
        block_hash_in_fetch: Option<H256>,
    },

    /// Fetched data is different from the one already in the hot database.
    #[error("values changed")]
    ValuesChanged {
        /// url address used for rpc call
        url: String,

        /// what exactly has changed
        what: Changed,
    },

    /// Fetched network specs are not suitable for use in Signer.
    #[error("faulty specs")]
    FaultySpecs {
        /// url address used for rpc cal
        url: String,

        /// what exactly is wrong with the network specs
        error: SpecsError,
    },

    /// Fetched genesis hash could not be transformed in expected [u8; 32] value.
    #[error("unexpected fetched genesis hash format")]
    UnexpectedFetchedGenesisHashFormat {
        /// genesis hash value as received through rpc call
        value: String,
    },

    /// Fetched block hash could not be transformed in expected [u8; 32] value.
    #[error("unexpected fetched block hash format {value}")]
    UnexpectedFetchedBlockHashFormat {
        /// block hash value as received through rpc call
        value: String,
    },

    /// User-entered block hash has invalid length
    #[error("block hash length")]
    BlockHashLength,

    #[error(transparent)]
    Specs(#[from] SpecsError),

    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error("address book entry with name {name}")]
    AddressBookEntryWithName { name: String },

    #[error(transparent)]
    Input(#[from] InputActive),

    /// Provided data signature (entered separately or as a part of
    /// [`SufficientCrypto`](definitions::crypto::SufficientCrypto) input) is invalid
    /// for given public key and data.
    #[error("bad signature")]
    BadSignature,

    /// A key needed to run the command was not provided.
    ///
    /// Associated data is [`CommandNeedKey`] with more details.
    #[error("need key")]
    NeedKey(CommandNeedKey),

    #[error(transparent)]
    CommandParser(#[from] CommandParser),

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),
}
