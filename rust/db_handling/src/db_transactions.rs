use parity_scale_codec::{Decode, Encode};
use sled::{Batch, Transactional};
#[cfg(feature = "signer")]
use sp_runtime::MultiSigner;

use constants::{ADDRTREE, HISTORY, METATREE, SETTREE, SPECSTREE, TRANSACTION, VERIFIERS};
#[cfg(feature = "active")]
use constants::{ADDRESS_BOOK, SPECSTREEPREP};
#[cfg(feature = "signer")]
use constants::{DRV, GENERALVERIFIER, SIGN, STUB, TYPES};

use definitions::error::ErrorSource;
#[cfg(feature = "active")]
use definitions::error_active::{Active, ErrorActive};
#[cfg(feature = "signer")]
use definitions::{error_signer::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, NotFoundSigner, Signer}, helpers::multisigner_to_public, history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay, NetworkVerifierDisplay, SignDisplay, SignMessageDisplay, TypesDisplay}, keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{NetworkSpecs, NetworkSpecsToSend, CurrentVerifier, ValidCurrentVerifier, Verifier, VerifierValue}, qr_transfers::ContentLoadTypes, users::AddressDetails};

use crate::helpers::{open_db, open_tree};
#[cfg(feature = "signer")]
use crate::{helpers::{make_batch_clear_tree, verify_checksum}, manage_history::events_to_batch};


pub struct TrDbCold {
    for_addresses: Batch,
    for_history: Batch,
    for_metadata: Batch,
    for_network_specs: Batch,
    for_settings: Batch,
    for_transaction: Batch,
    for_verifiers: Batch,
}

#[cfg(feature = "active")]
pub struct TrDbHot {
    for_address_book: Batch,
    for_metadata: Batch,
    for_network_specs_prep: Batch,
    for_settings: Batch,
}

#[derive(Decode, Encode)]
pub struct BatchStub {
    removals: Vec<Vec<u8>>, // vector of keys to be removed from the database
    additions: Vec<(Vec<u8>, Vec<u8>)>, // vector of (key, value) to be added into the database
}

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
            out.remove(&key[..])
        }
        for (key, value) in self.additions.iter() {
            out.insert(&key[..], &value[..])
        }
        out
    }
    // new addition element into queue
    pub fn new_addition(mut self, key: Vec<u8>, value: Vec<u8>) -> Self {
        self.additions.push((key, value));
        self
    }
    // new removal element into queue
    pub fn new_removal(mut self, key: Vec<u8>) -> Self {
        self.removals.push(key);
        self
    }
}


/// Hot database contains following trees: 
/// address_book tree, by default filled with values for standard networks;
/// metadata tree, by default empty;
/// network_specs_prep tree, by default filled with values for standard networks;
/// settings tree, by default containing types information.
/// Trees address_book, metadata, and network_specs_prep are routinely updated by database users.
/// Struct TrDbHot contains set of batches that could be aplied to hot database.
#[cfg(feature = "active")]
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
    /// function to apply constructed set of batches within TrDbHot to the database in a single transaction
    pub fn apply(&self, database_name: &str) -> Result<(), ErrorActive> {
        let database = open_db::<Active>(database_name)?;
        let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
        let metadata = open_tree::<Active>(&database, METATREE)?;
        let network_specs_prep = open_tree::<Active>(&database, SPECSTREEPREP)?;
        let settings = open_tree::<Active>(&database, SETTREE)?;
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
            Err(e) => Err(<Active>::db_transaction(e)),
        }
    }
}

#[cfg(feature = "active")]
impl Default for TrDbHot {
    fn default() -> Self {
        Self::new()
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
    /// function to apply constructed set of batches within TrDbCold to the database in a single transaction
    /// Not that since creating cold database is done on the Active side, both ErrorSource veriants are applicable
    pub fn apply<T: ErrorSource>(&self, database_name: &str) -> Result<(), T::Error> {
        let database = open_db::<T>(database_name)?;
        let addresses = open_tree::<T>(&database, ADDRTREE)?;
        let history = open_tree::<T>(&database, HISTORY)?;
        let metadata = open_tree::<T>(&database, METATREE)?;
        let network_specs = open_tree::<T>(&database, SPECSTREE)?;
        let settings = open_tree::<T>(&database, SETTREE)?;
        let transaction = open_tree::<T>(&database, TRANSACTION)?;
        let verifiers = open_tree::<T>(&database, VERIFIERS)?;
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
            Err(e) => Err(<T>::db_transaction(e)),
        }
    }
}

impl Default for TrDbCold {
    fn default() -> Self {
        Self::new()
    }
}

