//! This is experimental crossplatform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

//do we support mutex?
use lazy_static::lazy_static;
use std::sync::{Mutex, TryLockError};

use definitions::{error::Signer, keyring::NetworkSpecsKey};

mod actions;
use actions::Action;
pub mod alerts;
pub mod modals;
mod navstate;
use navstate::{Navstate, State};
pub mod screens;
#[cfg(test)]
mod tests;

//TODO: multithread here some day!
lazy_static! {
///Navigation state of the app
///
///Navigation state is unsafe either way, since it has to persist
///No matter if here or beyond FFI
    pub static ref STATE: Mutex<State> = Mutex::new(
        State{
            navstate: Navstate::new(),
            dbname: None,
            seed_names: Vec::new(),
            networks: Vec::new(),
        }
    );
}

///This should be called from UI; returns new UI information as JSON
pub fn do_action(action_str: &str, details_str: &str, secret_seed_phrase: &str) -> String {
    //If can't lock - debounce failed, ignore action
    //
    //guard is defined here to outline lifetime properly
    let guard = STATE.try_lock();
    match guard {
        Ok(mut state) => {
            let action = Action::parse(action_str);
            let details = (*state).perform(action, details_str, secret_seed_phrase);
            (*state).generate_json(&details)
        }
        Err(TryLockError::Poisoned(_)) => {
            //TODO: maybe more grace here?
            //Maybe just silently restart navstate? But is it safe?
            panic!("Concurrency error! Restart the app.");
        }
        Err(TryLockError::WouldBlock) => "".to_string(),
    }
}

///Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(dbname: &str, seed_names: &str) {
    //This operation has to happen; lock thread and do not ignore.
    let guard = STATE.lock();
    match guard {
        Ok(mut navstate) => {
            (*navstate).dbname = Some(dbname.to_string());
            if !seed_names.is_empty() {
                (*navstate).seed_names = seed_names.split(',').map(|a| a.to_string()).collect();
                (*navstate).seed_names.sort();
                (*navstate).seed_names.dedup();
            } else {
                (*navstate).seed_names = Vec::new();
            }
            match db_handling::network_details::get_all_networks::<Signer>(dbname) {
                Ok(a) => {
                    for x in a.iter() {
                        (*navstate)
                            .networks
                            .push(NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption));
                    }
                }
                Err(e) => println!("No networks could be fetched: {:?}", e),
            };
        }
        Err(_) => {
            //TODO: maybe more grace here?
            panic!("Concurrency error! Restart the app.");
        }
    }
}

///Should be called in the beginning to recall things stored only by phone
pub fn update_seed_names(seed_names: &str) {
    let guard = STATE.lock();
    match guard {
        Ok(mut navstate) => {
            if !seed_names.is_empty() {
                (*navstate).seed_names = seed_names.split(',').map(|a| a.to_string()).collect();
                (*navstate).seed_names.sort();
                (*navstate).seed_names.dedup();
            } else {
                (*navstate).seed_names = Vec::new();
            }
        }
        Err(_) => {
            //TODO: maybe more grace here?
            panic!("Concurrency error! Restart the app.");
        }
    }
}
