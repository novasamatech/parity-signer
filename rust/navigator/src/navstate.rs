//! Navigation state of the app

//use hex;
use zeroize::Zeroize;

use crate::screens::{AddressState, DeriveState, KeysState, Screen, TransactionState};
use crate::modals::Modal;
use crate::actions::Action;
use crate::alerts::Alert;

//use plot_icon;
use db_handling;
use definitions::{error::{ErrorSource, Signer}, keyring::NetworkSpecsKey};
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
#[derive(PartialEq, Debug, Clone)]
pub struct Navstate {
    pub screen: Screen,
    pub modal: Modal,
    pub alert: Alert,
}

impl Navstate {
    pub fn new() -> Navstate {
        Navstate {
            screen: Screen::Log,
            modal: Modal::Empty,
            alert: Alert::Empty,
        }
    }
}

impl State {
    ///Decide what to do and do it!
    pub fn perform(&mut self, action: Action, details_str: &str, secret_seed_phrase: &str) -> String {
        let mut new_navstate = self.navstate.to_owned();
        let mut new_screen = self.navstate.screen.to_owned();
        let mut seed_names = &(*self).seed_names;
        let mut go_back_allowed = false;
        let mut screen_details = String::new();

        let mut seed_phrase = String::new();

        let mut errorline = String::new();
        if let Some(dbname) = &self.dbname {
           //Try to perform action
            match action {
                //App init
                Action::Start => {
                    println!("Seednames: {:?}, total: {}", seed_names, seed_names.len());
                    if seed_names.len() == 0 {
                        new_navstate.screen = Screen::SeedSelector;
                        new_navstate.modal = Modal::NewSeedMenu;
                        new_navstate.alert = Alert::Empty;
                    } else {
                        new_navstate = Navstate::clean_screen(Screen::Log);
                    }
                },

                //Simple navigation commands
                Action::NavbarLog => {
                    new_navstate = Navstate::clean_screen(Screen::Log);
                },
                Action::NavbarScan => {
                    new_navstate = Navstate::clean_screen(Screen::Scan);
                },
                Action::NavbarKeys => {
                    new_navstate = Navstate::clean_screen(Screen::SeedSelector);
                },
                Action::NavbarSettings => {
                    new_navstate = Navstate::clean_screen(Screen::Settings);
                },

                //General back action is defined here
                Action::GoBack => {
                    if self.navstate.alert == Alert::Empty {
                        if self.navstate.modal == Modal::Empty {
                            match &self.navstate.screen {
                                Screen::LogDetails => {
                                    new_navstate.screen = Screen::Log;
                                },
                                Screen::Transaction(_) => {
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
                                Screen::RecoverSeedPhrase(_) => {
                                    new_navstate.screen = Screen::RecoverSeedName;
                                },
                                Screen::DeriveKey(d) => {
                                    new_navstate.screen = Screen::Keys(d.get_keys_state());
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
                    } else {
                        new_navstate.alert = Alert::Empty;
                    }
                },
                Action::GoForward => {
                    match self.navstate.screen {
                        Screen::NewSeed => {
                            //TODO: check zeroize here!
                            match db_handling::identities::try_create_seed_with_length(details_str, 24, dbname) {
                                Ok(a) => {
                                    seed_phrase = a;

                                },
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&e.to_string());
                                },
                            };
                        },
                        Screen::RecoverSeedName => {
                            match db_handling::identities::get_addresses_by_seed_name(dbname, details_str) {
                                Ok(a) => {
                                    if a.len() == 0 {new_navstate = Navstate::clean_screen(Screen::RecoverSeedPhrase(details_str.to_string()))}
                                    else {
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str("Seed name already exists.");
                                    }
                                },
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                },
                            }
                        },
                        Screen::DeriveKey(ref derive_state) => {
                            new_navstate.screen = Screen::DeriveKey(derive_state.update(details_str));
                            match db_handling::identities::try_create_address (&derive_state.seed_name(), &secret_seed_phrase, details_str, &derive_state.network_specs_key(), dbname) {
                                Ok(()) => {
                                    match KeysState::new(&derive_state.seed_name(), dbname) {
                                        Ok(a) => {
                                            new_navstate = Navstate::clean_screen(Screen::Keys(a))
                                        },
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        },
                                    }
                                },
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                },
                            }
                        },
                        Screen::Transaction(ref t) => {
                            match t.action() {
                                transaction_parsing::Action::Sign{content: _, checksum, has_pwd, author_info: _, network_info: _} => {
                                    if has_pwd {
                                        match self.navstate.modal {
                                            Modal::EnterPassword => {
                                                if t.ok() {
                                                    new_navstate.screen = Screen::Transaction(t.plus_one());
                                                    let mut seed = t.seed();
                                                    match transaction_signing::handle_sign(checksum, &seed, details_str, &t.get_comment(), dbname) {
                                                         Ok(a) => {
                                                             seed.zeroize();
                                                             new_navstate.modal = Modal::SignatureReady(a);
                                                         },
                                                         Err(e) => {
                                                             seed.zeroize();
                                                             new_navstate.alert = Alert::Error;
                                                             errorline.push_str(&<Signer>::show(&e));
                                                         },
                                                     }
                                                }
                                                else {new_navstate = Navstate::clean_screen(Screen::Log);}
                                            },
                                            _ => {
                                                new_navstate.screen = Screen::Transaction(t.add_comment(details_str).update_seed(secret_seed_phrase));
                                                new_navstate.modal = Modal::EnterPassword;
                                            },
                                        }
                                    }
                                    else {
                                        match transaction_signing::handle_sign(checksum, secret_seed_phrase, "", details_str, dbname) {
                                            Ok(a) => {
                                                new_navstate.modal = Modal::SignatureReady(a);
                                            },
                                            Err(e) => {
                                                new_navstate.alert = Alert::Error;
                                                errorline.push_str(&<Signer>::show(&e));
                                            },
                                        }
                                    }
                                },
                                transaction_parsing::Action::Stub(_, checksum) => {
                                    match transaction_signing::handle_stub(checksum, dbname) {
                                        Ok(()) => {
                                            new_navstate = Navstate::clean_screen(Screen::Log);
                                        },
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        },
                                    }
                                },
                                transaction_parsing::Action::Read(_) => println!("GoForward does nothing here"),
                            }
                        },
                        _ => println!("GoForward does nothing here"),
                    };
                },
                Action::SelectSeed => {
                    match KeysState::new(details_str, dbname) {
                        Ok(a) => {
                            new_navstate = Navstate::clean_screen(Screen::Keys(a));
                        },
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                        },
                    }
                },
                Action::SelectKey => {
                    match self.navstate.screen {
                        Screen::Keys(ref keys_state) => {
                            match AddressState::new(details_str, keys_state, dbname) {
                                Ok(a) => {
                                    new_navstate = Navstate::clean_screen(Screen::KeyDetails(a));
                                },
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                },
                            }
                        },
                        _ => println!("SelectKey does nothing here"),
                    }
                },
                Action::NewKey => {
                    match self.navstate.screen {
                        Screen::Keys(ref keys_state) => {
                           new_navstate = Navstate::clean_screen(Screen::DeriveKey(DeriveState::new(details_str, keys_state)));
                        },
                        _ => println!("NewKey does nothing here"),
                    }
                },
                Action::RightButton => {
                    match &self.navstate.screen {
                        Screen::SeedSelector => 
                            if self.navstate.modal == Modal::NewSeedMenu {
                                new_navstate.modal = Modal::Empty;
                            } else {
                                new_navstate.modal = Modal::NewSeedMenu;
                            },
                        Screen::Keys(a) => new_navstate.modal = Modal::SeedMenu,
                        _ => {},
                    }
                },
                Action::Shield => {
                    new_navstate.alert = Alert::Shield;
                },
                Action::NewSeed => {
                    new_navstate = Navstate::clean_screen(Screen::NewSeed);
                },
                Action::RecoverSeed => {
                    new_navstate = Navstate::clean_screen(Screen::RecoverSeedName);
                },
                Action::BackupSeed => {
                    if details_str == "" {
                        match &self.navstate.screen {
                            Screen::Keys(ref keys_state) => {
                                new_navstate.modal = Modal::Backup(keys_state.seed_name());
                            },
                            _ => println!("BackupSeed without seed_name does nothing here"),
                        }
                    } else {
                        new_navstate = match KeysState::new(details_str, dbname) {
                            Ok(a) => {
                                Navstate {
                                    screen: Screen::Keys(a),
                                    modal: Modal::Backup(details_str.to_string()),
                                    alert: Alert::Empty,
                                }
                            },
                            Err(e) => {
                                errorline.push_str(&<Signer>::show(&e));
                                Navstate {
                                    screen: Screen::Log,
                                    modal: Modal::Empty,
                                    alert: Alert::Error,
                                }
                            },
                        };
                    }
                },
                Action::NetworkSelector => {
                    if let Modal::NetworkSelector(_) = self.navstate.modal {
                        new_navstate.modal = Modal::Empty;
                    } else {
                        match &self.navstate.screen {
                            Screen::Keys(ref keys_state) => {
                                new_navstate.modal = Modal::NetworkSelector(keys_state.network_specs_key());
                            },
                            _ => println!("NetworkSelector does nothing here"),
                        }
                    }
                },
                Action::NextUnit => {
                    match self.navstate.screen {
                        Screen::KeyDetails(ref address_state) => {
                            new_navstate = Navstate::clean_screen(Screen::KeyDetails(address_state.next()));
                        },
                        _ => println!("NextUnit does nothing here"),
                    }
                },
                Action::PreviousUnit => {
                    match self.navstate.screen {
                        Screen::KeyDetails(ref address_state) => {
                            new_navstate = Navstate::clean_screen(Screen::KeyDetails(address_state.previous()));
                        },
                        _ => println!("PreviousUnit does nothing here"),
                    }
                },
                Action::ChangeNetwork => {
                    match NetworkSpecsKey::from_hex(details_str) {
                        Ok(network_specs_key) => {
                            if let Screen::Keys(ref k) = self.navstate.screen {new_navstate.screen = Screen::Keys(k.change_network(&network_specs_key));}
                            if let Modal::NetworkSelector(_) = self.navstate.modal {new_navstate.modal = Modal::NetworkSelector(network_specs_key);}
                        },
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                        },
                    }
                },
                Action::CheckPassword => {
                    match self.navstate.screen {
                        Screen::DeriveKey(ref derive_state) => {
                            new_navstate.screen = Screen::DeriveKey(derive_state.update(details_str));
                            new_navstate.modal = Modal::PasswordConfirm;
                        },
                        _ => println!("No password to check"),
                    }
                },
                Action::TransactionFetched => {
                    new_navstate = Navstate::clean_screen(Screen::Transaction(TransactionState::new(details_str, dbname)));
                }
                Action::Nothing => {
                    println!("no action was passed in action");
                },
            };

            //Prepare screen details
            let screen_details = match new_navstate.screen {
                Screen::Log => {
                    let log_pack = match db_handling::manage_history::print_history_page(0, &dbname) {
                        Ok(a) => {
                            format!("\"log\":{}", a)
                        },
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&e.to_string());
                            "".to_string()
                        },
                    };
                    format!("{}", log_pack)
                },
                //Screen::LogDetails => "",
                Screen::Scan => "".to_string(),
                Screen::Transaction(ref t) => {
                    match t.action() {
                        transaction_parsing::Action::Sign{content, checksum: _, has_pwd: _, author_info, network_info} => format!("\"content\":{{{}}},\"author_info\":{{{}}},\"network_info\":{{{}}},\"type\":\"sign\"", content, author_info, network_info),
                        transaction_parsing::Action::Stub(content, _) => format!("\"content\":{{{}}},\"type\":\"stub\"", content),
                        transaction_parsing::Action::Read(content) => format!("\"content\":{{{}}},\"type\":\"read\"", content),
                    }
                },
                Screen::SeedSelector => {
                    let cards = match db_handling::interface_signer::print_all_seed_names_with_identicons(&dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "[]".to_string()
                        },
                    };
                    format!("\"seedNameCards\":{}", cards)
                },
                Screen::Keys(ref k) => {
                    match db_handling::interface_signer::print_identities_for_seed_name_and_network(dbname, &k.seed_name(), &k.network_specs_key()) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::KeyDetails(ref a) => {
                    match db_handling::interface_signer::export_key (dbname, &a.multisigner(), &a.seed_name(), &a.network_specs_key()) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                }
                //Screen::Backup => "this should be popover",
                //Screen::NewSeed => "",
                //Screen::RecoverSeedName => "Recover Seed",
                //Screen::RecoverSeedPhrase => "Recover Seed",
                Screen::DeriveKey(ref d) => {
                    match db_handling::interface_signer::derive_prep (dbname, &d.seed_name(), &d.network_specs_key(), &details_str) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                //Screen::Settings => "Settings",
                //Screen::Verifier => "VERIFIER CERTIFICATE",
                //Screen::ManageNetwork => "MANAGE NETWORKS",
                Screen::Nowhere => "".to_string(),
                _ => "".to_string(),
            };

            //Prepare modal details
            let modal_details = match new_navstate.modal {
                Modal::Backup(ref seed_name) => match db_handling::interface_signer::backup_prep(dbname, &seed_name) {
                    Ok(a) => a,
                    Err(e) => {
                        new_navstate.alert = Alert::Error;
                        errorline.push_str(&<Signer>::show(&e));
                        "".to_string()
                    },
                }
                Modal::NetworkSelector(ref network_specs_key) => match db_handling::interface_signer::show_all_networks_with_flag (dbname, &network_specs_key) {
                    Ok(a) => a,
                    Err(e) => {
                        new_navstate.alert = Alert::Error;
                        errorline.push_str(&<Signer>::show(&e));
                        "".to_string()
                    },
                },
                Modal::PasswordConfirm => {
                    match new_navstate.screen {
                        Screen::DeriveKey(ref derive_state) => {
                            let mut path = derive_state.path();
                            match db_handling::identities::cut_path(&path) {
                                Ok((cropped_path, pwd)) => {
                                    path.zeroize();
                                    format!("\"seed_name\":\"{}\",\"cropped_path\":\"{}\",\"pwd\":\"{}\"", derive_state.seed_name(), cropped_path, pwd)
                                },
                                Err(e) => {
                                    path.zeroize();
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                    "".to_string()
                                },
                            }
                        },
                        _ => "".to_string(),
                    }
                },
                Modal::SignatureReady(ref a) => format!("\"signature\":\"{}\"", a),
                Modal::EnterPassword => {
                    match new_navstate.screen {
                        Screen::Transaction(ref t) => {
                            if let transaction_parsing::Action::Sign{content: _, checksum: _, has_pwd: _, author_info, network_info: _} = t.action() {format!("\"author_info\":{{{}}},\"counter\":{}", author_info, t.counter())}
                            else {"".to_string()}
                        },
                        _ => "".to_string(),
                    }
                },
                _ => "".to_string(),
            };
            
            //Prepare alert details
            //Important! No errors could be handled in this block!
            let alert_details = match new_navstate.alert {
                Alert::Error => format!("\"error\":\"{}\"", errorline),
                Alert::Empty => "".to_string(),
                Alert::Shield => "\"shield_state\":\"unknown\"".to_string(),
            };

            let mut output = String::new();
            self.navstate = new_navstate;
            output = format!("\"screenData\":{{{}}},\"modalData\":{{{}}},\"alertData\":{{{}}}", screen_details, modal_details, alert_details);
            output
        } else {
            "\"error\":\"db not initialized\"".to_string()
        }
    }


    ///This converts state into renderable block
    //TODO: clean this up
    pub fn generate_json(&self, details: &str) -> String {
        let mut output = String::from("{");
        let screen = self.navstate.screen.to_owned();
        let modal = self.navstate.modal.to_owned();
        let alert = self.navstate.alert;
        if let Some(screen_name) = screen.get_name() {
            output.push_str(&format!("\"screen\":\"{}\",\"screenLabel\":\"{}\",\"back\":{},\"footer\":{},\"footerButton\":\"{}\",\"rightButton\":\"{}\",\"screenNameType\":\"{}\",", screen_name, self.get_screen_label(), screen.has_back(), true, self.get_active_navbutton(), self.get_right_button(), self.get_screen_name_type()));
        }
        output.push_str(&format!("\"modal\":\"{}\",", modal.get_name()));
        output.push_str(&format!("\"alert\":\"{}\",", alert.get_name()));
        output.push_str(details);
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
            Screen::Transaction(_) => "Scan",
            Screen::SeedSelector => "Keys",
            Screen::Keys(_) => "Keys",
            Screen::KeyDetails(_) => "Keys",
            Screen::NewSeed => "Keys",
            Screen::RecoverSeedName => "Keys",
            Screen::RecoverSeedPhrase(_) => "Keys",
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
            Screen::Transaction(_) => "None",
            Screen::SeedSelector => "NewSeed",
            Screen::Keys(_) => "Backup",
            Screen::KeyDetails(_) => "None",
            Screen::NewSeed => "None",
            Screen::RecoverSeedName => "None",
            Screen::RecoverSeedPhrase(_) => "None",
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
            Screen::Transaction(_) => "h1",
            Screen::SeedSelector => "h1",
            Screen::Keys(_) => "h4",
            Screen::KeyDetails(_) => "h4",
            Screen::NewSeed => "h1",
            Screen::RecoverSeedName => "h1",
            Screen::RecoverSeedPhrase(_) => "h1",
            Screen::DeriveKey(_) => "h1",
            Screen::Settings => "h4",
            Screen::Verifier => "h4",
            Screen::ManageNetwork => "h4",
            Screen::Nowhere => "h4",
        }.to_string()

    }
}

impl Navstate {
    pub fn clean_screen(screen: Screen) -> Navstate {
        Navstate {
            screen: screen,
            modal: Modal::Empty,
            alert: Alert::Empty,
        }
    }
}

//TODO: tests should probably be performed here, as static object in lib.rs
//will only allow for 1-2 integration tests
