//! List of all alerts

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Alert {
    Empty,
    Error,
    Shield,
}

impl Alert {
    pub fn get_name(&self) -> String {
        match self {
            Alert::Empty => String::from("Empty"),
            Alert::Error => String::from("Error"),
            Alert::Shield => String::from("Shield"),
        }
    }
}
