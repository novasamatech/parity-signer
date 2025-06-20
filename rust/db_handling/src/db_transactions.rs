//! Atomic transactions in cold and hot databases
//!
//! Additions and removals of entries in cold and hot database occur through
//! atomic [transactions](sled::transaction).
//! Each tree gets updated with its own [`Batch`], updates occur within a single
//! transaction.
//!
//! For transactions scanned into Vault, currently a temporary database entry
//! is made to store transaction details while they are displayed to user.

// TODO this is a temporary solution, the data eventually could be stored in
// `navigator` state.
use parity_scale_codec::{Decode, Encode};
use sled::{transaction::TransactionResult, Batch, Transactional};
use sp_runtime::MultiSigner;

#[cfg(feature = "active")]
use constants::{ADDRESS_BOOK, META_HISTORY, SPECSTREEPREP};
use constants::{ADDRTREE, HISTORY, METATREE, SETTREE, SPECSTREE, TRANSACTION, VERIFIERS};
use constants::{GENERALVERIFIER, SIGN, STUB, TYPES};

use definitions::{
    history::{
        Event, MetaValuesDisplay, NetworkSpecsDisplay, NetworkVerifierDisplay, SignDisplay,
        SignMessageDisplay, TypesDisplay,
    },
    keyring::{MetaKey, NetworkSpecsKey, VerifierKey},
    metadata::MetaValues,
    network_specs::{
        CurrentVerifier, NetworkSpecs, OrderedNetworkSpecs, ValidCurrentVerifier, Verifier,
        VerifierValue,
    },
    qr_transfers::ContentLoadTypes,
};

use crate::helpers::open_tree;
use crate::Error;
use crate::Result;
use crate::{
    helpers::{make_batch_clear_tree, verify_checksum},
    manage_history::events_to_batch,
};

/// Cold database transaction data containing [`Batch`] elements that will be
/// applied to each [`Tree`](sled::Tree).
///
/// Cold database tree names and content information could be found in
/// [`constants`] crate. All trees are routinely updated as Vault is used.
///
/// [`TrDbCold`] is applied to the cold database in an atomic transaction.
///
/// [`TrDbCold`] is used both by the Vault side (for all database-related
/// actions) and the active side (to generate and populate the cold database).
///
/// Note that all the checking is done as the [`TrDbCold`] is generated,
/// `apply` method does not do any checks on its own.
#[derive(Debug)]
pub struct TrDbCold {
    /// `Batch` to be applied to [`ADDRTREE`] tree
    for_addresses: Batch,

    /// `Batch` to be applied to [`HISTORY`] tree
    for_history: Batch,

    /// `Batch` to be applied to [`METATREE`] tree
    for_metadata: Batch,

    /// `Batch` to be applied to [`SPECSTREE`] tree
    for_network_specs: Batch,

    /// `Batch` to be applied to [`SETTREE`] tree
    for_settings: Batch,

    /// `Batch` to be applied to [`TRANSACTION`] tree
    for_transaction: Batch,

    /// `Batch` to be applied to [`VERIFIERS`] tree
    for_verifiers: Batch,
}

impl TrDbCold {
    /// Construct new empty [`TrDbCold`].
    pub fn new() -> Self {
        Self {
            for_addresses: Batch::default(),
            for_history: Batch::default(),
            for_metadata: Batch::default(),
            for_network_specs: Batch::default(),
            for_settings: Batch::default(),
            for_transaction: Batch::default(),
            for_verifiers: Batch::default(),
        }
    }

    /// Set `for_addresses` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`ADDRTREE`] tree.
    pub fn set_addresses(mut self, for_addresses: Batch) -> Self {
        self.for_addresses = for_addresses;
        self
    }

    /// Set `for_history` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`HISTORY`] tree.
    pub fn set_history(mut self, for_history: Batch) -> Self {
        self.for_history = for_history;
        self
    }

    /// Set `for_metadata` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`METATREE`] tree.
    pub fn set_metadata(mut self, for_metadata: Batch) -> Self {
        self.for_metadata = for_metadata;
        self
    }

    /// Set `for_network_specs` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`SPECSTREE`] tree.
    pub fn set_network_specs(mut self, for_network_specs: Batch) -> Self {
        self.for_network_specs = for_network_specs;
        self
    }

    /// Set `for_settings` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`SETTREE`] tree.
    pub fn set_settings(mut self, for_settings: Batch) -> Self {
        self.for_settings = for_settings;
        self
    }

