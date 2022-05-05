//! This crate is intended to support the
//! [Signer](https://github.com/paritytech/parity-signer) from the active
//! (non air-gapped) side.
//!
//! This crate is mainly used to:
//!
//! - fetch network data through rpc calls
//! - prepare Signer update payloads
//! - generate Signer update QR codes, either signed or unsigned, to be scanned
//! into Signer
//! - maintain the `hot` database on the network-connected device, to store and
//! manage the data that went into QR codes
//! - maintain Signer default network metadata set in `default` crate and
//! prepare the `cold` database for the Signer release
//!
//! Signer air-gap holds true as long as the data loaded into Signer from the
//! start is valid and the updates generated and received through the air-gap
//! are valid and uncompomised.
//!
//! Parity maintains [Metadata Portal](https://metadata.parity.io) with network
//! specs and fresh network metadata QR codes, signed by Parity-associated key.
//!
//! To load into Signer network specs and network metadata for the networks not
//! published on the metadata portal yet or to keep a different trusted
//! [`CurrentVerifier`](definitions::network_specs::CurrentVerifier) for a
//! network, crate `generate_message` should be used.
//!
//! # Supported Signer updates
//!
//! Crate `generate_message` can generate and the Signer can accept following
//! updates:
//!
//! - `add_specs`, to add a new network (i.e. the network specs) into the Signer
//! - `load_metadata`, to load into the Signer the network metadata, for
//! networks that already have corresponding network specs entry in the Signer
//! database
//! - `load_types`, to load types information (it  is used to support the
//! transactions parsing in networks with legacy metadata, `RuntimeMetadata`
//! version below V14)
//! - `derivations`, for bulk-import of password-free derivations
//!
//! Updates are assembled as `Vec<u8>` and could be transformed into:
//!
//! - `png` QR codes, static or dynamic multiframe depending on the data size
//! - hex-encoded string (used mostly for tests)
//!
//! Information in `add_specs`, `load_metadata` and `load_types` could be either
//! signed or unsigned. Information in `derivations` could only be unsigned.
//!
//! Signed updates contain public key of the payload verifier and its
//! signature for the payload data.
//!
//! Unsigned updates have no public key or signature, and in place of code for
//! encryption algorithm have a special indicator that payload is unsigned (`ff`
//! in hex-encoded string).
//!
//! Updates `add_specs`, `load_metadata`, `load_types` all are build from the
//! following elements:
//! - prelude `53xxyy` (in hex format) where `xx` is the encryption type, and
//! `yy` is the message type  
//! - verifier public key (if the QR code is signed by verifier)  
//! - content  
//! - verifier signature (if the QR code is signed by verifier) 
//!
//! Content of the updates is described in [definitions::qr_transfers].
//!
//! Note that the signable payloads are build in such a way that the length of
//! the payload is always easily found in the update content. This is done to
//! future-proof the updates if the multi-signing is ever implemented for them.
//!
//! # Adding a network to the Signer and `add_specs` payload
//!
//! (purpose, verifiers)
//! (commands, keys, signing or send to manual?)
//!
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
//!
//! # Loading network metadata into the Signer and `load_metadata` payload
//!
//! 
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
mod make_message;
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
        Command::RestoreDefaults => default_hot(),
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
