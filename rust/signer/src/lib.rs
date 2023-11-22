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
use db_handling::identities::{import_all_addrs, inject_derivations_has_pwd};
use db_handling::{Error as DbHandlingError, Error};
use definitions::keyring::AddressKey;
use lazy_static::lazy_static;
use navigator::Error as NavigatorError;
use sled::Db;
use std::{
    collections::HashMap,
    fmt::Display,
    str::FromStr,
    sync::{Arc, RwLock},
};
use transaction_parsing::dynamic_derivations::process_dynamic_derivations;
use transaction_parsing::entry_to_transactions_with_decoding;
use transaction_parsing::Error as TxParsingError;
use transaction_signing::SufficientContent;

lazy_static! {
    static ref DB: Arc<RwLock<Option<Db>>> = Arc::new(RwLock::new(None));
}

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
    DbNotInitialized,
    /// Tried to load metadata for unknown network.
    LoadMetaUnknownNetwork {
        /// Name of the network not known to the Vault.
        name: String,
    },
    /// Tried to add specs already present in Vault.
    SpecsKnown {
        name: String,
        encryption: Encryption,
    },
    /// The metadata with this network version already in db.
    MetadataKnown {
        name: String,
        version: u32,
    },
    /// Do not have an up-to-date version of metadata in db
    MetadataOutdated {
        name: String,
        have: u32,
        want: u32,
    },
    /// Tried to sign transaction with an unknown network
    UnknownNetwork {
        genesis_hash: H256,
        encryption: Encryption,
    },
    /// No metadata for a known network found in store
    NoMetadata {
        name: String,
    },
    /// Provided password is incorrect
    WrongPassword,
    /// Database schema mismatch
    DbSchemaMismatch,
}

impl From<NavigatorError> for ErrorDisplayed {
    fn from(e: NavigatorError) -> Self {
        match &e {
            NavigatorError::MutexPoisoned => Self::MutexPoisoned,
            NavigatorError::DbNotInitialized => Self::DbNotInitialized,
            NavigatorError::TransactionParsing(t) => match t {
                TxParsingError::LoadMetaUnknownNetwork { name } => {
                    Self::LoadMetaUnknownNetwork { name: name.clone() }
                }
                TxParsingError::SpecsKnown { name, encryption } => Self::SpecsKnown {
                    name: name.clone(),
                    encryption: *encryption,
                },
                TxParsingError::MetadataKnown { name, version } => Self::MetadataKnown {
                    name: name.clone(),
                    version: *version,
                },
                TxParsingError::AllExtensionsParsingFailed {
                    ref network_name,
                    ref errors,
                } => {
                    if let Some((want, parser::Error::WrongNetworkVersion { in_metadata, .. })) =
                        errors.first()
                    {
                        Self::MetadataOutdated {
                            name: network_name.to_string(),
                            have: *in_metadata,
                            want: *want,
                        }
                    } else {
                        Self::Str { s: format!("{e}") }
                    }
                }
                TxParsingError::UnknownNetwork {
                    genesis_hash,
                    encryption,
                } => Self::UnknownNetwork {
                    genesis_hash: *genesis_hash,
                    encryption: *encryption,
                },
                TxParsingError::NoMetadata { name } => Self::NoMetadata {
                    name: name.to_string(),
                },
                _ => Self::Str { s: format!("{e}") },
            },
            NavigatorError::TransactionSigning(transaction_signing::Error::WrongPassword) => {
                Self::WrongPassword
            }
            _ => Self::Str { s: format!("{e}") },
        }
    }
}

impl From<DbHandlingError> for ErrorDisplayed {
    fn from(e: DbHandlingError) -> Self {
        match &e {
            Error::DbSchemaMismatch { .. } => Self::DbSchemaMismatch,
            _ => Self::Str { s: format!("{e}") },
        }
    }
}