    /// Set `for_transaction` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`TRANSACTION`] tree.
    pub fn set_transaction(mut self, for_transaction: Batch) -> Self {
        self.for_transaction = for_transaction;
        self
    }

    /// Set `for_verifiers` field in [`TrDbCold`] with `Batch` that will be
    /// applied to [`VERIFIERS`] tree.
    pub fn set_verifiers(mut self, for_verifiers: Batch) -> Self {
        self.for_verifiers = for_verifiers;
        self
    }

    /// Apply constructed set of batches within [`TrDbCold`] to the database
    /// with a given name, in a single transaction.
    ///
    /// Note that both `ErrorSource` variants are available.
    pub fn apply(&self, database: &sled::Db) -> Result<()> {
        let addresses = open_tree(database, ADDRTREE)?;
        let history = open_tree(database, HISTORY)?;
        let metadata = open_tree(database, METATREE)?;
        let network_specs = open_tree(database, SPECSTREE)?;
        let settings = open_tree(database, SETTREE)?;
        let transaction = open_tree(database, TRANSACTION)?;
        let verifiers = open_tree(database, VERIFIERS)?;
        let s = (
            &addresses,
            &history,
            &metadata,
            &network_specs,
            &settings,
            &transaction,
            &verifiers,
        );
        let res: TransactionResult<(), sled::Error> = s.transaction(
            |(
                tx_addresses,
                tx_history,
                tx_metadata,
                tx_network_specs,
                tx_settings,
                tx_transaction,
                tx_verifiers,
            )| {
                tx_addresses.apply_batch(&self.for_addresses)?;
                tx_addresses.flush();
                tx_history.apply_batch(&self.for_history)?;
                tx_history.flush();
                tx_metadata.apply_batch(&self.for_metadata)?;
                tx_metadata.flush();
                tx_network_specs.apply_batch(&self.for_network_specs)?;
                tx_network_specs.flush();
                tx_settings.apply_batch(&self.for_settings)?;
                tx_settings.flush();
                tx_transaction.apply_batch(&self.for_transaction)?;
                tx_transaction.flush();
                tx_verifiers.apply_batch(&self.for_verifiers)?;
                tx_verifiers.flush();
                Ok(())
            },
        );

        Ok(res?)
    }
}

impl Default for TrDbCold {
    /// Default value for [`TrDbCold`]. Empty.
    fn default() -> Self {
        Self::new()
    }
}

/// Hot database transaction data containing [`Batch`] elements that will be
/// applied to each [`Tree`](sled::Tree).
///
/// Hot database tree names and content information could be found in
/// [`constants`] crate.
///
/// All trees are addressed when the database is generated or restored with
/// default values. Trees [`ADDRESS_BOOK`], [`METATREE`], and
/// [`SPECSTREEPREP`] are routinely updated by the database users.
///
/// [`TrDbHot`] is applied to the hot database in an atomic transaction and is
/// used by the active side only.
///
/// Note that all the checking is done as the [`TrDbHot`] is generated,
/// `apply` method does not do any checks on its own.
#[cfg(feature = "active")]
#[derive(Debug)]
pub struct TrDbHot {
    /// `Batch` to be applied to [`ADDRESS_BOOK`] tree
    for_address_book: Batch,

    /// `Batch` to be applied to [`METATREE`] tree
    for_metadata: Batch,

    /// `Batch` to be applied to [`META_HISTORY`] tree
    for_meta_history: Batch,

    /// `Batch` to be applied to [`SPECSTREEPREP`] tree
    for_network_specs_prep: Batch,

    /// `Batch` to be applied to [`SETTREE`] tree
    for_settings: Batch,
}

#[cfg(feature = "active")]
impl TrDbHot {
    /// Construct new empty [`TrDbHot`].
    pub fn new() -> Self {
        Self {
            for_address_book: Batch::default(),
            for_metadata: Batch::default(),
            for_meta_history: Batch::default(),
            for_network_specs_prep: Batch::default(),
            for_settings: Batch::default(),
        }
    }

    /// Set `for_address_book` field in [`TrDbHot`] with `Batch` that will be
    /// applied to [`ADDRESS_BOOK`] tree.
    pub fn set_address_book(mut self, for_address_book: Batch) -> Self {
        self.for_address_book = for_address_book;
        self
    }

