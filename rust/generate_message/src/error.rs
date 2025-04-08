use definitions::{
    crypto::Encryption, error::MetadataError, keyring::NetworkSpecsKey, metadata::AddressBookEntry,
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
    Codec(#[from] parity_scale_codec::Error),

    #[error(transparent)]
    Specs(#[from] SpecsError),

    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error("qr error {0}")]
    Qr(Box<dyn std::error::Error>),

    #[error("Signature wrong length, expected {0}, got {1}")]
    SignatureWrongLength(usize, usize),

    #[error("Public key wrong length, expected {0}, got {1}")]
    PublicKeyWrongLength(usize, usize),

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
    /// Hot database does not allow to store more than one trusted URL address
    /// for RPC calls for same network.
    ///
    /// Alternative URL address could be used if the database is not updated
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
    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) entries
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

        /// URL address
        url: String,
    },

    /// Tried to fetch with `-u` key using address not known to the database,
    /// but got genesis hash that is already known.
    ///
    /// Likely tried to fetch with different address when one already is in the
    /// database.
    ///
    /// Hot database does not allow to store more than one trusted URL address
    /// for RPC calls for same network.
    ///
    /// Alternative URL address could be used if the database is not updated
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

        /// URL address used for fetch
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
    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) value
    /// stored in `SPECSTREEPREP` tree of the hot database. `NetworkSpecs`
    /// has field `name` with network name.
    ///
    /// This error appears if the `name` from `NetworkSpecs` differs from
    /// the `name` in `AddressBookEntry`.
    #[error(
        "Address book name {address_book_name} does not match the \
        name in corresponding network specs {specs_name}"
    )]
    AddressBookSpecsName {
        /// name in [`AddressBookEntry`](definitions::metadata::AddressBookEntry)
        address_book_name: String,

        /// name in [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
        specs_name: String,
    },

    #[error("network specs to send")]
    NetworkSpecs(NetworkSpecsKey),

    /// `ADDRESS_BOOK` tree of the hot database contains
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) entries with same
    /// `address` field and different `name` fields.
    ///
    /// Name in address book is taken from the metadata, metadata is fetched
    /// using RPC call, so one URL address can correspond to only one network
    /// name.
    #[error("Two different network names in entries for url address {url} in address book.")]
    TwoNamesForUrl {
        /// URL address, for which two condlicting names were found
        url: String,
    },

    /// `METATREE` of the hot database should contain at most two latest
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
        /// URL address used for RPC call
        url: String,

        /// what exactly has changed
        what: Changed,
    },

    /// Fetched network specs are not suitable for use in Vault.
    #[error("Problem with network specs from {url}. {error}")]
    FaultySpecs {
        /// URL address used for RPC cal
        url: String,

        /// what exactly is wrong with the network specs
        error: SpecsError,
    },

    /// Fetched genesis hash could not be transformed in expected `[u8; 32]` value.
    #[error(
        "Fetched genesis hash {value} has unexpected format and does not fit into [u8;32] array."
    )]
    UnexpectedFetchedGenesisHashFormat {
        /// genesis hash value as received through RPC call
        value: String,
    },

    /// Fetched block hash could not be transformed in expected `[u8; 32]` value.
    #[error(
        "Fetched block hash {value} has unexpected format \
        and does not fit into [u8;32] array."
    )]
    UnexpectedFetchedBlockHashFormat {
        /// block hash value as received through RPC call
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

    #[error("Unexpected public key length.")]
    UnexpectedPubKeyLength,

    #[error("Unexpected signature length.")]
    UnexpectedSignatureLength,
}

/// Errors on the active side with network specs received through RPC call
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SpecsError {
    /// Network base58 prefix information is not found neither in results of
    /// the `system_properties` RPC call, nor in `System` pallet of the metadata
    /// fetched with `state_getMetadata` RPC call.
    #[error("no base58 prefix")]
    NoBase58Prefix,

    /// Network base58 prefix information found through `system_properties` RPC
    /// call differs from the one from `System` pallet of the metadata fetched
    /// with `state_getMetadata` RPC call.
    ///
    /// Associated data is corresponding base58 prefixes.
    #[error("base58 prefix mismatch {specs}:{meta}")]
    Base58PrefixMismatch { specs: u16, meta: u16 },

    /// Network base58 prefix information received through `system_properties`
    /// RPC call could not be transformed into expected `u16` prefix.
    ///
    /// Associated data is base58 prefix as received.
    #[error("base58 prefix format not supported {value}")]
    Base58PrefixFormatNotSupported { value: String },

    /// Network decimals information **is not found** in the results if the
    /// `system_properties` RPC call, but the unit information **is found**.
    ///
    /// Associated data is the fetched unit value.
    #[error("unit no decimals {0}")]
    UnitNoDecimals(String),

    /// Network decimals information received through `system_properties`
    /// RPC call could not be transformed into expected `u8` value.
    ///
    /// Associated data is decimals information as received.
    #[error("decimals format not supported {value}")]
    DecimalsFormatNotSupported { value: String },

    /// Network unit information **is not found** in the results if the
    /// `system_properties` RPC call, but the decimals information **is found**.
    ///
    /// Associated data is the fetched decimals value, could be array too.
    #[error("decimals no unit {0}")]
    DecimalsNoUnit(String),

    /// Network unit information received through `system_properties`
    /// RPC call could not be transformed into expected `String` value.
    ///
    /// Associated data is unit information as received.
    #[error("unit format not supported {value}")]
    UnitFormatNotSupported { value: String },

    /// An array with more than one element is received for network decimals
    /// through `system_properties` RPC call. Received units are not an array.
    #[error("decimals array units not")]
    DecimalsArrayUnitsNot,

    /// Both the network decimals and network units are received as arrays,
    /// but the array length is different, i.e. something not straightforward
    /// is going on with the network.
    ///
    /// Associated data are the printed sets as they are received through the
    /// `system_properties` RPC call.
    #[error("decimals units array length {decimals} {unit}")]
    DecimalsUnitsArrayLength { decimals: String, unit: String },

    /// An array with more than one element is received for network units
    /// through `system_properties` RPC call. Received decimals are not an array.
    #[error("units array decimals not")]
    UnitsArrayDecimalsNot,

    /// Unit and decimal override is not allowed, when network has a single
    /// token.
    ///
    /// The only case when the decimals and unit override is permitted is when
    /// the network has a matching set of decimals and units, and user has to
    /// select the needed set element manually.
    ///
    /// If the network has a single decimals value and a single unit value, i.e.
    /// the values that would be suitable on their own, and user attempts to
    /// override it, this error appears.
    #[error("override ignored single")]
    OverrideIgnoredSingle,

    /// Unit and decimal override is not allowed, when network has no token.
    #[error("override ignored none")]
    OverrideIgnoredNone,
}

