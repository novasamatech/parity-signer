use constants::{COLD_DB_NAME, COLD_DB_NAME_RELEASE, HOT_DB_NAME};
use definitions::network_specs::Verifier;
use anyhow;

pub mod chainspecs;
pub mod cold_default;
    use cold_default::{reset_cold_database_no_addresses, populate_cold};
pub mod db_transactions;
pub mod error;
pub mod helpers;
mod hot_default;
    use hot_default::reset_hot_database;
pub mod identities;
pub mod manage_history;
pub mod metadata;
pub mod network_details;
pub mod prep_messages;
pub mod remove_network;

/// Function to re-populate default "cold" database with default values.
/// This database should be copied into Signer's resources.
pub fn default_cold_release () -> anyhow::Result<()> {
    let database_name = COLD_DB_NAME_RELEASE;
    reset_cold_database_no_addresses(&database_name, Verifier(None))
}

/// Function to re-populate default "cold" database with default values.
/// Currently this cold database is used for transaction_parsing crate
/// and needs Alice & Co identities
pub fn default_cold () -> anyhow::Result<()> {
    let database_name = COLD_DB_NAME;
    populate_cold(&database_name, Verifier(None))
}

/// Function to reset default "hot" database.
pub fn default_hot () -> anyhow::Result<()> {
    let database_name = HOT_DB_NAME;
    reset_hot_database(&database_name)
}