    /// Set `for_metadata` field in [`TrDbHot`] with `Batch` that will be
    /// applied to [`METATREE`] tree.
    pub fn set_metadata(mut self, for_metadata: Batch) -> Self {
        self.for_metadata = for_metadata;
        self
    }

    /// Set `for_meta_history` field in [`TrDbHot`] with `Batch` that will be
    /// applied to [`META_HISTORY`] tree.
    pub fn set_meta_history(mut self, for_meta_history: Batch) -> Self {
        self.for_meta_history = for_meta_history;
        self
    }

    /// Set `for_network_specs_prep` field in [`TrDbHot`] with `Batch` that
    /// will be applied to [`SPECSTREEPREP`] tree.
    pub fn set_network_specs_prep(mut self, for_network_specs_prep: Batch) -> Self {
        self.for_network_specs_prep = for_network_specs_prep;
        self
    }

    /// Set `for_settings` field in [`TrDbHot`] with `Batch` that will be
    /// applied to [`SETTREE`] tree.
    pub fn set_settings(mut self, for_settings: Batch) -> Self {
        self.for_settings = for_settings;
        self
    }

    /// Apply constructed set of batches within [`TrDbHot`] to the database
    /// with a given name, in a single transaction.
    pub fn apply(&self, database: &sled::Db) -> Result<()> {
        let address_book = open_tree(database, ADDRESS_BOOK)?;
        let metadata = open_tree(database, METATREE)?;
        let meta_history = open_tree(database, META_HISTORY)?;
        let network_specs_prep = open_tree(database, SPECSTREEPREP)?;
        let settings = open_tree(database, SETTREE)?;
        let s = (
            &address_book,
            &metadata,
            &meta_history,
            &network_specs_prep,
            &settings,
        );

        let res: TransactionResult<(), sled::Error> = s.transaction(
            |(
                tx_address_book,
                tx_metadata,
                tx_meta_history,
                tx_network_specs_prep,
                tx_settings,
            )| {
                tx_address_book.apply_batch(&self.for_address_book)?;
                tx_address_book.flush();
                tx_metadata.apply_batch(&self.for_metadata)?;
                tx_metadata.flush();
                tx_meta_history.apply_batch(&self.for_meta_history)?;
                tx_meta_history.flush();
                tx_network_specs_prep.apply_batch(&self.for_network_specs_prep)?;
                tx_network_specs_prep.flush();
                tx_settings.apply_batch(&self.for_settings)?;
                tx_settings.flush();
                Ok(())
            },
        );

        Ok(res?)
    }
}

#[cfg(feature = "active")]
impl Default for TrDbHot {
    /// Default value for [`TrDbHot`]. Empty.
    fn default() -> Self {
        Self::new()
    }
}

/// SCALE-encodeable draft for [`Batch`], that will be a part of database atomic
/// transaction.
///
/// [`Batch`] does not support SCALE-encoding, so [`BatchStub`] is constructed
/// from a set of keys to be removed from the database and a set of (key, value)
/// pairs to be added into the database. Keys and values are SCALE-compatible
/// `Vec<u8>`.
///
/// When applying [`BatchStub`], i.e. transforming it into [`Batch`], the
/// removals are always applied before additions, to avoid accidental replacing
/// of just added value.
#[derive(Debug, Decode, Encode)]
struct BatchStub {
    /// Vector of keys to be removed from the database.
    removals: Vec<Vec<u8>>,

    /// Vector of (key, value) pairs to be added into the database.
    additions: Vec<(Vec<u8>, Vec<u8>)>,
}

impl BatchStub {
    /// Generate empty [`BatchStub`].
    fn empty() -> Self {
        Self {
            removals: Vec::new(),
            additions: Vec::new(),
        }
    }

    /// Transform [`BatchStub`] into [`Batch`], removals first.
    fn make_batch(&self) -> Batch {
        self.extend_batch(Batch::default())
    }

    /// Add elements from [`BatchStub`], removals first, in queue after
    /// instructions already present in input [`Batch`]
    fn extend_batch(&self, batch: Batch) -> Batch {
        let mut out = batch;
        for key in self.removals.iter() {
            out.remove(&key[..])
        }
        for (key, value) in self.additions.iter() {
            out.insert(&key[..], &value[..])
        }
        out
    }

    /// Add a new addition element into [`BatchStub`] `additions` queue.
    fn new_addition(mut self, key: Vec<u8>, value: Vec<u8>) -> Self {
        self.additions.push((key, value));
        self
    }

