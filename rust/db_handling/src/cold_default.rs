//! Making and restoring **cold** database with default content
//!
//! Cold database is the database that is operated from inside the Signer.
//!
//! Cold database contains following trees:
//!
//! - `ADDRTREE` with public addresses data
//! - `HISTORY` with Signer history log
//! - `METATREE` with network metadata
//! - `SETTREE` with settings: types information, Signer dangerous exposures
//! record and Signer database general verifier
//! - `SPECSTREE` with network specs
//! - `TRANSACTION` for temporary storage of the transaction data
//! - `VERIFIERS` with network verifiers data
//!
//! For release, the cold database is generated on the hot side and then copied
//! verbatim into Signer files during the build.
//!
//! Before the database could be used, it must be initiated:
//!
//! - History log old entries (if any are present) are removed and a new entry
//! `Event::DatabaseInitiated` is added
//! - General verifier is set and this event is recorded inthe history log. By
//! default, Signer sets up Parity-associated key as a general verifier. This
//! could be later on changed by the user.
//!
//! Signer then reads and updates the database as it operates.
//!
//! There are two ways to reset the database from the Signer user interface.
//! Either would remove all the keys and restore the database to the release
//! state. The difference would be only in the general verifier setting:
//!
//! - `Wipe all data` would set the general verifier to default `Some(_)`,
//! with Parity-associated key inside
//! - `Remove general certificate` would set the general verifier to `None`.
//! User would then be able to set up own general verifier, preferably
//! immediately afterwards, by loading to Signer any verified data.
//! Setting up a new general verifier would remove all data associated with the
//! general verifier from the Signer database to avoid confusion as to who
//! verified what information.
#[cfg(any(feature = "active", feature = "signer"))]
use parity_scale_codec::Encode;
#[cfg(any(feature = "active", feature = "signer"))]
use sled::Batch;

#[cfg(feature = "active")]
use constants::{ADDRTREE, DANGER, METATREE, SETTREE, SPECSTREE, TRANSACTION, TYPES, VERIFIERS};
#[cfg(any(feature = "active", feature = "signer"))]
use constants::{GENERALVERIFIER, HISTORY};

#[cfg(feature = "signer")]
use definitions::error_signer::{ErrorSigner, Signer};
#[cfg(feature = "active")]
use definitions::{
    danger::DangerRecord,
    error_active::{Active, ErrorActive},
    keyring::{MetaKey, NetworkSpecsKey},
};
#[cfg(any(feature = "active", feature = "signer"))]
use definitions::{error::ErrorSource, history::Event, network_specs::Verifier};

#[cfg(feature = "signer")]
use defaults::default_general_verifier;
#[cfg(feature = "active")]
use defaults::{default_chainspecs, default_types_content, default_verifiers, release_metadata};
#[cfg(feature = "test")]
use defaults::{nav_test_metadata, test_metadata};

#[cfg(feature = "test")]
use crate::identities::generate_test_identities;
#[cfg(any(feature = "active", feature = "signer"))]
use crate::{
    db_transactions::TrDbCold, helpers::make_batch_clear_tree, manage_history::events_in_batch,
};

/// Cold database generation purpose, determining the metadata to be loaded.
///
/// Default metadata is loaded into the cold database for default networks:
/// Polkadot, Kusama, Westend. `Purpose` determines the metadata source folder
/// and the versions to be loaded.
#[cfg(any(feature = "active", feature = "test"))]
enum Purpose {
    /// Two (or fewer) latest released versions of the metadata for each of the
    /// default networks
    #[cfg(feature = "active")]
    Release,

    /// Old metadata set, used mostly in `transaction_parsing` tests
    #[cfg(feature = "test")]
    Test,

    /// Not so old metadata set, used for `navigator` tests
    // TODO combine all testing together
    #[cfg(feature = "test")]
    TestNavigator,
}

