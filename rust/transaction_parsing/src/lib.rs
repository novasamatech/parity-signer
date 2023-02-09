#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use db_handling::identities::TransactionBulk;
use definitions::helpers::unhex;
use parity_scale_codec::Decode;

pub use definitions::navigation::{StubNav, TransactionAction};
mod add_specs;
use add_specs::add_specs;
pub mod cards;
pub mod check_signature;
mod derivations;
pub use derivations::prepare_derivations_preview;
use derivations::process_derivations;
mod helpers;
mod holds;
mod load_metadata;
use load_metadata::load_metadata;
mod load_types;
use load_types::load_types;
mod message;
use message::process_message;
mod parse_transaction;
pub use parse_transaction::entry_to_transactions_with_decoding;
use parse_transaction::parse_transaction;
mod error;
#[cfg(test)]
mod tests;

pub use crate::error::{Error, Result};

/// Payload in hex format as it arrives into handling contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, ** - transaction type),
/// see the standard for details,
/// - actual content (differs between transaction types, could be even empty)
/// actual content is handled individually depending on prelude

fn handle_scanner_input(database: &sled::Db, payload: &str) -> Result<TransactionAction> {
    let data_hex = {
        if let Some(a) = payload.strip_prefix("0x") {
            a
        } else {
            payload
        }
    };

    if data_hex.len() < 6 {
        return Err(Error::TooShort);
    }

    if &data_hex[..2] != "53" {
        return Err(Error::NotSubstrate(data_hex[..2].to_string()));
    }

    match &data_hex[4..6] {
        "00" | "02" => parse_transaction(database, data_hex, false),
        "03" => process_message(database, data_hex),
        "04" => parse_transaction_bulk(database, data_hex),
        "80" => load_metadata(database, data_hex),
        "81" => load_types(database, data_hex),
        "c1" => add_specs(database, data_hex),
        "de" => process_derivations(database, data_hex),
        _ => Err(Error::PayloadNotSupported(data_hex[4..6].to_string())),
    }
}

fn parse_transaction_bulk(database: &sled::Db, payload: &str) -> Result<TransactionAction> {
    let decoded_data = unhex(payload)?;

    let bulk = TransactionBulk::decode(&mut &decoded_data[3..])?;

    match bulk {
        TransactionBulk::V1(b) => {
            let mut actions = vec![];

            let mut checksum = 0;
            for t in &b.encoded_transactions {
                let encoded = hex::encode(t);
                let encoded = "53".to_string() + &encoded;
                let action = parse_transaction(database, &encoded, true)?;
                if let TransactionAction::Sign {
                    actions: a,
                    checksum: c,
                } = action
                {
                    checksum = c;
                    actions.push(a[0].clone());
                }
            }

            Ok(TransactionAction::Sign { actions, checksum })
        }
    }
}

pub fn produce_output(database: &sled::Db, payload: &str) -> Result<TransactionAction> {
    handle_scanner_input(database, payload)
}
