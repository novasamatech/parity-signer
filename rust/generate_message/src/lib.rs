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
//! # Networks and verifiers in Signer
//!
//! Signer operates only with the networks for which it has
//! [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) in the database:
//! only for these networks it is possible to upload the metadata and parse
//! transactions.
//!
//! By default, Signer supports networks Polkadot, Kusama, and Westend. Any
//! other Substrate-based network could be added to Signer by scanning and
//! accepting QR code with `add_specs` payload.
//!
//! Each network has an associated verifier, i.e. some entity that user
//! trusts to produce valid payloads. Verifier could be anything from more or
//! less centralized source to individual users themselves.
//!
//! Verifier gets set as soon as the `add_specs` payload is accepted by Signer.
//! Signer keeps track of what verifier has been used for the network, and does
//! not support verifier downgrades, i.e. using a verifier weaker than the one
//! used before.
//!
//! Network metadata (`load_metadata`) updates could be accepted only if they
//! have exactly same verifier as the one already in the database for that
//! network.
//!
//! It is possible to accept `add_specs` with stronger verifier, in which case
//! all previously known network matadata will be removed, `NetworkSpecs` entry
//! will get updated, and the verifier will be changed.
//!
//! Users should practice caution when removing the networks.
//!
//! Removing the network verified previously by the general verifier will not
//! change the fact that the expected verifier is the general one, should the
//! network be added back to Signer.
//!
//! Removing the network verified previously by custom verifier with `Some(_)`
//! value will cause the network to be **blocked** in Signer until the Signer is
//! reset. This is a security measure.
//!
//! # Verifiers and payload signing
//!
//! In Signer, network verifiers, i.e. the verifiers for `add_specs` and
//! `load_metadata` updates could be general or custom. Update `load_types` can
//! be verified only by the general verifier.
//!
//! General verifier is the strongest and the most reliable verifier known to
//! the Signer. By default it is set to Parity-associated key, but users can
//! remove it and set their own. There could be only one general verifier at any
//! time. Resetting general verifier to a different value (with trust on first
//! use basis), would remove all the data verified by the previous general
//! verifier.
//!
//! Custom verifier is a verifier used specifically for a given network,
//! different from the general verifier. There could be as many custom verifiers
//! for different networks as needed.
//!
//! Internal verifier-related Signer logic is described in more detail in
//! [definitions::network_specs].
//!
//! Any of the verifiers could be `None` or `Some(_)` with public key of the
//! trusted entity inside. Although keeping verifiers `None` is dangerous, it
//! is certainly possible.
//!
//! Verifier `None` originates from unsigned updates, verifier `Some(_)` - from
//! signed updates. If Signer has `None` for the general verifier, it sets up
//! `Some(_)` value from first of the accepted signed updates as the general
//! verifier. After the general verifier is set, without its removal, only
//! custom verifiers could be set for the networks.
//!
//! # Signing the payloads
//!
//! ## With signatures produced by `subkey`
//!
//!
//! ## With special dedicated Signer
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
// TODO add some notes here on the language. "update" is whole qr thing that
// could be read by the Signer. "payload" is the un-husked content part, without
// prelude, public key, signature. If I mess them up now, reader surely will.
// QR or text = output format
//!
//! # Add this somewhere visible probably
//!
//!
//! Signer updates with payload `add_specs` are used to add networks into the
//! Signer.
//!
//! `add_specs` updates could be signed or unsigned. For the networks not yet
//! known to the Signer, `add_specs` signature author becomes the network
//! verifier in the Signer database. If `add_specs` payload is unsigned,
//! verifier is set to `None`.
//!
//! If the network is already known to the Signer, the network information could
//! be accepted if signed by same or stronger verifier, details are described in
//! [definitions::network_specs].
//!
//! Updates `add_specs` for a known network (as determined by genesis hash),
//! including network with known genesis hash and different
//! [`Encryption`](definitions::crypto::Encryption), could be accepted if signed
//! by already established verifier or the stronger one. Updates `load_metadata`
//! for a known network could be accepted only if signed by already established
//! verifier.
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
use show::{check_file, show_address_book, show_database};
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
            Show::CheckFile(path) => check_file(path),
        },
        Command::Types => prep_types::<Active>(HOT_DB_NAME)?.write(TYLO),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Specs(instruction) => gen_add_specs(instruction),
        Command::Make(make) => make_message(make),
        Command::Remove(info) => remove_info(info),
        Command::RestoreDefaults => default_hot(),
        Command::MakeColdRelease(opt_path) => default_cold_release(opt_path),
        Command::TransferMetaRelease(opt_path) => {
            let cold_database_path = match opt_path {
                Some(ref path) => path.to_str().unwrap_or(COLD_DB_NAME_RELEASE),
                None => COLD_DB_NAME_RELEASE,
            };
            transfer_metadata_to_cold(HOT_DB_NAME, cold_database_path)
        }
        Command::Derivations(x) => process_derivations(x),
        Command::Unwasm {
            filename,
            update_db,
        } => unwasm(&filename, update_db),
        Command::MetaDefaultFile { name, version } => meta_default_file(&name, version),
    }
}
