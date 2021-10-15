use sled::{Batch, Transactional};
use anyhow;
use constants::{ADDRESS_BOOK, ADDRTREE, GENERALVERIFIER, HISTORY, METATREE, SETTREE, SPECSTREE, SPECSTREEPREP, STUB, TRANSACTION, TYPES, VERIFIERS};
use definitions::{history::Event, keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{ChainSpecs, CurrentVerifier, Verifier}, qr_transfers::ContentLoadTypes, users::{AddressDetails}};
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;

use crate::error::{Error, NotDecodeable, NotFound};
use crate::helpers::{open_db, open_tree, make_batch_clear_tree, verify_checksum};
use crate::manage_history::events_to_batch;

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct TrDbColdStub {
    addresses_stub: BatchStub,
    history_stub: Vec<Event>,
    metadata_stub: BatchStub,
    network_specs_stub: BatchStub,
    settings_stub: BatchStub,
    verifiers_stub: BatchStub,
}

pub struct TrDbCold {
    for_addresses: Batch,
    for_history: Batch,
    for_metadata: Batch,
    for_network_specs: Batch,
    for_settings: Batch,
    for_transaction: Batch,
    for_verifiers: Batch,
}

pub struct TrDbHot {
    for_address_book: Batch,
    for_metadata: Batch,
    for_network_specs_prep: Batch,
    for_settings: Batch,
}

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct BatchStub {
    removals: Vec<Vec<u8>>, // vector of keys to be removed from the database
    additions: Vec<(Vec<u8>, Vec<u8>)>, // vector of (key, value) to be added into the database
}

/// TODO check that removal-addition order do not mess things up
impl BatchStub {
    // generate empty BatchStub
    pub fn empty() -> Self {
        Self {
            removals: Vec::new(),
            additions: Vec::new(),
        }
    }
    // transform BatchStub into Batch, removals first
    pub fn make_batch(&self) -> Batch {
        self.extend_batch(Batch::default())
    }
    // add instructions from BatchStub, removals first,
    // in queue after instructions from incoming Batch
    pub fn extend_batch(&self, batch: Batch) -> Batch {
        let mut out = batch;
        for key in self.removals.iter() {
            out.remove(key.to_vec())
        }
        for (key, value) in self.additions.iter() {
            out.insert(key.to_vec(), value.to_vec())
        }
        out
    }
    // new addition element into queue
    pub fn new_addition(mut self, key: Vec<u8>, value: Vec<u8>) {
        self.additions.push((key, value))
    }
    // new removal element into queue
    pub fn new_removal(mut self, key: Vec<u8>) {
        self.removals.push(key)
    }
}


/// Hot database contains following trees: 
/// address_book tree, by default filled with values for standard networks;
/// metadata tree, by default empty;
/// network_specs_prep tree, by default filled with values for standard networks;
/// settings tree, by default containing types information.
/// Trees address_book, metadata, and network_specs_prep are routinely updated by database users.
/// Struct TrDbHot contains set of batches that could be aplied to hot database.
impl TrDbHot {
    /// function to construct new empty TrDbHot
    pub fn new() -> Self {
        Self {
            for_address_book: Batch::default(),
            for_metadata: Batch::default(),
            for_network_specs_prep: Batch::default(),
            for_settings: Batch::default(),
        }
    }
    /// functions to set each of the four elements:
    /// address_book batch
    pub fn set_address_book(mut self, for_address_book: Batch) -> Self {
        self.for_address_book = for_address_book;
        self
    }
    /// metadata batch
    pub fn set_metadata(mut self, for_metadata: Batch) -> Self {
        self.for_metadata = for_metadata;
        self
    }
    /// network_specs_prep batch
    pub fn set_network_specs_prep(mut self, for_network_specs_prep: Batch) -> Self {
        self.for_network_specs_prep = for_network_specs_prep;
        self
    }
    /// settings batch
    pub fn set_settings(mut self, for_settings: Batch) -> Self {
        self.for_settings = for_settings;
        self
    }
    // function to apply constructed set of batches within TrDbHot to the database in a single transaction
    pub fn apply(&self, database_name: &str) -> anyhow::Result<()> {
        let database = open_db(database_name)?;
        let address_book = open_tree(&database, ADDRESS_BOOK)?;
        let metadata = open_tree(&database, METATREE)?;
        let network_specs_prep = open_tree(&database, SPECSTREEPREP)?;
        let settings = open_tree(&database, SETTREE)?;
        match (&address_book, &metadata, &network_specs_prep, &settings).transaction(|(tx_address_book, tx_metadata, tx_network_specs_prep, tx_settings)| {
            tx_address_book.apply_batch(&self.for_address_book)?;
            tx_address_book.flush();
            tx_metadata.apply_batch(&self.for_metadata)?;
            tx_metadata.flush();
            tx_network_specs_prep.apply_batch(&self.for_network_specs_prep)?;
            tx_network_specs_prep.flush();
            tx_settings.apply_batch(&self.for_settings)?;
            tx_settings.flush();
            Ok(())
        }) {
            Ok(()) => Ok(()),
            Err(e) => return Err(Error::DatabaseTransactionError(e).show()),
        }
    }
}

