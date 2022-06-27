//!List of all screens
use sp_runtime::MultiSigner;
use zeroize::Zeroize;

use db_handling::{
    helpers::get_address_details,
    interface_signer::{first_network, SeedDraft},
};
use definitions::{
    error::AddressKeySource,
    error_signer::{ErrorSigner, ExtraAddressKeySourceSigner, Signer},
    helpers::{make_identicon_from_multisigner, multisigner_to_public},
    keyring::{AddressKey, NetworkSpecsKey},
    navigation::{Address, MSignatureReady, TransactionCardSet},
    network_specs::NetworkSpecs,
    users::AddressDetails,
};
use transaction_parsing;
use transaction_signing;

const MAX_COUNT_SET: u8 = 3;

///All screens
#[derive(Clone, Debug)]
pub enum Screen {
    Log,
    LogDetails(u32),
    Scan,
    Transaction(Box<TransactionState>),
    SignatureReady(MSignatureReady),
    SeedSelector,
    Keys(KeysState),
    KeyDetails(AddressState),
    KeyDetailsMulti(AddressStateMulti),
    NewSeed,
    RecoverSeedName(String),
    RecoverSeedPhrase(RecoverSeedPhraseState),
    DeriveKey(DeriveState),
    Settings,
    Verifier,
    ManageNetworks,
    NetworkDetails(NetworkSpecsKey),
    SelectSeedForBackup,
    SignSufficientCrypto(SufficientCryptoState),
    Documents,
    Nowhere,
}

#[derive(Debug, Clone)]
pub enum SpecialtyKeysState {
    None,
    Swiped(MultiSigner),
    MultiSelect(Vec<MultiSigner>),
}

///State of keys screen
#[derive(Debug, Clone)]
pub struct KeysState {
    seed_name: String,
    network_specs_key: NetworkSpecsKey,
    specialty: SpecialtyKeysState,
}

///State of screen with 1 key
#[derive(Debug, Clone)]
pub struct AddressState {
    seed_name: String,
    network_specs_key: NetworkSpecsKey,
    multisigner: MultiSigner,
    is_root: bool,
}

///State of screen with multiple keys
#[derive(Debug, Clone)]
pub struct AddressStateMulti {
    seed_name: String,
    network_specs_key: NetworkSpecsKey,
    selected: usize,
    set: Vec<(MultiSigner, bool)>,
}

///State of derive key screen
#[derive(Debug, Clone)]
pub struct DeriveState {
    entered_info: EnteredInfo,
    keys_state: KeysState,
    collision: Option<(MultiSigner, AddressDetails)>,
}

///State of transaction screen
#[derive(Clone, Debug)]
pub struct TransactionState {
    entered_info: EnteredInfo,
    action: transaction_parsing::TransactionAction,
    comment: String,
    counter: u8,
}

///State of screen generating sufficient crypto
#[derive(Debug, Clone)]
pub struct SufficientCryptoState {
    key_selected: Option<(MultiSigner, AddressDetails, Address)>,
    entered_info: EnteredInfo,
    content: transaction_signing::SufficientContent,
    counter: u8,
}

///State of screen recover seed phrase
#[derive(Debug, Clone)]
pub struct RecoverSeedPhraseState {
    seed_name: String,
    seed_draft: SeedDraft,
}

///EnteredInfo, path+pwd entered by the user, zeroizeable
#[derive(Debug, Clone, Zeroize)]
#[zeroize(drop)]
pub struct EnteredInfo(pub String);

