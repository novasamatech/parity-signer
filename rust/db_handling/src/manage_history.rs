//! Displaying and updating history log
//!
//! The Vault keeps a history log of all events that change the database and
//! affect security. It is stored in [`HISTORY`] tree of the cold database.
//!
//! Each history log [`Entry`] contains [`Event`] set and a timestamp. Database
//! key for [`Entry`] value is [`Order`], SCALE-encoded number of the entry.
//!
//! In addition to keeping the log, Vault also displays [`HISTORY`] tree
//! checksum for user to possibly keep the track of.
// TODO: substantial part of this will go obsolete with interface updates;
// some functions are not called at the moment from the user interface - kept
// for now in case they make a return, commented.
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sled::Batch;

use constants::DANGER;
use constants::HISTORY;
use definitions::{
    history::{Entry, Event},
    keyring::Order,
};

use definitions::danger::DangerRecord;

use crate::helpers::open_tree;
use crate::Error;
use crate::Result;
use crate::{db_transactions::TrDbCold, helpers::make_batch_clear_tree};

/// Print total number of pages, for maximum [`HISTORY_PAGE_SIZE`](constants::HISTORY_PAGE_SIZE) number of
/// entries per page.
pub fn history_total_pages(database: &sled::Db) -> Result<u32> {
    use constants::HISTORY_PAGE_SIZE;

    let history = get_history(database)?;
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
pub fn get_history(database: &sled::Db) -> Result<Vec<(Order, Entry)>> {
    let history = open_tree(database, HISTORY)?;
    let mut out: Vec<(Order, Entry)> = Vec::new();
    for (order_encoded, history_entry_encoded) in history.iter().flatten() {
        let order = Order::from_ivec(&order_encoded)?;
        let history_entry = <Entry>::decode(&mut &history_entry_encoded[..])?;
        out.push((order, history_entry));
    }
    out.sort_by(|a, b| b.0.stamp().cmp(&a.0.stamp()));
    Ok(out)
}

/// Get from the database a history log [`Entry`] by `u32` order identifier
/// received from the frontend.
pub fn get_history_entry_by_order(database: &sled::Db, order: u32) -> Result<Entry> {
    let history = open_tree(database, HISTORY)?;
    let mut found = None;
    let order = Order::from_number(order);
    for (order_encoded, history_entry_encoded) in history.iter().flatten() {
        let order_found = Order::from_ivec(&order_encoded)?;
        if order_found == order {
            found = Some(<Entry>::decode(&mut &history_entry_encoded[..])?);
            break;
        }
    }
    found.ok_or_else(|| Error::HistoryEntryNotFound(order.stamp()))
}

/// Clear Vault history and make a log [`Entry`] that history was cleared.
pub fn clear_history(database: &sled::Db) -> Result<()> {
    let batch = make_batch_clear_tree(database, HISTORY)?;
    let events = vec![Event::HistoryCleared];
    let for_history = events_in_batch(database, true, batch, events)?;
    TrDbCold::new().set_history(for_history).apply(database)?;

    Ok(())
}

/// Timestamp [`Event`] set and make with it a new [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
pub fn events_to_batch(database: &sled::Db, events: Vec<Event>) -> Result<Batch> {
    events_in_batch(database, false, Batch::default(), events)
}

/// Timestamp [`Event`] set and add it to existing [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
///
/// Note that existing [`Batch`] must contain no [`Entry`] additions, only
/// removals are possible. Only one [`Event`] set, transformed into [`Entry`]
/// with a single timestamp could be added in a single database transaction.
pub(crate) fn events_in_batch(
    database: &sled::Db,
    start_zero: bool,
    mut out_prep: Batch,
    events: Vec<Event>,
) -> Result<Batch> {
    let history = open_tree(database, HISTORY)?;
    let order = {
        if start_zero {
            Order::from_number(0u32)
        } else {
            Order::from_number(history.len() as u32)
        }
    };
    let timestamp = time::OffsetDateTime::now_utc().format(&time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
    ))?;
    let history_entry = Entry { timestamp, events };
    out_prep.insert(order.store(), history_entry.encode());
    Ok(out_prep)
}

/// Enter [`Event`] set into the database as a single database transaction.
pub fn enter_events(database: &sled::Db, events: Vec<Event>) -> Result<()> {
    TrDbCold::new()
        .set_history(events_to_batch(database, events)?)
        .apply(database)
}

/// Enter user-generated [`Event`] into the database.
pub fn history_entry_user(database: &sled::Db, string_from_user: &str) -> Result<()> {
    let events = vec![Event::UserEntry {
        user_entry: string_from_user.to_string(),
    }];
    enter_events(database, events)
}

/// Enter system-generated [`Event`] into the database.
// TODO possibly obsolete
pub fn history_entry_system(database: &sled::Db, event: Event) -> Result<()> {
    // let events = vec![Event::SystemEntry(string_from_system)];
    let events = vec![event];
    enter_events(database, events)
}

/// Process the fact that the Vault device was online.
///
/// - Add history log entry with `Event::DeviceWasOnline`.
/// - Update [`DangerRecord`] stored in [`SETTREE`](constants::SETTREE) with
///   `device_was_online = true` flag.
///
/// Unacknowledged non-safe [`DangerRecord`] block the use of Vault in the
/// frontend.
pub fn device_was_online(database: &sled::Db) -> Result<()> {
    let events = vec![Event::DeviceWasOnline];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::set_was_online().store());
    TrDbCold::new()
        .set_history(events_to_batch(database, events)?)
        .set_settings(settings_batch)
        .apply(database)
}

/// Acknowledge that the Vault device was online and reset the
/// [`DangerRecord`] back to safe.
///
/// - Add history log entry with `Event::ResetDangerRecord`.
/// - Reset [`DangerRecord`] stored in [`SETTREE`](constants::SETTREE) to
///   `safe`, i.e. with `device_was_online = false` flag.
///
/// Acknowledged and reset [`DangerRecord`] allow to resume the use of Vault in
/// the frontend. Use it wisely.
pub fn reset_danger_status_to_safe(database: &sled::Db) -> Result<()> {
    let events = vec![Event::ResetDangerRecord];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::safe().store());
    TrDbCold::new()
        .set_history(events_to_batch(database, events)?)
        .set_settings(settings_batch)
        .apply(database)
}

/// Record in history log that certain seed was shown on Vault screen,
/// presumably for backup.
///
/// Seeds are distinguished by the seed name.
pub fn seed_name_was_shown(database: &sled::Db, seed_name_was_shown: String) -> Result<()> {
    let events = vec![Event::SeedNameWasShown {
        seed_name_was_shown,
    }];
    enter_events(database, events)
}
