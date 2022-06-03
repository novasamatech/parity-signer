//! Displaying and updating history log
//!
//! The Signer keeps a history log of all events that change the database and
//! affect security. It is stored in [`HISTORY`] tree of the cold database.
//!
//! Each history log [`Entry`] contains [`Event`] set and a timestamp. Database
//! key for [`Entry`] value is [`Order`], SCALE-encoded number of the entry.
//!
//! In addition to keeping the log, Signer also displays [`HISTORY`] tree
//! checksum for user to possibly keep the track of.
// TODO: substantial part of this will go obsolete with interface updates;
// some functions are not called at the moment from the user interface - kept
// for now in case they make a return, commented.
use chrono::Utc;
#[cfg(feature = "signer")]
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sled::Batch;

#[cfg(feature = "signer")]
use constants::DANGER;
use constants::HISTORY;
use definitions::{
    error::ErrorSource,
    history::{Entry, Event},
    keyring::Order,
};

#[cfg(feature = "signer")]
use definitions::{
    danger::DangerRecord,
    error_signer::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, NotFoundSigner, Signer},
};

use crate::helpers::{open_db, open_tree};
#[cfg(feature = "signer")]
use crate::{db_transactions::TrDbCold, helpers::make_batch_clear_tree};

/// Print total number of pages, for maximum [`HISTORY_PAGE_SIZE`] number of
/// entries per page.
#[cfg(feature = "signer")]
pub fn history_total_pages(database_name: &str) -> Result<u32, ErrorSigner> {
    use constants::HISTORY_PAGE_SIZE;

    let history = get_history(database_name)?;
    let total_pages = {
        if history.len() % HISTORY_PAGE_SIZE == 0 {
            history.len() / HISTORY_PAGE_SIZE
        } else {
            history.len() / HISTORY_PAGE_SIZE + 1
        }
    };
    Ok(total_pages as u32)
}

/// Get history log contents from the database.
#[cfg(feature = "signer")]
pub fn get_history(database_name: &str) -> Result<Vec<(Order, Entry)>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let mut out: Vec<(Order, Entry)> = Vec::new();
    for (order_encoded, history_entry_encoded) in history.iter().flatten() {
        let order = Order::from_ivec(&order_encoded)?;
        let history_entry = match <Entry>::decode(&mut &history_entry_encoded[..]) {
            Ok(a) => a,
            Err(_) => {
                return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(
                    EntryDecodingSigner::HistoryEntry(order),
                )))
            }
        };
        out.push((order, history_entry));
    }
    out.sort_by(|a, b| b.0.stamp().cmp(&a.0.stamp()));
    Ok(out)
}

/// Get from the database a history log [`Entry`] by `u32` order identifier
/// received from the frontend.
#[cfg(feature = "signer")]
pub fn get_history_entry_by_order(order: u32, database_name: &str) -> Result<Entry, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let mut found = None;
    let order = Order::from_number(order);
    for (order_encoded, history_entry_encoded) in history.iter().flatten() {
        let order_found = Order::from_ivec(&order_encoded)?;
        if order_found == order {
            match <Entry>::decode(&mut &history_entry_encoded[..]) {
                Ok(b) => {
                    found = Some(b);
                    break;
                }
                Err(_) => {
                    return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(
                        EntryDecodingSigner::HistoryEntry(order),
                    )))
                }
            }
        }
    }
    match found {
        Some(a) => Ok(a),
        None => Err(ErrorSigner::NotFound(NotFoundSigner::HistoryEntry(order))),
    }
}

/// Clear Signer history and make a log [`Entry`] that history was cleared.
#[cfg(feature = "signer")]
pub fn clear_history(database_name: &str) -> Result<(), ErrorSigner> {
    let batch = make_batch_clear_tree::<Signer>(database_name, HISTORY)?;
    let events = vec![Event::HistoryCleared];
    let for_history = events_in_batch::<Signer>(database_name, true, batch, events)?;
    TrDbCold::new()
        .set_history(for_history)
        .apply::<Signer>(database_name)
}

