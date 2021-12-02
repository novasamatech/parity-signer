use constants::{DANGER, HISTORY, HISTORY_PAGE_SIZE};
use definitions::{danger::DangerRecord, error::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, ErrorSource, InterfaceSigner, KeyDecodingSignerDb, NotFoundSigner, Signer}, history::{Event, Entry}};
use parity_scale_codec::{Decode, Encode};
use anyhow;
use chrono::Utc;
use sled::Batch;
use definitions::print::export_complex_vector;

use crate::db_transactions::TrDbCold;
use crate::helpers::{open_db, open_tree, make_batch_clear_tree};

/// Key for history entries.
/// Order in which entries have appeared in the database.
type Order = u32;

/// Function to print history entries.
/// Interacts with user interface.
pub fn print_history(database_name: &str) -> anyhow::Result<String> {
    let history = get_history(database_name).map_err(|e| e.anyhow())?;
    Ok(export_complex_vector(&history, |(order, entry)| format!("\"order\":{},{}", order, entry.show())))
}

/// Function to print total number of pages for pre-set number of entries per page.
/// Interacts with user interface.
pub fn history_total_pages(database_name: &str) -> anyhow::Result<u32> {
    let history = get_history(database_name).map_err(|e| e.anyhow())?;
    let total_pages = {
        if history.len()%HISTORY_PAGE_SIZE == 0 {history.len()/HISTORY_PAGE_SIZE}
        else {history.len()/HISTORY_PAGE_SIZE + 1}
    };
    Ok(total_pages as u32)
}

/// Function to print given page number from the history set.
/// Interacts with user interface.
pub fn print_history_page(page_number: u32, database_name: &str) -> anyhow::Result<String> {
    let history = get_history(database_name).map_err(|e| e.anyhow())?;
    let total_pages = history_total_pages(database_name)?;
    let n = page_number as usize;
    let history_subset = match history.get(n*HISTORY_PAGE_SIZE..(n+1)*HISTORY_PAGE_SIZE) {
        Some(a) => a.to_vec(),
        None => match history.get(n*HISTORY_PAGE_SIZE..) {
            Some(a) => a.to_vec(),
            None => return Err(ErrorSigner::Interface(InterfaceSigner::HistoryPageOutOfRange{page_number, total_pages}).anyhow()),
        },
    };
    Ok(export_complex_vector(&history_subset, |(order, entry)| format!("\"order\":{},{}", order, entry.show())))
}

/// Local helper function to retrieve history entries from the database.
/// Applicable only to Signer side.
fn get_history(database_name: &str) -> Result<Vec<(Order, Entry)>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let mut out: Vec<(Order, Entry)> = Vec::new();
    for x in history.iter() {
        if let Ok((order_encoded, history_entry_encoded)) = x {
            let order = match <Order>::decode(&mut &order_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::EntryOrder(order_encoded.to_vec())))),
            };
            let history_entry = match <Entry>::decode(&mut &history_entry_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::HistoryEntry(order)))),
            };
            out.push((order, history_entry));
        }
    }
    out.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(out)
}

/// Function to retrieve history entry by its order.
/// Applicable only to Signer side.
pub fn get_history_entry_by_order(order: u32, database_name: &str) -> Result<Entry, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let mut found = None;
    for x in history.iter() {
        if let Ok((order_encoded, history_entry_encoded)) = x {
            match <Order>::decode(&mut &order_encoded[..]) {
                Ok(a) => {
                    if a == order {
                        match <Entry>::decode(&mut &history_entry_encoded[..]) {
                            Ok(b) => {
                                found = Some(b);
                                break;
                            },
                            Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::HistoryEntry(order)))),
                        }
                    }
                },
                Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::EntryOrder(order_encoded.to_vec())))),
            }
        }
    }
    match found {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::HistoryEntry(order))),
    }
}

/// Function to clear Signer history.
/// Naturally, applicable only to the Signer side.
/// Interacts with user interface.
pub fn clear_history(database_name: &str) -> anyhow::Result<()> {
    let batch = make_batch_clear_tree::<Signer>(database_name, HISTORY).map_err(|e| e.anyhow())?;
    let events = vec![Event::HistoryCleared];
    let for_history = events_in_batch::<Signer>(&database_name, true, batch, events).map_err(|e| e.anyhow())?;
    TrDbCold::new()
        .set_history(for_history)
        .apply::<Signer>(&database_name)
        .map_err(|e| e.anyhow())
}

/// Function to collect history events set into batch
pub fn events_to_batch <T: ErrorSource> (database_name: &str, events: Vec<Event>) -> Result<Batch, T::Error> {
    events_in_batch::<T>(database_name, false, Batch::default(), events)
}

