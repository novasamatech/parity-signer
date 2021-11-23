//!List of all interactive actions in app

use super::screens::Screen;

///All actions
#[derive(PartialEq, Debug)]
pub enum Action {
    NavbarLog,
    NavbarScan,
    NavbarKeys,
    NavbarSettings,
    GoBack,
    Nothing,
}

impl Action {
    ///Decode action name string supplied from UI
    pub fn parse(input: &str) -> Action {
        match input {
            "NavbarLog" => Action::NavbarLog,
            "NavbarScan" => Action::NavbarScan,
            "NavbarKeys" => Action::NavbarKeys,
            "NavbarSettings" => Action::NavbarSettings,
            "GoBack" => Action::GoBack,
            _ => Action::Nothing,
        }
    }

    ///Decide what to do and do it!
    pub fn perform(self, details_str: &str) -> String {
        match self {
            //Simple navigation commands
            Action::NavbarLog => {
                "{\"screen\":\"Log\"}"
            },
            Action::NavbarScan => "{\"screen\":\"Scan\"}",
            Action::NavbarKeys => "{\"screen\":\"Keys\"}",
            Action::NavbarSettings => "{\"screen\":\"Settings\"}",
            Action::GoBack => {
                "{\"screen\":\"Log\"}"
            },
            Action::Nothing => {
                println!("no action was passed in action");
                ""
            },
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_some_actions() {
        assert_eq!(Action::parse("GoBack"), Action::GoBack);
        assert_eq!(Action::parse(""), Action::Nothing);
        assert_eq!(Action::parse("Booom!"), Action::Nothing);
    }
}
