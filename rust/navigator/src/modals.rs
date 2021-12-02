//! List of all modals

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Modal {
    Shield,
    NewSeedMenu,
    SeedMenu,
    Empty,
    Error,
    Message,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Shield => String::from("Shield"),
            Modal::NewSeedMenu => String::from("NewSeedMenu"),
            Modal::SeedMenu => String::from("SeedMenu"),
            Modal::Error => String::from("Error"),
            Modal::Message => String::from("Message"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