impl KeysState {
    pub fn new(seed_name: &str, database_name: &str) -> Result<Self, ErrorSigner> {
        let network_specs = first_network(database_name)?;
        Ok(Self {
            seed_name: seed_name.to_string(),
            network_specs_key: NetworkSpecsKey::from_parts(
                &network_specs.genesis_hash,
                &network_specs.encryption,
            ),
            specialty: SpecialtyKeysState::None,
        })
    }
    pub fn new_in_network(seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Self {
        Self {
            seed_name: seed_name.to_string(),
            network_specs_key: network_specs_key.to_owned(),
            specialty: SpecialtyKeysState::None,
        }
    }
    pub fn change_network(&self, network_specs_key: &NetworkSpecsKey) -> Self {
        Self {
            seed_name: self.seed_name(),
            network_specs_key: network_specs_key.to_owned(),
            specialty: SpecialtyKeysState::None,
        }
    }
    pub fn seed_name(&self) -> String {
        self.seed_name.to_owned()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.network_specs_key.to_owned()
    }
    pub fn get_specialty(&self) -> SpecialtyKeysState {
        self.specialty.to_owned()
    }
    pub fn swipe(&self, multisigner: &MultiSigner) -> Self {
        let specialty = match &self.specialty {
            SpecialtyKeysState::None => SpecialtyKeysState::Swiped(multisigner.to_owned()),
            SpecialtyKeysState::Swiped(swiped_multisigner) => {
                if swiped_multisigner == multisigner {
                    SpecialtyKeysState::None
                } else {
                    SpecialtyKeysState::Swiped(multisigner.to_owned())
                }
            }
            SpecialtyKeysState::MultiSelect(_) => self.specialty.to_owned(),
        };
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty,
        }
    }
    pub fn select_single(&self, multisigner: &MultiSigner) -> Self {
        let specialty = match &self.specialty {
            SpecialtyKeysState::None => {
                SpecialtyKeysState::MultiSelect(vec![multisigner.to_owned()])
            }
            SpecialtyKeysState::Swiped(_) => self.specialty.to_owned(),
            SpecialtyKeysState::MultiSelect(multiselect) => {
                let mut new_multiselect = multiselect.to_owned();
                if multiselect.contains(multisigner) {
                    new_multiselect = new_multiselect
                        .into_iter()
                        .filter(|a| a != multisigner)
                        .collect();
                } else {
                    new_multiselect.push(multisigner.to_owned());
                }
                SpecialtyKeysState::MultiSelect(new_multiselect)
            }
        };
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty,
        }
    }
    pub fn select_set(&self, set: Vec<MultiSigner>) -> Self {
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty: SpecialtyKeysState::MultiSelect(set),
        }
    }
    pub fn get_swiped_key(&self) -> Option<MultiSigner> {
        if let SpecialtyKeysState::Swiped(ref multisigner) = self.specialty {
            Some(multisigner.to_owned())
        } else {
            None
        }
    }
    pub fn get_multiselect_keys(&self) -> Vec<MultiSigner> {
        if let SpecialtyKeysState::MultiSelect(ref multiselect) = self.specialty {
            multiselect.to_vec()
        } else {
            Vec::new()
        }
    }
    pub fn is_multiselect(&self) -> bool {
        matches!(self.specialty, SpecialtyKeysState::MultiSelect(_))
    }
    pub fn deselect_specialty(&self) -> Self {
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty: SpecialtyKeysState::None,
        }
    }
}

impl AddressState {
    pub fn new(
        hex_address_key: &str,
        keys_state: &KeysState,
        database_name: &str,
    ) -> Result<Self, ErrorSigner> {
        let address_key = AddressKey::from_hex(hex_address_key)?;
        let multisigner = address_key.multi_signer::<Signer>(AddressKeySource::Extra(
            ExtraAddressKeySourceSigner::Interface,
        ))?;
        let is_root =
            get_address_details(database_name, &AddressKey::from_multisigner(&multisigner))?
                .is_root();
        Ok(Self {
            seed_name: keys_state.seed_name(),
            network_specs_key: keys_state.network_specs_key(),
            multisigner,
            is_root,
        })
    }
    pub fn blank_keys_state(&self) -> KeysState {
        KeysState {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty: SpecialtyKeysState::None,
        }
    }
    pub fn seed_name(&self) -> String {
        self.seed_name.to_owned()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.network_specs_key.to_owned()
    }
    pub fn multisigner(&self) -> MultiSigner {
        self.multisigner.to_owned()
    }
    pub fn is_root(&self) -> bool {
        self.is_root
    }
}

