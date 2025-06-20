use std::fmt::Write;

use definitions::{
    crypto::Encryption,
    error::MetadataError,
    error_signer::GeneralVerifierForContent,
    keyring::NetworkSpecsKey,
    network_specs::{ValidCurrentVerifier, Verifier, VerifierValue},
};
use sp_core::H256;

/// Transaction parsing result.
pub type Result<T> = std::result::Result<T, Error>;

/// Transaction parsing error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Definitions(#[from] definitions::error::Error),

    #[error(transparent)]
    Sled(#[from] sled::Error),

    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error(transparent)]
    Parser(#[from] parser::Error),

    /// Received transaction is unexpectedly short, more bytes were expected.
    #[error("Input is too short.")]
    TooShort,

    /// Key corresponding to the address was not found in the db
    #[error("Address {0} was not found in DB")]
    AddrNotFound(String),

    /// All transactions are expected to be the Substrate ones, starting with
    /// hexadecimal `53`.
    ///
    /// Associated data is the first two elements of the hexadecimal string in
    /// received transaction.
    #[error(
        "Only Substrate transaction are supported. \
        Transaction is expected to start with 0x53, this one starts with 0x{0}"
    )]
    NotSubstrate(String),

    /// Update payload signature is invalid for given public key, encryption
    /// algorithm and payload content
    #[error("Bad signature.")]
    BadSignature,

    /// There is a limited number of payloads supported by the Vault. Payload
    /// type is declared in the transaction prelude `53xxyy` in `yy` part.
    ///
    /// Currently supported payloads are:
    ///
    /// - `00` mortal signable transaction
    /// - `02` immortal signable transaction
    /// - `03` text message
    /// - `80` `load_metadata` update
    /// - `81` `load_types` update
    /// - `c1` `add_specs` update
    /// - `de` `derivations` update
    /// - `f0` print all available cards (testing tool)
    ///
    /// Other codes are not supported, the error associated data contains the
    /// hexadecimal string with unsupported payload code.
    #[error("Payload type with code 0x{0} is not supported.")]
    PayloadNotSupported(String),

    /// DB error.
    #[error("Database error. Internal error. {0}")]
    DbError(#[from] db_handling::Error),

    /// Network metadata needed to parse historical transaction, no entries at
    /// all for a given network name.
    #[error(
        "Historical transaction was generated in network {name} and processed. \
        Currently there are no metadata entries for the network, and \
        transaction could not be processed again. Add network metadata."
    )]
    HistoricalMetadata { name: String },

    /// More than one entry found for network specs with given `name` and
    /// `encryption`, when trying to parse transaction from historical record.
    // TODO: goes obsolete if we add `genesis_hash` field to `SignDisplay`
    #[error("specs collision: {name}")]
    SpecsCollision {
        name: String,
        encryption: Encryption,
    },

    /// [`OrderedNetworkSpecs`] needed to parse
    /// historical transactions saved into history log, searched by network
    /// name and encryption.
    ///
    /// [`OrderedNetworkSpecs`]: definitions::network_specs::OrderedNetworkSpecs
    #[error(
        "Could not find network specs for {name} with encryption {} \
        needed to decode historical transaction.",
        encryption.show()
    )]
    HistoryNetworkSpecs {
        name: String,
        encryption: Encryption,
    },

    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
    /// received in `add_specs` payload are for a network that already has
    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Vault database with **same**
    /// [`NetworkSpecsKey`], and the permanent components of the network
    /// specs stores and received are different.
    ///
    /// The components that could not be changed by an update, without removing
    /// the network completely, are:
    ///
    /// - `base58prefix`, network-associated base58 prefix  
    /// - `decimals`  
    /// - `name`, network name, as it appears in the network metadata  
    /// - `unit`
    #[error(
        "Similar network specs are already stored in the database under key {}. \
        Network specs in received payload have different unchangeable values \
        (base58 prefix, decimals, encryption, network name, unit).",
        hex::encode(.0.key())
    )]
    ImportantSpecsChanged(NetworkSpecsKey),

    /// Network name and version from metadata received in `load_metadata`
    /// message already have a corresponding entry in `METATREE` tree of the
    /// Vault database. However, the received metadata is different from
    /// the one already stored in the database.
    #[error(
        "Metadata for {name}{version} is already in the database and is \
        different from the one in received payload."
    )]
    SameNameVersionDifferentMeta { name: String, version: u32 },

    /// There is a limited number of encryption algorithms supported by the
    /// Vault. Encryption algorithm is declared in the transaction prelude
    /// `53xxyy` in `xx` part.
    ///
    /// For signable transactions (i.e. with prelude `53xx00`, `53xx02` and
    /// `53xx03`) currently supported encryption algorithms are:
    ///
    /// - `00` for `Ed25519`
    /// - `01` for `Sr25519`
    /// - `02` for `Ecdsa`
    ///
    /// In signable transaction the encryption algorithm corresponds to the
    /// encryption associated with the address that generated the transaction
    /// and can sign it (and thus to the encryption supported by the network
    /// in which the transaction is generated).
    ///
    /// Update transactions have currently supported encryption codes:
    ///
    /// - `00` for `Ed25519`
    /// - `01` for `Sr25519`
    /// - `02` for `Ecdsa`
    /// - `ff` for unsigned update transactions
    ///
    /// In signed update transactions the encryption code indicates which
    /// algorithm to use for update signature verification.
    ///
    /// Unsigned update transactions have no associated signature, are not
    /// checked and are strongly discouraged.
    ///
    /// Other encryption codes are not supported, the error associated data
    /// contains the hexadecimal string with unsupported encryption code.
    #[error("encryption not supported {0}")]
    EncryptionNotSupported(String),

    /// Error parsing extensions of a signable transaction with all available
    /// versions of metadata for given network.
    #[error(
        "Failed to decode extensions. Please try updating metadata for \
        {network_name} network. {}",
        display_parsing_errors(network_name, errors)
    )]
    AllExtensionsParsingFailed {
        network_name: String,
        errors: Vec<(u32, parser::Error)>,
    },

    /// Can not separate method from extensions, bad transaction.
    #[error("Unable to separate transaction method and extensions.")]
    SeparateMethodExtensions,

    /// Received transaction that should be parsed prior to approval (with
    /// prelude `53xx00` or `53xx02`) is generated in the network that has no
    /// metadata entries in the `METATREE` tree of the database.
    ///
    /// Without metadata the transaction could not be decoded.
    #[error(
        "Input transaction is generated in network {name}. \
        Currently there are no metadata entries for it, and \
        transaction could not be processed. Add network metadata."
    )]
    NoMetadata { name: String },

    /// Received signable transaction (with prelude `53xx00`, `53xx02` or
    /// `53xx03`) is generated in the network that has no corresponding
    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in the
    /// `SPECSTREE` tree of the database.
    #[error(
        "Input generated within unknown network and could not be processed. \
        Add network with genesis hash {} and encryption {}.",
        hex::encode(genesis_hash),
        encryption.show()
    )]
    UnknownNetwork {
        genesis_hash: H256,

        encryption: Encryption,
    },

    #[error("Received message could not be read. {0}")]
    Codec(#[from] parity_scale_codec::Error),

    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
    /// received in `add_specs` payload are for a network that already has
    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Vault database with not necessarily
    /// same encryption, i.e. **possibly different** [`NetworkSpecsKey`],
    /// and base58 prefix in stored network specs is different from the base58
    /// prefix in the received ones.
    #[error(
        "Network {name} with genesis hash {} already has entries in \
        the database with base58 prefix {base58_database}. Received \
        network specs have same genesis hash and different base58 prefix {base58_input}.",
        hex::encode(genesis_hash)
    )]
    AddSpecsDifferentBase58 {
        genesis_hash: H256,
        name: String,
        base58_database: u16,
        base58_input: u16,
    },

    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
    /// received in `add_specs` payload are for a network that already has
    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Vault database with not necessarily
    /// same encryption, i.e. **possibly different** [`NetworkSpecsKey`],
    /// and network name in stored network specs is different from the network
    /// name in the received ones.
    #[error(
        "Network with genesis hash {} has name {name_database} in the database. \
        Received network specs have same genesis hash and name {name_input}.",
        hex::encode(genesis_hash)
    )]
    AddSpecsDifferentName {
        genesis_hash: H256,
        name_database: String,
        name_input: String,
    },

    /// [`NetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) from the
    /// received `add_specs` payload already have an entry in `SPECSTREE` tree
    /// of the database.
    ///
    /// Not exactly an error, but Vault can't do anything and complains.
    #[error(
        "Exactly same network specs for network {name} with encryption {} \
        are already in the database.",
        encryption.show()
    )]
    SpecsKnown {
        /// network name
        name: String,

        /// network [`Encryption`]
        encryption: Encryption,
    },

    /// Received `add_specs` or `load_metadata` update payload is not verified.
    ///
    /// Network, however, was verified previously by verifier with certain
    /// [`VerifierValue`] and corresponding entry in `VERIFIERS` tree of the
    /// database is
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(verifier_value)))`.
    ///
    /// Vault does not allow downgrading the verifiers.
    #[error(
        "Saved network {name} information was signed by verifier {}. \
        Received information is not signed.",
        verifier_value.show_error(),
    )]
    NeedVerifier {
        /// network name
        name: String,

        /// expected verifier for this network
        verifier_value: VerifierValue,
    },

    /// Received `add_specs` update payload is signed by `new_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(old_verifier_value)))`,
    /// but `new_verifier_value` and `old_verifier_value` are different, and
    /// `new_verifier_value` is not the general verifier.
    ///
    /// Custom verifier could be upgraded only to general one, see
    /// [here](definitions::network_specs).
    #[error(
        "Network {name} current verifier is {}. Received add_specs message \
        is verified by {}, which is neither current network verifier not the \
        general verifier. Changing the network verifier to another non-general \
        one would require wipe and reset of Vault.",
        old_verifier_value.show_error(),
        new_verifier_value.show_error(),
    )]
    AddSpecsVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    /// Received update payload is not verified, although the verification by
    /// currently used general verifier with certain [`VerifierValue`] was
    /// expected.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`.
    #[error(
        "General verifier in the database is {}. Received unsigned {} \
        could be accepted only if signed by the general verifier.",
        verifier_value.show_error(),
        display_general_verifier_for_content(content),
    )]
    NeedGeneralVerifier {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// [`VerifierValue`] currently associated with the general verifier,
        /// expected verifier for the data
        verifier_value: VerifierValue,
    },

    /// Received `add_specs` or `load_types` is signed by
    /// `new_general_verifier_value`.
    // TODO: maybe combine with the LoadMetaGeneralVerifierChanged,
    // modify GeneralVerifierForContent into 3 variants
    #[error(
        "General verifier in the database is {}. Received {} could be \
        accepted only if verified by the same general verifier. \
        Current message is verified by {}.",
        .old_general_verifier_value.show_error(),
        display_general_verifier_for_content(.content),
        .new_general_verifier_value.show_error(),
    )]
    GeneralVerifierChanged {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    /// Types information received in the `load_types` payload is exactly
    /// same, as the one already stored in the `SETTREE` tree of the database
    /// under the key `TYPES`.
    ///
    /// Not exactly an error, but Vault can't do anything and complains.
    #[error("Exactly same types information is already in the database.")]
    TypesKnown,

    /// User attempted to load into Vault the metadata for the network that
    /// has no [`CurrentVerifier`](definitions::network_specs::CurrentVerifier) entry
    /// in the `VERIFIERS` tree of the Vault database.
    #[error(
        "Network {name} is not in the database. Add network specs before loading the metadata."
    )]
    LoadMetaUnknownNetwork {
        /// network name as it is in the received metadata
        name: String,
    },

    /// User attempted to load into Vault the metadata for the network that
    /// has no associated [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs)
    /// entries in the `SPECSTREE` tree of the Vault database, although it has
    /// an associated
    /// [`ValidCurrentVerifier`](definitions::network_specs::ValidCurrentVerifier),
    /// i.e. it was known to user at some point and never disabled.
    #[error(
        "Network {name} was previously known to the database with verifier {}. \
        However, no network specs are in the database at the moment. Add network \
        specs before loading the metadata.",
        show_verifier(valid_current_verifier, general_verifier)
    )]
    LoadMetaNoSpecs {
        /// network name as it is in the received metadata
        name: String,

        /// network-associated
        /// [`ValidCurrentVerifier`](definitions::network_specs::ValidCurrentVerifier)
        valid_current_verifier: ValidCurrentVerifier,

        /// Vault general verifier
        general_verifier: Verifier,
    },

    /// User attempted to load into Vault the metadata for the network that
    /// has a [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in the
    /// `SPECSTREE` tree of the Vault database, but specs have a different
    /// network name.
    ///
    /// Most likely, wrong genesis hash was attached to the metadata update.
    ///
    /// Since the network metadata in `METATREE` is identified by network name,
    /// and verifier is identified by the genesis hash, this should be checked
    /// on `load_metadata`.
    #[error(
        "Update payload contains metadata for network {name_metadata}. \
        Genesis hash in payload({}) matches database genesis hash for another \
        network, {name_specs}.",
        hex::encode(genesis_hash)
    )]
    LoadMetaWrongGenesisHash {
        /// network name as it is in the received metadata
        name_metadata: String,

        /// network name as it is in the network specs for genesis hash
        name_specs: String,

        /// genesis hash from the `load_metadata` payload, that was used to find
        /// the network specs and verifier information
        genesis_hash: H256,
    },

    /// Received `load_metadata` update payload is signed.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(None))`, i.e. it was
    /// never verified previously and its network specs were loaded unverified.
    ///
    /// Verified `add_specs` must be loaded before any verified `load_metadata`.
    #[error(
        "Network {name} currently has no verifier set up. Received load_metadata \
        message is verifier by {}. In order to accept verified metadata, first \
        download properly verified network specs.",
        new_verifier_value.show_error()
    )]
    LoadMetaSetVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the update payload
        new_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by `new_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(old_verifier_value)))`,
    /// but `new_verifier_value` and `old_verifier_value` are different, and
    /// `new_verifier_value` is not the general verifier.
    ///
    /// Custom verifier could be upgraded only to general one, see
    /// [here](definitions::network_specs), and during that network specs must be
    /// updated prior to loading the metadata.
    #[error(
        "Network {name} current verifier is {}. Received load_metadata message \
        is verified by {}. Changing verifier for the network would require wipe \
        and reset of Vault.",
        old_verifier_value.show_error(),
        new_verifier_value.show_error(),
    )]
    LoadMetaVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by
    /// `new_general_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`,
    /// and database value for general verifier is `None`, i.e. the network
    /// specs for this network are not verified.
    ///
    /// Verified `add_specs` must be loaded before any verified `load_metadata`.
    #[error(
        "Network {name} is set to be verified by the general verifier, however, \
        general verifier is not yet established. Received load_metadata message \
        is verified by {}. In order to accept verified metadata and set up the \
        general verifier, first download properly verified network specs.",
        new_general_verifier_value.show_error(),
    )]
    LoadMetaSetGeneralVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the payload instead of the
        /// known general verifier
        new_general_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by
    /// `new_general_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`,
    /// and database value for general verifier is
    /// `Some(old_general_verifier_value)`.
    ///
    /// General verifier with assigned [`VerifierValue`] could not be changed
    /// without Vault wipe. If the Vault is reset with no general verifier,
    /// and the network in question is the default one (currently Polkadot,
    /// Kusama, and Westend), the network will still be recorded as the one
    /// verified by the general verifier and accepting verified `add_specs` for
    /// it would result in setting the general verifier. If the network is not
    /// the default one and if by the time its `add_specs` are loaded the
    /// general verifier already has an associated `VerifierValue`, loading
    /// verified `add_specs` would result in the network having custom verifier.
    #[error(
        "Network {name} is verified by the general verifier which currently is {}. \
        Received load_metadata message is verified by {}. Changing the general \
        verifier or changing the network verifier to custom would require wipe \
        and reset of Vault.",
        old_general_verifier_value.show_error(),
        new_general_verifier_value.show_error(),
    )]
    LoadMetaGeneralVerifierChanged {
        /// network name
        name: String,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    /// Network name and version from metadata received in `load_metadata`
    /// message already have a corresponding entry in `METATREE` tree of the
    /// Vault database, with exactly same metadata.
    ///
    /// Not exactly an error, but Vault can't do anything and complains.
    #[error("Metadata for {name}{version} is already in the database.")]
    MetadataKnown {
        /// network name (identical for received and for stored metadata)
        name: String,

        /// network version (identical for received and for stored metadata)
        version: u32,
    },

    /// [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) for network in
    /// which the imported derivations are user to create addresses.
    #[error(
        "Unable to import derivations for network with genesis hash {} \
        and encryption {}. Network is unknown. Please add corresponding \
        network specs.",
        hex::encode(genesis_hash),
        encryption.show(),
    )]
    NetworkForDerivationsImport {
        /// network genesis hash
        genesis_hash: H256,

        /// network supported encryption
        encryption: Encryption,
    },

    #[error("Message payload must be wrapped with tags <Bytes></Bytes>")]
    InvalidMessagePayload,
}

fn display_parsing_errors(network_name: &str, errors: &[(u32, parser::Error)]) -> String {
    let mut insert = String::new();
    for (i, (version, parser_error)) in errors.iter().enumerate() {
        if i > 0 {
            insert.push(' ')
        }
        let _ = write!(
            &mut insert,
            "Parsing with {network_name}{version} metadata: {parser_error}",
        );
    }

    insert
}

fn display_general_verifier_for_content(g: &GeneralVerifierForContent) -> String {
    match g {
        GeneralVerifierForContent::Network { name } => format!("{name} network information"),
        GeneralVerifierForContent::Types => String::from("types information"),
    }
}

fn show_verifier(v: &ValidCurrentVerifier, general_verifier: &Verifier) -> String {
    match v {
        ValidCurrentVerifier::General => {
            format!("{} (general verifier)", general_verifier.show_error())
        }
        ValidCurrentVerifier::Custom { v } => format!("{} (custom verifier)", v.show_error()),
    }
}
