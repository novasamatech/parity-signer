//!List of all interactive actions in app

use super::screens::Screen;
use crate::navstate::Navstate;

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
    pub fn perform(self, navstate: Navstate, details_str: &str) -> Navstate {
        let mut new_navstate = navstate;
        let mut new_screen = navstate.screen;
        let mut go_back_allowed = false;
        match self {
            //Simple navigation commands
            Action::NavbarLog => {
                new_navstate.screen = Screen::Log;
            },
            Action::NavbarScan => {
                new_navstate.screen = Screen::Scan;
            },
            Action::NavbarKeys => {
                new_navstate.screen = Screen::SeedSelector;
            },
            Action::NavbarSettings => {
                new_navstate.screen = Screen::Settings;
            },

            //General back action is defined here
            Action::GoBack => {
                match navstate.screen {
                    Screen::LogDetails => {
                        new_navstate.screen = Screen::Log;
                    },
                    Screen::Transaction => {
                        new_navstate.screen = Screen::Scan;
                    },
                    Screen::Keys => {
                        new_navstate.screen = Screen::SeedSelector;
                    },
                    Screen::KeyDetails => {
                        new_navstate.screen = Screen::Keys;
                    },
                    Screen::NewSeed => {
                        new_navstate.screen = Screen::SeedSelector;
                    },
                    Screen::RecoverSeedName => {
                        new_navstate.screen = Screen::SeedSelector;
                    },
                    Screen::RecoverSeedPhrase => {
                        new_navstate.screen = Screen::RecoverSeedName;
                    },
                    Screen::DeriveKey => {
                        new_navstate.screen = Screen::Keys;
                    },
                    Screen::Verifier => {
                        new_navstate.screen = Screen::Settings;
                    },
                    Screen::ManageNetwork => {
                        new_navstate.screen = Screen::Settings;
                    },
                    _ => {
                        println!("Back button pressed at the bottom of navigation");
                    },
                };
            },
            Action::Nothing => {
                println!("no action was passed in action");
            },
        };
        new_navstate
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