impl AddressStateMulti {
    pub fn new(
        seed_name: String,
        network_specs_key: NetworkSpecsKey,
        multiselect: &[MultiSigner],
        database_name: &str,
    ) -> Result<Self, ErrorSigner> {
        let mut set: Vec<(MultiSigner, bool)> = Vec::new();
        for multisigner in multiselect.iter() {
            let address_details =
                get_address_details(database_name, &AddressKey::from_multisigner(multisigner))?;
            set.push((multisigner.to_owned(), address_details.is_root()))
        }
        Ok(Self {
            seed_name,
            network_specs_key,
            selected: 0,
            set,
        })
    }
    pub fn blank_keys_state(&self) -> KeysState {
        KeysState {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            specialty: SpecialtyKeysState::None,
        }
    }
    pub fn seed_name(&self) -> String {
        self.seed_name.to_owned()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.network_specs_key.to_owned()
    }
    pub fn multisigner(&self) -> MultiSigner {
        self.set[self.selected].0.to_owned()
    }
    pub fn set(&self) -> Vec<(MultiSigner, bool)> {
        self.set.to_owned()
    }
    pub fn next(&self) -> Self {
        let selected = {
            if self.selected + 1 == self.set.len() {
                0
            } else {
                self.selected + 1
            }
        };
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            selected,
            set: self.set(),
        }
    }
    pub fn previous(&self) -> Self {
        let selected = {
            if self.selected == 0 {
                self.set.len() - 1
            } else {
                self.selected - 1
            }
        };
        Self {
            seed_name: self.seed_name(),
            network_specs_key: self.network_specs_key(),
            selected,
            set: self.set(),
        }
    }
    pub fn number(&self) -> usize {
        self.selected + 1
    }
    pub fn out_of(&self) -> usize {
        self.set.len()
    }
    pub fn is_root(&self) -> bool {
        self.set[self.selected].1
    }
}

impl DeriveState {
    pub fn new(
        entered_string: &str,
        keys_state: &KeysState,
        collision: Option<(MultiSigner, AddressDetails)>,
    ) -> Self {
        Self {
            entered_info: EnteredInfo(entered_string.to_string()),
            keys_state: keys_state.to_owned(),
            collision,
        }
    }
    pub fn blank_keys_state(&self) -> KeysState {
        KeysState {
            seed_name: self.keys_state.seed_name(),
            network_specs_key: self.keys_state.network_specs_key(),
            specialty: SpecialtyKeysState::None,
        }
    }
    pub fn seed_name(&self) -> String {
        self.keys_state.seed_name()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.keys_state.network_specs_key()
    }
    pub fn path(&self) -> String {
        self.entered_info.0.to_owned()
    }
    pub fn collision(&self) -> Option<(MultiSigner, AddressDetails)> {
        self.collision.to_owned()
    }
    pub fn update(&self, new_secret_string: &str) -> Self {
        Self {
            entered_info: EnteredInfo(new_secret_string.to_string()),
            keys_state: self.blank_keys_state(),
            collision: self.collision(),
        }
    }
    pub fn collided_with(
        &self,
        multisigner: &MultiSigner,
        address_details: &AddressDetails,
    ) -> Self {
        Self {
            entered_info: self.entered_info.to_owned(),
            keys_state: self.blank_keys_state(),
            collision: Some((multisigner.to_owned(), address_details.to_owned())),
        }
    }
}

impl TransactionState {
    pub fn new(details_str: &str, dbname: &str) -> Self {
        Self {
            entered_info: EnteredInfo("".to_string()),
            action: transaction_parsing::produce_output(details_str, dbname),
            comment: "".to_string(),
            counter: 1,
        }
    }
    pub fn update_seed(&self, new_secret_string: &str) -> Self {
        Self {
            entered_info: EnteredInfo(new_secret_string.to_string()),
            action: self.action(),
            comment: self.comment.to_string(),
            counter: self.counter,
        }
    }
    pub fn add_comment(&self, comment: &str) -> Self {
        Self {
            entered_info: self.entered_info.to_owned(),
            action: self.action(),
            comment: comment.to_string(),
            counter: self.counter,
        }
    }
    pub fn update_checksum_sign(
        &self,
        new_checksum: u32,
        content: TransactionCardSet,
        has_pwd: bool,
        author_info: Address,
        network_info: NetworkSpecs,
    ) -> Self {
        let action = transaction_parsing::TransactionAction::Sign {
            content,
            checksum: new_checksum,
            has_pwd,
            author_info,
            network_info,
        };
        Self {
            entered_info: self.entered_info.to_owned(),
            action,
            comment: self.comment.to_string(),
            counter: self.counter + 1,
        }
    }
    pub fn action(&self) -> transaction_parsing::TransactionAction {
        self.action.to_owned()
    }
    pub fn seed(&self) -> String {
        self.entered_info.0.to_string()
    }
    pub fn get_comment(&self) -> String {
        self.comment.to_owned()
    }
    pub fn ok(&self) -> bool {
        self.counter < MAX_COUNT_SET
    }
    pub fn counter(&self) -> u8 {
        self.counter
    }
}

