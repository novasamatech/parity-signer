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

    #[error("too short")]
    TooShort,

    #[error("not substrate: {0}")]
    NotSubstrate(String),

    #[error("bad signature")]
    BadSignature,

    #[error("payload not supported {0}")]
    PayloadNotSupported(String),

    #[error("Database error. Internal error. {0}")]
    DbError(#[from] db_handling::Error),

    #[error("historical metadata {name}")]
    HistoricalMetadata { name: String },

    #[error("specs collision: {name}")]
    SpecsCollision {
        name: String,
        encryption: Encryption,
    },

    #[error("history network specs {name}")]
    HistoryNetworkSpecs {
        name: String,
        encryption: Encryption,
    },

    #[error(transparent)]
    Sled(#[from] sled::Error),

    #[error("important specs changed")]
    ImportantSpecsChanged(NetworkSpecsKey),

    #[error("name version different meta {name} {version}")]
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

    #[error("separate method extensions")]
    SeparateMethodExtensions,

    #[error("no metadata {name}")]
    NoMetadata { name: String },

    #[error("unknown network")]
    UnknownNetwork {
        genesis_hash: H256,

        encryption: Encryption,
    },

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),

    #[error("different base58")]
    AddSpecsDifferentBase58 {
        genesis_hash: H256,
        name: String,
        base58_database: u16,
        base58_input: u16,
    },

    #[error("add specs different name")]
    AddSpecsDifferentName {
        genesis_hash: H256,
        name_database: String,
        name_input: String,
    },

    #[error("specs known")]
    SpecsKnown {
        /// network name
        name: String,

        /// network [`Encryption`]
        encryption: Encryption,
    },

    #[error("need verifier")]
    NeedVerifier {
        /// network name
        name: String,

        /// expected verifier for this network
        verifier_value: VerifierValue,
    },

    #[error("add specs verifier changed")]
    AddSpecsVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    #[error("need general verifier")]
    NeedGeneralVerifier {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// [`VerifierValue`] currently associated with the general verifier,
        /// expected verifier for the data
        verifier_value: VerifierValue,
    },

    #[error("general verifier changed")]
    GeneralVerifierChanged {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    #[error("types known")]
    TypesKnown,

    #[error("load meta unknown network")]
    LoadMetaUnknownNetwork {
        /// network name as it is in the received metadata
        name: String,
    },
    #[error("load meta no specs")]
    LoadMetaNoSpecs {
        /// network name as it is in the received metadata
        name: String,

        /// network-associated
        /// [`ValidCurrentVerifier`](crate::network_specs::ValidCurrentVerifier)
        valid_current_verifier: ValidCurrentVerifier,

        /// Signer general verifier
        general_verifier: Verifier,
    },

    #[error("load meta wrong genesis hash")]
    LoadMetaWrongGenesisHash {
        /// network name as it is in the received metadata
        name_metadata: String,

        /// network name as it is in the network specs for genesis hash
        name_specs: String,

        /// genesis hash from the `load_metadata` payload, that was used to find
        /// the network specs and verifier information
        genesis_hash: H256,
    },

    #[error("load meta set verifier")]
    LoadMetaSetVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the update payload
        new_verifier_value: VerifierValue,
    },

    #[error("load meta verifier changed")]
    LoadMetaVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    #[error("load meta set general verifier")]
    LoadMetaSetGeneralVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the payload instead of the
        /// known general verifier
        new_general_verifier_value: VerifierValue,
    },

    #[error("load meta general verifier changed")]
    LoadMetaGeneralVerifierChanged {
        /// network name
        name: String,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    #[error("metadata known")]
    MetadataKnown {
        /// network name (identical for received and for stored metadata)
        name: String,

        /// network version (identical for received and for stored metadata)
        version: u32,
    },

    #[error("network for derivations import")]
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
