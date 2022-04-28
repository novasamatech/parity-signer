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

/// User-initiated removal of the types information from the Signer database.
///
/// Types information is not necessary to process transactions in networks with
/// metadata having `RuntimeVersionV14`, as the types information after `V14`
/// is a part of the metadata itself.
///
/// Types information is installed in Signer by default and could be removed by
/// user manually, through this function.
///
/// Types information is verified by the general verifier. When the general
/// verifier gets changed, the types information is also removed from the
/// Signer database through so-called `GeneralHold` processing, to avoid
/// confusion regarding what data was verified by whom. Note that this situation
/// in **not** related to the `remove_types_info` function and is handled
/// elsewhere.
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
