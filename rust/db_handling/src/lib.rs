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

#[cfg(feature = "active")]
use constants::{COLD_DB_NAME, COLD_DB_NAME_RELEASE, HOT_DB_NAME};
#[cfg(feature = "active")]
use definitions::{error_active::ErrorActive, network_specs::Verifier};
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

#[cfg(feature = "active")]
pub mod metadata;

pub mod network_details;

pub mod prep_messages;

#[cfg(feature = "signer")]
pub mod remove_network;

#[cfg(feature = "signer")]
pub mod remove_types;

#[cfg(feature = "test")]
pub mod tests;

#[cfg(feature = "active")]
use cold_default::{populate_cold, populate_cold_release};
#[cfg(feature = "active")]
use hot_default::reset_hot_database;

/// Generate "cold" database with default values, **for release build**.
///
/// Resulting database should be copied verbatim into Signer files during the
/// build.
///
/// The location of the generated database is either optional user-provided path,
/// or default `../database/database_cold_release` folder.
///
/// The cold release database contains: 
///
/// - network specs for default networks (Polkadot, Kusama, Westend)
/// - verifier information for default networks, with verifiers set to the
/// general one
/// - two latest metadata versions for default networks
/// - default types information
///
/// Also during the cold release database generation the trees `ADDRTREE`,
/// `HISTORY`, and `TRANSACTION` are cleared.
///
/// Note that resulting database history is not initialized and general
/// verifier is not set.
///
/// This operation is performed **not** on Signer device, and is governed by
/// the active side.
#[cfg(feature = "active")]
pub fn default_cold_release(path: Option<PathBuf>) -> Result<(), ErrorActive> {
    let database_name = match path {
        Some(ref path) => path.to_str().unwrap_or(COLD_DB_NAME_RELEASE),
        None => COLD_DB_NAME_RELEASE,
    };
    populate_cold_release(database_name)
}

/// Generate "cold" database with default values and Alice identities, **for
/// tests**.
///
/// This was previously used for tests in Signer, may go obsolete.
///
/// The location of the generated database is `../database/database_cold` folder.
///
/// The test cold database contains: 
///
/// - network specs for default networks (Polkadot, Kusama, Westend)
/// - verifier information for default networks, with verifiers set to the
/// general one
/// - test metadata entries for default networks (old ones)
/// - default types information
/// - addressed for Alice in test networks
/// - initiated history
/// - general verifier value set to `None`
///
/// `TRANSACTION` tree is cleared.
///
/// This operation is performed **not** on Signer device, and is governed by
/// the active side.
#[cfg(feature = "active")]
pub fn default_cold() -> Result<(), ErrorActive> {
    let database_name = COLD_DB_NAME;
    populate_cold(database_name, Verifier(None))
}

/// Function to reset default "hot" database.
/// Active side operation, ErrorActive is used
#[cfg(feature = "active")]
pub fn default_hot() -> Result<(), ErrorActive> {
    let database_name = HOT_DB_NAME;
    reset_hot_database(database_name)
}