    /// Add a new removal element into [`BatchStub`] `removals` queue.
    fn new_removal(mut self, key: Vec<u8>) -> Self {
        self.removals.push(key);
        self
    }
}

/// Draft for cold database atomic transaction, constructed for Vault update
/// transaction (`add_specs`, `load_metadata`, `load_types`).
///
/// [`TrDbColdStub`] is stored SCALE-encoded in [`TRANSACTION`] tree
/// of the cold database under key [`STUB`] while the update is considered by
/// the user. Draft is applied atomically to the cold database if the update is
/// accepted.
///
/// Accepting an update could result in adding or removing database data.
///
/// [`TrDbColdStub`] contains [`Event`] set for [`HISTORY`] tree update and
/// `BatchStub` update drafts with corresponding removals and additions for
/// database trees:
///
/// - [`ADDRTREE`]
/// - [`METATREE`]
/// - [`SPECSTREE`]
/// - [`SETTREE`]
/// - [`VERIFIERS`]
///
/// Note that all the checking is done before the [`TrDbColdStub`] is written
/// into [`TRANSACTION`] tree, `apply` method will check only that the checksum
/// known to the user is the same as the one database has currently.
#[derive(Debug, Decode, Encode)]
pub struct TrDbColdStub {
    /// `BatchStub` to be transformed into `Batch` for [`ADDRTREE`] tree.
    addresses_stub: BatchStub,

    /// `Vec<Event>` to be entered into [`HISTORY`] tree, the
    /// [`Entry`](definitions::history::Entry) with a timestamp is generated
    /// only when the payload is approved by the user.
    history_stub: Vec<Event>,

    /// `BatchStub` to be transformed into `Batch` for [`METATREE`] tree.
    metadata_stub: BatchStub,

    /// `BatchStub` to be transformed into `Batch` for [`SPECSTREE`] tree.
    network_specs_stub: BatchStub,

    /// `BatchStub` to be transformed into `Batch` for [`SETTREE`] tree.
    settings_stub: BatchStub,

    /// `BatchStub` to be transformed into `Batch` for [`VERIFIERS`] tree.
    verifiers_stub: BatchStub,
}

impl TrDbColdStub {
    /// Construct new empty [`TrDbColdStub`].
    pub fn new() -> Self {
        Self {
            addresses_stub: BatchStub::empty(),
            history_stub: Vec::new(),
            metadata_stub: BatchStub::empty(),
            network_specs_stub: BatchStub::empty(),
            settings_stub: BatchStub::empty(),
            verifiers_stub: BatchStub::empty(),
        }
    }

    /// Recover [`TrDbColdStub`] from storage in the cold database.
    ///
    /// Function requires correct checksum to make sure the transaction is
    /// still the one that was shown to the user previously, and no changes to
    /// the database have occured after the atomic transaction draft was placed
    /// into storage.
    ///
    /// [`TRANSACTION`] tree is cleared in the process.
    pub fn from_storage(database: &sled::Db, checksum: u32) -> Result<Self> {
        let stub_encoded = {
            verify_checksum(database, checksum)?;
            let transaction = open_tree(database, TRANSACTION)?;
            transaction.get(STUB)?.ok_or(Error::Stub)?
        };
        TrDbCold::new()
            .set_transaction(make_batch_clear_tree(database, TRANSACTION)?) // clear transaction tree
            .apply(database)?;
        Ok(Self::decode(&mut &stub_encoded[..])?)
    }