/// Database transaction stub for cold database is formed while running parse_transaction in Signer.
/// It contains BatchStubs for address, metadata, network_specs, settings, and verifiers trees,
/// and Vec<Event> from which the history tree is updated.
/// It is stored SCALE encoded in transaction tree of the cold database with key STUB.
#[derive(Decode, Encode)]
#[cfg(feature = "signer")]
pub struct TrDbColdStub {
    addresses_stub: BatchStub,
    history_stub: Vec<Event>,
    metadata_stub: BatchStub,
    network_specs_stub: BatchStub,
    settings_stub: BatchStub,
    verifiers_stub: BatchStub,
}

#[cfg(feature = "signer")]
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
    /// function to recover TrDbColdStub from storage in the Signer database
    pub fn from_storage(database_name: &str, checksum: u32) -> Result<Self, ErrorSigner> {
        let stub_encoded = {
            let database = open_db::<Signer>(database_name)?;
            verify_checksum(&database, checksum)?;
            let transaction = open_tree::<Signer>(&database, TRANSACTION)?;
            match transaction.get(STUB) {
                Ok(Some(a)) => a,
                Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Stub)),
                Err(e) => return Err(<Signer>::db_internal(e)),
            }
        };
        TrDbCold::new()
            .set_transaction(make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?) // clear transaction tree
            .apply::<Signer>(database_name)?;
        match Self::decode(&mut &stub_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Stub))),
        }
    }
    /// function to put TrDbColdStub into storage in the database
    pub fn store_and_get_checksum(&self, database_name: &str) -> Result<u32, ErrorSigner> {
        let mut transaction_batch = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
        transaction_batch.insert(STUB, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree and insert the stub
            .apply::<Signer>(database_name)?;
        let database = open_db::<Signer>(database_name)?;
        match database.checksum() {
            Ok(x) => Ok(x),
            Err(e) => Err(<Signer>::db_internal(e)),
        }
    }
    /// function to add new event in history preparation in TrDbColdStub
    pub fn new_history_entry(mut self, event: Event) -> Self {
        self.history_stub.push(event);
        self
    }
    /// function to put metadata unit in addition queue in TrDbColdStub
    pub fn add_metadata(mut self, meta_values: &MetaValues) -> Self {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        self.metadata_stub = self.metadata_stub.new_addition(meta_key.key(), meta_values.meta.to_vec());
        self.history_stub.push(Event::MetadataAdded(MetaValuesDisplay::get(meta_values)));
        self
    }
    /// function to put meta_key in removal queue in TrDbColdStub
    /// is used for clean up when the general verifier or network verifier is reset
    pub fn remove_metadata(mut self, meta_values: &MetaValues) -> Self {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        self.metadata_stub = self.metadata_stub.new_removal(meta_key.key());
        self.history_stub.push(Event::MetadataRemoved(MetaValuesDisplay::get(meta_values)));
        self
    }
    /// function to put network_specs unit in addition queue in TrDbColdStub
    pub fn add_network_specs(mut self, network_specs_to_send: &NetworkSpecsToSend, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier, database_name: &str) -> Result<Self, ErrorSigner> {
        let network_specs_key = NetworkSpecsKey::from_parts(&network_specs_to_send.genesis_hash, &network_specs_to_send.encryption);
        let order = {
            let database = open_db::<Signer>(database_name)?;
            let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
            chainspecs.len()
        } as u8;
        let network_specs = network_specs_to_send.to_store(order);
        self.network_specs_stub = self.network_specs_stub.new_addition(network_specs_key.key(), network_specs.encode());
        self.history_stub.push(Event::NetworkSpecsAdded(NetworkSpecsDisplay::get(&network_specs, valid_current_verifier, general_verifier)));
        {
            let database = open_db::<Signer>(database_name)?;
            let identities = open_tree::<Signer>(&database, ADDRTREE)?;
            for (address_key_vec, address_entry) in identities.iter().flatten() {
                let address_key = AddressKey::from_ivec(&address_key_vec);
                let (multisigner, mut address_details) = AddressDetails::process_entry_with_key_checked::<Signer>(&address_key, address_entry)?;
                if address_details.path.is_empty() && !address_details.has_pwd && (address_details.encryption == network_specs.encryption) && !address_details.network_id.contains(&network_specs_key) {
                    address_details.network_id.push(network_specs_key.to_owned());
                    self.addresses_stub = self.addresses_stub.new_addition(address_key.key(), address_details.encode());
                    self.history_stub.push(Event::IdentityAdded(IdentityHistory::get(&address_details.seed_name, &address_details.encryption, &multisigner_to_public(&multisigner), &address_details.path, &network_specs.genesis_hash)));
                }
            }
        }
        Ok(self)
    }
    /// function to put network_specs_key in removal queue in TrDbColdStub
    /// is used for clean up when the general verifier or network verifier is reset
    pub fn remove_network_specs(mut self, network_specs: &NetworkSpecs, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier) -> Self {
        let network_specs_key = NetworkSpecsKey::from_parts(&network_specs.genesis_hash, &network_specs.encryption);
        self.network_specs_stub = self.network_specs_stub.new_removal(network_specs_key.key());
        self.history_stub.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(network_specs, valid_current_verifier, general_verifier)));
        self
    }
    /// function to put new general verifier in addition queue in TrDbColdStub
    pub fn new_general_verifier(mut self, general_verifier: &Verifier) -> Self {
        self.settings_stub = self.settings_stub.new_addition(GENERALVERIFIER.to_vec(), general_verifier.encode());
        self.history_stub.push(Event::GeneralVerifierSet(general_verifier.to_owned()));
        self
    }
    /// function to put new types entry in addition queue in TrDbColdStub
    pub fn add_types(mut self, types: &ContentLoadTypes, general_verifier: &Verifier) -> Self {
        self.settings_stub = self.settings_stub.new_addition(TYPES.to_vec(), types.store());
        self.history_stub.push(Event::TypesAdded(TypesDisplay::get(types, general_verifier)));
        self
    }
    /// function to put types in removal queue in TrDbColdStub
    /// is used for clean up when the general verifier is reset
    pub fn remove_types(mut self, types: &ContentLoadTypes, general_verifier: &Verifier) -> Self {
        self.settings_stub = self.settings_stub.new_removal(TYPES.to_vec());
        self.history_stub.push(Event::TypesAdded(TypesDisplay::get(types, general_verifier)));
        self
    }
    /// function to put new network verifier in addition queue in TrDbColdStub
    pub fn new_network_verifier(mut self, verifier_key: &VerifierKey, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier) -> Self {
        self.verifiers_stub = self.verifiers_stub.new_addition(verifier_key.key(), CurrentVerifier::Valid(valid_current_verifier.to_owned()).encode());
        self.history_stub.push(Event::NetworkVerifierSet(NetworkVerifierDisplay::get(verifier_key, valid_current_verifier, general_verifier)));
        self
    }
    /// function to apply TrDbColdStub to database
    pub fn apply(self, database_name: &str) -> Result<(), ErrorSigner> {
        TrDbCold {
            for_addresses: self.addresses_stub.make_batch(),
            for_history: events_to_batch::<Signer>(database_name, self.history_stub)?,
            for_metadata: self.metadata_stub.make_batch(),
            for_network_specs: self.network_specs_stub.make_batch(),
            for_settings: self.settings_stub.make_batch(),
            for_transaction: Batch::default(),
            for_verifiers: self.verifiers_stub.make_batch(),
        }.apply::<Signer>(database_name)
    }
}

