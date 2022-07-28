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

/// Missing key in `generate_message` command
#[derive(Debug, thiserror::Error)]
pub enum CommandNeedKey {
    /// Command `show` needs key:
    ///
    /// - `-address_book` to show the contents of the hot database `ADDRESS_BOOK`
    /// tree
    /// - `-database` to show the contents of the hot database `METATREE` tree
    #[error("show needs key")]
    Show,

    /// Commands `add_specs` and `load_metadata` need key specifying content:
    ///
    /// - `-a` to process all networks with entries in the `ADDRESS_BOOK` tree
    /// - `-n` to process network by provided network name (in case of
    /// `load_metadata`) or network address book title (in case of `add_specs`)
    /// - `-u` to process network by provided url address
    #[error("content needs key")]
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
    #[error("crypto key required")]
    Crypto,

    /// Commands `derivations`, `make`, `sign`, `unwasm` require `-payload` key
    /// to be used, followed by the name of the file to process.
    #[error("payload key required")]
    Payload,

    /// Commands `make` and `sign` require `-msgtype` key, followed by what is
    /// contained in the payload: `add_specs`, `load_metadata` or `load_types`.
    #[error("msgtype key required")]
    MsgType,

    /// Command `sign` requires `-sufficient` key, followed by
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input, matching
    /// the payload content.
    #[error("sufficient key required")]
    SufficientCrypto,

    /// Command `make` requires signature if the message is to be signed by
    /// real verifier.
    #[error("signature required")]
    Signature,

    /// Command `make` requires public key of the verifier if the message is to
    /// be signed.
    ///
    /// Verifier could be:
    /// - custom (real user, with own public key), in this case a real signature
    /// would be required
    /// - test verifier (Alice), no signature
    #[error("verifier public key required")]
    Verifier,

    /// Command `remove` needs one of these keys:
    ///
    /// - `-title`, followed by network address book title, to remove
    /// `ADDRESS_BOOK` entry for the network, and metadata in case there are no
    /// more networks that use this metadata
    /// - `-name`, followed by network name argument, key `-version`, and
    /// network version argument, to remove specific metadata entry from
    /// `METATREE` by name and version
    #[error("remove needs a key")]
    Remove,

    /// If command `remove` is processed with the key `-name`, it requires also
    /// a key `-version` followed by the metadata version, to specify the
    /// version to be deleted.
    #[error("remove version")]
    RemoveVersion,

    /// Transaction with derivation import is generated for a specific network,
    /// this network is addressed by `-title` key followed by network address
    /// book title.
    #[error("derivations title")]
    DerivationsTitle,

    /// Command `meta_default_file` must have `-name` key followed by the
    /// network name to specify the metadata being exported.
    #[error("meta default filename")]
    MetaDefaultFileName,

    /// Command `meta_default_file` must have `-version` key followed by the
    /// network metadata version to specify the metadata being exported.
    #[error("meta default file version")]
    MetaDefaultFileVersion,

    /// Command `meta_at_block` must have `-u` key followed by the network url.
    #[error("meta at block url")]
    MetaAtBlockUrl,

    /// Command `meta_at_block` must have `-block` key followed by the
    /// hexadecimal block hash.
    #[error("meta at block hash")]
    MetaAtBlockHash,
}

/// Errors in `generate_message` input
#[derive(Debug, thiserror::Error)]
pub enum InputActive {
    /// Unable to read the file with input.
    #[error(transparent)]
    File(#[from] std::io::Error),

    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) could not be
    /// decoded.
    #[error("decoding sufficient crypto")]
    DecodingSufficientCrypto,

    /// The length of public key does not match the selected encryption
    /// algorithm.
    #[error("public key length")]
    PublicKeyLength,

    /// The length of data signature does not match the selected encryption
    /// algorithm.
    #[error("signature length")]
    SignatureLength,

    /// Tried to apply signature (i.e. used command `make` or `sign`) to
    /// metadata that is not suitable for use in Signer
    #[error(transparent)]
    FaultyMetadataInPayload(#[from] MetadataError),

    /// Provided data signature (entered separately or as a part of
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input) is invalid
    /// for given public key and data.
    #[error("bad signature")]
    BadSignature,

    /// Provided file contains no valid derivations that could be exported
    #[error("no valid derivations to export")]
    NoValidDerivationsToExport,