impl From<anyhow::Error> for ErrorDisplayed {
    fn from(e: anyhow::Error) -> Self {
        Self::Str {
            s: format!("error on signer side: {e}"),
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
            qr_reader_phone::Error::BananaSplitError(e) => Self::BananaSplit { s: format!("{e}") },
            other => QrSequenceDecodeError::GenericError {
                s: format!("{other}"),
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
    let val = Some(sled::open(dbname).map_err(|e| ErrorDisplayed::from(e.to_string()))?);

    *DB.write().unwrap() = val;
    init_logging("Vault".to_string());
    Ok(navigator::init_navigation(
        DB.clone().read().unwrap().as_ref().unwrap().clone(),
        seed_names,
    )?)
}

/// Should be called every time any change could have been done to seeds. Accepts updated list of
/// seeds, completely disregards previously known list
fn update_seed_names(seed_names: Vec<String>) -> Result<(), ErrorDisplayed> {
    Ok(navigator::update_seed_names(seed_names)?)
}

/// Determines estimated required number of multiframe QR that should be gathered before decoding
/// is attempted
fn qrparser_get_packets_total(data: &str, cleaned: bool) -> anyhow::Result<u32, ErrorDisplayed> {
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

    Ok(res?)
}

fn get_db() -> Result<sled::Db, ErrorDisplayed> {
    DB.read()
        .unwrap()
        .clone()
        .ok_or(ErrorDisplayed::DbNotInitialized)
}

/// Exports secret (private) key as QR code
///
/// `public_key` is hex-encoded public key of the key to export. Can be taken from [`MKeyDetails`]
/// `network_specs_key` is hex-encoded [`NetworkSpecsKey`]. Can be taken from [`MSCNetworkInfo`]
fn generate_secret_key_qr(
    public_key: &str,
    expected_seed_name: &str,
    network_specs_key: &str,
    seed_phrase: &str,
    key_password: Option<String>,
) -> Result<MKeyDetails, ErrorDisplayed> {
    db_handling::identities::export_secret_key(
        &get_db()?,
        public_key,
        expected_seed_name,
        network_specs_key,
        seed_phrase,
        key_password,
    )
    .map_err(|e| e.to_string().into())
}

fn import_derivations(seed_derived_keys: Vec<SeedKeysPreview>) -> Result<(), ErrorDisplayed> {
    import_all_addrs(&get_db()?, seed_derived_keys).map_err(|e| e.to_string().into())
}

/// Calculate if derivation path has a password
fn populate_derivations_has_pwd(
    seeds: HashMap<String, String>,
    seed_derived_keys: Vec<SeedKeysPreview>,
) -> Result<Vec<SeedKeysPreview>, anyhow::Error> {
    inject_derivations_has_pwd(seed_derived_keys, seeds).map_err(Into::into)
}

fn preview_dynamic_derivations(
    seeds: HashMap<String, String>,
    payload: String,
) -> Result<DDPreview, ErrorDisplayed> {
    process_dynamic_derivations(&get_db()?, seeds, &payload).map_err(|e| e.to_string().into())
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
) -> Result<DerivationCheck, ErrorDisplayed> {
    Ok(db_handling::interface_signer::dynamic_path_check(
        &get_db()?,
        seed_name,
        path,
        network,
    ))
}

fn try_create_address(
    seed_name: &str,
    seed_phrase: &str,
    path: &str,
    network: &str,
) -> anyhow::Result<(), ErrorDisplayed> {
    let network = NetworkSpecsKey::from_hex(network).map_err(|e| format!("{e}"))?;
    db_handling::identities::try_create_address(&get_db()?, seed_name, seed_phrase, path, &network)
        .map_err(|e| e.to_string().into())
}

/// Must be called with `DecodeSequenceResult::DynamicDerivationTransaction` payload
fn sign_dd_transaction(
    payload: &[String],
    seeds: HashMap<String, String>,
) -> Result<MSignedTransaction, ErrorDisplayed> {
    navigator::sign_dd_transaction(&get_db()?, payload, seeds).map_err(|e| e.to_string().into())
}

/// Must be called once on normal first start of the app upon accepting conditions; relies on old
/// data being already removed
fn history_init_history_with_cert() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::cold_default::signer_init_with_cert(&get_db()?).map_err(|e| e.to_string().into())
}

/// Must be called once upon jailbreak (removal of general verifier) after all old data was removed
fn history_init_history_no_cert() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::cold_default::signer_init_no_cert(&get_db()?).map_err(|e| e.to_string().into())
}

/// Must be called every time network detector detects network. Sets alert flag in database that could
/// only be reset by full reset or calling [`history_acknowledge_warnings`]
///
/// This changes log, so it is expected to fail all operations that check that database remained
/// intact
fn history_device_was_online() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::device_was_online(&get_db()?).map_err(|e| e.to_string().into())
}

/// Checks if network alert flag was set
fn history_get_warnings() -> anyhow::Result<bool, ErrorDisplayed> {
    db_handling::helpers::get_danger_status(&get_db()?).map_err(|e| e.to_string().into())
}