#[cfg(feature = "signer")]
impl Default for TrDbColdStub {
    fn default() -> Self {
        Self::new()
    }
}

/// Temporary storage for transaction to be signed and its additional information,
/// produced during parse_transaction in Signer.
/// Contains transaction itself as Vec<u8>, information about the address that
/// will signing the transaction (path, has_pwd, address_key),
/// and relevant history entries.
/// It is stored SCALE encoded in transaction tree of the cold database with key SIGN.
#[derive(Decode, Encode)]
#[cfg(feature = "signer")]
pub struct TrDbColdSign {
    content: SignContent,
    network_name: String,
    path: String,
    has_pwd: bool,
    multisigner: MultiSigner,
    history: Vec<Event>,
}

#[derive(Decode, Encode)]
#[cfg(feature = "signer")]
pub enum SignContent {
    Transaction{method: Vec<u8>, extensions: Vec<u8>},
    Message(String),
}

#[cfg(feature = "signer")]
impl TrDbColdSign {
    /// function to generate TrDbColdSign
    pub fn generate(content: SignContent, network_name: &str, path: &str, has_pwd: bool, multisigner: &MultiSigner, history: Vec<Event>) -> Self {
        Self {
            content,
            network_name: network_name.to_string(),
            path: path.to_string(),
            has_pwd,
            multisigner: multisigner.to_owned(),
            history,
        }
    }
    /// function to recover TrDbColdSign from storage in the database
    pub fn from_storage(database_name: &str, checksum: u32) -> Result<Self, ErrorSigner> {
        let sign_encoded = {
            let database = open_db::<Signer>(database_name)?;
            verify_checksum(&database, checksum)?;
            let transaction = open_tree::<Signer>(&database, TRANSACTION)?;
            match transaction.get(SIGN) {
                Ok(Some(a)) => a,
                Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Sign)),
                Err(e) => return Err(<Signer>::db_internal(e)),
            }
        };
        match Self::decode(&mut &sign_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Sign))),
        }
    }
    /// function to get transaction content
    pub fn content(&self) -> &SignContent {
        &self.content
    }
    /// function to get path
    pub fn path(&self) -> String {
        self.path.to_string()
    }
    /// function to get has_pwd
    pub fn has_pwd(&self) -> bool {
        self.has_pwd
    }
    /// function to get address key
    pub fn multisigner(&self) -> MultiSigner {
        self.multisigner.to_owned()
    }
    /// function to put TrDbColdSign into storage in the database
    pub fn store_and_get_checksum(&self, database_name: &str) -> Result<u32, ErrorSigner> {
        let mut transaction_batch = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
        transaction_batch.insert(SIGN, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree and insert the stub
            .apply::<Signer>(database_name)?;
        let database = open_db::<Signer>(database_name)?;
        match database.checksum() {
            Ok(x) => Ok(x),
            Err(e) => Err(<Signer>::db_internal(e)),
        }
    }
    /// function to apply TrDbColdSign to database
    /// returns database checksum, to be collected and re-used in case of wrong password entry
    pub fn apply(self, wrong_password: bool, user_comment: &str, database_name: &str) -> Result<u32, ErrorSigner> {
        let signed_by = VerifierValue::Standard(self.multisigner());
        let mut history = self.history;
        let mut for_transaction = Batch::default();
        match self.content {
            SignContent::Transaction{method, extensions} => {
                let transaction = [method.encode(), extensions].concat();
                let sign_display = SignDisplay::get(&transaction, &self.network_name, &signed_by, user_comment);
                if wrong_password {history.push(Event::TransactionSignError(sign_display))}
                else {
                    history.push(Event::TransactionSigned(sign_display));
                    for_transaction = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
                }
            },
            SignContent::Message(message) => {
                let sign_message_display = SignMessageDisplay::get(&message, &self.network_name, &signed_by, user_comment);
                if wrong_password {history.push(Event::MessageSignError(sign_message_display))}
                else {
                    history.push(Event::MessageSigned(sign_message_display));
                    for_transaction = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
                }
            },
        }
        TrDbCold::new()
            .set_history(events_to_batch::<Signer>(database_name, history)?)
            .set_transaction(for_transaction)
            .apply::<Signer>(database_name)?;
        let database = open_db::<Signer>(database_name)?;
        match database.checksum() {
            Ok(x) => Ok(x),
            Err(e) => Err(<Signer>::db_internal(e)),
        }
    }
}

