//! This crate deals with standard database-related procedures used in
//! [Vault](https://github.com/paritytech/parity-signer) and Vault-supporting
//! ecosystem.  
//!
//! This crate:
//!
//! - contains helpers to operate the databases, used throughout Vault system
//! - generates cold (used in air-gapped Vault) and hot (used in
//!   `generate_message` client) databases with default settings
//! - deals with taking data out of the database and putting the data in the
//!   database
//! - contains Vault interface interactions, allowing exports of data to the
//!   interface
//! - deals with address generation for Vault
//!
//! # Features
//!
//! Feature `"active"` corresponds to all Vault-related things happening
//! **without** air-gap, including the generation of the database for Vault
//! during the build.

#![deny(rustdoc::broken_intra_doc_links)]

// possibly TODO: rename all database_name into database_path or whatever,
// currently it is quite confusing

#[cfg(feature = "active")]
use constants::{COLD_DB_NAME_RELEASE, HOT_DB_NAME};

pub mod cold_default;

pub mod db_transactions;

pub mod helpers;

#[cfg(feature = "active")]
mod hot_default;

pub mod identities;

pub mod interface_signer;

pub mod manage_history;

mod error;

pub use error::{Error, Result};

#[cfg(test)]
pub mod tests;

#[cfg(feature = "active")]
use cold_default::populate_cold_release;
#[cfg(feature = "active")]
use hot_default::reset_hot_database;

/// Generate or restore "cold" database with default values, **for release
/// build**.
///
/// Resulting database should be copied verbatim into Vault files during the
/// build.
///
/// The location of the generated database is either optional user-provided
/// path, or default [`COLD_DB_NAME_RELEASE`] folder.
///
/// The cold release database, as generated, contains:
///
/// - network specs for default networks (Polkadot, Kusama, Westend)
/// - verifier information for default networks, with verifiers set to the
///   general one
/// - two latest metadata versions for default networks
/// - default types information and clean danger status
///
/// The trees `ADDRTREE`, `HISTORY`, and `TRANSACTION` are empty.
///
/// Note that resulting database history is not initialized and general
/// verifier is not set.
///
/// This operation is performed **not** on Vault device, and is governed by
/// the active side.
#[cfg(feature = "active")]
pub fn default_cold_release(database: Option<&sled::Db>) -> Result<()> {
    let database = if let Some(database) = database {
        database.clone()
    } else {
        sled::open(COLD_DB_NAME_RELEASE)?
    };
    populate_cold_release(&database)
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
pub fn default_hot(database: Option<&sled::Db>) -> Result<()> {
    let database = if let Some(database) = database {
        database.clone()
    } else {
        sled::open(HOT_DB_NAME)?
    };

    reset_hot_database(&database)
}
