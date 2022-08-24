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
use std::path::Path;
// TODO: substantial part of this will go obsolete with interface updates;
// some functions are not called at the moment from the user interface - kept
// for now in case they make a return, commented.
#[cfg(feature = "signer")]
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sled::Batch;

#[cfg(feature = "signer")]
use constants::DANGER;
use constants::HISTORY;
use definitions::{
    history::{Entry, Event},
    keyring::Order,
};

#[cfg(feature = "signer")]
use definitions::danger::DangerRecord;

use crate::helpers::{open_db, open_tree};
#[cfg(feature = "signer")]
use crate::Error;
use crate::Result;
#[cfg(feature = "signer")]
use crate::{db_transactions::TrDbCold, helpers::make_batch_clear_tree};

/// Print total number of pages, for maximum [`HISTORY_PAGE_SIZE`](constants::HISTORY_PAGE_SIZE) number of
/// entries per page.
#[cfg(feature = "signer")]
pub fn history_total_pages<P>(db_path: P) -> Result<u32>
where
    P: AsRef<Path>,
{
    use constants::HISTORY_PAGE_SIZE;

    let history = get_history(db_path)?;
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
pub fn get_history<P>(db_path: P) -> Result<Vec<(Order, Entry)>>
where
    P: AsRef<Path>,
{
    let database = open_db(&db_path)?;
    let history = open_tree(&database, HISTORY)?;
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
#[cfg(feature = "signer")]
pub fn get_history_entry_by_order<P>(order: u32, db_path: P) -> Result<Entry>
where
    P: AsRef<Path>,
{
    let database = open_db(&db_path)?;
    let history = open_tree(&database, HISTORY)?;
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

/// Clear Signer history and make a log [`Entry`] that history was cleared.
#[cfg(feature = "signer")]
pub fn clear_history<P>(db_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let batch = make_batch_clear_tree(&db_path, HISTORY)?;
    let events = vec![Event::HistoryCleared];
    let for_history = events_in_batch(&db_path, true, batch, events)?;
    TrDbCold::new().set_history(for_history).apply(&db_path)?;

    Ok(())
}

/// Timestamp [`Event`] set and make with it a new [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
pub fn events_to_batch<P>(db_path: P, events: Vec<Event>) -> Result<Batch>
where
    P: AsRef<Path>,
{
    events_in_batch(db_path, false, Batch::default(), events)
}

/// Timestamp [`Event`] set and add it to existing [`Batch`], that could be
/// applied to the [`HISTORY`] tree.
///
/// Note that existing [`Batch`] must contain no [`Entry`] additions, only
/// removals are possible. Only one [`Event`] set, transformed into [`Entry`]
/// with a single timestamp could be added in a single database transaction.
pub(crate) fn events_in_batch<P>(
    db_path: P,
    start_zero: bool,
    mut out_prep: Batch,
    events: Vec<Event>,
) -> Result<Batch>
where
    P: AsRef<Path>,
{
    let database = open_db(&db_path)?;
    let history = open_tree(&database, HISTORY)?;
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
#[cfg(feature = "signer")]
pub fn enter_events<P>(db_path: P, events: Vec<Event>) -> Result<()>
where
    P: AsRef<Path>,
{
    TrDbCold::new()
        .set_history(events_to_batch(&db_path, events)?)
        .apply(&db_path)
}

/// Enter user-generated [`Event`] into the database.
#[cfg(feature = "signer")]
pub fn history_entry_user<P>(db_path: P, string_from_user: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let events = vec![Event::UserEntry {
        user_entry: string_from_user.to_string(),
    }];
    enter_events(db_path, events)
}

/// Enter system-generated [`Event`] into the database.
// TODO possibly obsolete
#[cfg(feature = "signer")]
pub fn history_entry_system<P>(db_path: P, event: Event) -> Result<()>
where
    P: AsRef<Path>,
{
    // let events = vec![Event::SystemEntry(string_from_system)];
    let events = vec![event];
    enter_events(db_path, events)
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
pub fn device_was_online<P>(db_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let events = vec![Event::DeviceWasOnline];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::set_was_online().store());
    TrDbCold::new()
        .set_history(events_to_batch(&db_path, events)?)
        .set_settings(settings_batch)
        .apply(&db_path)
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
pub fn reset_danger_status_to_safe<P>(db_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let events = vec![Event::ResetDangerRecord];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER, DangerRecord::safe().store());
    TrDbCold::new()
        .set_history(events_to_batch(&db_path, events)?)
        .set_settings(settings_batch)
        .apply(&db_path)
}

/// Record in history log that certain seed was shown on Signer screen,
/// presumably for backup.
///
/// Seeds are distinguished by the seed name.
#[cfg(feature = "signer")]
pub fn seed_name_was_shown<P>(db_path: P, seed_name_was_shown: String) -> Result<()>
where
    P: AsRef<Path>,
{
    let events = vec![Event::SeedNameWasShown {
        seed_name_was_shown,
    }];
    enter_events(db_path, events)
}
