//! Navigation state of the app

use crate::screens::Screen;
use crate::modals::Modal;
use crate::actions::Action;

use plot_icon;
use db_handling;
use transaction_parsing;
use transaction_signing;

///State of the app as remembered by backend
#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub navstate: Navstate,
    pub dbname: Option<String>,
    pub seed_names: Vec<String>,
}

///Navigation state is completely defined here
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Navstate {
    pub screen: Screen,
    pub modal: Modal,
}

impl State {
    ///Decide what to do and do it!
    pub fn perform(&mut self, action: Action, details_str: &str) -> String {
        let mut new_navstate = (*self).navstate;
        let mut new_screen = (*self).navstate.screen;
        let mut go_back_allowed = false;
        let mut screen_details = String::new();

        let mut errorline = String::new();
        if let Some(dbname) = &self.dbname {
           //Try to perform action
            match action {
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
                match self.navstate.screen {
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

        //Prepare screen details
        let screen_details = match new_navstate.screen {
            Screen::Log => {
                let log_pack = match db_handling::manage_history::print_history_page(0, &dbname) {
                    Ok(a) => {
                        format!("\"log\":{},", a)
                    },
                    Err(e) => {
                        new_navstate.modal = Modal::Error;
                        errorline.push_str(&e.to_string());
                        "".to_string()
                    },
                };
                format!("{}", log_pack)
            },
            /*
        	Screen::LogDetails => "",
        	Screen::Scan => "",
        	Screen::Transaction => "",
        	Screen::SeedSelector => "Select seed",
        	Screen::Keys => "",
        	Screen::KeyDetails => "Key",
        	Screen::Backup => "this should be popover",
        	Screen::NewSeed => "",
        	Screen::RecoverSeedName => "Recover Seed",
        	Screen::RecoverSeedPhrase => "Recover Seed",
        	Screen::DeriveKey => "",
        	Screen::Settings => "Settings",
        	Screen::Verifier => "VERIFIER CERTIFICATE",
        	Screen::ManageNetwork => "MANAGE NETWORKS",
            */
            Screen::Nowhere => "".to_string(),
            _ => "".to_string(),
        };

        //Prepare modal details
        let modal_details = match new_navstate.modal {
            Modal::Error => {
                format!("\"error\":\"{}\",", errorline)
            },
            _ => "".to_string(),
        };

        self.navstate = new_navstate;
        let mut output = format!("{}{}", screen_details, modal_details);
        output.pop();
        output
        } else {
            "\"error\":\"db not initialized\"".to_string()
        }
    }


    ///This converts state into renderable block
    pub fn generate_json(&self, details: String) -> String {
        let mut output = String::from("{");
        let screen = self.navstate.screen;
        let modal = self.navstate.modal;
        if let Some(screen_name) = screen.get_name() {
            output.push_str(&format!("\"screen\":\"{}\",\"screenLabel\":\"{}\",\"back\":{},\"footer\":{},\"footerButton\":\"{}\",", screen_name, self.get_screen_label(), false, true, self.get_active_navbutton()));
        }
        output.push_str(&format!("\"modal\":\"{}\",", modal.get_name()));
        output.push_str(&format!("\"content\":{{{}}}", details));
        output.push_str("}");
        output
    }

    ///Generate screen label taking into account state
    fn get_screen_label(&self) -> String {
        self.navstate.screen.get_default_label()
    }

    ///Decide which footer button should shine
    fn get_active_navbutton(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "Log",
        	Screen::LogDetails => "Log",
        	Screen::Scan => "Scan",
        	Screen::Transaction => "Scan",
        	Screen::SeedSelector => "Keys",
        	Screen::Keys => "Keys",
        	Screen::KeyDetails => "Keys",
        	Screen::Backup => "Keys",
        	Screen::NewSeed => "Keys",
        	Screen::RecoverSeedName => "Keys",
        	Screen::RecoverSeedPhrase => "Keys",
        	Screen::DeriveKey => "Keys",
        	Screen::Settings => "Settings",
        	Screen::Verifier => "Srttings",
        	Screen::ManageNetwork => "Settings",
            Screen::Nowhere => "None",
        }.to_string()
    }
}
