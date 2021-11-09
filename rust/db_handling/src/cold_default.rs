use sled::Batch;
use parity_scale_codec::Encode;
use constants::{ADDRTREE, DANGER, GENERALVERIFIER, HISTORY, METATREE, SETTREE, SPECSTREE, TRANSACTION, TYPES, VERIFIERS};
use defaults::{default_general_verifier, get_default_chainspecs, get_default_types, get_default_metadata, get_default_verifiers};
use definitions::{danger::DangerRecord, history::Event, keyring::{MetaKey, NetworkSpecsKey}, network_specs::Verifier};
use anyhow;

use crate::db_transactions::TrDbCold;
use crate::error::Error;
use crate::helpers::make_batch_clear_tree;
use crate::identities::generate_test_identities;
use crate::manage_history::events_in_batch;


/// Function to set default *start* history:
/// (1) purge all existing entries
/// (2) add DatabaseInitiated entry
fn default_cold_history (database_name: &str) -> anyhow::Result<Batch> {
    let batch = make_batch_clear_tree(&database_name, HISTORY)?;
    let events = vec![Event::DatabaseInitiated];
    let start_zero = true;
    events_in_batch(&database_name, start_zero, batch, events)
}


/// Preparing default metadata batch:
/// (1) purge all existing entries,
/// (2) add default entries with MetaKey in key form as a key,
/// and metadata as a value
fn default_cold_metadata (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(&database_name, METATREE)?;
    match get_default_metadata() {
        Ok(a) => {
            for x in a.iter() {
                let meta_key = MetaKey::from_parts(&x.name, x.version);
                batch.insert(meta_key.key(), x.meta.to_vec());
            }
            Ok(batch)
        },
        Err(e) => return Err(Error::BadDefaultMetadata(e).show()),
    }
}

/// Preparing default network_specs batch:
/// (1) purge all existing entries,
/// (2) add default entries with NetworkSpecsKey in key form as a key,
/// and encoded ChainSpecs as a value.
fn default_cold_network_specs (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(&database_name, SPECSTREE)?;
    for x in get_default_chainspecs().iter() {
        let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption);
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
fn default_cold_settings (database_name: &str, general_verifier: Verifier) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(&database_name, SETTREE)?;
    let types_prep = match get_default_types() {
        Ok(x) => x,
        Err(e) => return Err(Error::BadTypesFile(e).show()),
    };
    batch.insert(TYPES.to_vec(), types_prep.store());
    batch.insert(GENERALVERIFIER.to_vec(), general_verifier.encode());
    batch.insert(DANGER.to_vec(), DangerRecord::safe().store());
    Ok(batch)
}

/// Preparing default verifiers batch:
/// (1) purge all existing entries,
/// (2) add default entries with VerifierKey in key form as a key,
/// and encoded CurrentVerifier as a value.
fn default_cold_verifiers (database_name: &str) -> anyhow::Result<Batch> {
    let mut batch = make_batch_clear_tree(&database_name, VERIFIERS)?;
    for (verifier_key, current_verifier) in get_default_verifiers().iter() {
        batch.insert(verifier_key.key(), current_verifier.encode());
    }
    Ok(batch)
}

/// Function used to initiate the general verifier
fn signer_init_general_verifier (general_verifier: &Verifier) -> Batch {
    let mut batch = Batch::default();
    batch.insert(GENERALVERIFIER.to_vec(), general_verifier.encode());
    batch
}

/// Function used to initiate history for signer_init calls
fn signer_init_history (database_name: &str, general_verifier: &Verifier) -> anyhow::Result<Batch> {
    let batch = make_batch_clear_tree(&database_name, HISTORY)?;
    let events = vec![Event::DatabaseInitiated, Event::GeneralVerifierSet(general_verifier.to_owned())];
    let start_zero = true;
    events_in_batch(&database_name, start_zero, batch, events)
}

/// Function to reset cold database to defaults without addresses
pub fn reset_cold_database_no_addresses (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree(&database_name, ADDRTREE)?) // clear addresses
        .set_history(default_cold_history(&database_name)?) // set *start* history
        .set_metadata(default_cold_metadata(&database_name)?) // set default metadata
        .set_network_specs(default_cold_network_specs(&database_name)?) // set default network_specs
        .set_settings(default_cold_settings(&database_name, general_verifier)?) // set general verifier and load default types
        .set_transaction(make_batch_clear_tree(&database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(default_cold_verifiers(&database_name)?) // set default verifiers
        .apply(&database_name)
}

/// Function to initiate signer with given general verifier:
/// (1) clears history, adds "history initiated" and "general verifier set" entries
/// (2) sets general verifier
/// *does not* clear any other entries
/// *is not* a wipe
fn signer_init (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    TrDbCold::new()
        .set_history(signer_init_history(&database_name, &general_verifier)?) // set *start* history
        .set_settings(signer_init_general_verifier(&general_verifier)) // set general_verifier
        .apply(&database_name)
}

/// Function to initiate signer with default general verifier
pub fn signer_init_with_cert (database_name: &str) -> anyhow::Result<()> {
    signer_init(&database_name, default_general_verifier())
}

/// Function to initiate signer with Verifier::None as a general verifier
pub fn signer_init_no_cert (database_name: &str) -> anyhow::Result<()> {
    signer_init(&database_name, Verifier(None))
}

/// Function to populate cold database without adding any networks.
/// For tests.
pub fn populate_cold_no_networks (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree(&database_name, ADDRTREE)?) // clear addresses
        .set_history(default_cold_history(&database_name)?) // set *start* history
        .set_metadata(make_batch_clear_tree(&database_name, METATREE)?) // clear metadata
        .set_network_specs(make_batch_clear_tree(&database_name, SPECSTREE)?) // clear network_specs
        .set_settings(default_cold_settings(&database_name, general_verifier)?) // set general verifier as Verifier::None and load default types
        .set_transaction(make_batch_clear_tree(&database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(make_batch_clear_tree(&database_name, VERIFIERS)?) // clear verifiers
        .apply(&database_name)
}

/// Function to populate cold database with settings and network_specs, but without adding metadata.
/// For tests.
pub fn populate_cold_no_metadata (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree(&database_name, ADDRTREE)?) // clear addresses
        .set_history(default_cold_history(&database_name)?) // set *start* history
        .set_metadata(make_batch_clear_tree(&database_name, METATREE)?) // clear metadata
        .set_network_specs(default_cold_network_specs(&database_name)?) // set default network_specs
        .set_settings(default_cold_settings(&database_name, general_verifier)?) // set general verifier as Verifier::None and load default types
        .set_transaction(make_batch_clear_tree(&database_name, TRANSACTION)?) // clear transaction
        .set_verifiers(default_cold_verifiers(&database_name)?) // set default verifiers
        .apply(&database_name)
}

/// Function to populate cold database and set up the test identities.
/// For tests.
pub fn populate_cold (database_name: &str, general_verifier: Verifier) -> anyhow::Result<()> {
    reset_cold_database_no_addresses(&database_name, general_verifier)?;
    generate_test_identities(&database_name)
}
