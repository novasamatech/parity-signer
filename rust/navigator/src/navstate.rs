//! Navigation state of the app

use hex;

use crate::screens::{Screen, KeysState};
use crate::modals::Modal;
use crate::actions::Action;

use plot_icon;
use db_handling;
use definitions::keyring::NetworkSpecsKey;
use transaction_parsing;
use transaction_signing;

///State of the app as remembered by backend
#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub navstate: Navstate,
    pub dbname: Option<String>,
    pub seed_names: Vec<String>,
    pub networks: Vec<NetworkSpecsKey>,
}

///Navigation state is completely defined here
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Navstate {
    pub screen: Screen,
    pub modal: Modal,
}

impl Navstate {
    pub fn new() -> Navstate {
        Navstate {
            screen: Screen::Log,
            modal: Modal::Empty,
        }
    }
}

impl State {
    ///Decide what to do and do it!
    pub fn perform(&mut self, action: Action, details_str: &str) -> String {
        let mut new_navstate = (*self).navstate;
        let mut new_screen = (*self).navstate.screen;
        let mut seed_names = &(*self).seed_names;
        let mut go_back_allowed = false;
        let mut screen_details = String::new();

        let mut errorline = String::new();
        if let Some(dbname) = &self.dbname {
           //Try to perform action
            match action {
                //App init
                Action::Start => {
                    if seed_names.len() == 0 {
                        new_navstate.screen = Screen::SeedSelector;
                        new_navstate.modal = Modal::NewSeedMenu;
                    } else {
                        new_navstate.screen = Screen::Log;
                        new_navstate.modal = Modal::Empty;
                    }
                },

                //Simple navigation commands
                Action::NavbarLog => {
                    new_navstate.screen = Screen::Log;
                    new_navstate.modal = Modal::Empty;
                },
                Action::NavbarScan => {
                    new_navstate.screen = Screen::Scan;
                    new_navstate.modal = Modal::Empty;
                },
                Action::NavbarKeys => {
                    new_navstate.screen = Screen::SeedSelector;
                    new_navstate.modal = Modal::Empty;
                },
                Action::NavbarSettings => {
                    new_navstate.screen = Screen::Settings;
                    new_navstate.modal = Modal::Empty;
                },

                //General back action is defined here
                Action::GoBack => {
                    if self.navstate.modal == Modal::Empty {
                        match self.navstate.screen {
                            Screen::LogDetails => {
                                new_navstate.screen = Screen::Log;
                            },
                            Screen::Transaction => {
                                new_navstate.screen = Screen::Scan;
                            },
                            Screen::Keys(_) => {
                                new_navstate.screen = Screen::SeedSelector;
                            },
                            Screen::KeyDetails(a) => {
                                new_navstate.screen = Screen::Keys(a.get_keys_state());
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
                            Screen::DeriveKey(a) => {
                                new_navstate.screen = Screen::Keys(a.get_keys_state());
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
                    } else {
                        new_navstate.modal = Modal::Empty;
                    }
            },
            Action::SelectSeed => {
                match seed_names.binary_search(&details_str.to_string()) {
                    Ok(index) => {
                        new_navstate.screen = Screen::Keys(KeysState::new(index));
                        new_navstate.modal = Modal::Empty;
                    },
                    Err(e) => {
                        new_navstate.modal = Modal::Error;
                        errorline.push_str(&e.to_string());
                    },
                }
            },
            Action::RightButton => {
                match self.navstate.screen {
                    Screen::SeedSelector => new_navstate.modal = Modal::NewSeedMenu,
                    Screen::Keys(a) => new_navstate.modal = Modal::SeedMenu,
                    _ => {},
                }
            },
            Action::Shield => {
                new_navstate.modal = Modal::Shield;
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
        	//Screen::LogDetails => "",
        	Screen::Scan => "".to_string(),
        	//Screen::Transaction => "",
        	Screen::SeedSelector => {
                //TODO: seed selector cards
                let mut seed_pack = self.seed_names.join("\",\"");
                format!("\"seedNameCards\":[\"{}\"],", seed_pack)
            },
        	Screen::Keys(keystate) => {
                //TODO: separate seed key
                if let Some(seed_name) = self.seed_names.get(keystate.seed_name) {
                    if let Some(network_key) = self.networks.get(keystate.network) {
                        let keys_pack = match db_handling::identities::print_relevant_identities(seed_name, &hex::encode(network_key.key()), dbname) {
                            Ok(a) => a,
                            Err(e) => {
                                new_navstate.modal = Modal::Error;
                                errorline.push_str(&e.to_string());
                                "".to_string()
                            },
                        };
                        format!("\"keys\":{},\"seed\":\"{}\",", keys_pack, seed_name)
                    } else {
                        new_navstate.modal = Modal::Error;
                        errorline.push_str("Network was lost, report a bug");
                        "".to_string()
                    }
                } else {
                    new_navstate.modal = Modal::Error;
                    errorline.push_str("Seed was lost, report a bug");
                    "".to_string()
                } 
            },
        	//Screen::KeyDetails => "Key",
        	//Screen::Backup => "this should be popover",
        	//Screen::NewSeed => "",
        	//Screen::RecoverSeedName => "Recover Seed",
        	//Screen::RecoverSeedPhrase => "Recover Seed",
        	//Screen::DeriveKey => "",
        	//Screen::Settings => "Settings",
        	//Screen::Verifier => "VERIFIER CERTIFICATE",
        	//Screen::ManageNetwork => "MANAGE NETWORKS",
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
        let mut output = String::new();
        if new_navstate.modal == Modal::Error {
            self.navstate.modal = Modal::Error;
            output = format!("{}{}", screen_details, modal_details);
        } else {
            self.navstate = new_navstate;
            output = format!("{}{}", screen_details, modal_details);
        }
        output.pop();
        output
        } else {
            "\"error\":\"db not initialized\"".to_string()
        }
    }


    ///This converts state into renderable block
    //TODO: clean this up
    pub fn generate_json(&self, details: String) -> String {
        let mut output = String::from("{");
        let screen = self.navstate.screen;
        let modal = self.navstate.modal;
        if let Some(screen_name) = screen.get_name() {
            output.push_str(&format!("\"screen\":\"{}\",\"screenLabel\":\"{}\",\"back\":{},\"footer\":{},\"footerButton\":\"{}\",\"rightButton\":\"{}\",\"screenNameType\":\"{}\",", screen_name, self.get_screen_label(), screen.has_back(), true, self.get_active_navbutton(), self.get_right_button(), self.get_screen_name_type()));
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
        	Screen::Keys(_) => "Keys",
        	Screen::KeyDetails(_) => "Keys",
        	Screen::NewSeed => "Keys",
        	Screen::RecoverSeedName => "Keys",
        	Screen::RecoverSeedPhrase => "Keys",
        	Screen::DeriveKey(_) => "Keys",
        	Screen::Settings => "Settings",
        	Screen::Verifier => "Srttings",
        	Screen::ManageNetwork => "Settings",
            Screen::Nowhere => "None",
        }.to_string()
    }

    ///Should header have some button on the right?
    fn get_right_button(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "None",
        	Screen::LogDetails => "None",
        	Screen::Scan => "None",
        	Screen::Transaction => "None",
        	Screen::SeedSelector => "NewSeed",
        	Screen::Keys(_) => "Backup",
        	Screen::KeyDetails(_) => "None",
        	Screen::NewSeed => "None",
        	Screen::RecoverSeedName => "None",
        	Screen::RecoverSeedPhrase => "None",
        	Screen::DeriveKey(_) => "None",
        	Screen::Settings => "None",
        	Screen::Verifier => "None",
        	Screen::ManageNetwork => "None",
            Screen::Nowhere => "None",
        }.to_string()
    }

    ///Determine whether screen name should be h1 or h4
    fn get_screen_name_type(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "h4",
        	Screen::LogDetails => "h4",
        	Screen::Scan => "h1",
        	Screen::Transaction => "h1",
        	Screen::SeedSelector => "h4",
        	Screen::Keys(_) => "h4",
        	Screen::KeyDetails(_) => "h4",
        	Screen::NewSeed => "h1",
        	Screen::RecoverSeedName => "h1",
        	Screen::RecoverSeedPhrase => "h1",
        	Screen::DeriveKey(_) => "h1",
        	Screen::Settings => "h4",
        	Screen::Verifier => "h4",
        	Screen::ManageNetwork => "h4",
            Screen::Nowhere => "h4",
        }.to_string()

    }
}

//TODO: tests should probably be performed here, as static object in lib.rs
//will only allow for 1-2 integration tests
