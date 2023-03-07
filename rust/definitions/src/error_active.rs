//! Errors occurring on the active side, i.e. while operating `generate_message`
//! client
//!
//! Active side deals with both preparation of cold database that would be
//! loaded in Vault on build and with hot database operations. Cold database
//! could be the *release* cold database (the actual one for Vault build) or
//! the *test* cold database (with test Alice identities, used for tests).

use crate::error::MetadataError;

/// Errors with `wasm` files processing
// TODO add links to external errors and definitions, when the `sc-...` crates
// are published
#[derive(Debug, thiserror::Error)]
pub enum Wasm {
    /// Failed to make `Metadata_metadata` call on data extracted from `wasm`
    /// file.
    #[cfg(feature = "active")]
    #[error(transparent)]
    Executor(#[from] sc_executor_common::error::Error),

    /// Metadata extracted from `wasm` file could not be decoded.
    #[error("metadata from file could not be decoded")]
    DecodingMetadata,

    /// Metadata extracted from `wasm` file is not suitable to be used in
    /// Vault.
    ///
    /// Associated data is [`MetadataError`] specifying what exactly is wrong
    /// with the metadata.
    #[error(transparent)]
    FaultyMetadata(#[from] MetadataError),

    /// Error reading `wasm` file.
    #[error(transparent)]
    File(#[from] std::io::Error),

    #[cfg(feature = "active")]
    #[error(transparent)]
    WasmError(#[from] sc_executor_common::error::WasmError),
}

/// Error checking metadata file
#[derive(Debug)]
pub enum Check {
    /// Metadata extracted from the metadata file is not suitable to be used in
    /// Vault.
    ///
    /// Associated data is [`MetadataError`] specifying what exactly is wrong
    /// with the metadata.
    FaultyMetadata(MetadataError),

    /// Unable to read directory with default metadata
    MetadataFile(std::io::Error),
}
