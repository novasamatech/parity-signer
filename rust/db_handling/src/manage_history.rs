#[cfg(feature = "signer")]
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sled::Batch;

use constants::HISTORY;
#[cfg(feature = "signer")]
use constants::{DANGER, HISTORY_PAGE_SIZE};
use definitions::{
    error::ErrorSource,
    history::{Entry, Event},
    keyring::Order,
};

#[cfg(feature = "signer")]
use definitions::{
    danger::DangerRecord,
    error_signer::{
        DatabaseSigner, EntryDecodingSigner, ErrorSigner, InterfaceSigner, NotFoundSigner, Signer,
    },
    print::export_complex_vector,
};

#[cfg(feature = "signer")]
use crate::helpers::make_batch_clear_tree;
use crate::{
    db_transactions::TrDbCold,
    helpers::{open_db, open_tree},
};

/// Function to print history entries.
/// Interacts with user interface.
#[cfg(feature = "signer")]
pub fn print_history(database_name: &str) -> Result<String, ErrorSigner> {
    let history = get_history(database_name)?;
    Ok(format!(
        "\"log\":{},\"total_entries\":{}",
        export_complex_vector(&history, |(order, entry)| format!(
            "\"order\":{},{}",
            order.stamp(),
            entry.show(|b| format!("\"{}\"", hex::encode(b.transaction())))
        )),
        history.len()
    ))
}

/// Function to print total number of pages for pre-set number of entries per page.
/// Interacts with user interface.
#[cfg(feature = "signer")]
pub fn history_total_pages(database_name: &str) -> Result<u32, ErrorSigner> {
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

/// Function to print given page number from the history set.
/// Interacts with user interface.
#[cfg(feature = "signer")]
pub fn print_history_page(page_number: u32, database_name: &str) -> Result<String, ErrorSigner> {
    let history = get_history(database_name)?;
    let total_pages = history_total_pages(database_name)?;
    let n = page_number as usize;
    let history_subset = match history.get(n * HISTORY_PAGE_SIZE..(n + 1) * HISTORY_PAGE_SIZE) {
        Some(a) => a.to_vec(),
        None => match history.get(n * HISTORY_PAGE_SIZE..) {
            Some(a) => a.to_vec(),
            None => {
                return Err(ErrorSigner::Interface(
                    InterfaceSigner::HistoryPageOutOfRange {
                        page_number,
                        total_pages,
                    },
                ))
            }
        },
    };
    Ok(format!(
        "\"log\":{},\"total_entries\":{}",
        export_complex_vector(&history_subset, |(order, entry)| format!(
            "\"order\":{},{}",
            order.stamp(),
            entry.show(|b| format!("\"{}\"", hex::encode(b.transaction())))
        )),
        history.len()
    ))
}

/// Local helper function to retrieve history entries from the database.
/// Applicable only to Signer side.
#[cfg(feature = "signer")]
fn get_history(database_name: &str) -> Result<Vec<(Order, Entry)>, ErrorSigner> {
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

/// Function to retrieve history entry by its order.
/// Applicable only to Signer side.
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

/// Function to print history entry by order for entries without parseable transaction
#[cfg(feature = "signer")]
pub fn print_history_entry_by_order(
    order: u32,
    database_name: &str,
) -> Result<String, ErrorSigner> {
    let entry = get_history_entry_by_order(order, database_name)?;
    Ok(format!(
        "\"order\":{},{}",
        order,
        entry.show(|b| format!("\"{}\"", hex::encode(b.transaction())))
    ))
}

/// Function to clear Signer history.
/// Naturally, applicable only to the Signer side.
/// Interacts with user interface.
#[cfg(feature = "signer")]
pub fn clear_history(database_name: &str) -> Result<(), ErrorSigner> {
    let batch = make_batch_clear_tree::<Signer>(database_name, HISTORY)?;
    let events = vec![Event::HistoryCleared];
    let for_history = events_in_batch::<Signer>(database_name, true, batch, events)?;
    TrDbCold::new()
        .set_history(for_history)
        .apply::<Signer>(database_name)
}

/// Function to collect history events set into batch
pub fn events_to_batch<T: ErrorSource>(
    database_name: &str,
    events: Vec<Event>,
) -> Result<Batch, T::Error> {
    events_in_batch::<T>(database_name, false, Batch::default(), events)
}

/// Function to add history events set to existing batch
pub fn events_in_batch<T: ErrorSource>(
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

/// Function to load events into the database in single transaction.
/// Applicable both to Active side (for creating the test databases)
/// and for Signer side (loading actual events as part of Signer operation)
pub fn enter_events<T: ErrorSource>(
    database_name: &str,
    events: Vec<Event>,
) -> Result<(), T::Error> {
    TrDbCold::new()
        .set_history(events_to_batch::<T>(database_name, events)?)
        .apply::<T>(database_name)
}

/// Function for Signer user to add events.
/// Applicable only to Signer side.
/// Interacts with the user interface.
#[cfg(feature = "signer")]
pub fn history_entry_user(database_name: &str, string_from_user: &str) -> Result<(), ErrorSigner> {
    let events = vec![Event::UserEntry(string_from_user.to_string())];
    enter_events::<Signer>(database_name, events)
}

/// Function to add system-generated events during Signer operation.
/// Applicable only to Signer side.
/// Interacts with the user interface.
#[cfg(feature = "signer")]
pub fn history_entry_system(
    database_name: &str,
    string_from_system: String,
) -> Result<(), ErrorSigner> {
    let events = vec![Event::SystemEntry(string_from_system)];
    enter_events::<Signer>(database_name, events)
}

/// Function shows if the `device was online` indicator is on
/// Applicable only to Signer side.
/// Interacts with the user interface.
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

/// Function to reset the danger status to `safe` - use it wisely.
/// Applicable only to Signer side.
/// Interacts with the user interface.
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

/// Function to record in history log the fact that certain seed was shown on Signer screen.
/// Applicable only to Signer side.
/// Interacts with the user interface.
#[cfg(feature = "signer")]
pub fn seed_name_was_shown(database_name: &str, seed_name: String) -> Result<(), ErrorSigner> {
    let events = vec![Event::SeedNameWasShown(seed_name)];
    enter_events::<Signer>(database_name, events)
}
