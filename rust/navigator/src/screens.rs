//!List of all screens

use db_handling::interface_signer::first_network;
use definitions::{error::ErrorSigner, keyring::{AddressKey, NetworkSpecsKey}};

///All screens
#[derive(PartialEq, Debug, Clone)]
pub enum Screen {
    Log,
    LogDetails,
    Scan,
    Transaction,
    SeedSelector,
    Keys(KeysState),
    KeyDetails(AddressState),
    NewSeed,
    RecoverSeedName,
    RecoverSeedPhrase,
    DeriveKey(AddressState),
    Settings,
    Verifier,
    ManageNetwork,
    Nowhere,
}

//TODO: store references instead of indices?
///State of keys screen
#[derive(PartialEq, Debug, Clone)]
pub struct KeysState {
    seed_name: String,
    network: NetworkSpecsKey,
}

///State of screen with 1 key
///
///More general KeysState could always be determined as subset of this
#[derive(PartialEq, Debug, Clone)]
pub struct AddressState {
    keys_state: KeysState,
    key: AddressKey, //TODO: actual key here
}

impl KeysState {
    pub fn new(seed_name: &str, database_name: &str) -> Result<Self, ErrorSigner> {
        let network_specs = first_network(database_name)?;
        Ok(Self {
            seed_name: seed_name.to_string(),
            network: NetworkSpecsKey::from_parts(&network_specs.genesis_hash.to_vec(), &network_specs.encryption),
        })
    }
    pub fn seed_name(&self) -> String {
        self.seed_name.to_owned()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.network.to_owned()
    }
}

impl AddressState {
    ///Do this to go up
    pub fn get_keys_state(&self) -> KeysState {
        self.keys_state.to_owned()
    }
    pub fn seed_name(&self) -> String {
        self.keys_state.seed_name()
    }
    pub fn network_specs_key(&self) -> NetworkSpecsKey {
        self.keys_state.network_specs_key()
    }
    pub fn address_key(&self) -> AddressKey {
        self.key.to_owned()
    }
}

impl Screen {
    ///Encode screen name into string for UI
    pub fn get_name(&self) -> Option<String> {
        match self {
            Screen::Log => Some(String::from("Log")),
            Screen::LogDetails => Some(String::from("LogDetails")),
            Screen::Scan => Some(String::from("Scan")),
            Screen::Transaction => Some(String::from("Transaction")),
            Screen::SeedSelector => Some(String::from("SeedSelector")),
            Screen::Keys(_) => Some(String::from("Keys")),
            Screen::KeyDetails(_) => Some(String::from("KeyDetails")),
            Screen::NewSeed => Some(String::from("NewSeed")),
            Screen::RecoverSeedName => Some(String::from("RecoverSeedName")),
            Screen::RecoverSeedPhrase => Some(String::from("RecoverSeedPhrase")),
            Screen::DeriveKey(_) => Some(String::from("DeriveKey")),
            Screen::Settings => Some(String::from("Settings")),
            Screen::Verifier => Some(String::from("Verifier")),
            Screen::ManageNetwork => Some(String::from("ManageNetwork")),
            Screen::Nowhere => None,
        }
    }

    pub fn get_default_label(&self) -> String {
        match self {
            Screen::Log => "Log",
            Screen::LogDetails => "Event details",
            Screen::Scan => "",
            Screen::Transaction => "",
            Screen::SeedSelector => "Select seed",
            Screen::Keys(_) => "",
            Screen::KeyDetails(_) => "Key",
            Screen::NewSeed => "",
            Screen::RecoverSeedName => "Recover Seed",
            Screen::RecoverSeedPhrase => "Recover Seed",
            Screen::DeriveKey(_) => "",
            Screen::Settings => "Settings",
            Screen::Verifier => "VERIFIER CERTIFICATE",
            Screen::ManageNetwork => "MANAGE NETWORKS",
            Screen::Nowhere => "",
        }.to_string()
    }

    pub fn has_back(&self) -> bool {
        match self {
            Screen::Log => false,
            Screen::LogDetails => true,
            Screen::Scan => false,
            Screen::Transaction => true,
            Screen::SeedSelector => false,
            Screen::Keys(_) => true,
            Screen::KeyDetails(_) => true,
            Screen::NewSeed => true,
            Screen::RecoverSeedName => true,
            Screen::RecoverSeedPhrase => true,
            Screen::DeriveKey(_) => true,
            Screen::Settings => false,
            Screen::Verifier => true,
            Screen::ManageNetwork => true,
            Screen::Nowhere => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
