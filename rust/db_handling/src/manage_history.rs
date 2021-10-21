use constants::{DANGER, HISTORY};
use definitions::{history::{Event, Entry}, danger::DangerRecord};
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
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER.to_vec(), DangerRecord::not_safe().store());
    TrDbCold::new()
        .set_history(events_to_batch(&database_name, events)?)
        .set_settings(settings_batch)
        .apply(&database_name)
}

/// Function to reset the danger status to `safe` - use it wisely if at all
pub fn reset_danger_status_to_safe(database_name: &str) -> anyhow::Result<()> {
    let events = vec![Event::ResetDangerRecord];
    let mut settings_batch = Batch::default();
    settings_batch.insert(DANGER.to_vec(), DangerRecord::safe().store());
    TrDbCold::new()
        .set_history(events_to_batch(&database_name, events)?)
        .set_settings(settings_batch)
        .apply(&database_name)
}

pub fn seed_name_was_shown(database_name: &str, seed_name: String) -> anyhow::Result<()> {
    let events = vec![Event::SeedNameWasShown(seed_name)];
    enter_events(database_name, events)
}

#[cfg(test)]
mod tests {
    use definitions::history::all_events_preview;
    use super::*;
    
    #[test]
    fn test_all_events () {
        let dbname = "tests/test_all_events";
        let events = all_events_preview();
        enter_events(dbname, events).unwrap();
        let history = print_history(dbname).unwrap();
        let expected_history_part = r##"[{"event":"metadata_added","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8"}},{"event":"network_specs_added","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"3","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"network_removed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"3","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"network_verifier_set","payload":{"genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","current_verifier":{"type":"general","details":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}}},{"event":"general_verifier_added","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}},{"event":"types_added","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","verifier":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"types_removed","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","verifier":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"load_types_message_signed","payload":{"types_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"load_metadata_message_signed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"add_specs_message_signed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"}}},{"event":"transaction_signed","payload":{"transaction":"","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","path":"//","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","path":"//","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identities_wiped"},{"event":"device_online"},{"event":"reset_danger_record"},{"event":"seed_name_shown","payload":"AliceSecretSeed"},{"event":"warning","payload":"Received network information is not verified."},{"event":"sign_error","payload":{"transaction":"","signed_by":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","encryption":"sr25519"},"user_comment":"send to Alice","error":"wrong_password_entered"}},{"event":"wrong_password_entered"},{"event":"user_entered_event","payload":"Lalala!!!"},{"event":"system_entered_event","payload":"Blip blop"},{"event":"history_cleared"},{"event":"database_initiated"}]"##;
        assert!(history.contains(expected_history_part), "\nHistory generated:\n{}", history);
        std::fs::remove_dir_all(dbname).unwrap();
    }
}