    /// Put SCALE-encoded [`TrDbColdStub`] into storage in the [`TRANSACTION`]
    /// tree of the cold database under the key [`STUB`].
    ///
    /// Function returns `u32` checksum. This checksum is needed to recover
    /// stored [`TrDbColdStub`] using `from_storage` method.
    ///
    /// The [`TRANSACTION`] tree is cleared prior to adding data to storage.
    pub fn store_and_get_checksum(&self, database: &sled::Db) -> Result<u32> {
        let mut transaction_batch = make_batch_clear_tree(database, TRANSACTION)?;
        transaction_batch.insert(STUB, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree
            .apply(database)?;
        Ok(database.checksum()?)
    }

    /// Add new [`Event`] in `history_stub` field of the [`TrDbColdStub`]
    pub fn new_history_entry(mut self, event: Event) -> Self {
        self.history_stub.push(event);
        self
    }

    /// Prepare adding the metadata received as `load_metadata` update into the
    /// cold database:
    ///
    /// - Add a (key, value) pair to the metadata additions queue in
    ///   `metadata_stub`. Key is [`MetaKey`] in key form, value is metadata in
    ///   `Vec<u8>` format.
    /// - Add corresponding `Event::MetadataAdded(_)` into `history_stub`.
    pub fn add_metadata(mut self, meta_values: &MetaValues) -> Self {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        self.metadata_stub = self
            .metadata_stub
            .new_addition(meta_key.key(), meta_values.meta.to_vec());
        self.history_stub.push(Event::MetadataAdded {
            meta_values_display: MetaValuesDisplay::get(meta_values),
        });
        self
    }

    /// Prepare removing the metadata from the cold database:
    ///
    /// - Add [`MetaKey`] in key form to the metadata removals queue in
    ///   `metadata_stub`.
    /// - Add corresponding `Event::MetadataRemoved(_)` into `history_stub`.
    ///
    /// Function is used for `Hold` and `GeneralHold` processing when,
    /// respectively, the network verifier or the general verifier is changed.
    pub fn remove_metadata(mut self, meta_values: &MetaValues) -> Self {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        self.metadata_stub = self.metadata_stub.new_removal(meta_key.key());
        self.history_stub.push(Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay::get(meta_values),
        });
        self
    }

    /// Prepare adding [`OrderedNetworkSpecs`] into the cold database:
    ///
    /// - Transform received in `add_specs` payload [`NetworkSpecs`]
    ///   into [`OrderedNetworkSpecs`] by adding `order` field. Networks are always added
    ///   in the end of the network list, with order set to the total number of
    ///   network specs entries currently in Vault. When a network is removed,
    ///   the order of the remaining networks gets rearranged, see details in
    ///   function [`remove_network`](crate::helpers::remove_network).
    /// - Add a (key, value) pair to the network specs additions queue in
    ///   `network_specs_stub`. Key is [`NetworkSpecsKey`] in key form, value is
    ///   SCALE-encoded [`OrderedNetworkSpecs`].
    /// - Add corresponding `Event::NetworkSpecsAdded(_)` into `history_stub`.
    /// - Add root address for the network if the
    ///   [`AddressDetails`](definitions::users::AddressDetails) entry with
    ///   matching [`Encryption`](definitions::crypto::Encryption) already exists,
    ///   i.e. add (key, value) pair to the address additions queue in
    ///   `addresses_stub`. Key is
    ///   [`AddressKey`](definitions::keyring::AddressKey)
    ///   in key form, value is
    ///   SCALE-encoded updated
    ///   [`AddressDetails`](definitions::users::AddressDetails).
    /// - If address was added, add corresponding `Event::IdentityAdded(_)`
    ///   into `history_stub`.
    ///
    /// Note that `add_network_specs` does not deal with network verifiers:
    /// verifier data is not necessarily updated each time the network
    /// specs are added.
    pub fn add_network_specs(
        mut self,
        database: &sled::Db,
        network_specs_to_send: &NetworkSpecs,
        valid_current_verifier: &ValidCurrentVerifier,
        general_verifier: &Verifier,
    ) -> Result<Self> {
        let network_specs_key = NetworkSpecsKey::from_parts(
            &network_specs_to_send.genesis_hash,
            &network_specs_to_send.encryption,
        );
        let order = {
            let chainspecs = open_tree(database, SPECSTREE)?;
            chainspecs.len()
        } as u8;
        let network_specs = network_specs_to_send.to_store(order);
        self.network_specs_stub = self
            .network_specs_stub
            .new_addition(network_specs_key.key(), network_specs.encode());
        self.history_stub.push(Event::NetworkSpecsAdded {
            network_specs_display: NetworkSpecsDisplay::get(
                &network_specs,
                valid_current_verifier,
                general_verifier,
            ),
        });

        Ok(self)
    }

