#![deny(unused_crate_dependencies)]

use constants::{COLD_DB_NAME, COLD_DB_NAME_RELEASE, HOT_DB_NAME, TYLO};
use db_handling::{default_cold, default_cold_release, default_hot, metadata::transfer_metadata_to_cold, prep_messages::prep_types};
use definitions::error_active::{Active, ErrorActive};

mod derivations;
    use derivations::process_derivations;
pub mod fetch_metadata;
mod helpers;
pub mod interpret_specs;
mod load;
    use load::{gen_load_meta, unwasm};
pub mod parser;
    use parser::{Command, Show};
mod remove;
    use remove::remove_info;
mod show;
    use show::{show_database, show_address_book};
mod specs;
    use specs::gen_add_specs;
mod metadata_db_utils;
mod metadata_shortcut;
mod output_prep;
pub mod make_message;
    use make_message::make_message;


/// Function to process incoming command as interpreted by parser

pub fn full_run (command: Command) -> Result<(), ErrorActive> {
    match command {
        Command::Show(x) => {
            match x {
                Show::Database => show_database(),
                Show::AddressBook => show_address_book(),
            }
        },
        Command::Types => prep_types::<Active>(HOT_DB_NAME)?.write(TYLO),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Specs(instruction) => gen_add_specs(instruction),
        Command::Make(make) => make_message(make),
        Command::Remove(info) => remove_info(info),
        Command::RestoreDefaults => default_hot(),
        Command::MakeColdWithIdentities => default_cold(),
        Command::TransferMeta => transfer_metadata_to_cold(HOT_DB_NAME, COLD_DB_NAME),
        Command::MakeColdRelease => default_cold_release(),
        Command::TransferMetaRelease => transfer_metadata_to_cold(HOT_DB_NAME, COLD_DB_NAME_RELEASE),
        Command::Derivations(x) => process_derivations(x),
        Command::Unwasm{filename, update_db} => unwasm(&filename, update_db),
    }
}