/// Make [`Batch`] with default networks metadata, for [`METATREE`] tree.
///
/// - Purge all existing entries
/// - Add default metadata entries, according to [`Purpose`]
#[cfg(any(feature = "active", feature = "test"))]
fn default_cold_metadata(database_name: &str, purpose: Purpose) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, METATREE)?;
    let metadata_set = match purpose {
        #[cfg(feature = "active")]
        Purpose::Release => release_metadata()?,

        #[cfg(feature = "test")]
        Purpose::Test => test_metadata()?,

        #[cfg(feature = "test")]
        Purpose::TestNavigator => nav_test_metadata()?,
    };
    for x in metadata_set.iter() {
        let meta_key = MetaKey::from_parts(&x.name, x.version);
        batch.insert(meta_key.key(), &x.meta[..]);
    }
    Ok(batch)
}

/// Make [`Batch`] with default networks
/// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs), for
/// [`SPECSTREE`] tree.
///
/// - Purge all existing entries
/// - Add default network specs entries
#[cfg(feature = "active")]
fn default_cold_network_specs(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SPECSTREE)?;
    for x in default_chainspecs().iter() {
        let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption);
        batch.insert(network_specs_key.key(), x.encode());
    }
    Ok(batch)
}

/// Make [`Batch`] with default settings, for [`SETTREE`] tree.
///
/// - Purge all existing entries
/// - Add default entries: types information
/// [`ContentLoadTypes`](definitions::qr_transfers::ContentLoadTypes) and danger
/// record [`DangerRecord`]
///
/// Note that the general verifier is **not** set up here.
///
/// General verifier is set up separately, during the database initiation
/// [`init_db`]. Without general verifier (i.e. a value in the [`SETTREE`] tree
/// under the key [`GENERALVERIFIER`]) the database is not usable by the Signer.
#[cfg(feature = "active")]
fn default_cold_settings_init_later(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, SETTREE)?;
    let types_prep = default_types_content()?;
    batch.insert(TYPES, types_prep.store());
    batch.insert(DANGER, DangerRecord::safe().store());
    Ok(batch)
}

/// Make [`Batch`] with default networks verifiers, for [`VERIFIERS`] tree.
///
/// - Purge all existing entries
/// - Add default
/// [`CurrentVerifier`](definitions::network_specs::CurrentVerifier) entries
#[cfg(feature = "active")]
fn default_cold_verifiers(database_name: &str) -> Result<Batch, ErrorActive> {
    let mut batch = make_batch_clear_tree::<Active>(database_name, VERIFIERS)?;
    for (verifier_key, current_verifier) in default_verifiers().iter() {
        batch.insert(verifier_key.key(), current_verifier.encode());
    }
    Ok(batch)
}

/// Make or restore the cold database with default content, according to
/// [`Purpose`].
///
/// Function clears all database trees and loads into database defaults for:
///
/// - metadata
/// - network specs
/// - types information and danger status
/// - network verifiers
///
/// Note that the resulting database is not initiated and is not ready to be
/// used by the Signer.
#[cfg(any(feature = "active", feature = "test"))]
fn cold_database_no_init(database_name: &str, purpose: Purpose) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_history(make_batch_clear_tree::<Active>(database_name, HISTORY)?) // clear history, database needs initialization before start
        .set_metadata(default_cold_metadata(database_name, purpose)?) // set default metadata
        .set_network_specs(default_cold_network_specs(database_name)?) // set default network specs
        .set_settings(default_cold_settings_init_later(database_name)?) // set default types and danger status, no general verifier yet
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transactions
        .set_verifiers(default_cold_verifiers(database_name)?) // set default verifiers
        .apply::<Active>(database_name)
}

/// Initiate cold database and set up the database general verifier to given
/// [`Verifier`].
///
/// Function simultaneously sets up the general verifier and marks the new start
/// of the history log.
///
/// Could be used both from the Signer side (with `Wipe all data` and with
/// `Remove general certificate` procedures) and from the active side, when
/// when preparing the test databases.
///
/// After applying this function the database becomes ready to be used by the
/// Signer.
#[cfg(any(feature = "active", feature = "signer"))]
pub fn init_db<T: ErrorSource>(
    database_name: &str,
    general_verifier: Verifier,
) -> Result<(), T::Error> {
    let mut settings_batch = Batch::default();
    settings_batch.insert(GENERALVERIFIER, general_verifier.encode());

    let clear_history_batch = make_batch_clear_tree::<T>(database_name, HISTORY)?;
    let events = vec![
        Event::DatabaseInitiated,
        Event::GeneralVerifierSet {
            verifier: general_verifier,
        },
    ];
    let start_zero = true;
    let history_batch =
        events_in_batch::<T>(database_name, start_zero, clear_history_batch, events)?;

    TrDbCold::new()
        .set_history(history_batch) // set *start* history
        .set_settings(settings_batch) // set general_verifier
        .apply::<T>(database_name)
}