/// Timestamp [`Event`] set and make with it a new [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
pub fn events_to_batch<T: ErrorSource>(
    database_name: &str,
    events: Vec<Event>,
) -> Result<Batch, T::Error> {
    events_in_batch::<T>(database_name, false, Batch::default(), events)
}

/// Timestamp [`Event`] set and add it to existing [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
///
/// Note that existing [`Batch`] must contain no [`Entry`] additions, only
/// removals are possible. Only one [`Event`] set, transformed into [`Entry`]
/// with a single timestamp could be added in a single database transaction.
pub(crate) fn events_in_batch<T: ErrorSource>(
    database_name: &str,
    start_zero: bool,
    mut out_prep: Batch,
    events: Vec<Event>,
) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let history = open_tree::<T>(&database, HISTORY)?;
    let order = {
        if start_zero {
            Order::from_number(0u32)
        } else {
            Order::from_number(history.len() as u32)
        }
    };
    let timestamp = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap();
    let history_entry = Entry { timestamp, events };
    out_prep.insert(order.store(), history_entry.encode());
    Ok(out_prep)
}

/// Enter [`Event`] set into the database as a single database transaction.
#[cfg(any(feature = "signer", feature = "test"))]
pub(crate) fn enter_events<T: ErrorSource>(
    database_name: &str,
    events: Vec<Event>,
) -> Result<(), T::Error> {
    TrDbCold::new()
        .set_history(events_to_batch::<T>(database_name, events)?)
        .apply::<T>(database_name)
}

/// Enter user-generated [`Event`] into the database.
#[cfg(feature = "signer")]
pub fn history_entry_user(database_name: &str, string_from_user: &str) -> Result<(), ErrorSigner> {
    let events = vec![Event::UserEntry {
        user_entry: string_from_user.to_string(),
    }];
    enter_events::<Signer>(database_name, events)
}

/// Enter system-generated [`Event`] into the database.
// TODO possibly obsolete
#[cfg(feature = "signer")]
pub fn history_entry_system(database_name: &str, event: Event) -> Result<(), ErrorSigner> {
    // let events = vec![Event::SystemEntry(string_from_system)];
    let events = vec![event];
    enter_events::<Signer>(database_name, events)
}

/// Process the fact that the Signer device was online.
///
/// - Add history log entry with `Event::DeviceWasOnline`.
/// - Update [`DangerRecord`] stored in [`SETTREE`](constants::SETTREE) with
/// `device_was_online = true` flag.
///
/// Unacknowledged non-safe [`DangerRecord`] block the use of Signer in the
/// frontend.
#[cfg(feature = "signer")]
pub fn device_was_online(database_name: &str) -> Result<(), ErrorSigner> {
    let events = vec![Event::DeviceWasOnline];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::set_was_online().store());
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(database_name, events)?)
        .set_settings(settings_batch)
        .apply::<Signer>(database_name)
}

/// Acknowledge that the Signer device was online and reset the
/// [`DangerRecord`] back to safe.
///
/// - Add history log entry with `Event::ResetDangerRecord`.
/// - Reset [`DangerRecord`] stored in [`SETTREE`](constants::SETTREE) to
/// `safe`, i.e. with `device_was_online = false` flag.
///
/// Acknowledged and reset [`DangerRecord`] allow to resume the use of Signer in
/// the frontend. Use it wisely.
#[cfg(feature = "signer")]
pub fn reset_danger_status_to_safe(database_name: &str) -> Result<(), ErrorSigner> {
    let events = vec![Event::ResetDangerRecord];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::safe().store());
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(database_name, events)?)
        .set_settings(settings_batch)
        .apply::<Signer>(database_name)
}

/// Record in history log that certain seed was shown on Signer screen,
/// presumably for backup.
///
/// Seeds are distinguished by the seed name.
#[cfg(feature = "signer")]
pub fn seed_name_was_shown(
    database_name: &str,
    seed_name_was_shown: String,
) -> Result<(), ErrorSigner> {
    let events = vec![Event::SeedNameWasShown {
        seed_name_was_shown,
    }];
    enter_events::<Signer>(database_name, events)
}