/// Function to add history events set to existing batch
pub fn events_in_batch <T: ErrorSource> (database_name: &str, start_zero: bool, mut out_prep: Batch, events: Vec<Event>) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let history = open_tree::<T>(&database, HISTORY)?;
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

/// Function to load events into the database in single transaction.
/// Applicable both to Active side (for creating the test databases)
/// and for Signer side (loading actual events as part of Signer operation)
pub fn enter_events <T: ErrorSource> (database_name: &str, events: Vec<Event>) -> Result<(), T::Error> {
    TrDbCold::new()
        .set_history(events_to_batch::<T>(&database_name, events)?)
        .apply::<T>(&database_name)
}

/// Function for Signer user to add events.
/// Applicable only to Signer side.
/// Interacts with the user interface.
pub fn history_entry_user(database_name: &str, string_from_user: String) -> anyhow::Result<()> {
    let events = vec![Event::UserEntry(string_from_user)];
    enter_events::<Signer>(database_name, events).map_err(|e| e.anyhow())
}

/// Function to add system-generated events during Signer operation.
/// Applicable only to Signer side.
/// Interacts with the user interface.
pub fn history_entry_system(database_name: &str, string_from_system: String) -> anyhow::Result<()> {
    let events = vec![Event::SystemEntry(string_from_system)];
    enter_events::<Signer>(database_name, events).map_err(|e| e.anyhow())
}

/// Function shows if the `device was online` indicator is on
/// Applicable only to Signer side.
/// Interacts with the user interface.
pub fn device_was_online(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::DeviceWasOnline];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER.to_vec(), DangerRecord::not_safe().store());
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(&database_name, events).map_err(|e| e.anyhow())?)
        .set_settings(settings_batch)
        .apply::<Signer>(&database_name)
        .map_err(|e| e.anyhow())
}

/// Function to reset the danger status to `safe` - use it wisely.
/// Applicable only to Signer side.
/// Interacts with the user interface.
pub fn reset_danger_status_to_safe(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::ResetDangerRecord];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER.to_vec(), DangerRecord::safe().store());
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(&database_name, events).map_err(|e| e.anyhow())?)
        .set_settings(settings_batch)
        .apply::<Signer>(&database_name)
        .map_err(|e| e.anyhow())
}

/// Function to record in history log the fact that certain seed was shown on Signer screen.
/// Applicable only to Signer side.
/// Interacts with the user interface.
pub fn seed_name_was_shown(database_name: &str, seed_name: String) -> anyhow::Result<()> {
    let events = vec![Event::SeedNameWasShown(seed_name)];
    enter_events::<Signer>(database_name, events).map_err(|e| e.anyhow())
}

#[cfg(test)]
mod tests {
    use definitions::history::all_events_preview;
    use super::*;
    
    #[test]
    fn test_all_events () {
        let dbname = "for_tests/test_all_events";
        let events = all_events_preview();
        enter_events::<Signer>(dbname, events).unwrap();
        let history = print_history(dbname).unwrap();
        let expected_history_part = r##""events":[{"event":"metadata_added","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"event":"load_metadata_message_signed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"network_specs_added","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"3","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"network_removed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"3","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"add_specs_message_signed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"network_verifier_set","payload":{"genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"general_verifier_added","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}},{"event":"types_added","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","verifier":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"types_removed","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","verifier":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"load_types_message_signed","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"transaction_signed","payload":{"transaction":"","network_name":"westend","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice"}},{"event":"transaction_sign_error","payload":{"transaction":"","network_name":"westend","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice","error":"wrong_password_entered"}},{"event":"message_signed","payload":{"message":"5468697320697320416c6963650a526f676572","network_name":"westend","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice"}},{"event":"message_sign_error","payload":{"message":"5468697320697320416c6963650a526f676572","network_name":"westend","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice","error":"wrong_password_entered"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","path":"//","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","path":"//","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identities_wiped"},{"event":"device_online"},{"event":"reset_danger_record"},{"event":"seed_name_shown","payload":"AliceSecretSeed"},{"event":"warning","payload":"Received network information is not verified."},{"event":"wrong_password_entered"},{"event":"user_entered_event","payload":"Lalala!!!"},{"event":"system_entered_event","payload":"Blip blop"},{"event":"history_cleared"},{"event":"database_initiated"}]"##;
        assert!(history.contains(expected_history_part), "\nHistory generated:\n{}", history);
        std::fs::remove_dir_all(dbname).unwrap();
    }
}

