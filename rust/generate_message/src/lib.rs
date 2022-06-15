//! # Encryption override
//!
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
//! contains field `encryption`, specifying the encryption supported by the
//! network.
//!
//! This information could not be acquired through rpc call and always must
//! be provided for non-default networks.
//!
//! Network encryption is defined with encryption override key: `-ed25519`,
//! `-sr25519`, or `-ecdsa`.
//!
//! Command `add_specs` **requires** crypto key for key combinations:
//!
//! - `-d -u`, not update database, use url address
//! - `-p -n`, update database, do not print anything, use network address
//! book title
//! - `-p -u`, update database, do not print anything, use url address
//! - `-t -n` (same as `-n`), update database, print data, use network
//! address book title
//! - `-t -u` (same as `-u`), update database, print data, use url address
//!
//! Command `add_specs` **may accept** crypto key for key combinations:
//!
//! - `-f -n`, with data only from the database, i.e. without rpc calls,
//! update database with new specs entry, use network address book title as
//! an identifier
//! - `-f -u`, with data only from the database, i.e. without rpc calls,
//! update database with new specs entry, use network url address as
//! an identifier
#![deny(unused_crate_dependencies)]

use constants::{COLD_DB_NAME_RELEASE, HOT_DB_NAME, TYLO};
use db_handling::{
    default_cold_release, default_hot,
    helpers::{prep_types, transfer_metadata_to_cold},
};
use definitions::error_active::{Active, ErrorActive};

mod derivations;
use derivations::process_derivations;
pub mod fetch_metadata;
mod helpers;
pub mod interpret_specs;
mod load;
use load::{gen_load_meta, meta_default_file, unwasm};
pub mod parser;
use parser::{Command, Show};
mod remove;
use remove::remove_info;
mod show;
use show::{show_address_book, show_database};
mod specs;
use specs::gen_add_specs;
pub mod make_message;
mod metadata_db_utils;
mod metadata_shortcut;
mod output_prep;
use make_message::make_message;

/// Function to process incoming command as interpreted by parser

pub fn full_run(command: Command) -> Result<(), ErrorActive> {
    match command {
        Command::Show(x) => match x {
            Show::Database => show_database(),
            Show::AddressBook => show_address_book(),
        },
        Command::Types => prep_types::<Active>(HOT_DB_NAME)?.write(TYLO),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Specs(instruction) => gen_add_specs(instruction),
        Command::Make(make) => make_message(make),
        Command::Remove(info) => remove_info(info),
        Command::RestoreDefaults => default_hot(None),
        Command::MakeColdRelease(opt_path) => default_cold_release(opt_path),
        Command::TransferMetaRelease => {
            transfer_metadata_to_cold(HOT_DB_NAME, COLD_DB_NAME_RELEASE)
        }
        Command::Derivations(x) => process_derivations(x),
        Command::Unwasm {
            filename,
            update_db,
        } => unwasm(&filename, update_db),
        Command::MetaDefaultFile { name, version } => meta_default_file(&name, version),
    }
}
