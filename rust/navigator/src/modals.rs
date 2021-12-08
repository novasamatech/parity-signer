//! List of all modals

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Modal {
    Backup,
    NewSeedMenu,
    SeedMenu,
    NetworkSelector,
    Empty,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Backup => String::from("Backup"),
            Modal::NewSeedMenu => String::from("NewSeedMenu"),
            Modal::SeedMenu => String::from("SeedMenu"),
            Modal::NetworkSelector => String::from("NetworkSelector"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