    /// Prepare removing [`OrderedNetworkSpecs`] from the cold database:
    ///
    /// - Add [`NetworkSpecsKey`] in key form to the network specs removal queue
    ///   in `network_specs_stub`.
    /// - Add corresponding `Event::NetworkSpecsRemoved(_)` into `history_stub`.
    ///
    /// Function is used for `Hold` and `GeneralHold` processing when,
    /// respectively, the network verifier or the general verifier is changed.
    ///
    /// Note that function does not deal with the verifiers nor with the
    /// addresses.
    ///
    /// Verifiers remain unchanged during the hold processing.
    ///
    /// The addresses are not removed and will be again visible from the user
    /// interface when the properly verified network specs are loaded in Vault.
    pub fn remove_network_specs(
        mut self,
        network_specs: &OrderedNetworkSpecs,
        valid_current_verifier: &ValidCurrentVerifier,
        general_verifier: &Verifier,
    ) -> Self {
        let network_specs_key = NetworkSpecsKey::from_parts(
            &network_specs.specs.genesis_hash,
            &network_specs.specs.encryption,
        );
        self.network_specs_stub = self.network_specs_stub.new_removal(network_specs_key.key());
        self.history_stub.push(Event::NetworkSpecsRemoved {
            network_specs_display: NetworkSpecsDisplay::get(
                network_specs,
                valid_current_verifier,
                general_verifier,
            ),
        });
        self
    }

    /// Prepare adding new general verifier [`Verifier`] into the cold
    /// database:
    ///
    /// - Add a (key, value) pair to the settings additions queue in
    ///   `settings_stub`. Key is [`GENERALVERIFIER`] and the value is
    ///   SCALE-encoded [`Verifier`] that is set to be the new general verifier.
    /// - Add corresponding `Event::GeneralVerifierSet(_)` into `history_stub`.
    pub fn new_general_verifier(mut self, general_verifier: &Verifier) -> Self {
        self.settings_stub = self
            .settings_stub
            .new_addition(GENERALVERIFIER.to_vec(), general_verifier.encode());
        self.history_stub.push(Event::GeneralVerifierSet {
            verifier: general_verifier.to_owned(),
        });
        self
    }

    /// Prepare adding types information [`ContentLoadTypes`] received as
    /// `load_types` update into the cold database:
    ///
    /// - Add a (key, value) pair to the settings additions queue in
    ///   `settings_stub`. Key is [`TYPES`] and the value is [`ContentLoadTypes`]
    ///   types information in `store` format (SCALE-encoded).
    /// - Add corresponding `Event::TypesAdded(_)` into `history_stub`.
    pub fn add_types(mut self, types: &ContentLoadTypes, general_verifier: &Verifier) -> Self {
        self.settings_stub = self
            .settings_stub
            .new_addition(TYPES.to_vec(), types.store());
        self.history_stub.push(Event::TypesAdded {
            types_display: TypesDisplay::get(types, general_verifier),
        });
        self
    }

    /// Prepare removing types information from the cold database:
    ///
    /// - Add [`TYPES`] key to the settings removal queue in `settings_stub`.
    /// - Add corresponding `Event::TypesRemoved(_)` into `history_stub`.
    ///
    /// Function is used to process `GeneralHold` when general verifier is
    /// changed.
    pub fn remove_types(mut self, types: &ContentLoadTypes, general_verifier: &Verifier) -> Self {
        self.settings_stub = self.settings_stub.new_removal(TYPES.to_vec());
        self.history_stub.push(Event::TypesRemoved {
            types_display: TypesDisplay::get(types, general_verifier),
        });
        self
    }

    /// Prepare adding new network verifier [`ValidCurrentVerifier`] into the
    /// cold database:
    ///
    /// - Add a (key, value) pair to the verifiers additions queue in
    ///   `verifiers_stub`. Key is [`VerifierKey`] and the value is SCALE-encoded
    ///   [`ValidCurrentVerifier`] that is set to be the new verifier for the
    ///   network.
    /// - Add corresponding `Event::NetworkVerifierSet(_)` into `history_stub`.
    pub fn new_network_verifier(
        mut self,
        verifier_key: &VerifierKey,
        valid_current_verifier: &ValidCurrentVerifier,
        general_verifier: &Verifier,
    ) -> Self {
        self.verifiers_stub = self.verifiers_stub.new_addition(
            verifier_key.key(),
            CurrentVerifier::Valid(valid_current_verifier.to_owned()).encode(),
        );
        self.history_stub.push(Event::NetworkVerifierSet {
            network_verifier_display: NetworkVerifierDisplay::get(
                verifier_key,
                valid_current_verifier,
                general_verifier,
            ),
        });
        self
    }

