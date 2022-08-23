//! This crate deals with standard database-related procedures used in
//! [Signer](https://github.com/paritytech/parity-signer) and Signer-supporting
//! ecosystem.  
//!
//! This crate:
//!
//! - contains helpers to operate the databases, used throughout Signer system
//! - generates cold (used in air-gapped Signer) and hot (used in
//! `generate_message` client) databases with default settings
//! - deals with taking data out of the database and putting the data in the
//! database
//! - contains Signer interface interactions, allowing exports of data to the
//! interface
//! - deals with address generation for Signer
//!
//! # Features
//!
//! Feature `"signer"` corresponds to everything happening in Signer air-gapped
//! device.
//!
//! Feature `"active"` corresponds to all Signer-related things happening
//! **without** air-gap, including the generation of the database for Signer
//! during the build.
//!
//! Feature `"test"` includes both `"signer"` and `"active"` features, along
//! with some testing, and is the default one.  

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

// possibly TODO: rename all database_name into database_path or whatever,
// currently it is quite confusing

#[cfg(feature = "active")]
use constants::{COLD_DB_NAME_RELEASE, HOT_DB_NAME};
#[cfg(feature = "active")]
use std::path::PathBuf;

pub mod cold_default;

pub mod db_transactions;

pub mod helpers;

#[cfg(feature = "active")]
mod hot_default;

pub mod identities;

#[cfg(feature = "signer")]
pub mod interface_signer;

pub mod manage_history;

mod error;

pub use error::{Error, Result};

#[cfg(feature = "test")]
#[cfg(test)]
pub mod tests;

#[cfg(feature = "active")]
use cold_default::populate_cold_release;
#[cfg(feature = "active")]
use hot_default::reset_hot_database;

/// Generate or restore "cold" database with default values, **for release
/// build**.
///
/// Resulting database should be copied verbatim into Signer files during the
/// build.
///
/// The location of the generated database is either optional user-provided
/// path, or default [`COLD_DB_NAME_RELEASE`] folder.
///
/// The cold release database, as generated, contains:
///
/// - network specs for default networks (Polkadot, Kusama, Westend)
/// - verifier information for default networks, with verifiers set to the
/// general one
/// - two latest metadata versions for default networks
/// - default types information and clean danger status
///
/// The trees `ADDRTREE`, `HISTORY`, and `TRANSACTION` are empty.
///
/// Note that resulting database history is not initialized and general
/// verifier is not set.
///
/// This operation is performed **not** on Signer device, and is governed by
/// the active side.
#[cfg(feature = "active")]
pub fn default_cold_release(path: Option<PathBuf>) -> Result<()> {
    populate_cold_release(path.unwrap_or_else(|| COLD_DB_NAME_RELEASE.into()))
}

/// Generate or restore "hot" database with default values.
///
/// The location of the generated database is default [`HOT_DB_NAME`] folder.
///
/// The hot database, as generated, contains:
///
/// - address book entries for default networks (Polkadot, Kusama, Westend)
/// - network specs for default networks
/// - default types information
/// - **no** metadata entries
/// - **no** metadata block history entries
///
/// All metadata-related entries get in the hot database only through RPC calls.
#[cfg(feature = "active")]
pub fn default_hot(path: Option<PathBuf>) -> Result<()> {
    reset_hot_database(path.unwrap_or_else(|| HOT_DB_NAME.into()))
}
