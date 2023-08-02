//! This is experimental cross-platform navigation for Vault.
//! Ideally it should replace almost everything and become the only interface

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use db_handling::identities::{export_key_set_addrs, SignaturesBulk, SignaturesBulkV1};
//do we support mutex?
use lazy_static::lazy_static;
use sp_runtime::MultiSignature;
use std::sync::Mutex;
use transaction_signing::SignatureType;

use definitions::navigation::{
    ActionResult, ExportedSet, MKeysInfoExport, MKeysNew, MSignatureReady, QrData,
};
use parity_scale_codec::Encode;
use qrcode_rtx::make_data_packs;

mod error;

mod actions;
pub use actions::Action;
pub mod alerts;
pub mod modals;
mod navstate;
mod states;
use navstate::State;
pub mod screens;
#[cfg(test)]
mod tests;

pub use crate::error::{Error, Result};

//TODO: multithread here some day!
lazy_static! {
    /// Navigation state of the app
    ///
    /// Navigation state is unsafe either way, since it has to persist
    /// No matter if here or beyond FFI
    pub static ref STATE: Mutex<Option<State>> = Mutex::new(
        None
    );
}

/// User actions handler.
///
/// This method is called on every user [`Action`] in the UI, performs changes in backend
/// and returns new UI information as [`ActionResult`].
pub fn do_action(
    action: Action,
    details_str: &str,
    secret_seed_phrase: &str,
) -> Result<ActionResult> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate.as_mut().ok_or(Error::DbNotInitialized)?.perform(
        action,
        details_str,
        secret_seed_phrase,
    )
}

/// Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(db: sled::Db, seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    *navstate = Some(State::init_navigation(db, seed_names));
    Ok(())
}

/// Should be called when seed names are modified in native to synchronize data
pub fn update_seed_names(seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate
        .as_mut()
        .ok_or(Error::DbNotInitialized)?
        .update_seed_names(seed_names);

    Ok(())
}

/// Export key info with derivations.
pub fn export_key_info(
    database: &sled::Db,
    seed_name: &str,
    exported_set: ExportedSet,
) -> Result<MKeysInfoExport> {
    let export_all_addrs = export_key_set_addrs(database, seed_name, exported_set)?;

    let data = [&[0x53, 0xff, 0xde], export_all_addrs.encode().as_slice()].concat();
    let frames = make_data_packs(&data, 128).map_err(|e| Error::DataPacking(e.to_string()))?;

    Ok(MKeysInfoExport { frames })
}

/// Export signatures bulk.
pub fn export_signatures_bulk(
    signatures: &[(MultiSignature, SignatureType)],
) -> Result<MSignatureReady> {
    let signatures = if signatures.len() > 1 {
        let v1: SignaturesBulkV1 = signatures
            .iter()
            .map(|s| s.0.clone())
            .collect::<Vec<_>>()
            .as_slice()
            .into();
        let v1: SignaturesBulk = v1.into();
        let data = v1.encode();

        make_data_packs(&data, 128).map_err(|e| Error::DataPacking(e.to_string()))?
    } else {
        let encoded = match signatures[0].1 {
            SignatureType::Transaction => hex::encode(signatures[0].0.encode()),
            SignatureType::Message => match &signatures[0].0 {
                MultiSignature::Ed25519(a) => hex::encode(a),
                MultiSignature::Sr25519(a) => hex::encode(a),
                MultiSignature::Ecdsa(a) => hex::encode(a),
            },
        };
        vec![QrData::Regular {
            data: encoded.as_bytes().into(),
        }]
    };

    Ok(MSignatureReady { signatures })
}

/// Get keys by seed name
pub fn keys_by_seed_name(database: &sled::Db, seed_name: &str) -> Result<MKeysNew> {
    Ok(db_handling::interface_signer::keys_by_seed_name(
        database, seed_name,
    )?)
}