impl SufficientCryptoState {
    pub fn init(content: transaction_signing::SufficientContent) -> Self {
        Self {
            key_selected: None,
            entered_info: EnteredInfo("".to_string()),
            content,
            counter: 1,
        }
    }
    pub fn content(&self) -> transaction_signing::SufficientContent {
        self.content.to_owned()
    }
    pub fn key_selected(&self) -> Option<(MultiSigner, AddressDetails, Address)> {
        self.key_selected.to_owned()
    }
    pub fn update(
        &self,
        multisigner: &MultiSigner,
        address_details: &AddressDetails,
        new_secret_string: &str,
    ) -> Self {
        let identicon = make_identicon_from_multisigner(multisigner);
        let author_info = Address {
            base58: hex::encode(multisigner_to_public(multisigner)),
            identicon,
            seed_name: address_details.seed_name.clone(),
            path: address_details.path.clone(),
            has_pwd: address_details.has_pwd,
            multiselect: None,
        };
        Self {
            key_selected: Some((
                multisigner.to_owned(),
                address_details.to_owned(),
                author_info,
            )),
            entered_info: EnteredInfo(new_secret_string.to_string()),
            content: self.content(),
            counter: self.counter,
        }
    }
    pub fn seed(&self) -> String {
        self.entered_info.0.to_string()
    }
    pub fn plus_one(&self) -> Self {
        Self {
            key_selected: self.key_selected(),
            entered_info: self.entered_info.to_owned(),
            content: self.content(),
            counter: self.counter + 1,
        }
    }
    pub fn ok(&self) -> bool {
        self.counter < MAX_COUNT_SET
    }
    pub fn counter(&self) -> u8 {
        self.counter
    }
}

impl RecoverSeedPhraseState {
    pub fn new(seed_name: &str) -> Self {
        Self {
            seed_name: seed_name.to_string(),
            seed_draft: SeedDraft::initiate(),
        }
    }
    pub fn name(&self) -> String {
        self.seed_name.to_string()
    }
    pub fn draft(&self) -> &SeedDraft {
        &self.seed_draft
    }
    pub fn text_entry(&mut self, details_str: &str) {
        self.seed_draft.text_field_update(details_str)
    }
    pub fn push_word(&mut self, details_str: &str) {
        self.seed_draft.added(details_str, None);
    }
}

impl Screen {
    ///Encode screen name into string for UI
    pub fn get_name(&self) -> Option<String> {
        match self {
            Screen::Log => Some(String::from("Log")),
            Screen::LogDetails(_) => Some(String::from("LogDetails")),
            Screen::Scan => Some(String::from("Scan")),
            Screen::Transaction(_) => Some(String::from("Transaction")),
            Screen::SignatureReady(_) => Some(String::from("Signature ready")),
            Screen::SeedSelector => Some(String::from("SeedSelector")),
            Screen::Keys(_) => Some(String::from("Keys")),
            Screen::KeyDetails(_) => Some(String::from("KeyDetails")),
            Screen::KeyDetailsMulti(_) => Some(String::from("KeyDetailsMultiSelect")),
            Screen::NewSeed => Some(String::from("NewSeed")),
            Screen::RecoverSeedName(_) => Some(String::from("RecoverSeedName")),
            Screen::RecoverSeedPhrase(_) => Some(String::from("RecoverSeedPhrase")),
            Screen::DeriveKey(_) => Some(String::from("DeriveKey")),
            Screen::Settings => Some(String::from("Settings")),
            Screen::Verifier => Some(String::from("Verifier")),
            Screen::ManageNetworks => Some(String::from("ManageNetworks")),
            Screen::NetworkDetails(_) => Some(String::from("NetworkDetails")),
            Screen::SelectSeedForBackup => Some(String::from("SelectSeedForBackup")),
            Screen::SignSufficientCrypto(_) => Some(String::from("SignSufficientCrypto")),
            Screen::Documents => Some(String::from("Documents")),
            Screen::Nowhere => None,
        }
    }

