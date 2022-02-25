use constants::{COLD_DB_NAME, COLD_DB_NAME_RELEASE, HOT_DB_NAME};
use definitions::{error::ErrorActive, network_specs::Verifier};

pub mod cold_default;
    use cold_default::{populate_cold, populate_cold_release};
pub mod db_transactions;
pub mod helpers;
mod hot_default;
    use hot_default::reset_hot_database;
pub mod identities;
pub mod interface_signer;
pub mod manage_history;
pub mod metadata;
pub mod network_details;
pub mod prep_messages;
pub mod remove_network;
pub mod remove_types;


/// Function to re-populate default "cold" database with default values.
/// This database should be copied into Signer's resources.
/// Note that this operation is performed NOT on Signer device,
/// so ErrorActive is used
pub fn default_cold_release () -> Result<(), ErrorActive> {
    let database_name = COLD_DB_NAME_RELEASE;
    populate_cold_release(database_name)
}

/// Function to re-populate default "cold" database with default values.
/// Currently this cold database is used for transaction_parsing crate
/// and needs Alice & Co identities
/// Note that this operation is performed NOT on Signer device,
/// so ErrorActive is used
pub fn default_cold () -> Result<(), ErrorActive> {
    let database_name = COLD_DB_NAME;
    populate_cold(database_name, Verifier(None))
}

/// Function to reset default "hot" database.
/// Active side operation, ErrorActive is used
pub fn default_hot () -> Result<(), ErrorActive> {
    let database_name = HOT_DB_NAME;
    reset_hot_database(database_name)
}

