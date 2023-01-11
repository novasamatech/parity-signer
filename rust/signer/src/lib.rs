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
use db_handling::identities::import_all_addrs;
use definitions::derivations::{ExportAddrs, ExportAddrsContainer, SeedKeysPreviewSummary};
use std::path::Path;
use std::{collections::HashMap, fmt::Display, str::FromStr};
use transaction_parsing::prepare_derivations_preview;

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
    MutexPoisoned,
}

impl From<navigator::Error> for ErrorDisplayed {
    fn from(e: navigator::Error) -> Self {
        match e {
            navigator::Error::MutexPoisoned => Self::MutexPoisoned,
            _ => Self::Str {
                s: format!("{}", e),
            },
        }
    }
}

impl From<anyhow::Error> for ErrorDisplayed {
    fn from(e: anyhow::Error) -> Self {
        Self::Str {
            s: format!("error on signer side: {}", e),
        }
    }
}

impl FromStr for ErrorDisplayed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ErrorDisplayed::Str { s: s.to_string() })
    }
}

impl From<String> for ErrorDisplayed {
    fn from(s: String) -> Self {
        ErrorDisplayed::Str { s }
    }
}

impl Display for ErrorDisplayed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

/// An error type for QR sequence decoding errors.
#[derive(Debug)]
pub enum QrSequenceDecodeError {
    BananaSplitWrongPassword,
    BananaSplit { s: String },
    GenericError { s: String },
}

impl Display for QrSequenceDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

