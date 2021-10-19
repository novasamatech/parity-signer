use constants::HISTORY;
use definitions::history::{Event, Entry};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use chrono::Utc;
use sled::{Db, Batch};

use crate::db_transactions::TrDbCold;
use crate::error::{Error, NotDecodeable};
use crate::helpers::{open_db, open_tree, make_batch_clear_tree};

type Order = u64;

pub fn print_history(database_name: &str) -> anyhow::Result<String> {
    let database = open_db(database_name)?;
    print_history_tree(&database)
}

pub (crate) fn print_history_tree(database: &Db) -> anyhow::Result<String> {
    let history = open_tree(&database, HISTORY)?;
    let mut out = String::from("[");
    for x in history.iter() {
        if let Ok((order_encoded, history_entry_encoded)) = x {
            let order = match <Order>::decode(&mut &order_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::EntryOrder).show()),
            };
            let history_entry = match <Entry>::decode(&mut &history_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Entry).show()),
            };
            if out.len()>1 {out.push_str(",")}
            out.push_str(&format!("{{\"order\":{},{}}}", order, history_entry.show()));
        }
    }
    out.push_str("]");
    Ok(out)
}

pub fn clear_history(database_name: &str) -> anyhow::Result<()> {
    let batch = make_batch_clear_tree(database_name, HISTORY)?;
    let events = vec![Event::HistoryCleared];
    TrDbCold::new()
        .set_history(events_in_batch(&database_name, true, batch, events)?)
        .apply(&database_name)
}

pub fn events_to_batch(database_name: &str, events: Vec<Event>) -> anyhow::Result<Batch> {
    events_in_batch(database_name, false, Batch::default(), events)
}

pub fn events_in_batch(database_name: &str, start_zero: bool, mut out_prep: Batch, events: Vec<Event>) -> anyhow::Result<Batch> {
    let database = open_db(database_name)?;
    let history = open_tree(&database, HISTORY)?;
    let order = {
        if start_zero {0 as Order}
        else {history.len() as Order}
    };
    let timestamp = Utc::now().to_string();
    let history_entry = Entry {
        timestamp,
        events,
    };
    out_prep.insert(order.encode(), history_entry.encode());
    Ok(out_prep)
}

pub fn enter_events(database_name: &str, events: Vec<Event>) -> anyhow::Result<()> {
    TrDbCold::new()
        .set_history(events_to_batch(&database_name, events)?)
        .apply(&database_name)
}

pub fn history_entry_user(database_name: &str, string_from_user: String) -> anyhow::Result<()> {
    let events = vec![Event::UserEntry(string_from_user)];
    enter_events(database_name, events)
}

pub fn history_entry_system(database_name: &str, string_from_system: String) -> anyhow::Result<()> {
    let events = vec![Event::SystemEntry(string_from_system)];
    enter_events(database_name, events)
}

pub fn device_was_online(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::DeviceWasOnline];
    enter_events(database_name, events)
}

pub fn seed_name_was_shown(database_name: &str, seed_name: String) -> anyhow::Result<()> {
    let events = vec![Event::SeedNameWasShown(seed_name)];
    enter_events(database_name, events)
}
