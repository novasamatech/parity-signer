//! List of all modals

use definitions::keyring::NetworkSpecsKey;

#[derive(PartialEq, Debug, Clone)]
pub enum Modal {
    Backup(String),
    NewSeedMenu,
    SeedMenu,
    NetworkSelector(NetworkSpecsKey), 
    PasswordConfirm,
    SignatureReady(String),
    EnterPassword,
    LogRight,
    NetworkDetailsMenu,
    ManageMetadata(u32),
    Empty,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Backup(_) => String::from("Backup"),
            Modal::NewSeedMenu => String::from("NewSeedMenu"),
            Modal::SeedMenu => String::from("SeedMenu"),
            Modal::NetworkSelector(_) => String::from("NetworkSelector"),
            Modal::PasswordConfirm => String::from("PasswordConfirm"),
            Modal::SignatureReady(_) => String::from("SignatureReady"),
            Modal::EnterPassword => String::from("EnterPassword"),
            Modal::LogRight => String::from("LogRight"),
            Modal::NetworkDetailsMenu => String::from("NetworkDetailsMenu"),
            Modal::ManageMetadata(_) => String::from("ManageMetadata"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
