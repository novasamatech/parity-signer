//! List of all alerts

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Alert {
    Empty,
    Error,
    ErrorDisplay, // for rare cases when the screen could not be displayed
    Shield,
}

impl Alert {
    pub fn get_name(&self) -> String {
        match self {
            Alert::Empty => String::from("Empty"),
            Alert::Error => String::from("Error"),
            Alert::ErrorDisplay => String::from("ErrorDisplay"),
            Alert::Shield => String::from("Shield"),
        }
    }
}
