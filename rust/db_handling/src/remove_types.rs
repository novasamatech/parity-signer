use sled::Batch;

use constants::TYPES;
use definitions::{
    error_signer::{ErrorSigner, Signer},
    history::{Event, TypesDisplay},
    qr_transfers::ContentLoadTypes,
};

use crate::db_transactions::TrDbCold;
use crate::helpers::{get_general_verifier, get_types};
use crate::manage_history::events_to_batch;

pub fn remove_types_info(database_name: &str) -> Result<(), ErrorSigner> {
    let mut settings_batch = Batch::default();
    settings_batch.remove(TYPES);
    let events: Vec<Event> = vec![Event::TypesRemoved(TypesDisplay::get(
        &ContentLoadTypes::generate(&get_types::<Signer>(database_name)?),
        &get_general_verifier(database_name)?,
    ))];
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add history
        .set_settings(settings_batch) // upd settings
        .apply::<Signer>(database_name)
}