/// Resets network alert flag; makes record of reset in log
fn history_acknowledge_warnings() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::reset_danger_status_to_safe(&get_db()?)
        .map_err(|e| e.to_string().into())
}

/// Allows frontend to send events into log; TODO: maybe this is not needed
fn history_entry_system(event: Event) -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::history_entry_system(&get_db()?, event)
        .map_err(|e| e.to_string().into())
}

/// Must be called every time seed backup shows seed to user
///
/// Makes record in log
fn history_seed_was_shown(seed_name: &str) -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::seed_name_was_shown(&get_db()?, seed_name.to_string())
        .map_err(|e| e.to_string().into())
}

fn export_key_info(
    seed_name: &str,
    exported_set: ExportedSet,
) -> anyhow::Result<MKeysInfoExport, ErrorDisplayed> {
    navigator::export_key_info(&get_db()?, seed_name, exported_set)
        .map_err(|e| e.to_string().into())
}

fn keys_by_seed_name(seed_name: &str) -> anyhow::Result<MKeysNew, ErrorDisplayed> {
    navigator::keys_by_seed_name(&get_db()?, seed_name).map_err(|e| e.to_string().into())
}

/// Encode binary info into qr code
fn encode_to_qr(payload: &[u8], is_danger: bool) -> anyhow::Result<Vec<u8>, String> {
    use qrcode_static::DataType;
    let sensitivity = if is_danger {
        DataType::Sensitive
    } else {
        DataType::Regular
    };
    qrcode_static::png_qr(payload, sensitivity).map_err(|e| format!("{e}"))
}

/// Get all networks registered within this device
fn get_managed_networks() -> anyhow::Result<MManageNetworks, ErrorDisplayed> {
    Ok(MManageNetworks {
        networks: db_handling::interface_signer::show_all_networks(&get_db()?)
            .map_err(|e| e.to_string())?,
    })
}

fn get_logs() -> anyhow::Result<MLog, ErrorDisplayed> {
    let history = db_handling::manage_history::get_history(&get_db()?)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;
    let log: Vec<_> = history
        .into_iter()
        .map(|(order, entry)| History {
            order: order.stamp(),
            timestamp: entry.timestamp,
            events: entry.events,
        })
        .collect();

    Ok(MLog { log })
}

fn get_log_details(order: u32) -> anyhow::Result<MLogDetails, ErrorDisplayed> {
    let e = db_handling::manage_history::get_history_entry_by_order(&get_db()?, order)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    let timestamp = e.timestamp.clone();

    let events = entry_to_transactions_with_decoding(&get_db()?, e)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    Ok(MLogDetails { timestamp, events })
}

fn clear_log_history() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::clear_history(&get_db()?)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn handle_log_comment(string_from_user: &str) -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::manage_history::history_entry_user(&get_db()?, string_from_user)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn get_seeds(names_phone_knows: &[String]) -> anyhow::Result<MSeeds, ErrorDisplayed> {
    let seed_name_cards = db_handling::interface_signer::get_all_seed_names_with_identicons(
        &get_db()?,
        names_phone_knows,
    )
    .map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    Ok(MSeeds { seed_name_cards })
}

