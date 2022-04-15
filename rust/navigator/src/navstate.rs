//! Navigation state of the app

use sp_runtime::MultiSigner;
use zeroize::Zeroize;

use crate::actions::Action;
use crate::alerts::Alert;
use crate::modals::Modal;
use crate::screens::{
    AddressState, AddressStateMulti, DeriveState, KeysState, RecoverSeedPhraseState, Screen,
    SpecialtyKeysState, SufficientCryptoState, TransactionState,
};
use db_handling::interface_signer::{print_guess, BIP_CAP, SAFE_RESERVE, WORD_LENGTH};
use definitions::{
    error::{AddressGeneration, AddressGenerationCommon, AddressKeySource, ErrorSource},
    error_signer::{
        ErrorSigner, ExtraAddressKeySourceSigner, InputSigner, InterfaceSigner, Signer,
    },
    keyring::{AddressKey, NetworkSpecsKey},
    users::AddressDetails,
};

///State of the app as remembered by backend
#[derive(Debug, Clone)]
pub struct State {
    pub navstate: Navstate,
    pub dbname: Option<String>,
    pub seed_names: Vec<String>,
    pub networks: Vec<NetworkSpecsKey>,
}

///Navigation state is completely defined here
#[derive(Debug, Clone)]
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

impl Default for Navstate {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    ///Decide what to do and do it!
    pub fn perform(
        &mut self,
        action: Action,
        details_str: &str,
        secret_seed_phrase: &str,
    ) -> String {
        let mut new_navstate = self.navstate.to_owned();
        let seed_names = &(*self).seed_names;

        let mut errorline = String::new();
        if let Some(dbname) = &self.dbname {
            //Try to perform action
            match action {
                //App init
                Action::Start => {
                    println!("Seednames: {:?}, total: {}", seed_names, seed_names.len());
                    match db_handling::interface_signer::purge_transactions(dbname) {
                        Ok(()) => {
                            if seed_names.is_empty() {
                                new_navstate = self.correct_seed_selector();
                            } else {
                                new_navstate = Navstate::clean_screen(Screen::Log);
                            }
                        }
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                        }
                    }
                }
                //Simple navigation commands
                Action::NavbarLog => {
                    if self.get_footer() {
                        new_navstate = Navstate::clean_screen(Screen::Log)
                    }
                }
                Action::NavbarScan => {
                    if self.get_footer() {
                        new_navstate = Navstate::clean_screen(Screen::Scan)
                    }
                }
                Action::NavbarKeys => {
                    if self.get_footer() {
                        new_navstate = self.correct_seed_selector()
                    }
                }
                Action::NavbarSettings => {
                    if self.get_footer() {
                        new_navstate = Navstate::clean_screen(Screen::Settings)
                    }
                }

                //General back action is defined here
                Action::GoBack => match self.navstate.alert {
                    Alert::Empty => {
                        if let Modal::Empty = self.navstate.modal {
                            match &self.navstate.screen {
                                Screen::LogDetails(_) => {
                                    new_navstate.screen = Screen::Log;
                                }
                                Screen::Transaction(_) => {
                                    match db_handling::interface_signer::purge_transactions(dbname)
                                    {
                                        Ok(()) => new_navstate.screen = Screen::Scan,
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                                Screen::Keys(ref keys_state) => {
                                    match keys_state.get_specialty() {
                                        SpecialtyKeysState::MultiSelect(_) => {
                                            new_navstate.screen =
                                                Screen::Keys(keys_state.deselect_specialty())
                                        }
                                        _ => new_navstate = self.correct_seed_selector(),
                                    };
                                }
                                Screen::KeyDetails(address_state) => {
                                    new_navstate.screen =
                                        Screen::Keys(address_state.blank_keys_state());
                                }
                                Screen::KeyDetailsMulti(address_state_multi) => {
                                    new_navstate.screen =
                                        Screen::Keys(address_state_multi.blank_keys_state());
                                }
                                Screen::NewSeed => {
                                    new_navstate = self.correct_seed_selector();
                                }
                                Screen::RecoverSeedName(_) => {
                                    new_navstate = self.correct_seed_selector();
                                }
                                Screen::RecoverSeedPhrase(ref recover_seed_phrase_state) => {
                                    new_navstate.screen =
                                        Screen::RecoverSeedName(recover_seed_phrase_state.name());
                                }
                                Screen::DeriveKey(d) => {
                                    new_navstate.screen = Screen::Keys(d.blank_keys_state());
                                }
                                Screen::Verifier => {
                                    new_navstate.screen = Screen::Settings;
                                }
                                Screen::ManageNetworks => {
                                    new_navstate.screen = Screen::Settings;
                                }
                                Screen::NetworkDetails(_) => {
                                    new_navstate.screen = Screen::ManageNetworks;
                                }
                                Screen::SelectSeedForBackup => {
                                    new_navstate.screen = Screen::Settings;
                                }
                                Screen::SignSufficientCrypto(a) => match a.content() {
                                    transaction_signing::SufficientContent::AddSpecs(key) => {
                                        new_navstate.screen = Screen::NetworkDetails(key);
                                    }
                                    transaction_signing::SufficientContent::LoadMeta(key, _) => {
                                        new_navstate.screen = Screen::NetworkDetails(key);
                                    }
                                    transaction_signing::SufficientContent::LoadTypes => {
                                        new_navstate.screen = Screen::Settings;
                                    }
                                },
                                Screen::Documents => {
                                    new_navstate = Navstate::clean_screen(Screen::Settings)
                                }
                                _ => {
                                    println!("Back button pressed at the bottom of navigation");
                                }
                            };
                        } else {
                            match &self.navstate.screen {
                                Screen::Transaction(_) => {
                                    match db_handling::interface_signer::purge_transactions(dbname)
                                    {
                                        Ok(()) => {
                                            new_navstate = Navstate::clean_screen(Screen::Log)
                                        }
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                                Screen::SignSufficientCrypto(_) => {
                                    new_navstate = Navstate::clean_screen(Screen::Settings);
                                }
                                _ => {
                                    new_navstate.modal = Modal::Empty;
                                }
                            }
                        }
                    }
                    Alert::ErrorDisplay => {
                        new_navstate = Navstate::clean_screen(Screen::Settings);
                    }
                    _ => new_navstate.alert = Alert::Empty,
                },
                Action::GoForward => {
                    match self.navstate.screen {
                        Screen::Log => {
                            match self.navstate.modal {
                                Modal::LogComment => {
                                    // details_str is user entered comment
                                    match db_handling::manage_history::history_entry_user(
                                        dbname,
                                        details_str,
                                    ) {
                                        Ok(()) => {
                                            new_navstate = Navstate::clean_screen(Screen::Log)
                                        }
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                                _ => println!("GoForward does nothing here"),
                            }
                        }
                        Screen::NewSeed => {
                            // details_str is new seed name
                            match self.navstate.modal {
                                Modal::Empty => {
                                    new_navstate.modal =
                                        Modal::NewSeedBackup(details_str.to_string())
                                }
                                Modal::NewSeedBackup(ref seed_name) => {
                                    match details_str.parse::<bool>() {
                                        Ok(roots) => {
                                            match db_handling::identities::try_create_seed(
                                                seed_name,
                                                secret_seed_phrase,
                                                roots,
                                                dbname,
                                            ) {
                                                Ok(()) => match KeysState::new(seed_name, dbname) {
                                                    Ok(a) => {
                                                        new_navstate =
                                                            Navstate::clean_screen(Screen::Keys(a))
                                                    }
                                                    Err(e) => {
                                                        new_navstate.alert = Alert::Error;
                                                        errorline.push_str(&<Signer>::show(&e));
                                                    }
                                                },
                                                Err(e) => {
                                                    new_navstate.alert = Alert::Error;
                                                    errorline.push_str(&<Signer>::show(&e));
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(
                                                &ErrorSigner::Interface(
                                                    InterfaceSigner::FlagNotBool(
                                                        details_str.to_string(),
                                                    ),
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => println!("GoForward does nothing here"),
                            }
                        }
                        Screen::RecoverSeedName(_) => {
                            match db_handling::identities::get_addresses_by_seed_name(
                                dbname,
                                details_str,
                            ) {
                                Ok(a) => {
                                    if a.is_empty() {
                                        new_navstate =
                                            Navstate::clean_screen(Screen::RecoverSeedPhrase(
                                                RecoverSeedPhraseState::new(details_str),
                                            ))
                                    } else {
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str(&<Signer>::show(&ErrorSigner::Input(
                                            InputSigner::SeedNameExists(details_str.to_string()),
                                        )));
                                    }
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        Screen::RecoverSeedPhrase(ref recover_seed_phrase_state) => {
                            let seed_name = recover_seed_phrase_state.name();
                            match details_str.parse::<bool>() {
                                Ok(roots) => match db_handling::identities::try_create_seed(
                                    &seed_name,
                                    secret_seed_phrase,
                                    roots,
                                    dbname,
                                ) {
                                    Ok(()) => match KeysState::new(&seed_name, dbname) {
                                        Ok(a) => {
                                            new_navstate = Navstate::clean_screen(Screen::Keys(a))
                                        }
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    },
                                    Err(e) => {
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str(&<Signer>::show(&e));
                                    }
                                },
                                Err(_) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&ErrorSigner::Interface(
                                        InterfaceSigner::FlagNotBool(details_str.to_string()),
                                    )));
                                }
                            }
                        }
                        Screen::DeriveKey(ref derive_state) => {
                            new_navstate.screen =
                                Screen::DeriveKey(derive_state.update(details_str));
                            match db_handling::identities::try_create_address(
                                &derive_state.seed_name(),
                                secret_seed_phrase,
                                details_str,
                                &derive_state.network_specs_key(),
                                dbname,
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        KeysState::new_in_network(
                                            &derive_state.seed_name(),
                                            &derive_state.network_specs_key(),
                                        ),
                                    ))
                                }
                                Err(e) => {
                                    if let ErrorSigner::AddressGeneration(
                                        AddressGeneration::Common(
                                            AddressGenerationCommon::DerivationExists(
                                                ref multisigner,
                                                ref address_details,
                                                _,
                                            ),
                                        ),
                                    ) = e
                                    {
                                        new_navstate.screen = Screen::DeriveKey(
                                            derive_state
                                                .collided_with(multisigner, address_details),
                                        );
                                        new_navstate.modal = Modal::Empty;
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str(&<Signer>::show(&e));
                                    } else {
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str(&<Signer>::show(&e));
                                    }
                                }
                            }
                        }
                        Screen::Transaction(ref t) => {
                            match t.action() {
                                transaction_parsing::Action::Sign {
                                    content,
                                    checksum,
                                    has_pwd,
                                    author_info,
                                    network_info,
                                } => {
                                    if has_pwd {
                                        match self.navstate.modal {
                                            Modal::EnterPassword => {
                                                let mut seed = t.seed();
                                                match transaction_signing::handle_sign(
                                                    checksum,
                                                    &seed,
                                                    details_str,
                                                    &t.get_comment(),
                                                    dbname,
                                                ) {
                                                    Ok(a) => {
                                                        seed.zeroize();
                                                        new_navstate.modal =
                                                            Modal::SignatureReady(a);
                                                    }
                                                    Err(e) => {
                                                        seed.zeroize();
                                                        if let ErrorSigner::WrongPasswordNewChecksum(c) = e {
                                                            if t.ok() {new_navstate.screen = Screen::Transaction(t.update_checksum_sign(c, content, has_pwd, author_info, network_info));}
                                                            else {new_navstate = Navstate::clean_screen(Screen::Log);}
                                                        }
                                                        new_navstate.alert = Alert::Error;
                                                        errorline.push_str(&<Signer>::show(&e));
                                                    }
                                                }
                                            }
                                            _ => {
                                                new_navstate.screen = Screen::Transaction(
                                                    t.add_comment(details_str)
                                                        .update_seed(secret_seed_phrase),
                                                );
                                                new_navstate.modal = Modal::EnterPassword;
                                            }
                                        }
                                    } else {
                                        match transaction_signing::handle_sign(
                                            checksum,
                                            secret_seed_phrase,
                                            "",
                                            details_str,
                                            dbname,
                                        ) {
                                            Ok(a) => {
                                                new_navstate.modal = Modal::SignatureReady(a);
                                            }
                                            Err(e) => {
                                                new_navstate.alert = Alert::Error;
                                                errorline.push_str(&<Signer>::show(&e));
                                            }
                                        }
                                    }
                                }
                                transaction_parsing::Action::Stub(_, checksum, stub_nav) => {
                                    match transaction_signing::handle_stub(checksum, dbname) {
                                        Ok(()) => match stub_nav {
                                            transaction_parsing::StubNav::AddSpecs(
                                                network_specs_key,
                                            ) => {
                                                new_navstate = Navstate::clean_screen(
                                                    Screen::NetworkDetails(network_specs_key),
                                                );
                                            }
                                            transaction_parsing::StubNav::LoadMeta(
                                                network_specs_key,
                                            ) => {
                                                new_navstate = Navstate::clean_screen(
                                                    Screen::NetworkDetails(network_specs_key),
                                                );
                                            }
                                            transaction_parsing::StubNav::LoadTypes => {
                                                new_navstate =
                                                    Navstate::clean_screen(Screen::ManageNetworks);
                                            }
                                        },
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                                transaction_parsing::Action::Read(_) => {
                                    println!("GoForward does nothing here")
                                }
                                transaction_parsing::Action::Derivations {
                                    content: _,
                                    network_info: _,
                                    checksum,
                                    network_specs_key,
                                } => {
                                    match self.navstate.modal {
                                        Modal::SelectSeed => {
                                            // details_str is seed_name
                                            // secret_seed_phrase is seed_phrase
                                            match db_handling::identities::import_derivations(
                                                checksum,
                                                details_str,
                                                secret_seed_phrase,
                                                dbname,
                                            ) {
                                                Ok(()) => {
                                                    new_navstate = Navstate::clean_screen(
                                                        Screen::Keys(KeysState::new_in_network(
                                                            details_str,
                                                            &network_specs_key,
                                                        )),
                                                    );
                                                }
                                                Err(e) => {
                                                    new_navstate.alert = Alert::Error;
                                                    errorline.push_str(&<Signer>::show(&e));
                                                }
                                            }
                                        }
                                        Modal::Empty => {
                                            new_navstate.modal = Modal::SelectSeed;
                                        }
                                        _ => println!("GoForward does nothing here"),
                                    }
                                }
                            }
                        }
                        Screen::ManageNetworks => match NetworkSpecsKey::from_hex(details_str) {
                            Ok(network_specs_key) => {
                                new_navstate = Navstate::clean_screen(Screen::NetworkDetails(
                                    network_specs_key,
                                ))
                            }
                            Err(e) => {
                                new_navstate.alert = Alert::Error;
                                errorline.push_str(&<Signer>::show(&e));
                            }
                        },
                        Screen::SignSufficientCrypto(ref s) => {
                            match s.key_selected() {
                                Some((multisigner, address_details, _)) => {
                                    // can get here only if there is a password
                                    // details_str is password entry attempt
                                    if let Modal::EnterPassword = self.navstate.modal {
                                        let mut seed = s.seed();
                                        match transaction_signing::sign_content(
                                            &multisigner,
                                            &address_details,
                                            s.content(),
                                            dbname,
                                            &seed,
                                            details_str,
                                        ) {
                                            Ok(a) => {
                                                seed.zeroize();
                                                new_navstate.modal =
                                                    Modal::SufficientCryptoReady(a);
                                            }
                                            Err(e) => {
                                                seed.zeroize();
                                                if let ErrorSigner::WrongPassword = e {
                                                    if s.ok() {
                                                        new_navstate.screen =
                                                            Screen::SignSufficientCrypto(
                                                                s.plus_one(),
                                                            );
                                                    } else {
                                                        new_navstate =
                                                            Navstate::clean_screen(Screen::Log);
                                                    }
                                                }
                                                new_navstate.alert = Alert::Error;
                                                errorline.push_str(&<Signer>::show(&e));
                                            }
                                        }
                                    }
                                }
                                None => {
                                    // details_str is hex_address_key
                                    // secret_seed_phrase is seed phrase
                                    match process_hex_address_key_address_details(
                                        details_str,
                                        dbname,
                                    ) {
                                        Ok((multisigner, address_details)) => {
                                            if address_details.has_pwd {
                                                new_navstate.screen =
                                                    Screen::SignSufficientCrypto(s.update(
                                                        &multisigner,
                                                        &address_details,
                                                        secret_seed_phrase,
                                                    ));
                                                new_navstate.modal = Modal::EnterPassword;
                                            } else {
                                                match transaction_signing::sign_content(
                                                    &multisigner,
                                                    &address_details,
                                                    s.content(),
                                                    dbname,
                                                    secret_seed_phrase,
                                                    "",
                                                ) {
                                                    Ok(a) => {
                                                        new_navstate.screen =
                                                            Screen::SignSufficientCrypto(s.update(
                                                                &multisigner,
                                                                &address_details,
                                                                "",
                                                            ));
                                                        new_navstate.modal =
                                                            Modal::SufficientCryptoReady(a);
                                                    }
                                                    Err(e) => {
                                                        new_navstate.alert = Alert::Error;
                                                        errorline.push_str(&<Signer>::show(&e));
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                            }
                        }
                        _ => println!("GoForward does nothing here"),
                    };
                }
                Action::SelectSeed => {
                    match self.navstate.screen {
                        Screen::SeedSelector => {
                            if !details_str.is_empty() {
                                // details_str is seed name
                                match KeysState::new(details_str, dbname) {
                                    Ok(a) => {
                                        new_navstate = Navstate::clean_screen(Screen::Keys(a));
                                    }
                                    Err(e) => {
                                        new_navstate.alert = Alert::Error;
                                        errorline.push_str(&<Signer>::show(&e));
                                    }
                                }
                            } else {
                                println!("SelectSeed needs non-empty details_str")
                            }
                        }
                        _ => println!("SelectSeed does nothing here"),
                    }
                }
                Action::SelectKey => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        if keys_state.is_multiselect() {
                            match process_hex_address_key(details_str) {
                                Ok(multisigner) => {
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        keys_state.select_single(&multisigner),
                                    ));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        } else {
                            match AddressState::new(details_str, keys_state, dbname) {
                                Ok(a) => {
                                    new_navstate = Navstate::clean_screen(Screen::KeyDetails(a));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                    }
                    _ => println!("SelectKey does nothing here"),
                },
                Action::NewKey => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        new_navstate = Navstate::clean_screen(Screen::DeriveKey(DeriveState::new(
                            details_str,
                            keys_state,
                        )));
                    }
                    _ => println!("NewKey does nothing here"),
                },
                Action::RightButton => match &self.navstate.screen {
                    Screen::Log => new_navstate.modal = self.toggle_modal(Modal::LogRight),
                    Screen::SeedSelector => {
                        new_navstate.modal = self.toggle_modal(Modal::NewSeedMenu)
                    }
                    Screen::Keys(_) => new_navstate.modal = self.toggle_modal(Modal::SeedMenu),
                    Screen::KeyDetails(_) => {
                        new_navstate.modal = self.toggle_modal(Modal::KeyDetailsAction)
                    }
                    Screen::ManageNetworks => {
                        new_navstate.modal = self.toggle_modal(Modal::TypesInfo)
                    }
                    Screen::NetworkDetails(_) => {
                        new_navstate.modal = self.toggle_modal(Modal::NetworkDetailsMenu)
                    }
                    _ => {}
                },
                Action::Shield => {
                    new_navstate.alert = Alert::Shield;
                }
                Action::NewSeed => {
                    if let Screen::SeedSelector = self.navstate.screen {
                        if let Modal::NewSeedMenu = self.navstate.modal {
                            new_navstate = Navstate::clean_screen(Screen::NewSeed);
                        }
                    }
                }
                Action::RecoverSeed => {
                    if let Screen::SeedSelector = self.navstate.screen {
                        if let Modal::NewSeedMenu = self.navstate.modal {
                            new_navstate =
                                Navstate::clean_screen(Screen::RecoverSeedName(String::new()));
                        }
                    }
                }
                Action::BackupSeed => {
                    if details_str.is_empty() {
                        match &self.navstate.screen {
                            Screen::Keys(ref keys_state) => {
                                if let Modal::SeedMenu = self.navstate.modal {
                                    new_navstate.modal = Modal::Backup(keys_state.seed_name());
                                }
                            }
                            Screen::Settings => {
                                new_navstate = Navstate::clean_screen(Screen::SelectSeedForBackup);
                            }
                            _ => println!("BackupSeed without seed_name does nothing here"),
                        }
                    } else if let Screen::SelectSeedForBackup = self.navstate.screen {
                        new_navstate = match KeysState::new(details_str, dbname) {
                            Ok(a) => Navstate {
                                screen: Screen::Keys(a),
                                modal: Modal::Backup(details_str.to_string()),
                                alert: Alert::Empty,
                            },
                            Err(e) => {
                                errorline.push_str(&<Signer>::show(&e));
                                Navstate {
                                    screen: Screen::Log,
                                    modal: Modal::Empty,
                                    alert: Alert::Error,
                                }
                            }
                        };
                    }
                }
                Action::NetworkSelector => {
                    if let Modal::NetworkSelector(_) = self.navstate.modal {
                        new_navstate.modal = Modal::Empty;
                    } else {
                        match &self.navstate.screen {
                            Screen::Keys(ref keys_state) => {
                                new_navstate.modal =
                                    Modal::NetworkSelector(keys_state.network_specs_key());
                            }
                            _ => println!("NetworkSelector does nothing here"),
                        }
                    }
                }
                Action::NextUnit => match self.navstate.screen {
                    Screen::KeyDetailsMulti(ref address_state_multi) => {
                        new_navstate = Navstate::clean_screen(Screen::KeyDetailsMulti(
                            address_state_multi.next(),
                        ));
                    }
                    _ => println!("NextUnit does nothing here"),
                },
                Action::PreviousUnit => match self.navstate.screen {
                    Screen::KeyDetailsMulti(ref address_state_multi) => {
                        new_navstate = Navstate::clean_screen(Screen::KeyDetailsMulti(
                            address_state_multi.previous(),
                        ));
                    }
                    _ => println!("PreviousUnit does nothing here"),
                },
                Action::ChangeNetwork => {
                    if let Screen::Keys(ref keys_state) = self.navstate.screen {
                        if let Modal::NetworkSelector(_) = self.navstate.modal {
                            match NetworkSpecsKey::from_hex(details_str) {
                                Ok(network_specs_key) => {
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        keys_state.change_network(&network_specs_key),
                                    ))
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                    }
                }
                Action::CheckPassword => match self.navstate.screen {
                    Screen::DeriveKey(ref derive_state) => {
                        new_navstate.screen = Screen::DeriveKey(derive_state.update(details_str));
                        new_navstate.modal = Modal::PasswordConfirm;
                    }
                    _ => println!("No password to check"),
                },
                Action::TransactionFetched => {
                    if let Screen::Scan = self.navstate.screen {
                        new_navstate = Navstate::clean_screen(Screen::Transaction(
                            TransactionState::new(details_str, dbname),
                        ));
                    }
                }
                Action::RemoveNetwork => match self.navstate.screen {
                    Screen::NetworkDetails(ref network_specs_key) => {
                        if let Modal::NetworkDetailsMenu = self.navstate.modal {
                            match db_handling::remove_network::remove_network(
                                network_specs_key,
                                dbname,
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::ManageNetworks);
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                    }
                    _ => println!("RemoveNetwork does nothing here"),
                },
                Action::RemoveMetadata => match self.navstate.screen {
                    Screen::NetworkDetails(ref network_specs_key) => match self.navstate.modal {
                        Modal::ManageMetadata(network_version) => {
                            match db_handling::remove_network::remove_metadata(
                                network_specs_key,
                                network_version,
                                dbname,
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::NetworkDetails(
                                        network_specs_key.to_owned(),
                                    ));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        _ => println!("RemoveMetadata does nothing here"),
                    },
                    _ => println!("RemoveMetadata does nothing here"),
                },
                Action::RemoveTypes => match self.navstate.screen {
                    Screen::ManageNetworks => match self.navstate.modal {
                        Modal::TypesInfo => {
                            match db_handling::remove_types::remove_types_info(dbname) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Log);
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        _ => println!("RemoveTypes does nothing here"),
                    },
                    _ => println!("RemoveTypes does nothing here"),
                },
                Action::SignNetworkSpecs => match self.navstate.screen {
                    Screen::NetworkDetails(ref network_specs_key) => {
                        if let Modal::NetworkDetailsMenu = self.navstate.modal {
                            new_navstate = Navstate::clean_screen(Screen::SignSufficientCrypto(
                                SufficientCryptoState::init(
                                    transaction_signing::SufficientContent::AddSpecs(
                                        network_specs_key.to_owned(),
                                    ),
                                ),
                            ));
                        }
                    }
                    _ => println!("SignNetworkSpecs does nothing here"),
                },
                Action::SignMetadata => match self.navstate.screen {
                    Screen::NetworkDetails(ref network_specs_key) => {
                        if let Modal::ManageMetadata(network_version) = self.navstate.modal {
                            new_navstate = Navstate::clean_screen(Screen::SignSufficientCrypto(
                                SufficientCryptoState::init(
                                    transaction_signing::SufficientContent::LoadMeta(
                                        network_specs_key.to_owned(),
                                        network_version,
                                    ),
                                ),
                            ));
                        }
                    }
                    _ => println!("SignMetadata does nothing here"),
                },
                Action::SignTypes => match self.navstate.screen {
                    Screen::ManageNetworks => match self.navstate.modal {
                        Modal::TypesInfo => {
                            new_navstate = Navstate::clean_screen(Screen::SignSufficientCrypto(
                                SufficientCryptoState::init(
                                    transaction_signing::SufficientContent::LoadTypes,
                                ),
                            ));
                        }
                        _ => println!("SignTypes does nothing here"),
                    },
                    _ => println!("SignTypes does nothing here"),
                },
                Action::ManageNetworks => match self.navstate.screen {
                    Screen::Settings => {
                        new_navstate = Navstate::clean_screen(Screen::ManageNetworks);
                    }
                    _ => println!("ManageNetworks does nothing here"),
                },
                Action::ViewGeneralVerifier => match self.navstate.screen {
                    Screen::Settings => {
                        new_navstate = Navstate::clean_screen(Screen::Verifier);
                    }
                    _ => println!("ViewGeneralVerifier does nothing here"),
                },
                Action::ManageMetadata => match self.navstate.screen {
                    Screen::NetworkDetails(_) => match details_str.parse::<u32>() {
                        Ok(version) => {
                            new_navstate.modal = Modal::ManageMetadata(version);
                        }
                        Err(_) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&ErrorSigner::Interface(
                                InterfaceSigner::VersionNotU32(details_str.to_string()),
                            )));
                        }
                    },
                    _ => println!("ManageMetadata does nothing here"),
                },
                Action::RemoveKey => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => match keys_state.get_specialty() {
                        SpecialtyKeysState::Swiped(ref multisigner) => {
                            match db_handling::identities::remove_key(
                                dbname,
                                multisigner,
                                &keys_state.network_specs_key(),
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        KeysState::new_in_network(
                                            &keys_state.seed_name(),
                                            &keys_state.network_specs_key(),
                                        ),
                                    ));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        SpecialtyKeysState::MultiSelect(ref multiselect) => {
                            match db_handling::identities::remove_keys_set(
                                dbname,
                                multiselect,
                                &keys_state.network_specs_key(),
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        KeysState::new_in_network(
                                            &keys_state.seed_name(),
                                            &keys_state.network_specs_key(),
                                        ),
                                    ));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        SpecialtyKeysState::None => println!("RemoveKey does nothing here"),
                    },
                    Screen::KeyDetails(ref address_state) => {
                        if let Modal::KeyDetailsAction = self.navstate.modal {
                            match db_handling::identities::remove_key(
                                dbname,
                                &address_state.multisigner(),
                                &address_state.network_specs_key(),
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Log);
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        } else {
                            println!("RemoveKey does nothing here")
                        }
                    }
                    _ => println!("RemoveKey does nothing here"),
                },
                Action::RemoveSeed => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        if let Modal::SeedMenu = self.navstate.modal {
                            match db_handling::identities::remove_seed(
                                dbname,
                                &keys_state.seed_name(),
                            ) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Log);
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        } else {
                            println!("RemoveSeed does nothing here")
                        }
                    }
                    _ => println!("RemoveSeed does nothing here"),
                },
                Action::ClearLog => match self.navstate.screen {
                    Screen::Log => {
                        if let Modal::LogRight = self.navstate.modal {
                            match db_handling::manage_history::clear_history(dbname) {
                                Ok(()) => {
                                    new_navstate = Navstate::clean_screen(Screen::Log);
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        } else {
                            println!("ClearLog does nothing here")
                        }
                    }
                    _ => println!("ClearLog does nothing here"),
                },
                Action::CreateLogComment => match self.navstate.screen {
                    Screen::Log => {
                        if let Modal::LogRight = self.navstate.modal {
                            new_navstate.modal = Modal::LogComment;
                        } else {
                            println!("CreateLogComment does nothing here")
                        }
                    }
                    _ => println!("CreateLogComment does nothing here"),
                },
                Action::ShowLogDetails => {
                    match self.navstate.screen {
                        Screen::Log => {
                            // details_str is u32 order which will be shown
                            match details_str.parse::<u32>() {
                                Ok(order) => {
                                    new_navstate = Navstate::clean_screen(Screen::LogDetails(order))
                                }
                                Err(_) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&ErrorSigner::Interface(
                                        InterfaceSigner::OrderNotU32(details_str.to_string()),
                                    )));
                                }
                            }
                        }
                        _ => println!("ShowLogDetails does nothing here"),
                    }
                }
                Action::Swipe => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => match process_hex_address_key(details_str) {
                        Ok(multisigner) => {
                            new_navstate = Navstate::clean_screen(Screen::Keys(
                                keys_state.swipe(&multisigner),
                            ));
                        }
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                        }
                    },
                    _ => println!("Swipe does nothing here"),
                },
                Action::LongTap => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => match process_hex_address_key(details_str) {
                        Ok(multisigner) => {
                            new_navstate = Navstate::clean_screen(Screen::Keys(
                                keys_state.select_single(&multisigner),
                            ));
                        }
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                        }
                    },
                    _ => println!("LongTap does nothing here"),
                },
                Action::SelectAll => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => match keys_state.get_specialty() {
                        SpecialtyKeysState::MultiSelect(ref multiselect) => {
                            match db_handling::interface_signer::addresses_set_seed_name_network(
                                dbname,
                                &keys_state.seed_name(),
                                &keys_state.network_specs_key(),
                            ) {
                                Ok(set) => {
                                    let all: Vec<MultiSigner> = set
                                        .into_iter()
                                        .map(|(multisigner, _)| multisigner)
                                        .collect();
                                    let mut complete = true;
                                    for multisigner in all.iter() {
                                        if !multiselect.contains(multisigner) {
                                            complete = false
                                        }
                                    }
                                    let new_multiselect = {
                                        if complete {
                                            Vec::new()
                                        } else {
                                            all
                                        }
                                    };
                                    new_navstate = Navstate::clean_screen(Screen::Keys(
                                        keys_state.select_set(new_multiselect),
                                    ));
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        }
                        _ => println!("SelectAll does nothing here"),
                    },
                    _ => println!("SelectAll does nothing here"),
                },
                Action::ExportMultiSelect => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        if let SpecialtyKeysState::MultiSelect(ref multiselect) =
                            keys_state.get_specialty()
                        {
                            match AddressStateMulti::new(
                                keys_state.seed_name(),
                                keys_state.network_specs_key(),
                                multiselect,
                                dbname,
                            ) {
                                Ok(a) => {
                                    new_navstate =
                                        Navstate::clean_screen(Screen::KeyDetailsMulti(a))
                                }
                                Err(e) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                }
                            }
                        } else {
                            println!("ExportMultiSelect does nothing here")
                        }
                    }
                    _ => println!("ExportMultiSelect does nothing here"),
                },
                Action::Increment => match self.navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        if let SpecialtyKeysState::Swiped(multisigner) = keys_state.get_specialty()
                        {
                            match details_str.parse::<u32>() {
                                Ok(increment) => {
                                    match db_handling::identities::create_increment_set(
                                        increment,
                                        &multisigner,
                                        &keys_state.network_specs_key(),
                                        secret_seed_phrase,
                                        dbname,
                                    ) {
                                        Ok(()) => {
                                            new_navstate = Navstate::clean_screen(Screen::Keys(
                                                KeysState::new_in_network(
                                                    &keys_state.seed_name(),
                                                    &keys_state.network_specs_key(),
                                                ),
                                            ));
                                        }
                                        Err(e) => {
                                            new_navstate.alert = Alert::Error;
                                            errorline.push_str(&<Signer>::show(&e));
                                        }
                                    }
                                }
                                Err(_) => {
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&ErrorSigner::Interface(
                                        InterfaceSigner::IncNotU32(details_str.to_string()),
                                    )));
                                }
                            }
                        } else {
                            println!("Increment does nothing here")
                        }
                    }
                    _ => println!("Increment does nothing here"),
                },
                Action::ShowDocuments => match self.navstate.screen {
                    Screen::Settings => new_navstate = Navstate::clean_screen(Screen::Documents),
                    _ => println!("ShowDocuments does nothing here"),
                },
                Action::TextEntry => match self.navstate.screen {
                    Screen::RecoverSeedPhrase(ref mut recover_seed_phrase_state) => {
                        recover_seed_phrase_state.text_entry(details_str);
                        new_navstate = Navstate::clean_screen(Screen::RecoverSeedPhrase(
                            recover_seed_phrase_state.to_owned(),
                        ));
                    }
                    _ => println!("TextEntry does nothing here"),
                },
                Action::PushWord => match self.navstate.screen {
                    Screen::RecoverSeedPhrase(ref mut recover_seed_phrase_state) => {
                        recover_seed_phrase_state.push_word(details_str);
                        new_navstate = Navstate::clean_screen(Screen::RecoverSeedPhrase(
                            recover_seed_phrase_state.to_owned(),
                        ));
                    }
                    _ => println!("PushWord does nothing here"),
                },
                Action::Nothing => {
                    println!("no action was passed in action");
                }
            };

            //Prepare screen details
            let screen_details = match new_navstate.screen {
                Screen::Log => {
                    match db_handling::manage_history::print_history(dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::LogDetails(order) => {
                    match transaction_parsing::print_history_entry_by_order_with_decoding(order, dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::Scan => "".to_string(),
                Screen::Transaction(ref t) => {
                    match t.action() {
                        transaction_parsing::Action::Derivations{content, network_info, checksum: _, network_specs_key: _} => format!("\"content\":{{{}}},\"network_info\":{{{}}},\"type\":\"import_derivations\"", content, network_info),
                        transaction_parsing::Action::Sign{content, checksum: _, has_pwd: _, author_info, network_info} => {
                            let action_type = match new_navstate.modal {
                                Modal::SignatureReady(_) => "done",
                                _ => "sign",
                            };
                            format!("\"content\":{{{}}},\"author_info\":{{{}}},\"network_info\":{{{}}},\"type\":\"{}\"", content, author_info, network_info, action_type)
                        },
                        transaction_parsing::Action::Stub(content, _, _) => format!("\"content\":{{{}}},\"type\":\"stub\"", content),
                        transaction_parsing::Action::Read(content) => format!("\"content\":{{{}}},\"type\":\"read\"", content),
                    }
                },
                Screen::SeedSelector | Screen::SelectSeedForBackup => {
                    match db_handling::interface_signer::print_all_seed_names_with_identicons(dbname, &self.seed_names) {
                        Ok(a) => format!("\"seedNameCards\":{}", a),
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "\"seedNameCards\":[]".to_string()
                        },
                    }
                },
                Screen::Keys(ref keys_state) => {
                    match db_handling::interface_signer::print_identities_for_seed_name_and_network(dbname, &keys_state.seed_name(), &keys_state.network_specs_key(), keys_state.get_swiped_key(), keys_state.get_multiselect_keys()) {
                        Ok(a) => {
                            let count = {
                                if let SpecialtyKeysState::MultiSelect(ref multiselect) = keys_state.get_specialty() {multiselect.len().to_string()}
                                else {String::new()}
                            };
                            format!("{},\"multiselect_mode\":{},\"multiselect_count\":\"{}\"", a, keys_state.is_multiselect(), count)
                        },
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::KeyDetails(ref address_state) => {
                    match db_handling::interface_signer::export_key (dbname, &address_state.multisigner(), &address_state.seed_name(), &address_state.network_specs_key()) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::KeyDetailsMulti(ref address_state_multi) => {
                    match db_handling::interface_signer::export_key (dbname, &address_state_multi.multisigner(), &address_state_multi.seed_name(), &address_state_multi.network_specs_key()) {
                        Ok(a) => format!("{},\"current_number\":\"{}\",\"out_of\":\"{}\"", a, address_state_multi.number(), address_state_multi.out_of()),
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::NewSeed => format!("\"keyboard\":{}", new_navstate.keyboard()),
                Screen::RecoverSeedName(ref seed_name) => format!("\"seed_name\":\"{}\",\"keyboard\":{}", seed_name, new_navstate.keyboard()),
                Screen::RecoverSeedPhrase(ref recover_seed_phrase_state) => {
                    let draft = recover_seed_phrase_state.draft();
                    let user_input = draft.user_input();
                    let guess_set = print_guess(user_input);
                    let open_part = format!("\"seed_name\":\"{}\",\"keyboard\":{},\"user_input\":\" {}\",\"guess_set\":{},\"draft\":", recover_seed_phrase_state.name(), new_navstate.keyboard(), user_input, guess_set); // first space in user input is intended
                    let mut out = String::with_capacity(open_part.len() + SAFE_RESERVE + (WORD_LENGTH+1)*BIP_CAP + 15); // fit open part, draft as json, ready seed as str
                    out.push_str(&open_part);

                    let mut seed_draft_print = draft.print();
                    out.push_str(&seed_draft_print);
                    seed_draft_print.zeroize();

                    let mut seed_finalized = draft.try_finalize();
                    if let Some(ref s) = seed_finalized {
                        out.push_str(",\"ready_seed\":\"");
                        out.push_str(s);
                        out.push('\"');
                    }
                    seed_finalized.zeroize();

                    out
                },
                Screen::DeriveKey(ref derive_state) => {
                    match db_handling::interface_signer::derive_prep (dbname, &derive_state.seed_name(), &derive_state.network_specs_key(), derive_state.collision(), details_str) {
                        Ok(a) => {
                            format!("{},\"keyboard\":{}", a, new_navstate.keyboard())
                        },
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            String::from("\"keyboard\":false")
                        },
                    }
                },
                Screen::Settings => { // for now has same content as Screen::Verifier
                    match db_handling::helpers::get_general_verifier(dbname) {
                        Ok(a) => a.show_card(),
                        Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
                    }
                },
                Screen::Verifier => {
                    match db_handling::helpers::get_general_verifier(dbname) {
                        Ok(a) => a.show_card(),
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::ManageNetworks => {
                    match db_handling::interface_signer::show_all_networks(dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::NetworkDetails(ref network_specs_key) => {
                    match db_handling::interface_signer::network_details_by_key(dbname, network_specs_key) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::SignSufficientCrypto(_) => {
                    match db_handling::interface_signer::print_all_identities(dbname) {
                        Ok(a) => format!("\"identities\":{}", a),
                        Err(e) => {
                            new_navstate.alert = Alert::ErrorDisplay;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        },
                    }
                },
                Screen::Nowhere => "".to_string(),
                _ => "".to_string(),
            };

            //Prepare modal details
            let modal_details = match new_navstate.modal {
                Modal::Backup(ref seed_name) => {
                    match db_handling::interface_signer::backup_prep(dbname, seed_name) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        }
                    }
                }
                Modal::SeedMenu => match new_navstate.screen {
                    Screen::Keys(ref keys_state) => {
                        let seed_name = keys_state.seed_name();
                        new_navstate.screen = Screen::Keys(keys_state.deselect_specialty());
                        format!("\"seed\":\"{}\"", seed_name)
                    }
                    _ => "".to_string(),
                },
                Modal::NewSeedBackup(ref new_seed_name) => {
                    match db_handling::interface_signer::print_new_seed(new_seed_name) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        }
                    }
                }
                Modal::NetworkSelector(ref network_specs_key) => {
                    match db_handling::interface_signer::show_all_networks_with_flag(
                        dbname,
                        network_specs_key,
                    ) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        }
                    }
                }
                Modal::PasswordConfirm => {
                    match new_navstate.screen {
                        Screen::DeriveKey(ref derive_state) => {
                            let mut path = derive_state.path();
                            match db_handling::identities::cut_path(&path) {
                                Ok((cropped_path, pwd)) => {
                                    path.zeroize();
                                    format!("\"seed_name\":\"{}\",\"cropped_path\":\"{}\",\"pwd\":\"{}\"", derive_state.seed_name(), cropped_path, pwd)
                                }
                                Err(e) => {
                                    path.zeroize();
                                    new_navstate.alert = Alert::Error;
                                    errorline.push_str(&<Signer>::show(&e));
                                    "".to_string()
                                }
                            }
                        }
                        _ => "".to_string(),
                    }
                }
                Modal::SignatureReady(ref a) => format!("\"signature\":\"{}\"", a),
                Modal::EnterPassword => match new_navstate.screen {
                    Screen::Transaction(ref t) => {
                        if let transaction_parsing::Action::Sign {
                            content: _,
                            checksum: _,
                            has_pwd: _,
                            author_info,
                            network_info: _,
                        } = t.action()
                        {
                            format!(
                                "\"author_info\":{{{}}},\"counter\":{}",
                                author_info,
                                t.counter()
                            )
                        } else {
                            "".to_string()
                        }
                    }
                    Screen::SignSufficientCrypto(ref s) => {
                        if let Some((_, _, author_info)) = s.key_selected() {
                            format!(
                                "\"author_info\":{{{}}},\"counter\":{}",
                                author_info,
                                s.counter()
                            )
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string(),
                },
                Modal::LogRight => {
                    match db_handling::interface_signer::history_hex_checksum(dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        }
                    }
                }
                Modal::ManageMetadata(network_version) => match new_navstate.screen {
                    Screen::NetworkDetails(ref network_specs_key) => {
                        match db_handling::interface_signer::metadata_details(
                            dbname,
                            network_specs_key,
                            network_version,
                        ) {
                            Ok(a) => a,
                            Err(e) => {
                                new_navstate.alert = Alert::Error;
                                errorline.push_str(&<Signer>::show(&e));
                                "".to_string()
                            }
                        }
                    }
                    _ => "".to_string(),
                },
                Modal::SufficientCryptoReady(ref a) => match new_navstate.screen {
                    Screen::SignSufficientCrypto(ref s) => {
                        if let Some((_, _, author_info)) = s.key_selected() {
                            format!("\"author_info\":{{{}}},{}", author_info, a)
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string(),
                },
                Modal::TypesInfo => {
                    match db_handling::interface_signer::show_types_status(dbname) {
                        Ok(a) => a,
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "".to_string()
                        }
                    }
                }
                Modal::SelectSeed => {
                    match db_handling::interface_signer::print_all_seed_names_with_identicons(
                        dbname,
                        &self.seed_names,
                    ) {
                        Ok(a) => format!("\"seedNameCards\":{}", a),
                        Err(e) => {
                            new_navstate.alert = Alert::Error;
                            errorline.push_str(&<Signer>::show(&e));
                            "\"seedNameCards\":[]".to_string()
                        }
                    }
                }
                _ => "".to_string(),
            };

            //Prepare alert details
            //Important! No errors could be handled in this block!
            let alert_details = match new_navstate.alert {
                Alert::Error => format!("\"error\":\"{}\"", errorline),
                Alert::ErrorDisplay => format!("\"error\":\"{}\"", errorline),
                Alert::Empty => "".to_string(),
                Alert::Shield => "\"shield_state\":\"unknown\"".to_string(),
            };

            self.navstate = new_navstate;
            format!(
                "\"screenData\":{{{}}},\"modalData\":{{{}}},\"alertData\":{{{}}}",
                screen_details, modal_details, alert_details
            )
        } else {
            "\"error\":\"db not initialized\"".to_string()
        }
    }

    fn toggle_modal(&self, modal: Modal) -> Modal {
        if modal == self.navstate.modal {
            Modal::Empty
        } else {
            modal
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
            output.push_str(&format!("\"screen\":\"{}\",\"screenLabel\":\"{}\",\"back\":{},\"footer\":{},\"footerButton\":\"{}\",\"rightButton\":\"{}\",\"screenNameType\":\"{}\",", screen_name, self.get_screen_label(), screen.has_back(), self.get_footer(), self.get_active_navbutton(), self.get_right_button(), self.get_screen_name_type()));
        }
        output.push_str(&format!("\"modal\":\"{}\",", modal.get_name()));
        output.push_str(&format!("\"alert\":\"{}\",", alert.get_name()));
        output.push_str(details);
        output.push('}');
        output
    }

    ///Generate screen label taking into account state
    fn get_screen_label(&self) -> String {
        match &self.navstate.screen {
            Screen::KeyDetails(ref address_state) => {
                if address_state.is_root() {
                    String::from("Seed Key")
                } else {
                    String::from("Derived Key")
                }
            }
            a => a.get_default_label(),
        }
    }

    fn get_footer(&self) -> bool {
        match self.navstate.screen {
            Screen::Log => true,
            Screen::LogDetails(_) => true,
            Screen::Scan => true,
            Screen::Transaction(_) => false,
            Screen::SeedSelector => true,
            Screen::Keys(_) => true,
            Screen::KeyDetails(_) => false,
            Screen::KeyDetailsMulti(_) => false,
            Screen::NewSeed => false,
            Screen::RecoverSeedName(_) => false,
            Screen::RecoverSeedPhrase(_) => false,
            Screen::DeriveKey(_) => false,
            Screen::Settings => true,
            Screen::Verifier => false,
            Screen::ManageNetworks => false,
            Screen::NetworkDetails(_) => false,
            Screen::SelectSeedForBackup => false,
            Screen::SignSufficientCrypto(_) => false,
            Screen::Documents => false,
            Screen::Nowhere => true,
        }
    }

    ///Decide which footer button should shine
    fn get_active_navbutton(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "Log",
            Screen::LogDetails(_) => "Log",
            Screen::Scan => "Scan",
            Screen::Transaction(_) => "Scan",
            Screen::SeedSelector => "Keys",
            Screen::Keys(_) => "Keys",
            Screen::KeyDetails(_) => "Keys",
            Screen::KeyDetailsMulti(_) => "Keys",
            Screen::NewSeed => "Keys",
            Screen::RecoverSeedName(_) => "Keys",
            Screen::RecoverSeedPhrase(_) => "Keys",
            Screen::DeriveKey(_) => "Keys",
            Screen::Settings => "Settings",
            Screen::Verifier => "Settings",
            Screen::ManageNetworks => "Settings",
            Screen::NetworkDetails(_) => "Settings",
            Screen::SelectSeedForBackup => "Settings",
            Screen::SignSufficientCrypto(_) => "Settings",
            Screen::Documents => "Settings",
            Screen::Nowhere => "None",
        }
        .to_string()
    }

    ///Should header have some button on the right?
    fn get_right_button(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "LogRight",
            Screen::LogDetails(_) => "None",
            Screen::Scan => "None",
            Screen::Transaction(_) => "None",
            Screen::SeedSelector => "NewSeed",
            Screen::Keys(ref keys_state) => {
                if let Modal::Backup(_) = self.navstate.modal {
                    "None"
                } else if keys_state.is_multiselect() {
                    "MultiSelect"
                } else {
                    "Backup"
                }
            }
            Screen::KeyDetails(_) => "KeyMenu",
            Screen::KeyDetailsMulti(_) => "KeyMenu",
            Screen::NewSeed => "None",
            Screen::RecoverSeedName(_) => "None",
            Screen::RecoverSeedPhrase(_) => "None",
            Screen::DeriveKey(_) => "None",
            Screen::Settings => "None",
            Screen::Verifier => "None",
            Screen::ManageNetworks => "TypesInfo",
            Screen::NetworkDetails(_) => "NDMenu",
            Screen::SelectSeedForBackup => "Backup",
            Screen::SignSufficientCrypto(_) => "None",
            Screen::Documents => "None",
            Screen::Nowhere => "None",
        }
        .to_string()
    }

    ///Determine whether screen name should be h1 or h4
    fn get_screen_name_type(&self) -> String {
        match self.navstate.screen {
            Screen::Log => "h4",
            Screen::LogDetails(_) => "h4",
            Screen::Scan => "h1",
            Screen::Transaction(_) => "h1",
            Screen::SeedSelector => "h1",
            Screen::Keys(_) => "h4",
            Screen::KeyDetails(_) => "h4",
            Screen::KeyDetailsMulti(_) => "h4",
            Screen::NewSeed => "h1",
            Screen::RecoverSeedName(_) => "h1",
            Screen::RecoverSeedPhrase(_) => "h1",
            Screen::DeriveKey(_) => "h1",
            Screen::Settings => "h4",
            Screen::Verifier => "h4",
            Screen::ManageNetworks => "h4",
            Screen::NetworkDetails(_) => "h4",
            Screen::SelectSeedForBackup => "h4",
            Screen::SignSufficientCrypto(_) => "h1",
            Screen::Documents => "h4",
            Screen::Nowhere => "h4",
        }
        .to_string()
    }

    fn correct_seed_selector(&self) -> Navstate {
        if self.seed_names.is_empty() {
            Navstate {
                screen: Screen::SeedSelector,
                modal: Modal::NewSeedMenu,
                alert: Alert::Empty,
            }
        } else {
            Navstate::clean_screen(Screen::SeedSelector)
        }
    }
}

impl Navstate {
    pub fn clean_screen(screen: Screen) -> Navstate {
        Navstate {
            screen,
            modal: Modal::Empty,
            alert: Alert::Empty,
        }
    }

    ///Check if keyboard should be shown
    fn keyboard(&self) -> bool {
        if let Modal::Empty = self.modal {
            matches!(self.alert, Alert::Empty)
        } else {
            false
        }
    }
}

fn process_hex_address_key_address_details(
    hex_address_key: &str,
    dbname: &str,
) -> Result<(MultiSigner, AddressDetails), ErrorSigner> {
    let address_key = AddressKey::from_hex(hex_address_key)?;
    let multisigner = address_key.multi_signer::<Signer>(AddressKeySource::Extra(
        ExtraAddressKeySourceSigner::Interface,
    ))?;
    let address_details = db_handling::helpers::get_address_details(dbname, &address_key)?;
    Ok((multisigner, address_details))
}

fn process_hex_address_key(hex_address_key: &str) -> Result<MultiSigner, ErrorSigner> {
    let address_key = AddressKey::from_hex(hex_address_key)?;
    address_key.multi_signer::<Signer>(AddressKeySource::Extra(
        ExtraAddressKeySourceSigner::Interface,
    ))
}

//TODO: tests should probably be performed here, as static object in lib.rs
//will only allow for 1-2 integration tests
