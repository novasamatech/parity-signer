use std::fmt::Write;

use definitions::{
    crypto::Encryption,
    error::MetadataError,
    error_signer::GeneralVerifierForContent,
    keyring::NetworkSpecsKey,
    network_specs::{ValidCurrentVerifier, Verifier, VerifierValue},
};
use sp_core::H256;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Definitions(#[from] definitions::error::Error),

    #[error("Input is too short.")]
    TooShort,

    #[error("Only Substrate transaction are supported. Transaction is expected to start with 0x53, this one starts with 0x{0}")]
    NotSubstrate(String),

    #[error("bad signature")]
    BadSignature,

    #[error("Payload type with code 0x{0} is not supported.")]
    PayloadNotSupported(String),

    #[error("Database error. Internal error. {0}")]
    DbError(#[from] db_handling::Error),

    #[error("Historical transaction was generated in network {name} and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata.")]
    HistoricalMetadata { name: String },

    #[error("specs collision: {name}")]
    SpecsCollision {
        name: String,
        encryption: Encryption,
    },

    #[error(
        "Could not find network specs for {name} with encryption {} needed to decode historical transaction.",
        .encryption.show()
    )]
    HistoryNetworkSpecs {
        name: String,
        encryption: Encryption,
    },

    #[error(transparent)]
    Sled(#[from] sled::Error),

    #[error(
        "Similar network specs are already stored in the database under key {}. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit).",
        hex::encode(.0.key())
    )]
    ImportantSpecsChanged(NetworkSpecsKey),

    #[error("Metadata for {name}{version} is already in the database and is different from the one in received payload.")]
    SameNameVersionDifferentMeta { name: String, version: u32 },

    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error("encryption not supported {0}")]
    EncryptionNotSupported(String),

    #[error(
        "Failed to decode extensions. Please try updating metadata for {network_name} network. {}",
        display_parsing_errors(.network_name, .errors)
    )]
    AllExtensionsParsingFailed {
        network_name: String,
        errors: Vec<(u32, parser::Error)>,
    },

    #[error(transparent)]
    Parser(#[from] parser::Error),

    #[error("Unable to separate transaction method and extensions.")]
    SeparateMethodExtensions,

    #[error("Input transaction is generated in network {name}. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata.")]
    NoMetadata { name: String },

    #[error(
        "Input generated within unknown network and could not be processed. Add network with genesis hash {} and encryption {}.",
        hex::encode(.genesis_hash),
        .encryption.show()
    )]
    UnknownNetwork {
        genesis_hash: H256,

        encryption: Encryption,
    },

    #[error("Received message could not be read.")]
    Codec(#[from] parity_scale_codec::Error),

    #[error(
        "Network {name} with genesis hash {} already has entries in the database with base58 prefix {base58_database}. Received network specs have same genesis hash and different base58 prefix {base58_input}.",
        hex::encode(.genesis_hash),
    )]
    AddSpecsDifferentBase58 {
        genesis_hash: H256,
        name: String,
        base58_database: u16,
        base58_input: u16,
    },

    #[error("Network with genesis hash {} has name {name_database} in the database. Received network specs have same genesis hash and name {name_input}.", hex::encode(.genesis_hash))]
    AddSpecsDifferentName {
        genesis_hash: H256,
        name_database: String,
        name_input: String,
    },

    #[error(
        "Exactly same network specs for network {name} with encryption {} are already in the database.",
        .encryption.show()
    )]
    SpecsKnown {
        /// network name
        name: String,

        /// network [`Encryption`]
        encryption: Encryption,
    },

    #[error(
        "Saved network {name} information was signed by verifier {}. Received information is not signed.",
        .verifier_value.show_error(),
    )]
    NeedVerifier {
        /// network name
        name: String,

        /// expected verifier for this network
        verifier_value: VerifierValue,
    },

    #[error("Network {name} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.",
        .old_verifier_value.show_error(),
        .new_verifier_value.show_error(),
    )]
    AddSpecsVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    #[error(
        "General verifier in the database is {}. Received unsigned {} could be accepted only if signed by the general verifier.",
        .verifier_value.show_error(),
        display_general_verifier_for_content(.content),
    )]
    NeedGeneralVerifier {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// [`VerifierValue`] currently associated with the general verifier,
        /// expected verifier for the data
        verifier_value: VerifierValue,
    },

    #[error(
        "General verifier in the database is {}. Received {} could be accepted only if verified by the same general verifier. Current message is verified by {}.",
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

    #[error("Exactly same types information is already in the database.")]
    TypesKnown,

    #[error(
        "Network {name} is not in the database. Add network specs before loading the metadata."
    )]
    LoadMetaUnknownNetwork {
        /// network name as it is in the received metadata
        name: String,
    },
    #[error(
        "Network {name} was previously known to the database with verifier {}. However, no network specs are in the database at the moment. Add network specs before loading the metadata.",
        show_verifier(.valid_current_verifier, .general_verifier),
    )]
    LoadMetaNoSpecs {
        /// network name as it is in the received metadata
        name: String,

        /// network-associated
        /// [`ValidCurrentVerifier`](crate::network_specs::ValidCurrentVerifier)
        valid_current_verifier: ValidCurrentVerifier,

        /// Signer general verifier
        general_verifier: Verifier,
    },

    #[error(
        "Update payload contains metadata for network {name_metadata}. Genesis hash in payload({}) matches database genesis hash for another network, {name_specs}.",
        hex::encode(.genesis_hash)
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

    #[error(
        "Network {name} currently has no verifier set up. Received load_metadata message is verifier by {}. In order to accept verified metadata, first download properly verified network specs.",
        .new_verifier_value.show_error()
    )]
    LoadMetaSetVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the update payload
        new_verifier_value: VerifierValue,
    },

    #[error(
        "Network {name} current verifier is {}. Received load_metadata message is verified by {}. Changing verifier for the network would require wipe and reset of Signer.",
        .old_verifier_value.show_error(),
        .new_verifier_value.show_error(),
    )]
    LoadMetaVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    #[error(
        "Network {name} is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by {}. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.",
        .new_general_verifier_value.show_error(),
    )]
    LoadMetaSetGeneralVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the payload instead of the
        /// known general verifier
        new_general_verifier_value: VerifierValue,
    },

    #[error(
        "Network {name} is verified by the general verifier which currently is {}. Received load_metadata message is verified by {}. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.",
        .old_general_verifier_value.show_error(),
        .new_general_verifier_value.show_error(),
    )]
    LoadMetaGeneralVerifierChanged {
        /// network name
        name: String,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    #[error("Metadata for {name}{version} is already in the database.")]
    MetadataKnown {
        /// network name (identical for received and for stored metadata)
        name: String,

        /// network version (identical for received and for stored metadata)
        version: u32,
    },

    #[error(
        "Unable to import derivations for network with genesis hash {} and encryption {}. Network is unknown. Please add corresponding network specs.",
        hex::encode(.genesis_hash),
        .encryption.show(),
    )]
    NetworkForDerivationsImport {
        /// network genesis hash
        genesis_hash: H256,

        /// network supported encryption
        encryption: Encryption,
    },
}

fn display_parsing_errors(network_name: &str, errors: &[(u32, parser::Error)]) -> String {
    let mut insert = String::new();
    for (i, (version, parser_error)) in errors.iter().enumerate() {
        if i > 0 {
            insert.push(' ')
        }
        let _ = write!(
            &mut insert,
            "Parsing with {}{} metadata: {}",
            network_name, version, parser_error,
        );
    }

    insert
}

fn display_general_verifier_for_content(g: &GeneralVerifierForContent) -> String {
    match g {
        GeneralVerifierForContent::Network { name } => format!("{} network information", name),
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
