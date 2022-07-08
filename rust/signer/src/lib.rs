// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! This crate serves as interface between native frontend and Rust code. Try to avoid placing any
//! logic here, just interfacing. When porting to new platform, all Rust changes will probably
//! happen here.

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::let_unit_value)]

mod ffi_types;

use crate::ffi_types::*;
use std::fmt::Display;

/// Container for severe error message
///
/// TODO: implement properly or remove completely
#[derive(Debug)]
pub enum ErrorDisplayed {
    /// String description of error
    Str {
        /// Error description
        s: String,
    },
}

impl From<anyhow::Error> for ErrorDisplayed {
    fn from(e: anyhow::Error) -> Self {
        Self::Str {
            s: format!("error on signer side: {}", e),
        }
    }
}

impl Display for ErrorDisplayed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

include!(concat!(env!("OUT_DIR"), "/signer.uniffi.rs"));

fn action_get_name(action: &Action) -> Option<FooterButton> {
    match action {
        Action::NavbarLog => Some(FooterButton::Log),
        Action::NavbarScan => Some(FooterButton::Scan),
        Action::NavbarKeys => Some(FooterButton::Keys),
        Action::NavbarSettings => Some(FooterButton::Settings),
        Action::GoBack => Some(FooterButton::Back),
        _ => None,
    }
}

/// Perform action in frontend.
///
/// This call should be debounced.
///
/// Action tries to acquire lock on app state mutex and is ignored on failure.
///
/// 'seed_phrase' field is zeroized, it is expected to be used for secrets only.
///
/// `details` field is not always zeroized.
///
/// App view contents are returned as result, this should be sufficient to render view.
fn backend_action(
    action: Action,
    details: &str,
    seed_phrase: &str,
) -> Result<Option<ActionResult>, ErrorDisplayed> {
    navigator::do_action(action, details, seed_phrase).map_err(|s| ErrorDisplayed::Str { s })
}

/// Should be called once at start of the app and could be called on app reset
///
/// Accepts list of seed names to avoid calling [`update_seed_names`] every time
fn init_navigation(dbname: &str, seed_names: Vec<String>) {
    navigator::init_navigation(dbname, seed_names)
}

/// Should be called every time any change could have been done to seeds. Accepts updated list of
/// seeds, completely disregards previously known list
fn update_seed_names(seed_names: Vec<String>) {
    navigator::update_seed_names(seed_names)
}

/// Determines estimated required number of multiframe QR that should be gathered before decoding
/// is attempted
fn qrparser_get_packets_total(data: &str, cleaned: bool) -> anyhow::Result<u32, ErrorDisplayed> {
    qr_reader_phone::get_length(data, cleaned).map_err(Into::into)
}

/// Attempts to convert QR data (transfered as json-like string) into decoded but not parsed UOS
/// payload
///
/// `cleaned` is platform-specific flag indicating whether QR payloads have QR prefix stripped by
/// QR parsing code
fn qrparser_try_decode_qr_sequence(
    data: &str,
    cleaned: bool,
) -> anyhow::Result<String, anyhow::Error> {
    qr_reader_phone::decode_sequence(data, cleaned)
}

/// Checks derivation path for validity and collisions
///
/// Returns struct that has information on collisions, presence of password and validity of path;
/// in case of valid path without collisions frontend should make a decision on whether to access
/// secure storage already or check password by requesting user to re-type it, so this could not be
/// isolated in backend navigation for now.
fn substrate_path_check(
    seed_name: &str,
    path: &str,
    network: &str,
    dbname: &str,
) -> DerivationCheck {
    db_handling::interface_signer::dynamic_path_check(dbname, seed_name, path, network)
}

/// Must be called once on normal first start of the app upon accepting conditions; relies on old
/// data being already removed
fn history_init_history_with_cert(dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::cold_default::signer_init_with_cert(dbname).map_err(|e| e.anyhow())
}

/// Must be called once upon jailbreak (removal of general verifier) after all old data was removed
fn history_init_history_no_cert(dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::cold_default::signer_init_no_cert(dbname).map_err(|e| e.anyhow())
}

/// Must be called every time network detector detects network. Sets alert flag in database that could
/// only be reset by full reset or calling [`history_acknowledge_warnings`]
///
/// This changes log, so it is expected to fail all operations that check that database remained
/// intact
fn history_device_was_online(dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::manage_history::device_was_online(dbname).map_err(|e| e.anyhow())
}

/// Checks if network alert flag was set
fn history_get_warnings(dbname: &str) -> anyhow::Result<bool, anyhow::Error> {
    db_handling::helpers::get_danger_status(dbname).map_err(|e| e.anyhow())
}

/// Resets network alert flag; makes record of reset in log
fn history_acknowledge_warnings(dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::manage_history::reset_danger_status_to_safe(dbname).map_err(|e| e.anyhow())
}

/// Allows frontend to send events into log; TODO: maybe this is not needed
fn history_entry_system(event: Event, dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::manage_history::history_entry_system(dbname, event).map_err(|e| e.anyhow())
}

/// Must be called every time seed backup shows seed to user
///
/// Makes record in log
fn history_seed_name_was_shown(seed_name: &str, dbname: &str) -> anyhow::Result<(), anyhow::Error> {
    db_handling::manage_history::seed_name_was_shown(dbname, seed_name.to_string())
        .map_err(|e| e.anyhow())
}

/// Test function to show all transaction cards
fn get_all_tx_cards() -> TransactionCardSet {
    match transaction_parsing::test_all_cards::make_all_cards() {
        TransactionAction::Derivations { content, .. } => content,
        TransactionAction::Sign { content, .. } => content,
        TransactionAction::Stub { s, .. } => s,
        TransactionAction::Read { r } => r,
    }
}

/// Test function to show all possible log events
fn get_all_log_cards() -> String {
    String::new()
    // TODO: definitions::history::print_all_events()
}

/// Must be called once to initialize logging from Rust in development mode.
///
/// Do not use in production.
#[cfg(target_os = "android")]
fn init_logging(tag: String) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace) // limit log level
            .with_tag(tag) // logs will show under mytag tag
            .with_filter(
                // configure messages for specific crate
                android_logger::FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );
}

/// Placeholder to init logging on non-android platforms
///
/// TODO: is this used?
#[cfg(not(target_os = "android"))]
fn init_logging(_tag: String) {
    env_logger::init();
}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
    //use super::*;
}
