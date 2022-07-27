use definitions::{
    crypto::Encryption,
    error::MetadataError,
    error_active::{Changed, CommandNeedKey, CommandParser, InputActive, SpecsError},
    keyring::NetworkSpecsKey,
    metadata::AddressBookEntry,
};
use sp_core::H256;

/// Generate Message result.
pub type Result<T> = std::result::Result<T, Error>;

/// Generate Message error.
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

    #[error(transparent)]
    CommandParser(#[from] CommandParser),

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),

    #[error(transparent)]
    Input(#[from] InputActive),

    #[error(transparent)]
    Specs(#[from] SpecsError),

    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error("qr error {0}")]
    Qr(Box<dyn std::error::Error>),

    /// Network specs are already in the database
    #[error(
        "Network specs entry for {name} and encryption {} is already in database.",
        .encryption.show(),
    )]
    SpecsInDb {
        /// network name
        name: String,

        /// network supported encryption
        encryption: Encryption,
    },

    #[error("not found {0}")]
    NotFound(String),

    #[error("Not supported.")]
    NotSupported,

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) entries with same
    /// `name` field and different `genesis_hash` values.
    ///
    /// This is not allowed, as it would cause uncertainty in `load_metadata`
    /// message generation, which is build using metadata and genesis hash.
    #[error("Two different genesis hash entries for network {name} in address book.")]
    TwoGenesisHashVariantsForName {
        /// network name
        name: String,
    },

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) entries with same
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
    #[error("Two different url entries for network {name} in address book.")]
    TwoUrlVariantsForName {
        /// network name
        name: String,
    },

    /// `SPECSTREEPREP` tree of the hot database contains
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) entries
    /// with same network name, but different base58 prefix.
    #[error("Two different base58 entries for network {name}.")]
    TwoBase58ForName {
        /// network name
        name: String,
    },

    /// `ADDRESS_BOOK` tree of the hot database has no entries to process.
    #[error("Address book is empty")]
    AddressBookEmpty,

    /// Tried to fetch with `-u` key using address already known to the database
    #[error(
        "There is already an entry with address {url} \
            for network {title}. Known networks should be processed \
            with `-n` content key."
    )]
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
    #[error(
        "Fetch at {url} resulted in data already known to the hot database. \
        Network {} with genesis hash {} \
        has address set to {}. To change the url, delete old entry.",
        .address_book_entry.name,
        hex::encode(.address_book_entry.genesis_hash),
        .address_book_entry.address,
    )]
    UKeyHashInDb {
        /// address book entry with exactly matching genesis hash
        address_book_entry: AddressBookEntry,

        /// url address used for fetch
        url: String,
    },

    #[error("Unexpected metadata format.")]
    UnexpectedMetadataFormat,

    #[error("Unexpected genesis hash format.")]
    UnexpectedGenesisHashFormat,

    #[error("Unexpected system properties format.")]
    UnexpectedSystemPropertiesFormat,

    #[error("Unexpected block hash format.")]
    UnexpectedBlockHashFormat,

    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) in hot database
    /// contains `encryption` and `genesis_hash` fields, from which the
    /// corresponding [`NetworkSpecsKey`] could be built.
    ///
    /// `NetworkSpecsKey` has an associated
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) value
    /// stored in `SPECSTREEPREP` tree of the hot database. `NetworkSpecsToSend`
    /// has field `name` with network name.
    ///
    /// This error appears if the `name` from `NetworkSpecsToSend` differs from
    /// the `name` in `AddressBookEntry`.
    #[error(
        "Address book name {address_book_name} does not match the \
        name in corresponding network specs {specs_name}"
    )]
    AddressBookSpecsName {
        /// name in [`AddressBookEntry`](definitions::metadata::AddressBookEntry)
        address_book_name: String,

        /// name in [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
        specs_name: String,
    },

    #[error("network specs to send")]
    NetworkSpecsToSend(NetworkSpecsKey),

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) entries with same
    /// `address` field and different `name` fields.
    ///
    /// Name in address book is taken from the metadata, metadata is fetched
    /// using rpc call, so one url address can correspond to only one network
    /// name.
    #[error("Two different network names in entries for url address {url} in address book.")]
    TwoNamesForUrl {
        /// url address, for which two condlicting names were found
        url: String,
    },

    /// `METATREE` of the hot database shoud contain at most two latest
    /// metadata versions for each network, with the older entries being
    /// deleted as the new ones appear.
    ///
    /// This error appears if during the processing more than two metadata
    /// entries for the network are found.
    #[error("More than two entries for network {name} in hot database.")]
    HotDatabaseMetadataOverTwoEntries {
        /// network name
        name: String,
    },

    /// `METATREE` of the hot database has two entries for a network with the
    /// same metadata version.
    ///
    /// Note: at this moment should be unreachable, since the entries are
    /// getting checked for consistency with [`MetaKey`].
    ///
    /// [`MetaKey`]: definitions::keyring::MetaKey
    #[error("Two entries for {name} version {version}.")]
    HotDatabaseMetadataSameVersionTwice {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// Fetched network metadata version is lower than the one already in the
    /// hot database.
    #[error(
        "For {name} the newly received version ({new_version}) is lower \
        than the latest version in the hot database ({old_version})."
    )]
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
    #[error(
        "Metadata {name}{version} fetched now at block hash {} \
        differs from the one in the hot database, block hash {}.",
        .block_hash_in_db.map(hex::encode).unwrap_or_else(|| String::from("unknown")),
        .block_hash_in_fetch.map(hex::encode).unwrap_or_else(|| String::from("unknown")),
    )]
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
    #[error("Network from {url} error: {what}")]
    ValuesChanged {
        /// url address used for rpc call
        url: String,

        /// what exactly has changed
        what: Changed,
    },

    /// Fetched network specs are not suitable for use in Signer.
    #[error("Problem with network specs from {url}. {error}")]
    FaultySpecs {
        /// url address used for rpc cal
        url: String,

        /// what exactly is wrong with the network specs
        error: SpecsError,
    },

    /// Fetched genesis hash could not be transformed in expected [u8; 32] value.
    #[error(
        "Fetched genesis hash {value} has unexpected format and does not fit into [u8;32] array."
    )]
    UnexpectedFetchedGenesisHashFormat {
        /// genesis hash value as received through rpc call
        value: String,
    },

    /// Fetched block hash could not be transformed in expected [u8; 32] value.
    #[error(
        "Fetched block hash {value} has unexpected format \
        and does not fit into [u8;32] array."
    )]
    UnexpectedFetchedBlockHashFormat {
        /// block hash value as received through rpc call
        value: String,
    },

    /// User-entered block hash has invalid length
    #[error("Provided block hash has wrong length.")]
    BlockHashLength,

    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) searched in
    /// `ADDRESS_BOOK` tree of the hot database by matching the `name` field.
    ///
    /// Associated data is the network name used for searching.
    #[error("Could not find address book entry for network name {name}")]
    AddressBookEntryWithName { name: String },

    /// Provided data signature (entered separately or as a part of
    /// [`SufficientCrypto`](definitions::crypto::SufficientCrypto) input) is invalid
    /// for given public key and data.
    #[error("Bad signature.")]
    BadSignature,

    /// A key needed to run the command was not provided.
    ///
    /// Associated data is [`CommandNeedKey`] with more details.
    #[error("Key needs to be used {0}")]
    NeedKey(#[from] CommandNeedKey),
}
