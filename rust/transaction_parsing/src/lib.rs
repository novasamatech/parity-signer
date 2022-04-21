#![deny(unused_crate_dependencies)]

use db_handling::manage_history::get_history_entry_by_order;
use definitions::error_signer::{ErrorSigner, InputSigner};

pub use definitions::navigation::{StubNav, TransactionAction};
mod add_specs;
use add_specs::add_specs;
pub mod cards;
use cards::Card;
pub mod check_signature;
mod derivations;
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
use parse_transaction::{decode_signable_from_history, parse_transaction};
pub mod test_all_cards;
use test_all_cards::make_all_cards;
#[cfg(feature = "test")]
#[cfg(test)]
mod tests;

/// Payload in hex format as it arrives into handling contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, ** - transaction type),
/// see the standard for details,
/// - actual content (differs between transaction types, could be even empty)
/// actual content is handled individually depending on prelude

fn handle_scanner_input(payload: &str, dbname: &str) -> Result<TransactionAction, ErrorSigner> {
    let data_hex = {
        if let Some(a) = payload.strip_prefix("0x") {
            a
        } else {
            payload
        }
    };

    if data_hex.len() < 6 {
        return Err(ErrorSigner::Input(InputSigner::TooShort));
    }

    if &data_hex[..2] != "53" {
        return Err(ErrorSigner::Input(InputSigner::NotSubstrate(
            data_hex[..2].to_string(),
        )));
    }

    match &data_hex[4..6] {
        "00" | "02" => parse_transaction(data_hex, dbname),
        "03" => process_message(data_hex, dbname),
        "80" => load_metadata(data_hex, dbname),
        "81" => load_types(data_hex, dbname),
        "c1" => add_specs(data_hex, dbname),
        "de" => process_derivations(data_hex, dbname),
        "f0" => Ok(make_all_cards()),
        _ => Err(ErrorSigner::Input(InputSigner::PayloadNotSupported(
            data_hex[4..6].to_string(),
        ))),
    }
}

pub fn produce_output(payload: &str, dbname: &str) -> TransactionAction {
    match handle_scanner_input(payload, dbname) {
        Ok(out) => out,
        Err(e) => TransactionAction::Read {
            r: format!("\"error\":[{}]", Card::Error(e).card(&mut 0, 0)),
        },
    }
}

/// Function to print history entry by order for entries without parseable transaction
pub fn print_history_entry_by_order_with_decoding(
    _order: u32,
    _database_name: &str,
) -> Result<String, ErrorSigner> {
    // TODO
    Ok(String::new())
    /*
    let entry = get_history_entry_by_order(order, database_name)?;
    Ok(
        entry.show(|a| match decode_signable_from_history(a, database_name) {
            Ok(b) => format!("{{{}}}", b),
            Err(e) => format!(
                "\"error\":[{}],\"raw_transaction\":\"{}\"",
                Card::Error(e).card(&mut 0, 0),
                hex::encode(a.transaction())
            ),
        }),
    )
    */
}
