use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRTREE, DANGER, GENERALVERIFIER, HISTORY, METATREE, SETTREE, SPECSTREE, TRANSACTION, TYPES, VERIFIERS};
use defaults::{default_general_verifier, get_default_chainspecs, get_default_types_content, get_release_metadata, get_nav_test_metadata, get_test_metadata, get_default_verifiers};
use definitions::{danger::DangerRecord, error::{Active, ErrorActive, ErrorSigner, ErrorSource, Signer}, history::Event, keyring::{MetaKey, NetworkSpecsKey}, network_specs::Verifier};

use crate::db_transactions::TrDbCold;
use crate::helpers::make_batch_clear_tree;
use crate::identities::generate_test_identities;
use crate::manage_history::events_in_batch;

enum Purpose {
    Release,
    Test,
    TestNavigator,
}

/// Function to set default *start* history in test cold database:
/// (1) purges all existing entries
/// (2) adds DatabaseInitiated entry
/// Function is used on the active side.
fn default_test_cold_history (database_name: &str) -> Result<Batch, ErrorActive> {
    let batch = make_batch_clear_tree::<Active>(database_name, HISTORY)?;
    let events = vec![Event::DatabaseInitiated];
    let start_zero = true;
    events_in_batch::<Active>(database_name, start_zero, batch, events)
}

/// Preparing default metadata batch in test cold database:
/// (1) purge all existing entries,
/// (2) add default entries with MetaKey in key form as a key,
/// and metadata as a value
fn default_cold_metadata (database_name: &str, purpose: Purpose) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, METATREE)?;
    let metadata_set = match purpose {
        Purpose::Release => get_release_metadata()?,
        Purpose::Test => get_test_metadata()?,
        Purpose::TestNavigator => get_nav_test_metadata()?,
    };
    for x in metadata_set.iter() {
        let meta_key = MetaKey::from_parts(&x.name, x.version);
        batch.insert(meta_key.key(), &x.meta[..]);
    }
    Ok(batch)
}

/// Preparing default network_specs batch:
/// (1) purge all existing entries,
/// (2) add default entries with NetworkSpecsKey in key form as a key,
/// and encoded NetworkSpecs as a value.
fn default_cold_network_specs (database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SPECSTREE)?;
    for x in get_default_chainspecs().iter() {
        let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Preparing default settings batch:
/// (1) purge all existing entries,
/// (2) add default entry with TYPES as a key,
/// and ContentLoadTypes (i.e. encoded Vec<TypeEntry>) as a value,
/// (3) add default entry with GENERALVERIFIER as a key,
/// and encoded general_verifier as a value
fn default_cold_settings (database_name: &str, general_verifier: Verifier) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SETTREE)?;
    let types_prep = get_default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    batch.insert(GENERALVERIFIER, general_verifier.encode());
    batch.insert(DANGER, DangerRecord::safe().store());
    Ok(batch)
}

/// Preparing default settings batch:
/// (1) purge all existing entries,
/// (2) add default entry with TYPES as a key,
/// and ContentLoadTypes (i.e. encoded Vec<TypeEntry>) as a value,
fn default_cold_settings_init_later (database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SETTREE)?;
    let types_prep = get_default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    batch.insert(DANGER, DangerRecord::safe().store());
    Ok(batch)
}

/// Preparing default verifiers batch:
/// (1) purge all existing entries,
/// (2) add default entries with VerifierKey in key form as a key,
/// and encoded CurrentVerifier as a value.
fn default_cold_verifiers (database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, VERIFIERS)?;
    for (verifier_key, current_verifier) in get_default_verifiers().iter() {
        batch.insert(verifier_key.key(), current_verifier.encode());
    }
    Ok(batch)
}

/// Function used to initiate the general verifier.
/// Function is applicable only to Signer side.
fn init_general_verifier (general_verifier: &Verifier) -> Batch {
    let mut batch = Batch::default();
    batch.insert(GENERALVERIFIER, general_verifier.encode());
    batch
}

/// Function used to initiate history for signer_init calls.
/// Function is applicable only to Signer side.
fn init_history<T: ErrorSource> (database_name: &str, general_verifier: &Verifier) -> Result<Batch, T::Error> {
    let batch = make_batch_clear_tree::<T>(database_name, HISTORY)?;
    let events = vec![Event::DatabaseInitiated, Event::GeneralVerifierSet(general_verifier.to_owned())];
    let start_zero = true;
    events_in_batch::<T>(database_name, start_zero, batch, events)
}

