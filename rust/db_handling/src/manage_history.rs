use definitions::{constants::HISTORY, history::{Event, Entry}};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use chrono::Utc;
use sled::{Db, Tree};

use crate::helpers::{open_db, open_tree, clear_tree, flush_db, insert_into_tree};
use crate::error::{Error, NotDecodeable};

pub type Order = u64;

pub fn print_history(database_name: &str) -> anyhow::Result<String> {
    let database = open_db(database_name)?;
    print_history_tree(&database)
}

pub fn print_history_tree(database: &Db) -> anyhow::Result<String> {
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
    let events = vec![Event::HistoryCleared];
    clear_and_add(database_name, events)
}

pub fn init_history(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::DatabaseInitiated];
    clear_and_add(database_name, events)
}

fn clear_and_add(database_name: &str, events: Vec<Event>) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    let history = open_tree(&database, HISTORY)?;
    clear_tree(&history)?;
    flush_db(&database)?;
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    Ok(())
}

pub fn enter_events(database_name: &str, events: Vec<Event>) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    let history = open_tree(&database, HISTORY)?;
    enter_events_into_tree(&history, events)?;
    flush_db(&database)?;
    Ok(())
}

pub fn enter_events_into_tree(history: &Tree, events: Vec<Event>) -> anyhow::Result<()> {
    let order = history.len() as Order;
    let timestamp = Utc::now().to_string();
    let history_entry = Entry {
        timestamp,
        events,
    };
    insert_into_tree(order.encode(), history_entry.encode(), &history)?;
    Ok(())
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

pub fn seeds_were_accessed(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::SeedsWereAccessed];
    enter_events(database_name, events)
}

pub fn seeds_were_shown(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::SeedsWereShown];
    enter_events(database_name, events)
}