/// Temporary storage for information needed to import derivations
/// Secret seed and seed name are required to approve
#[derive(Decode, Encode)]
#[cfg(feature = "signer")]
pub struct TrDbColdDerivations {
    checked_derivations: Vec<String>,
    network_specs: NetworkSpecs,
}

#[cfg(feature = "signer")]
impl TrDbColdDerivations {
    /// function to generate TrDbColdDerivations
    pub fn generate(checked_derivations: &[String], network_specs: &NetworkSpecs) -> Self {
        Self {
            checked_derivations: checked_derivations.to_owned(),
            network_specs: network_specs.to_owned(),
        }
    }
    /// function to recover TrDbColdDerivations from storage in the database
    pub fn from_storage(database_name: &str, checksum: u32) -> Result<Self, ErrorSigner> {
        let drv_encoded = {
            let database = open_db::<Signer>(database_name)?;
            verify_checksum(&database, checksum)?;
            let transaction = open_tree::<Signer>(&database, TRANSACTION)?;
            match transaction.get(DRV) {
                Ok(Some(a)) => a,
                Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Derivations)),
                Err(e) => return Err(<Signer>::db_internal(e)),
            }
        };
        match Self::decode(&mut &drv_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Derivations))),
        }
    }
    /// function to get checked derivations
    pub fn checked_derivations(&self) -> &Vec<String> {
        &self.checked_derivations
    }
    /// function to get network specs
    pub fn network_specs(&self) -> &NetworkSpecs {
        &self.network_specs
    }
    /// function to put TrDbColdDerivations into storage in the database
    pub fn store_and_get_checksum(&self, database_name: &str) -> Result<u32, ErrorSigner> {
        let mut transaction_batch = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
        transaction_batch.insert(DRV, self.encode());
        TrDbCold::new()
            .set_transaction(transaction_batch) // clear transaction tree and insert the stub
            .apply::<Signer>(database_name)?;
        let database = open_db::<Signer>(database_name)?;
        match database.checksum() {
            Ok(x) => Ok(x),
            Err(e) => Err(<Signer>::db_internal(e)),
        }
    }
}