/// Initiate Signer database with default general verifier (Parity-associated
/// key).
///
/// Function is applied during the initial start of the Signer and during
/// `Wipe all data` procedure.
#[cfg(feature = "signer")]
pub fn signer_init_with_cert(database_name: &str) -> Result<(), ErrorSigner> {
    init_db::<Signer>(database_name, default_general_verifier())
}

/// Initiate Signer database with general verifier set up to `Verifier(None)`.
///
/// Function is applied during `Remove general certificate` procedure.
#[cfg(feature = "signer")]
pub fn signer_init_no_cert(database_name: &str) -> Result<(), ErrorSigner> {
    init_db::<Signer>(database_name, Verifier { v: None })
}

/// Generate initiated test cold database with no network-associated data.
///
/// Function clears all database trees and loads into database only the defaults
/// for types information and danger status. Then the database is initiated with
/// given general verifier.
#[cfg(feature = "test")]
pub fn populate_cold_no_networks(
    database_name: &str,
    general_verifier: Verifier,
) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs(make_batch_clear_tree::<Active>(database_name, SPECSTREE)?) // clear network specs
        .set_settings(default_cold_settings_init_later(database_name)?) // set general verifier and load default types
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transactions
        .set_verifiers(make_batch_clear_tree::<Active>(database_name, VERIFIERS)?) // clear verifiers
        .apply::<Active>(database_name)?;
    init_db::<Active>(database_name, general_verifier)
}

/// Generate initiated test cold database without network metadata.
///
/// Function clears all database trees and loads into database defaults for:
///
/// - network specs
/// - types information and danger status
/// - network verifiers
///
/// Then the database is initiated with given general verifier.
#[cfg(feature = "test")]
pub fn populate_cold_no_metadata(
    database_name: &str,
    general_verifier: Verifier,
) -> Result<(), ErrorActive> {
    TrDbCold::new()
        .set_addresses(make_batch_clear_tree::<Active>(database_name, ADDRTREE)?) // clear addresses
        .set_metadata(make_batch_clear_tree::<Active>(database_name, METATREE)?) // clear metadata
        .set_network_specs(default_cold_network_specs(database_name)?) // set default network specs
        .set_settings(default_cold_settings_init_later(database_name)?) // set general verifier and load default types
        .set_transaction(make_batch_clear_tree::<Active>(database_name, TRANSACTION)?) // clear transactions
        .set_verifiers(default_cold_verifiers(database_name)?) // set default verifiers
        .apply::<Active>(database_name)?;
    init_db::<Active>(database_name, general_verifier)
}

/// Generate initiated test cold database with default content, and create in it
/// Alice default addresses.
#[cfg(feature = "test")]
pub fn populate_cold(database_name: &str, general_verifier: Verifier) -> Result<(), ErrorActive> {
    cold_database_no_init(database_name, Purpose::Test)?;
    init_db::<Active>(database_name, general_verifier)?;
    generate_test_identities(database_name)
}

/// Generate **not initiated** release cold database.
#[cfg(feature = "active")]
pub(crate) fn populate_cold_release(database_name: &str) -> Result<(), ErrorActive> {
    cold_database_no_init(database_name, Purpose::Release)
}

/// Generate **not initiated** test cold database for `navigator` testing.
#[cfg(feature = "test")]
pub fn populate_cold_nav_test(database_name: &str) -> Result<(), ErrorActive> {
    cold_database_no_init(database_name, Purpose::TestNavigator)
}