impl From<qr_reader_phone::Error> for QrSequenceDecodeError {
    fn from(value: qr_reader_phone::Error) -> Self {
        match value {
            qr_reader_phone::Error::BananaSplitWrongPassword => Self::BananaSplitWrongPassword,
            qr_reader_phone::Error::BananaSplitError(e) => Self::BananaSplit {
                s: format!("{}", e),
            },
            other => QrSequenceDecodeError::GenericError {
                s: format!("{}", other),
            },
        }
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
/// `seed_phrase` field is zeroized, it is expected to be used for secrets only.
///
/// `details` field is not always zeroized.
///
/// App view contents are returned as result, this should be sufficient to render view.
fn backend_action(
    action: Action,
    details: &str,
    seed_phrase: &str,
) -> Result<ActionResult, ErrorDisplayed> {
    Ok(navigator::do_action(action, details, seed_phrase)?)
}

/// Should be called once at start of the app and could be called on app reset
///
/// Accepts list of seed names to avoid calling [`update_seed_names`] every time
fn init_navigation(dbname: &str, seed_names: Vec<String>) -> Result<(), ErrorDisplayed> {
    init_logging("Signer".to_string());
    Ok(navigator::init_navigation(dbname, seed_names)?)
}

/// Should be called every time any change could have been done to seeds. Accepts updated list of
/// seeds, completely disregards previously known list
fn update_seed_names(seed_names: Vec<String>) -> Result<(), ErrorDisplayed> {
    Ok(navigator::update_seed_names(seed_names)?)
}

/// Determines estimated required number of multiframe QR that should be gathered before decoding
/// is attempted
fn qrparser_get_packets_total(data: &str, cleaned: bool) -> anyhow::Result<u32, ErrorDisplayed> {
    log::warn!("get packets total {}", data);
    qr_reader_phone::get_length(data, cleaned).map_err(|e| e.to_string().into())
}

/// Attempts to convert QR data (transfered as json-like string) into decoded but not parsed UOS
/// payload
///
/// `cleaned` is platform-specific flag indicating whether QR payloads have QR prefix stripped by
/// QR parsing code
fn qrparser_try_decode_qr_sequence(
    data: &[String],
    password: Option<String>,
    cleaned: bool,
) -> anyhow::Result<DecodeSequenceResult, QrSequenceDecodeError> {
    let res = qr_reader_phone::decode_sequence(data, &password, cleaned);

    log::warn!("result in qr_parser_try_decode_qr {:?}", res);

    Ok(res?)
}

/// Exports secret (private) key as QR code
///
/// `public_key` is hex-encoded public key of the key to export. Can be taken from [`MKeyDetails`]
/// `network_specs_key` is hex-encoded [`NetworkSpecsKey`]. Can be taken from [`MSCNetworkInfo`]
fn generate_secret_key_qr(
    dbname: &str,
    public_key: &str,
    expected_seed_name: &str,
    network_specs_key: &str,
    seed_phrase: &str,
    key_password: Option<String>,
) -> Result<MKeyDetails, anyhow::Error> {
    db_handling::identities::export_secret_key(
        dbname,
        public_key,
        expected_seed_name,
        network_specs_key,
        seed_phrase,
        key_password,
    )
    .map_err(Into::into)
}

fn import_derivations(
    dbname: &str,
    seed_derived_keys: SeedKeysPreviewSummary,
) -> Result<(), anyhow::Error> {
    import_all_addrs(dbname, seed_derived_keys).map_err(Into::into)
}

/// Calculate if derivation path has a password
fn get_derivations_preview<P>(
    dbname: P,
    export_info: ExportAddrsContainer,
    seeds: HashMap<String, String>,
) -> Result<SeedKeysPreviewSummary, anyhow::Error>
where
    P: AsRef<Path>,
{
    prepare_derivations_preview(dbname, export_info, seeds).map_err(Into::into)
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

fn try_create_address(
    seed_name: &str,
    seed_phrase: &str,
    path: &str,
    network: &str,
    dbname: &str,
) -> anyhow::Result<(), String> {
    let network = NetworkSpecsKey::from_hex(network).map_err(|e| format!("{}", e))?;
    db_handling::identities::try_create_address(seed_name, seed_phrase, path, &network, dbname)
        .map_err(|e| format!("{}", e))
}

/// Must be called once on normal first start of the app upon accepting conditions; relies on old
/// data being already removed
fn history_init_history_with_cert(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::cold_default::signer_init_with_cert(dbname).map_err(|e| format!("{}", e))
}

/// Must be called once upon jailbreak (removal of general verifier) after all old data was removed
fn history_init_history_no_cert(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::cold_default::signer_init_no_cert(dbname).map_err(|e| format!("{}", e))
}

/// Must be called every time network detector detects network. Sets alert flag in database that could
/// only be reset by full reset or calling [`history_acknowledge_warnings`]
///
/// This changes log, so it is expected to fail all operations that check that database remained
/// intact
fn history_device_was_online(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::device_was_online(dbname).map_err(|e| format!("{}", e))
}

/// Checks if network alert flag was set
fn history_get_warnings(dbname: &str) -> anyhow::Result<bool, String> {
    db_handling::helpers::get_danger_status(dbname).map_err(|e| format!("{}", e))
}

/// Resets network alert flag; makes record of reset in log
fn history_acknowledge_warnings(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::reset_danger_status_to_safe(dbname).map_err(|e| format!("{}", e))
}

/// Allows frontend to send events into log; TODO: maybe this is not needed
fn history_entry_system(event: Event, dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::history_entry_system(dbname, event).map_err(|e| format!("{}", e))
}

/// Must be called every time seed backup shows seed to user
///
/// Makes record in log
fn history_seed_name_was_shown(seed_name: &str, dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::seed_name_was_shown(dbname, seed_name.to_string())
        .map_err(|e| format!("{}", e))
}

fn export_key_info(
    dbname: &str,
    selected_names: HashMap<String, ExportedSet>,
) -> anyhow::Result<MKeysInfoExport, String> {
    navigator::export_key_info(dbname, selected_names).map_err(|e| format!("{}", e))
}

fn keys_by_seed_name(dbname: &str, seed_name: &str) -> anyhow::Result<MKeysNew, String> {
    navigator::keys_by_seed_name(dbname, seed_name).map_err(|e| format!("{}", e))
}

/// Encode binary info into qr code
fn encode_to_qr(payload: &[u8], is_danger: bool) -> anyhow::Result<Vec<u8>, String> {
    use qrcode_static::DataType;
    let sensitivity = if is_danger {
        DataType::Sensitive
    } else {
        DataType::Regular
    };
    qrcode_static::png_qr(payload, sensitivity).map_err(|e| format!("{}", e))
}

/// Get all networks registered within this device
fn get_all_networks(dbname: &str) -> anyhow::Result<Vec<MMNetwork>, String> {
    db_handling::interface_signer::show_all_networks(dbname).map_err(|e| format!("{}", e))
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

#[cfg(target_os = "ios")]
fn init_logging(_tag: String) {
    use uniffi::deps::log::LevelFilter;

    let _ = oslog::OsLogger::new("io.parity.signer")
        .level_filter(LevelFilter::Warn)
        .category_level_filter("SIGNER", LevelFilter::Trace)
        .init();
}

/// Placeholder to init logging on non-android platforms
///
/// TODO: is this used?
#[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
fn init_logging(_tag: String) {
    env_logger::init();
}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
    //use super::*;
}