    pub fn get_default_label(&self) -> String {
        match self {
            Screen::Log => "",
            Screen::LogDetails(_) => "Event details",
            Screen::Scan => "",
            Screen::Transaction(_) => "",
            Screen::SignatureReady(_) => "",
            Screen::SeedSelector => "Select seed",
            Screen::Keys(_) => "",
            Screen::KeyDetails(_) => "Key",
            Screen::KeyDetailsMulti(_) => "Key",
            Screen::NewSeed => "New Seed",
            Screen::RecoverSeedName(_) => "Recover Seed",
            Screen::RecoverSeedPhrase(_) => "Recover Seed",
            Screen::DeriveKey(_) => "Derive Key",
            Screen::Settings => "",
            Screen::Verifier => "VERIFIER CERTIFICATE",
            Screen::ManageNetworks => "MANAGE NETWORKS",
            Screen::NetworkDetails(_) => "Network details",
            Screen::SelectSeedForBackup => "Select seed",
            Screen::SignSufficientCrypto(_) => "Sign SufficientCrypto",
            Screen::Documents => "ABOUT",
            Screen::Nowhere => "",
        }
        .to_string()
    }

    pub fn has_back(&self) -> bool {
        match self {
            Screen::Log
            | Screen::Scan
            | Screen::Settings
            | Screen::SeedSelector
            | Screen::Nowhere => false,
            Screen::Transaction(_)
            | Screen::SignatureReady(_)
            | Screen::LogDetails(_)
            | Screen::Keys(_)
            | Screen::KeyDetails(_)
            | Screen::KeyDetailsMulti(_)
            | Screen::NewSeed
            | Screen::RecoverSeedName(_)
            | Screen::RecoverSeedPhrase(_)
            | Screen::DeriveKey(_)
            | Screen::Verifier
            | Screen::ManageNetworks
            | Screen::NetworkDetails(_)
            | Screen::SelectSeedForBackup
            | Screen::SignSufficientCrypto(_)
            | Screen::Documents => true,
        }
    }
}

#[cfg(test)]
mod tests {

    use sp_core::sr25519::Public;

    use super::*;

    use definitions::{crypto::Encryption, users::AddressDetails};

    const PUBLIC: [u8; 32] = [
        142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147,
        201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
    ];
    fn test_address_details() -> AddressDetails {
        AddressDetails {
            seed_name: String::from("Alice"),
            path: String::from("//alice"),
            has_pwd: false,
            network_id: Vec::new(),
            encryption: Encryption::Sr25519,
        }
    }

    /// EnteredInfo holds String with sensitive information arriving either from Signer phone memory or from the user.
    /// EnteredInto is used in several structs, and should always be zeroized properly.
    #[test]
    fn zeroize_entered_info_in_sufficient_crypto_state() {
        let secret_ptr: *const u8;
        // using secret entered info in this scope, zeroize afterwards
        {
            let mock_seed = "super secret seed element";
            let sufficient_crypto_test =
                SufficientCryptoState::init(transaction_signing::SufficientContent::LoadTypes);
            let sufficient_crypto_test_updated = sufficient_crypto_test.update(
                &MultiSigner::Sr25519(Public::from_raw(PUBLIC)),
                &test_address_details(),
                mock_seed,
            );
            secret_ptr = unsafe {
                sufficient_crypto_test_updated
                    .entered_info
                    .0
                    .as_ptr()
                    .offset(-100)
            };
        }
        println!("zeroize should have happened");
        let memory: &[u8] = unsafe { std::slice::from_raw_parts(secret_ptr, 300) };
        let memory_string = String::from_utf8_lossy(memory);
        assert!(
            !memory_string.contains("super secret seed element"),
            "\n{:?}",
            memory_string
        );
    }
}
