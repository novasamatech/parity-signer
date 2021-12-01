//! List of all modals

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Modal {
    Empty,
    Error,
    Message,
}

impl Modal {
    pub fn get_name(&self) -> String {
        match self {
            Modal::Error => String::from("error"),
            Modal::Message => String::from("message"),
            Modal::Empty => String::from("Empty"),
        }
    }
}