    /// User-entered block hash has invalid length
    #[error("block hash length")]
    BlockHashLength,
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
#[derive(Debug, thiserror::Error)]
pub enum CommandParser {
    /// Agrument sequence could not be processed.
    #[error("argument sequence could not be processed")]
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
    #[error("only one network id")]
    OnlyOneNetworkId,

    /// A key needed to run the command was not provided.
    ///
    /// Associated data is [`CommandNeedKey`] with more details.
    #[error(transparent)]
    NeedKey(#[from] CommandNeedKey),

    /// Same key was encountered more than once.
    ///
    /// Associated data is [`CommandDoubleKey`] with more details.
    #[error(transparent)]
    DoubleKey(#[from] CommandDoubleKey),

    /// A key must have been followed by some argument, but was not.
    ///
    /// Associated data is [`CommandNeedArgument`] with more details.
    #[error(transparent)]
    NeedArgument(#[from] CommandNeedArgument),

    /// An argument following the key is unsuitable.
    ///
    /// Associated data is [`CommandBadArgument`] with more details.
    #[error(transparent)]
    BadArgument(#[from] CommandBadArgument),

    /// Unexpected excessive entry in the command.
    ///
    /// Associated data is [`CommandUnexpected`] with more details.
    #[error(transparent)]
    Unexpected(#[from] CommandUnexpected),

    /// Command is not known.
    #[error("unknown command")]
    UnknownCommand,

    /// No command provided.
    #[error("no command provided")]
    NoCommand,
}

/// Key in `generate_message` command encountered twice
#[derive(Debug, thiserror::Error)]
pub enum CommandDoubleKey {
    /// Commands `add_specs` and `load_metadata` allow only one content key:
    ///
    /// - `-a` to process all networks with entries in the `ADDRESS_BOOK` tree
    /// - `-n` to process network by provided network name (in case of
    /// `load_metadata`) or network address book title (in case of `add_specs`)
    /// - `-u` to process network by provided url address
    #[error("content")]
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
    #[error("set")]
    Set,

    /// Command `add_specs` may use encryption override key to specify the
    /// encryption supported by the network. Encryption override must be used
    /// for networks without an entry in `ADDRESS_BOOK` tree. Encryption
    /// override could be also used for networks already recorded in
    /// `ADDRESS_BOOK` if the network supports more than one [`Encryption`].
    ///
    /// Encryption override key (`-ed25519`, `-sr25519`, `-ecdsa`) could not be
    /// used more than once.
    #[error("crypto override")]
    CryptoOverride,

    /// Command `add_specs`, when used for networks without network specs
    /// entries in `SPECSTREEPREP` and with more than one token supported,
    /// could use token override. For this, key `-token` followed by `u8`
    /// decimals and `String` unit arguments is used.
    ///
    /// Token override key may be used only once.
    #[error("token override")]
    TokenOverride,

    /// Command `add_specs` could use network title override to set up the
    /// network title displayed in Signer.
    ///
    /// Title override key may be used only once.
    #[error("title override")]
    TitleOverride,

    /// Command `make` must have exactly one `-crypto` key, followed by the
    /// encryption argument.
    #[error("crypto key")]
    CryptoKey,

    /// Commands `make` and `sign` must have exactly one `-msgtype` key,
    /// followed by the message type argument.
    #[error("msg type")]
    MsgType,

    /// Command `make` can have at most one `-verifier` key.
    #[error("verifier")]
    Verifier,

    /// Commands `derivations`, `make`, `sign`, `unwasm` must have exactly one
    /// `-payload` key, followed by the name of the file to process.
    #[error("payload")]
    Payload,

    /// Command `make` can have at most one `-signeture` key.
    #[error("signature")]
    Signature,

    /// Commands `make` and `sign` can have at most one `-name` key,
    /// followed by the custom name of the export file in `../files/signed/`
    /// folder.
    #[error("name")]
    Name,

    /// Command `sign` must have exactly one `-sufficient` key, followed by the
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) input, matching
    /// the payload content.
    #[error("sufficient")]
    SufficientCrypto,

    /// Command `remove` must have exactly one content key:
    ///
    /// - `-title`, followed by network address book title, to remove
    /// `ADDRESS_BOOK` entry for the network, and metadata in case there are no
    /// more networks that use this metadata
    /// - `-name`, followed by network name argument, key `-version`, and
    /// network version argument, to remove specific metadata entry from
    /// `METATREE` by name and version
    #[error("remove")]
    Remove,

    /// Command `derivations` must have exactly one `-title` key followed by
    /// network address book title for network in which the derivation export
    /// is generated.
    #[error("derivations title")]
    DerivationsTitle,

    /// Command `meta_default_file` must have exactly one `-name` key followed
    /// by the network name.
    #[error("meta default file name")]
    MetaDefaultFileName,

    /// Command `meta_default_file` must have exactly one `-version` key followed
    /// by the network version.
    #[error("meta default file version")]
    MetaDefaultFileVersion,

    /// Command `meta_at_block` must have exactly one `-u` key followed by the
    /// network url.
    #[error("meta at block url")]
    MetaAtBlockUrl,

    /// Command `meta_at_block` must have exactly one `-block` key followed by
    /// the hexadecimal block hash.
    #[error("meta at block hash")]
    MetaAtBlockHash,
}

/// Missing argument for the key in `generate_message` command
#[derive(Debug, thiserror::Error)]
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
    #[error("token unit")]
    TokenUnit,

    /// Key `-token` in `add_specs` command was supposed to be followed by `u8`
    /// decimals and `String` unit agruments.
    ///
    /// Decimals argument was not provided.
    ///
    /// Note: this error can occur only when `-token` is the last key in the
    /// sequence. Otherwise the parser will try to interpret as `u8` decimals
    /// the next key and complain that it is not `u8`.
    #[error("token decimals")]
    TokenDecimals,

    /// Key `-title` in `add_specs` command was supposed to be followed by
    /// `String` argument, which was not provided.
    #[error("title override")]
    TitleOverride,

    /// Commands `add_specs` and `load_metadata` with content key `-n` require
    /// network identifier: network address book title for `add_specs` and
    /// network name for `load_metadata`
    #[error("network name")]
    NetworkName,

    /// Commands `add_specs` and `load_metadata` with content key `-u` require
    /// url address input for rpc calls
    #[error("network url")]
    NetworkUrl,

    /// Key `-crypto` in `make` command was supposed to be followed by an
    /// agrument:
    ///
    /// - `ed25519`
    /// - `sr25519`
    /// - `ecdsa`
    /// - `none`
    #[error("crypto key")]
    CryptoKey,

    /// Key `-verifier` in `make` command was supposed to be followed by an
    /// argument or additional key:
    ///
    /// - argument `Alice`
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    #[error("verifier")]
    Verifier,

    /// Key sequence `-verifier -hex` in `make` command must be followed by a
    /// hexadecimal verifier public key
    #[error("verifier hex")]
    VerifierHex,

    /// Key sequence `-verifier -file` in `make` command must be followed by a
    /// filename of the file in dedicated `FOLDER` with verifier public key as
    /// `&[u8]`.
    #[error("verifier file")]
    VerifierFile,

    /// Key `-payload` must be followed by a filename of the file:
    /// - in dedicated `FOLDER` for `make` and `sign` commands
    /// - in `../generate_message/` for `derivations` and `unwasm` commands
    #[error("payload")]
    Payload,

    /// Key `-msgtype` in `make` and `sign` must be followed by a valid message
    /// type argument:
    ///
    /// - `add_specs`
    /// - `load_metadata`
    /// - `load_types`
    #[error("msgtype")]
    MsgType,

    /// Key `-signature` in `make` command was supposed to be followed by an
    /// additional key:
    ///
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    #[error("signature")]
    Signature,

    /// Key sequence `-signature -hex` in `make` command must be followed by a
    /// hexadecimal signature.
    #[error("signature hex")]
    SignatureHex,

    /// Key sequence `-signature -file` in `make` command must be followed by a
    /// filename of the file in dedicated `FOLDER` with signature as `&[u8]`.
    #[error("signature file")]
    SignatureFile,

    /// Key `-name` in `make` and `sign` commands, if used, must be followed by
    /// a filename of target file in `../files/signed`.
    #[error("name")]
    Name,

    /// Key `-sufficient` in `sign` command was supposed to be followed by an
    /// additional key:
    ///
    /// - key `-hex` followed by hexadecimal input argument
    /// - key `-file` followed by filename argument
    #[error("sufficient crypto")]
    SufficientCrypto,

    /// Key sequence `-sufficient -hex` in `sign` command must be followed by a
    /// hexadecimal SCALE-encoded `SufficientCrypto` string.
    #[error("sufficient crypto hex")]
    SufficientCryptoHex,

    /// Key sequence `-sufficient -file` in `sign` command must be followed by a
    /// filename of the file in dedicated `FOLDER` with SCALE-encoded
    /// `SufficientCrypto` as `&[u8]`.
    #[error("sufficient crypto file")]
    SufficientCryptoFile,

    /// Command `make` must be followed by additional keys.
    #[error("make")]
    Make,

    /// Command `sign` must be followed by additional keys.
    #[error("sign")]
    Sign,

    /// Key `-title` in `remove` command must be followed by network address
    /// book title.
    #[error("remove title")]
    RemoveTitle,

    /// Key `-name` in `remove` command must be followed by network name.
    #[error("remove name")]
    RemoveName,

    /// Key-argument sequence `remove -name <***> -version` in `remove` command
    /// must be followed by network version.
    #[error("remove version")]
    RemoveVersion,

    /// Command `derivations` must be followed by additional keys.
    #[error("derivations")]
    Derivations,

    /// Key `-title` in `derivations` command must be followed by network
    /// address book title.
    #[error("derivations title")]
    DerivationsTitle,

    /// Key `-name` in `meta_default_file` command must be followed by network
    /// name.
    #[error("meta default file name")]
    MetaDefaultFileName,

    /// Key `-version` in `meta_default_file` command must be followed by
    /// network version.
    #[error("meta default file version")]
    MetaDefaultFileVersion,

    /// Command `check_file` must be followed by the file path
    #[error("checkfile")]
    CheckFile,

    /// Command `show -specs` must be followed by the network address book title
    #[error("show specs title")]
    ShowSpecsTitle,

    /// Key `-u` in `meta_at_block` command must be followed by the network url.
    #[error("meta at block url")]
    MetaAtBlockUrl,

    /// Key `-block` in `meta_at_block` command must be followed by the
    /// hexadecimal block hash.
    #[error("meta at block hash")]
    MetaAtBlockHash,
}

/// Unsuitable argument for the key in `generate_message` command
#[derive(Debug, thiserror::Error)]
pub enum CommandBadArgument {
    /// The valid arguments for key `-crypto` are:
    ///
    /// - `ed25519`
    /// - `sr25519`
    /// - `ecdsa`
    /// - `none`
    #[error("crypto key")]
    CryptoKey,

    /// Key `-token` must be followed by `u8` decimals and `String` unit values.
    ///
    /// This error appears if the value immediately after `-token` could not be
    /// parsed as `u8`.
    #[error("decimals format")]
    DecimalsFormat,

    /// The valid arguments for key `-msgtype` are:
    ///
    /// - `add_specs`
    /// - `load_metadata`
    /// - `load_types`
    #[error("msg type")]
    MsgType,

    /// Signature may be entered from a file or as a hexadecimal string.
    /// Key `-signature` may be followed by:
    ///
    /// `-file` followed by the name of the file in dedicated `FOLDER` with
    /// signature as `&[u8]`
    /// `-hex` followed by hexadecimal signature
    #[error("signature")]
    Signature,

    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) may be entered
    /// from a file or as a hexadecimal string.
    /// Key `-sufficient` may be followed by:
    ///
    /// `-file` followed by the name of the file in dedicated `FOLDER` with
    /// SCALE-encoded `SufficientCrypto` as `&[u8]`
    /// `-hex` followed by hexadecimal SCALE-encoded `SufficientCrypto` string
    #[error("sufficient crypto")]
    SufficientCrypto,

    /// Verifier entered after key `-verifier` may be:
    ///
    /// `-file` followed by name of the file in dedicated `FOLDER` with verifier
    /// public key as `&[u8]`
    /// `-hex` followed by hexadecimal verifier public key
    /// `Alice`
    #[error("verifier")]
    Verifier,

    /// Commands `remove` and `meta_default_file` require network version to be
    /// specified after key `-version`.
    ///
    /// This error appears if the value immediately after `-version` could not be
    /// parsed as `u32`.
    #[error("version format")]
    VersionFormat,
}

/// Unexpected content in `generate_message` command
#[derive(Debug, thiserror::Error)]
pub enum CommandUnexpected {
    /// Command `make` with `-verifier Alice` can not accept the signature.
    #[error("alice signature")]
    AliceSignature,

    /// Commands `add_specs` and `load_metadata` can not accept name or url
    /// address if `-a` (process all) content key is used.
    #[error("key a content")]
    KeyAContent,

    /// Command `make` with `-crypto none` can not accept the signature.
    #[error("signature no crypto")]
    SignatureNoCrypto,

    /// Command `make` with `-crypto none` can not accept the verifier public key.
    #[error("verifier no crypto")]
    VerifierNoCrypto,
}

/// Errors on the active side with network specs received through rpc call
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SpecsError {
    /// Network base58 prefix information is not found neither in results of
    /// the `system_properties` rpc call, nor in `System` pallet of the metadata
    /// fetched with `state_getMetadata` rpc call.
    #[error("no base58 prefix")]
    NoBase58Prefix,

    /// Network base58 prefix information found through `system_properties` rpc
    /// call differs from the one from `System` pallet of the metadata fetched
    /// with "state_getMetadata" rpc call.
    ///
    /// Associated data is corresponding base58 prefixes.
    #[error("base58 prefix mismatch {specs}:{meta}")]
    Base58PrefixMismatch { specs: u16, meta: u16 },

    /// Network base58 prefix information received through `system_properties`
    /// rpc call could not be transformed into expected `u16` prefix.
    ///
    /// Associated data is base58 prefix as received.
    #[error("base58 prefix format not supported {value}")]
    Base58PrefixFormatNotSupported { value: String },

    /// Network decimals information **is not found** in the results if the
    /// `system_properties` rpc call, but the unit information **is found**.
    ///
    /// Associated data is the fetched unit value.
    #[error("unit no decimals {0}")]
    UnitNoDecimals(String),

    /// Network decimals information received through `system_properties`
    /// rpc call could not be transformed into expected `u8` value.
    ///
    /// Associated data is decimals information as received.
    #[error("decimals format not supported {value}")]
    DecimalsFormatNotSupported { value: String },

    /// Network unit information **is not found** in the results if the
    /// `system_properties` rpc call, but the decimals information **is found**.
    ///
    /// Associated data is the fetched decimals value, could be array too.
    #[error("decimals no unit {0}")]
    DecimalsNoUnit(String),

    /// Network unit information received through `system_properties`
    /// rpc call could not be transformed into expected `String` value.
    ///
    /// Associated data is unit information as received.
    #[error("unit format not supported {value}")]
    UnitFormatNotSupported { value: String },

    /// An array with more than one element is received for network decimals
    /// through `system_properties` rpc call. Received units are not an array.
    #[error("decimals array units not")]
    DecimalsArrayUnitsNot,

    /// Both the network decimals and network units are received as arrays,
    /// but the array length is different, i.e. something not straightforward
    /// is going on with the network.
    ///
    /// Associated data are the printed sets as they are received through the
    /// `system_properties` rpc call.
    #[error("decimals units array length {decimals} {unit}")]
    DecimalsUnitsArrayLength { decimals: String, unit: String },

    /// An array with more than one element is received for network units
    /// through `system_properties` rpc call. Received decimals are not an array.
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

    /// Network decimals value in
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// stored in `SPECSTREEPREP` tree of the hot database has some value,
    /// freshly fetched specs have no decimals.
    ///
    /// Network decimals value is expected to be permanent. Override for no
    /// decimals at the moment is blocked.
    DecimalsBecameNone { old: u8 },

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
    ///
    /// [`MetaKey`]: crate::keyring::MetaKey
    Name { old: String, new: String },

    /// Network unit value in
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// stored in `SPECSTREEPREP` tree of the hot database is different from
    /// the one fetched through a new rpc call.
    ///
    /// Network unit value is expected to be permanent.
    Unit { old: String, new: String },

    /// Network unit value in
    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
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
            "Network {} differs from the one in the hot database. Old: {}. New: {}.",
            insert, old, new
        )
    }
}

/// NotHex errors occurring on the Active side
///
/// Expected to receive data in hexadecimal format, got something different.
/// [`NotHexActive`] specifies what was expected.
#[derive(Debug)]
pub enum NotHexActive {
    /// Network genesis hash, fetched through rpc call.
    ///
    /// Associated data is the url address used for the fetching.
    FetchedGenesis { url: String },

    /// Network block hash, fetched through rpc call.
    ///
    /// Associated data is the url address used for the fetching.
    FetchedBlock { url: String },

    /// User-entered block hash for metadata fetching
    EnteredBlock,
}