    /// Transform [`TrDbColdStub`] into [`TrDbCold`] and apply to the database
    /// with a given name, in a single transaction.
    ///
    /// The [`TRANSACTION`] tree gets cleared in the process.
    ///
    /// It is unlikely that this clearing is ever doing anything, as the
    /// intended use of the [`TrDbColdStub`] is to recover it from the database
    /// (with clearing the [`TRANSACTION`] tree) and then immediately apply.
    pub fn apply(self, database: &sled::Db) -> Result<()> {
        let for_transaction = make_batch_clear_tree(database, TRANSACTION)?;
        TrDbCold {
            for_addresses: self.addresses_stub.make_batch(),
            for_history: events_to_batch(database, self.history_stub)?,
            for_metadata: self.metadata_stub.make_batch(),
            for_network_specs: self.network_specs_stub.make_batch(),
            for_settings: self.settings_stub.make_batch(),
            for_transaction,
            for_verifiers: self.verifiers_stub.make_batch(),
        }
        .apply(database)
    }
}

impl Default for TrDbColdStub {
    /// Default value for [`TrDbColdStub`]. Empty.
    fn default() -> Self {
        Self::new()
    }
}

/// Temporary storage for signable transaction and associated data.
///
/// Signable transaction received by the Vault must always be parsed prior to
/// signing, and when it is, [`TrDbColdSign`] is generated and the transaction
/// details are shown to user.
///
/// If the user signs the transaction or tries to sign and enters wrong
/// password, the transaction data will be recorded in Vault history log.
///
/// While the user considers the transaction, [`TrDbColdSign`] is stored
/// SCALE-encoded in [`TRANSACTION`] tree of the cold database under the key
/// [`SIGN`].
///
/// [`TrDbColdSign`] contains:
///
/// - [`SignContent`] with data to sign
/// - name of the network in which the transaction is made
/// - derivation path of the address used, whether the address has password,
///   corresponding [`MultiSigner`] value
/// - relevant history [`Event`] set: warnings that were shown during the
///   parsing
#[derive(Debug, Decode, Default, Encode)]
pub struct TrDbColdSign {
    /// Bulk of transactions to sign.
    pub signing_bulk: Vec<TrDbColdSignOne>,
}

impl TrDbColdSign {
    /// Recover [`TrDbColdSign`] from storage in the cold database.
    ///
    /// Function requires an optional correct checksum to make sure
    /// the signable transaction is still the one that was shown to
    /// the user previously, and no changes to the database have occurred.
    /// While parsing a bulk no checksum is provided and no checks are done.
    ///
    /// [`TRANSACTION`] tree is **not** cleared in the process. User is allowed
    /// to try entering password several times, for all this time the
    /// transaction remains in the database.
    pub fn from_storage(database: &sled::Db, checksum: Option<u32>) -> Result<Option<Self>> {
        let sign_encoded = {
            if let Some(checksum) = checksum {
                verify_checksum(database, checksum)?;
            }
            let transaction = open_tree(database, TRANSACTION)?;
            match transaction.get(SIGN)? {
                Some(a) => a,
                None => return Ok(None),
            }
        };
        Ok(Some(Self::decode(&mut &sign_encoded[..])?))
    }