fn get_key_set_public_key(
    address: &str,
    network_specs_key: &str,
) -> anyhow::Result<MKeyDetails, ErrorDisplayed> {
    let address_key =
        AddressKey::from_hex(address).map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    let network_specs_key = NetworkSpecsKey::from_hex(network_specs_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    let address_details = db_handling::helpers::get_address_details(&get_db()?, &address_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;

    db_handling::interface_signer::export_key(
        &get_db()?,
        address_key.multi_signer(),
        &address_details.seed_name,
        &network_specs_key,
    )
    .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn remove_derived_key(
    address: &str,
    network_specs_key: &str,
) -> anyhow::Result<(), ErrorDisplayed> {
    let address_key =
        AddressKey::from_hex(address).map_err(|e| ErrorDisplayed::from(e.to_string()))?;
    let network_specs_key = NetworkSpecsKey::from_hex(network_specs_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;
    db_handling::identities::remove_key(&get_db()?, address_key.multi_signer(), &network_specs_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn remove_key_set(seed_name: &str) -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::identities::remove_seed(&get_db()?, seed_name)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn get_managed_network_details(
    network_key: &str,
) -> anyhow::Result<MNetworkDetails, ErrorDisplayed> {
    let network_key = NetworkSpecsKey::from_hex(network_key).map_err(|e| format!("{e}"))?;
    db_handling::interface_signer::network_details_by_key(&get_db()?, &network_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn remove_metadata_on_managed_network(
    network_key: &str,
    metadata_specs_version: &str,
) -> anyhow::Result<(), ErrorDisplayed> {
    let network_key = NetworkSpecsKey::from_hex(network_key).map_err(|e| format!("{e}"))?;
    let version = metadata_specs_version
        .parse::<u32>()
        .map_err(|e| format!("{e}"))?;
    db_handling::helpers::remove_metadata(&get_db()?, &network_key, version)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn seed_phrase_guess_words(user_input: &str) -> Vec<String> {
    db_handling::interface_signer::guess(user_input)
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
}

fn get_verifier_details() -> anyhow::Result<MVerifierDetails, ErrorDisplayed> {
    Ok(db_handling::helpers::get_general_verifier(&get_db()?)
        .map_err(|e| e.to_string())?
        .show_card())
}

fn remove_managed_network(network_key: &str) -> anyhow::Result<(), ErrorDisplayed> {
    let network_key = NetworkSpecsKey::from_hex(network_key).map_err(|e| format!("{e}"))?;
    db_handling::helpers::remove_network(&get_db()?, &network_key)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn print_new_seed(new_seed_name: &str) -> anyhow::Result<MNewSeedBackup, ErrorDisplayed> {
    db_handling::interface_signer::print_new_seed(new_seed_name)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn validate_seed_phrase(seed_phrase: &str) -> bool {
    db_handling::helpers::validate_mnemonic(seed_phrase)
}

fn create_key_set(
    seed_name: &str,
    seed_phrase: &str,
    networks: Vec<String>,
) -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::identities::create_key_set(&get_db()?, seed_name, seed_phrase, networks)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))
}

fn check_db_version() -> anyhow::Result<(), ErrorDisplayed> {
    db_handling::helpers::assert_db_version(&get_db()?).map_err(ErrorDisplayed::from)
}

fn get_keys_for_signing() -> Result<MSignSufficientCrypto, ErrorDisplayed> {
    let identities = db_handling::interface_signer::print_all_identities(&get_db()?)
        .map_err(|e| ErrorDisplayed::from(e.to_string()))?;
    Ok(MSignSufficientCrypto { identities })
}

fn validate_key_password(
    address_key: &str,
    seed_phrase: &str,
    password: &str,
) -> Result<bool, ErrorDisplayed> {
    let address_key = AddressKey::from_hex(address_key).map_err(|e| format!("{e}"))?;
    db_handling::identities::validate_key_password(&get_db()?, &address_key, seed_phrase, password)
        .map_err(|e| e.into())
}

fn sign_metadata_with_key(
    network_key: &str,
    metadata_specs_version: &str,
    signing_address_key: &str,
    seed_phrase: &str,
    password: Option<String>,
) -> Result<MSufficientCryptoReady, ErrorDisplayed> {
    let network_key = NetworkSpecsKey::from_hex(network_key).map_err(|e| format!("{e}"))?;
    let version = metadata_specs_version
        .parse::<u32>()
        .map_err(|e| format!("{e}"))?;
    let address_key = AddressKey::from_hex(signing_address_key).map_err(|e| format!("{e}"))?;
    navigator::sign_sufficient_content(
        &get_db()?,
        &address_key,
        SufficientContent::LoadMeta(network_key, version),
        seed_phrase,
        &password.unwrap_or("".to_owned()),
    )
    .map_err(|e| e.into())
}

fn sign_network_spec_with_key(
    network_key: &str,
    signing_address_key: &str,
    seed_phrase: &str,
    password: Option<String>,
) -> Result<MSufficientCryptoReady, ErrorDisplayed> {
    let network_key = NetworkSpecsKey::from_hex(network_key).map_err(|e| format!("{e}"))?;
    let address_key = AddressKey::from_hex(signing_address_key).map_err(|e| format!("{e}"))?;
    navigator::sign_sufficient_content(
        &get_db()?,
        &address_key,
        SufficientContent::AddSpecs(network_key),
        seed_phrase,
        &password.unwrap_or("".to_owned()),
    )
    .map_err(|e| e.into())
}

/// Must be called once to initialize logging from Rust in development mode.
///
/// Do not use in production.
#[cfg(target_os = "android")]
fn init_logging(tag: String) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace) // limit log level
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