/// Cold database contains following trees: 
/// address tree, empty for release and populated with Alice-related addresses for test database;
/// history tree, populated by history event entries; in "fresh start" contains only message that db was initiated;
/// metadata tree, populated by default with metadata of kusama, polkadot, rococo, and westend;
/// network_specs tree, by default populated with network specs of standard networks;
/// settings tree, by default containing types and general verifier information;
/// transaction tree, by default empty; is used to keep database update information or signer transaction information
/// between transaction_parsing and transaction_signing;
/// verifiers tree, by default populated with network verifier information for standard networks.
/// All trees are routinely updated as Signer is used.
/// Struct TrDbCold contains set of batches to be aplied to cold database.
impl TrDbCold {
    /// function to construct new empty TrDbCold
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
    /// functions to set each of the seven elements:
    /// addresses batch
    pub fn set_addresses(mut self, for_addresses: Batch) -> Self {
        self.for_addresses = for_addresses;
        self
    }
    /// set history batch
    pub fn set_history(mut self, for_history: Batch) -> Self {
        self.for_history = for_history;
        self
    }
    /// metadata batch
    pub fn set_metadata(mut self, for_metadata: Batch) -> Self {
        self.for_metadata = for_metadata;
        self
    }
    /// network_specs batch
    pub fn set_network_specs(mut self, for_network_specs: Batch) -> Self {
        self.for_network_specs = for_network_specs;
        self
    }
    /// settings batch
    pub fn set_settings(mut self, for_settings: Batch) -> Self {
        self.for_settings = for_settings;
        self
    }
    /// transaction batch
    pub fn set_transaction(mut self, for_transaction: Batch) -> Self {
        self.for_transaction = for_transaction;
        self
    }
    /// verifiers batch
    pub fn set_verifiers(mut self, for_verifiers: Batch) -> Self {
        self.for_verifiers = for_verifiers;
        self
    }
    // function to apply constructed set of batches within TrDbHot to the database in a single transaction
    pub fn apply(&self, database_name: &str) -> anyhow::Result<()> {
        let database = open_db(database_name)?;
        let addresses = open_tree(&database, ADDRTREE)?;
        let history = open_tree(&database, HISTORY)?;
        let metadata = open_tree(&database, METATREE)?;
        let network_specs = open_tree(&database, SPECSTREE)?;
        let settings = open_tree(&database, SETTREE)?;
        let transaction = open_tree(&database, TRANSACTION)?;
        let verifiers = open_tree(&database, VERIFIERS)?;
        match (&addresses, &history, &metadata, &network_specs, &settings, &transaction, &verifiers)
            .transaction(|(tx_addresses, tx_history, tx_metadata, tx_network_specs, tx_settings, tx_transaction, tx_verifiers)| {
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
        }) {
            Ok(()) => Ok(()),
            Err(e) => return Err(Error::DatabaseTransactionError(e).show()),
        }
    }
}

