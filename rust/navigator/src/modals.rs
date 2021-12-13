//! List of all modals

use definitions::keyring::NetworkSpecsKey;

#[derive(PartialEq, Debug, Clone)]
pub enum Modal {
    Backup(String),
    NewSeedMenu,
    SeedMenu,
    NetworkSelector(NetworkSpecsKey), 
    PasswordConfirm(String),
    Empty,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Backup(_) => String::from("Backup"),
            Modal::NewSeedMenu => String::from("NewSeedMenu"),
            Modal::SeedMenu => String::from("SeedMenu"),
            Modal::NetworkSelector(_) => String::from("NetworkSelector"),
            Modal::PasswordConfirm(_) => String::from("PasswordConfirm"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