    /// Put SCALE-encoded [`TrDbColdSign`] into storage in the [`TRANSACTION`]
    /// tree of the cold database under the key [`SIGN`].
    ///
    /// Function returns `u32` checksum. This checksum is needed to recover
    /// stored [`TrDbColdSign`] using `from_storage` method.
    ///
    /// The [`TRANSACTION`] tree is cleared prior to adding data to storage.
    pub fn store_and_get_checksum(&self, database: &sled::Db) -> Result<u32> {
        let mut transaction_batch = make_batch_clear_tree(database, TRANSACTION)?;
        transaction_batch.insert(SIGN, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree
            .apply(database)?;
        Ok(database.checksum()?)
    }

    /// Use [`TrDbColdSign`] to add history log data into the cold database.
    ///
    /// Possible history log entries are:
    ///
    /// - `Event::TransactionSigned(_)` and `Event::MessageSigned(_)` for the
    ///   cases when the signature was generated and displayed through the user
    ///   interface
    /// - `Event::TransactionSignError(_)` and `Event::MessageSignError(_)` for
    ///   the cases when the user has entered the wrong password and no signature
    ///   was generated. Vault current policy is to log all wrong password entry
    ///   attempts.
    ///
    /// Required input:
    ///
    /// - `wrong_password` flag; for entries with `true` value the signature
    ///   was not generated, because user has entered the wrong password;
    /// - user-added text comment for the transaction
    /// - index of the transaction in the bulk
    /// - database name, into which the data is added
    ///
    /// Function returns database checksum, to be collected and re-used in case
    /// of wrong password entry.
    pub fn apply(
        self,
        database: &sled::Db,
        wrong_password: bool,
        user_comment: &str,
        idx: usize,
    ) -> Result<u32> {
        let mut history = vec![];
        let mut for_transaction = Batch::default();
        let s = &self.signing_bulk[idx];
        let signed_by = VerifierValue::Standard { m: s.multisigner() };
        history.append(&mut s.history.clone());
        match &s.content {
            SignContent::Transaction { method, extensions } => {
                let transaction = [method.encode(), extensions.clone()].concat();
                let sign_display =
                    SignDisplay::get(&transaction, &s.network_name, &signed_by, user_comment);
                if wrong_password {
                    history.push(Event::TransactionSignError { sign_display })
                } else {
                    history.push(Event::TransactionSigned { sign_display });
                    // if this is the last transaction in the bulk and the password was right,
                    // all is done and the DB can be cleared.
                    if idx == self.signing_bulk.len() - 1 {
                        for_transaction = make_batch_clear_tree(database, TRANSACTION)?;
                    }
                }
            }
            SignContent::Message(message) => {
                let hex_message = hex::encode(message);
                let sign_message_display =
                    SignMessageDisplay::get(&hex_message, &s.network_name, &signed_by, user_comment);
                if wrong_password {
                    history.push(Event::MessageSignError {
                        sign_message_display,
                    })
                } else {
                    history.push(Event::MessageSigned {
                        sign_message_display,
                    });
                    // if this is the last transaction in the bulk and the password was right,
                    // all is done and the DB can be cleared.
                    if idx == self.signing_bulk.len() - 1 {
                        for_transaction = make_batch_clear_tree(database, TRANSACTION)?;
                    }
                }
            }
        }
        TrDbCold::new()
            .set_history(events_to_batch(database, history)?)
            .set_transaction(for_transaction)
            .apply(database)?;
        Ok(database.checksum()?)
    }
}

impl From<TrDbColdSignOne> for TrDbColdSign {
    fn from(t: TrDbColdSignOne) -> Self {
        Self {
            signing_bulk: vec![t],
        }
    }
}

#[derive(Debug, Decode, Encode)]
pub struct TrDbColdSignOne {
    /// data to sign
    content: SignContent,

    /// name of the network in which the transaction is made
    network_name: String,

    /// derivation path of the address by which the transaction was generated
    path: String,

    /// is address by which the transaction was generated passworded?
    has_pwd: bool,

    /// [`MultiSigner`] corresponding to the address by which the transaction
    /// was generated
    multisigner: MultiSigner,

    /// [`Event`] set produced during parsing
    history: Vec<Event>,
}

/// Signable transaction content
///
/// Vault can sign:
/// - transactions
/// - messages
///
/// Mortal signable transactions have prelude `53xx00`, immortal have prelude
/// `53xx02`. Signable transactions consist of method with call details and
/// extensions.
///
/// Messages contain SCALE-encoded text messages.
#[derive(Debug, Decode, Encode, Clone)]
pub enum SignContent {
    /// `53xx00` or `53xx02` transaction
    Transaction {
        /// method as raw data
        method: Vec<u8>,

        /// extensions as raw data
        extensions: Vec<u8>,
    },

    /// `53xx03` text message
    Message(Vec<u8>),
}

impl TrDbColdSignOne {
    /// Construct [`TrDbColdSign`] from components.
    ///
    /// Required input:
    ///
    /// - [`SignContent`] with data to sign
    /// - name of the network in which the transaction is made
    /// - derivation path of the address used, whether the address has password,
    ///   corresponding [`MultiSigner`] value
    /// - relevant history [`Event`] set
    pub fn generate(
        content: SignContent,
        network_name: &str,
        path: &str,
        has_pwd: bool,
        multisigner: &MultiSigner,
        history: Vec<Event>,
    ) -> Self {
        Self {
            content,
            network_name: network_name.to_string(),
            path: path.to_string(),
            has_pwd,
            multisigner: multisigner.to_owned(),
            history,
        }
    }

    /// Get transaction content.
    pub fn content(&self) -> &SignContent {
        &self.content
    }

    /// Get derivation path.
    pub fn path(&self) -> String {
        self.path.to_string()
    }

    /// Get `has_pwd` flag.
    pub fn has_pwd(&self) -> bool {
        self.has_pwd
    }

    /// Get [`MultiSigner`] value
    pub fn multisigner(&self) -> MultiSigner {
        self.multisigner.to_owned()
    }
}
