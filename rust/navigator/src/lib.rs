//! This is experimental cross-platform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

//do we support mutex?
use lazy_static::lazy_static;
use std::sync::Mutex;

use definitions::{keyring::NetworkSpecsKey, navigation::ActionResult};

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

pub use crate::error::{Error, Result};

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
) -> Result<ActionResult> {
    //If can't lock - debounce failed, ignore action
    //
    //guard is defined here to outline lifetime properly
    let mut state = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    state.perform(action, details_str, secret_seed_phrase)
}

/// Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(dbname: &str, seed_names: Vec<String>) -> Result<()> {
    //This operation has to happen; lock thread and do not ignore.
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    (*navstate).dbname = Some(dbname.to_string());
    (*navstate).seed_names = seed_names;

    let networks = db_handling::helpers::get_all_networks(dbname)?;
    for x in &networks {
        (*navstate)
            .networks
            .push(NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption));
    }

    Ok(())
}

/// Should be called when seed names are modified in native to synchronize data
pub fn update_seed_names(seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    (*navstate).seed_names = seed_names;

    Ok(())
}
