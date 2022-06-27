//! List of all modals

use definitions::{keyring::NetworkSpecsKey, navigation::MSCContent};

#[derive(PartialEq, Debug, Clone)]
pub enum Modal {
    Backup(String),
    NewSeedMenu,
    NewSeedBackup(String),
    SeedMenu,
    NetworkSelector(NetworkSpecsKey),
    PasswordConfirm,
    EnterPassword,
    LogComment,
    LogRight,
    NetworkDetailsMenu,
    ManageMetadata(u32),
    SufficientCryptoReady((Vec<u8>, MSCContent)),
    KeyDetailsAction,
    TypesInfo,
    SelectSeed,
    Empty,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Backup(_) => String::from("Backup"),
            Modal::NewSeedMenu => String::from("NewSeedMenu"),
            Modal::NewSeedBackup(_) => String::from("NewSeedBackup"),
            Modal::SeedMenu => String::from("SeedMenu"),
            Modal::NetworkSelector(_) => String::from("NetworkSelector"),
            Modal::PasswordConfirm => String::from("PasswordConfirm"),
            Modal::EnterPassword => String::from("EnterPassword"),
            Modal::LogComment => String::from("LogComment"),
            Modal::LogRight => String::from("LogRight"),
            Modal::NetworkDetailsMenu => String::from("NetworkDetailsMenu"),
            Modal::ManageMetadata(_) => String::from("ManageMetadata"),
            Modal::SufficientCryptoReady(_) => String::from("SufficientCryptoReady"),
            Modal::KeyDetailsAction => String::from("KeyDetailsAction"),
            Modal::TypesInfo => String::from("TypesInfo"),
            Modal::SelectSeed => String::from("SelectSeed"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