/// Database transaction stub for cold database is formed while running parse_transaction in Signer.
/// It contains BatchStubs for address, metadata, network_specs, settings, and verifiers trees,
/// and Vec<Event> from which the history tree is updated.
/// Database transaction stub is stored SCALE encoded in transaction tree of the cold database.
impl TrDbColdStub {
    /// function to construct new empty TrDbColdStub
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
    /// function to recover TrDbColdStub from storage in the database
    pub fn from_storage(database_name: &str, checksum: u32) -> anyhow::Result<Self> {
        let stub_encoded = {
            let database = open_db(&database_name)?;
            verify_checksum(&database, checksum)?;
            let transaction = open_tree(&database, TRANSACTION)?;
            match transaction.get(STUB) {
                Ok(Some(a)) => a,
                Ok(None) => return Err(Error::NotFound(NotFound::Stub).show()),
                Err(e) => return Err(Error::InternalDatabaseError(e).show()),
            }
        };
        TrDbCold::new()
            .set_transaction(make_batch_clear_tree(&database_name, TRANSACTION)?) // clear transaction tree
            .apply(&database_name)?;
        match Self::decode(&mut &stub_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Stub).show()),
        }
    }
    /// function to put TrDbColdStub into storage in the database
    pub fn store_and_get_checksum(&self, database_name: &str) -> anyhow::Result<u32> {
        let mut transaction_batch = make_batch_clear_tree(database_name, TRANSACTION)?;
        transaction_batch.insert(STUB, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree and insert the stub
            .apply(&database_name)?;
        let database = open_db(&database_name)?;
        match database.checksum() {
            Ok(x) => Ok(x),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        }
    }
    /// function to put (address_key, address_details) unit in addition queue in TrDbColdStub
    pub fn new_addition_addresses(self, address_key: &AddressKey, address_details: AddressDetails) {
        self.addresses_stub.new_addition(address_key.key(), address_details.encode())
    }
    /// function to put address_key in removal queue in TrDbColdStub
    pub fn new_removal_addresses(self, address_key: &AddressKey) {
        self.addresses_stub.new_removal(address_key.key())
    }
    /// function to add new event in history preparation in TrDbColdStub
    pub fn new_history_entry(mut self, event: Event) {
        self.history_stub.push(event)
    }
    /// function to put metadata unit in addition queue in TrDbColdStub
    pub fn new_addition_metadata(self, meta_values: MetaValues) {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        self.metadata_stub.new_addition(meta_key.key(), meta_values.meta)
    }
    /// function to put meta_key in removal queue in TrDbColdStub
    pub fn new_removal_metadata(self, meta_key: &MetaKey) {
        self.metadata_stub.new_removal(meta_key.key())
    }
    /// function to put network_specs unit in addition queue in TrDbColdStub
    pub fn new_addition_network_specs(self, network_specs: ChainSpecs) {
        let network_specs_key = NetworkSpecsKey::from_parts(&network_specs.genesis_hash.to_vec(), &network_specs.encryption);
        self.network_specs_stub.new_addition(network_specs_key.key(), network_specs.encode())
    }
    /// function to put network_specs_key in removal queue in TrDbColdStub
    pub fn new_removal_network_specs(self, network_specs_key: &NetworkSpecsKey) {
        self.network_specs_stub.new_removal(network_specs_key.key())
    }
    /// function to put new general verifier in addition queue in TrDbColdStub
    pub fn new_general_verifier(self, verifier: Verifier) {
        self.settings_stub.new_addition(GENERALVERIFIER.to_vec(), verifier.encode())
    }
    /// function to put new types entry in addition queue in TrDbColdStub
    pub fn new_types(self, types: ContentLoadTypes) {
        self.settings_stub.new_addition(TYPES.to_vec(), types.store())
    }
    /// function to put types in removal queue in TrDbColdStub
    pub fn remove_types(self) {
        self.settings_stub.new_removal(TYPES.to_vec())
    }
    /// function to put new network verifier in addition queue in TrDbColdStub
    pub fn new_network_verifier(self, verifier_key: &VerifierKey, current_verifier: CurrentVerifier) {
        self.verifiers_stub.new_addition(verifier_key.key(), current_verifier.encode())
    }
    /// function to apply TrDbColdStub to database
    pub fn apply(self, database_name: &str) -> anyhow::Result<()> {
        TrDbCold {
            for_addresses: self.addresses_stub.make_batch(),
            for_history: events_to_batch(&database_name, self.history_stub)?,
            for_metadata: self.metadata_stub.make_batch(),
            for_network_specs: self.network_specs_stub.make_batch(),
            for_settings: self.settings_stub.make_batch(),
            for_transaction: Batch::default(),
            for_verifiers: self.verifiers_stub.make_batch(),
        }.apply(&database_name)
    }
}