/// Function to reset cold database to defaults without addresses
fn reset_cold_database_no_addresses (database_name: &str, purpose: Purpose) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_history(make_batch_clear_tree::<Active>(database_name, HISTORY)?) // set *empty* history, database needs initialization before start
        .set_metadata(default_cold_metadata(database_name, purpose)?) // set default metadata
        .set_network_specs(default_cold_network_specs(database_name)?) // set default network_specs
        .set_settings(default_cold_settings_init_later(database_name)?) // set general verifier and load default types
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(default_cold_verifiers(database_name)?) // set default verifiers
        .apply::<Active>(database_name)
}

/// Function to initiate signer with given general verifier:
/// (1) clears history, adds "history initiated" and "general verifier set" entries
/// (2) sets general verifier
/// *does not* clear any other entries
/// *is not* a wipe
/// Function is applicable only to Signer side.
pub fn signer_init (database_name: &str, general_verifier: Verifier) -> Result<(), ErrorSigner> {
    TrDbCold::new()
        .set_history(init_history::<Signer>(database_name, &general_verifier)?) // set *start* history
        .set_settings(init_general_verifier(&general_verifier)) // set general_verifier
        .apply::<Signer>(database_name)
}

/// Function to initiate signer with default general verifier
/// Function is applicable only to Signer side, interacts with user interface.
pub fn signer_init_with_cert (database_name: &str) -> Result<(), ErrorSigner> {
    signer_init(database_name, default_general_verifier())
}

/// Function to initiate signer with Verifier::None as a general verifier
/// Function is applicable only to Signer side, interacts with user interface.
pub fn signer_init_no_cert (database_name: &str) -> Result<(), ErrorSigner> {
    signer_init(database_name, Verifier(None))
}

/// Function to populate cold database without adding any networks.
/// For tests. Note that history is initialized.
pub fn populate_cold_no_networks (database_name: &str, general_verifier: Verifier) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_history(default_test_cold_history(database_name)?) // set *start* history
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs(make_batch_clear_tree::<Active>(database_name, SPECSTREE)?) // clear network_specs
        .set_settings(default_cold_settings(database_name, general_verifier)?) // set general verifier as Verifier::None and load default types
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(make_batch_clear_tree::<Active>(database_name, VERIFIERS)?) // clear verifiers
        .apply::<Active>(database_name)
}

/// Function to populate cold database with settings and network_specs, but without adding metadata.
/// For tests. Note that history is initialized.
pub fn populate_cold_no_metadata (database_name: &str, general_verifier: Verifier) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_history(default_test_cold_history(database_name)?) // set *start* history
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs(default_cold_network_specs(database_name)?) // set default network_specs
        .set_settings(default_cold_settings(database_name, general_verifier)?) // set general verifier as Verifier::None and load default types
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(default_cold_verifiers(database_name)?) // set default verifiers
        .apply::<Active>(database_name)
}

/// Function to populate test cold database and set up the test identities.
/// For tests. History is initialized as in Signer, together with setting of the general verifier.
pub fn populate_cold (database_name: &str, general_verifier: Verifier) -> Result<(), ErrorActive> {
    reset_cold_database_no_addresses(database_name, Purpose::Test)?;
    TrDbCold::new()
        .set_history(init_history::<Active>(database_name, &general_verifier)?) // set *start* history
        .set_settings(init_general_verifier(&general_verifier))
        .apply::<Active>(database_name)?;
    generate_test_identities(database_name)
}

/// Function to populate release cold database.
/// No initialization of history is made. For creating database for Signer.
pub fn populate_cold_release (database_name: &str) -> Result<(), ErrorActive> {
    reset_cold_database_no_addresses(database_name, Purpose::Release)
}

/// Function to populate navigator test cold database.
/// No initialization of history is made. For Signer navigation tests.
pub fn populate_cold_nav_test (database_name: &str) -> Result<(), ErrorActive> {
    reset_cold_database_no_addresses(database_name, Purpose::TestNavigator)
}
