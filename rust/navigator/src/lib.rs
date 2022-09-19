//! This is experimental cross-platform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use error::Error;
//do we support mutex?
use lazy_static::lazy_static;
use std::sync::{Mutex, TryLockError};

use definitions::{
    keyring::NetworkSpecsKey,
    navigation::{
        ActionResult, MKeys, MLog, MLogDetails, MManageNetworks, MRecoverSeedName, MSeeds,
        MSettings, MSignSufficientCrypto, MTransaction, MVerifierDetails,
    },
};

mod error;

mod actions;
pub use actions::Action;
pub mod alerts;
pub mod modals;
mod navstate;
use navstate::{Navstate, State};
pub mod screens;
#[cfg(test)]
mod tests;

//TODO: multithread here some day!
lazy_static! {
/// Navigation state of the app
///
/// Navigation state is unsafe either way, since it has to persist
/// No matter if here or beyond FFI
    pub static ref STATE: Mutex<State> = Mutex::new(
        State{
            navstate: Navstate::new(),
            dbname: None,
            seed_names: Vec::new(),
            networks: Vec::new(),
        }
    );
}

/// User actions handler.
///
/// This method is called on every user [`Action`] in the UI, performs changes in backend
/// and returns new UI information as [`ActionResult`].
pub fn do_action(
    action: Action,
    details_str: &str,
    secret_seed_phrase: &str,
) -> Result<Option<ActionResult>, String> {
    //If can't lock - debounce failed, ignore action
    //
    //guard is defined here to outline lifetime properly
    let guard = STATE.try_lock();
    match guard {
        Ok(mut state) => state
            .perform(action, details_str, secret_seed_phrase)
            .map(Some)
            .map_err(|e| format!("{}", e)),
        Err(TryLockError::Poisoned(_)) => {
            //TODO: maybe more grace here?
            //Maybe just silently restart navstate? But is it safe?
            panic!("Concurrency error! Restart the app.");
        }
        Err(TryLockError::WouldBlock) => Ok(None),
    }
}

/// Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(dbname: &str, seed_names: Vec<String>) {
    //This operation has to happen; lock thread and do not ignore.
    let guard = STATE.lock();
    match guard {
        Ok(mut navstate) => {
            (*navstate).dbname = Some(dbname.to_string());
            (*navstate).seed_names = seed_names;
            match db_handling::helpers::get_all_networks(dbname) {
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

pub fn get_log() -> Result<MLog, Error> {
    STATE.lock().map_err(|_| Error::StatePoisoned)?.get_log()
}

pub fn get_log_details(order: u32) -> Result<MLogDetails, Error> {
    STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .get_log_details(order)
}

pub fn get_transaction() -> Result<MTransaction, Error> {
    STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .get_transaction()
}

pub fn get_seeds() -> Result<MSeeds, Error> {
    STATE.lock().map_err(|_| Error::StatePoisoned)?.get_seeds()
}

pub fn get_keys() -> Result<MKeys, Error> {
    STATE.lock().map_err(|_| Error::StatePoisoned)?.get_keys()
}

pub fn recover_seed_name(keyboard: bool, seed_name: &str) -> Result<MRecoverSeedName, Error> {
    Ok(STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .recover_seed_name(keyboard, seed_name))
}

pub fn settings() -> Result<MSettings, Error> {
    Ok(STATE.lock().map_err(|_| Error::StatePoisoned)?.settings())
}

pub fn get_verifier() -> Result<MVerifierDetails, Error> {
    STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .get_verifier()
}

pub fn manage_networks() -> Result<MManageNetworks, Error> {
    STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .manage_networks()
}

pub fn get_sign_sufficient_crypto() -> Result<MSignSufficientCrypto, Error> {
    STATE
        .lock()
        .map_err(|_| Error::StatePoisoned)?
        .get_sign_sufficient_crypto()
}

/// Should be called when seed names are modified in native to synchronize data
pub fn update_seed_names(seed_names: Vec<String>) -> Result<(), Error> {
    STATE.lock().map_err(|_| Error::StatePoisoned)?.seed_names = seed_names;
    Ok(())
}
