//!List of all screens

///All screens
#[derive(PartialEq, Debug)]
pub enum Screen {
    Scan,
    Keys,
    Settings,
    Log,
    Nowhere,
}

impl Screen {
    ///Decode screen name string supplied from UI
    pub fn parse(input: &str) -> Screen {
        match input {
            "Scan" => Screen::Scan,
            "Keys" => Screen::Keys,
            "Settings" => Screen::Settings,
            "Log" => Screen::Log,
            _ => Screen::Nowhere,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_some_screens() {
        assert_eq!(Screen::parse("Log"), Screen::Log);
        assert_eq!(Screen::parse(""), Screen::Nowhere);
        assert_eq!(Screen::parse("Sea of thought"), Screen::Nowhere);
    }
}