/// Data received through RPC call is different from the data in hot database
#[derive(Debug)]
pub enum Changed {
    /// Network base58 prefix in hot database (consistent between the metadata
    /// in `METATREE` and network specs in `SPECSPREPTREE`) is different from
    /// the one received through new RPC calls (also consistent).
    ///
    /// Associated data is the base58 prefix values in question.
    Base58Prefix { old: u16, new: u16 },

    /// Network genesis hash in hot database is different from the one fetched
    /// through a new RPC call.
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
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
    /// stored in `SPECSTREEPREP` tree of the hot database is different from
    /// the one fetched through a new RPC call.
    ///
    /// Network decimals value is expected to be permanent.
    Decimals { old: u8, new: u8 },

    /// Network decimals value in
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
    /// stored in `SPECSTREEPREP` tree of the hot database has some value,
    /// freshly fetched specs have no decimals.
    ///
    /// Network decimals value is expected to be permanent. Override for no
    /// decimals at the moment is blocked.
    DecimalsBecameNone { old: u8 },

    /// Network name is stored in multiple places in the hot database:
    ///
    /// - in `name` field of network specs
    ///   [`NetworkSpecs`](crate::network_specs::NetworkSpecs) stored
    ///   in `SPECSTREEPREP` tree
    /// - in `name` field of address book entry
    ///   [`AddressBookEntry`](crate::metadata::AddressBookEntry) stored in
    ///   `ADDRESS_BOOK` tree
    /// - encoded as a part of [`MetaKey`] and inside the network metadata
    ///   stored in `METATREE` tree
    ///
    /// All those entries eventually are produced from network name that is
    /// part of `Version` constant in `System` pallet of the network metadata.
    ///
    /// Network name is expected to be permanent. This error appears if the
    /// name derived from metadata fetched through a new RPC call is different.
    ///
    /// [`MetaKey`]: crate::keyring::MetaKey
    Name { old: String, new: String },

    /// Network unit value in
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
    /// stored in `SPECSTREEPREP` tree of the hot database is different from
    /// the one fetched through a new RPC call.
    ///
    /// Network unit value is expected to be permanent.
    Unit { old: String, new: String },

    /// Network unit value in
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
    /// stored in `SPECSTREEPREP` tree of the hot database has some value,
    /// freshly fetched specs have no unit.
    ///
    /// Network unit value is expected to be permanent. Override for no
    /// unit at the moment is blocked.
    UnitBecameNone { old: String },
}

impl std::fmt::Display for Changed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (insert, old, new) = match self {
            Changed::Base58Prefix { old, new } => {
                ("base58 prefix", old.to_string(), new.to_string())
            }
            Changed::GenesisHash { old, new } => {
                ("genesis hash", hex::encode(old), hex::encode(new))
            }
            Changed::Decimals { old, new } => ("decimals value", old.to_string(), new.to_string()),
            Changed::DecimalsBecameNone { old } => {
                ("decimals value", old.to_string(), "no value".to_string())
            }
            Changed::Name { old, new } => ("name", old.to_string(), new.to_string()),
            Changed::Unit { old, new } => ("unit", old.to_string(), new.to_string()),
            Changed::UnitBecameNone { old } => ("unit", old.to_string(), "no value".to_string()),
        };

        write!(
            f,
            "Network {insert} differs from the one in the hot database. Old: {old}. New: {new}."
        )
    }
}

/// `NotHex` errors occurring on the Active side
///
/// Expected to receive data in hexadecimal format, got something different.
/// [`NotHexActive`] specifies what was expected.
#[allow(dead_code)]
#[derive(Debug)]
pub enum NotHexActive {
    /// Network genesis hash, fetched through RPC call.
    ///
    /// Associated data is the URL address used for the fetching.
    FetchedGenesis { url: String },

    /// Network block hash, fetched through RPC call.
    ///
    /// Associated data is the URL address used for the fetching.
    FetchedBlock { url: String },

    /// User-entered block hash for metadata fetching
    EnteredBlock,
}
