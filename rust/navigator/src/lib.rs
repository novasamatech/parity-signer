//! This is experimental cross-platform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use db_handling::identities::export_all_addrs;
//do we support mutex?
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

use definitions::navigation::{ActionResult, MKeysInfoExport, MKeysNew, PathAndNetwork};
use parity_scale_codec::Encode;
use qrcode_rtx::make_data_packs;

mod error;

mod actions;
pub use actions::Action;
pub mod alerts;
pub mod modals;
mod navstate;
use navstate::State;
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
        State::default()
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
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate.perform(action, details_str, secret_seed_phrase)
}

/// Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(dbname: &str, seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate.init_navigation(dbname, seed_names)
}

/// Should be called when seed names are modified in native to synchronize data
pub fn update_seed_names(seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate.update_seed_names(seed_names);

    Ok(())
}

/// Export key info with derivations.
pub fn export_key_info(
    dbname: &str,
    selected_names: Option<HashMap<String, Vec<PathAndNetwork>>>,
) -> Result<MKeysInfoExport> {
    let export_all_addrs = export_all_addrs(dbname, selected_names)?;

    let data = [&[0x53, 0xff, 0xde], export_all_addrs.encode().as_slice()].concat();
    let frames = make_data_packs(&data, 128).map_err(|e| Error::DataPacking(e.to_string()))?;

    Ok(MKeysInfoExport { frames })
}

/// Get keys by seed name
pub fn keys_by_seed_name(dbname: &str, seed_name: &str) -> Result<MKeysNew> {
    Ok(db_handling::interface_signer::keys_by_seed_name(
        dbname, seed_name,
    )?)
}
