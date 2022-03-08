#![deny(unused_crate_dependencies)]

use db_handling::manage_history::get_history_entry_by_order;
use definitions::{error_signer::{ErrorSigner, InputSigner}, keyring::NetworkSpecsKey};

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
    use parse_transaction::{parse_transaction, decode_signable_from_history};
pub mod test_all_cards;
    use test_all_cards::make_all_cards;
#[cfg(feature = "test")]
mod tests;

/// Enum containing card sets for three different outcomes:
/// signing (Sign), accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Derivations{content: String, network_info: String, checksum: u32, network_specs_key: NetworkSpecsKey},
    Sign{content: String, checksum: u32, has_pwd: bool, author_info: String, network_info: String},
    Stub(String, u32, StubNav),
    Read(String),
}

/// Enum describing Stub content.
/// Is used for proper navigation. Variants:
/// AddSpecs (with associated NetworkSpecsKey), LoadMeta (with associated 
/// NetworkSpecsKey for the first by order network using those metadata),
/// and LoadTypes
#[derive(PartialEq, Debug, Clone)]
pub enum StubNav {
    AddSpecs(NetworkSpecsKey),
    LoadMeta(NetworkSpecsKey),
    LoadTypes,
}


/// Payload in hex format as it arrives into handling contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, ** - transaction type),
/// see the standard for details,
/// - actual content (differs between transaction types, could be even empty)
/// actual content is handled individually depending on prelude


fn handle_scanner_input (payload: &str, dbname: &str) -> Result<Action, ErrorSigner> {

    let data_hex = {
        if let Some(a) = payload.strip_prefix("0x") {a}
        else {&payload}
    };
    
    if data_hex.len() < 6 {return Err(ErrorSigner::Input(InputSigner::TooShort))}
    
    if &data_hex[..2] != "53" {return Err(ErrorSigner::Input(InputSigner::NotSubstrate(data_hex[..2].to_string())))}
    
    match &data_hex[4..6] {
        "00"|"02" => parse_transaction(data_hex, dbname),
        "03" => process_message(data_hex, dbname),
        "80" => load_metadata(data_hex, dbname),
        "81" => load_types(data_hex, dbname),
        "c1" => add_specs(data_hex, dbname),
        "de" => process_derivations(data_hex, dbname),
        "f0" => Ok(make_all_cards()),
        _ => Err(ErrorSigner::Input(InputSigner::PayloadNotSupported(data_hex[4..6].to_string()))),
    }
}

pub fn produce_output (payload: &str, dbname: &str) -> Action {
    match handle_scanner_input (payload, dbname) {
        Ok(out) => out,
        Err(e) => Action::Read(format!("\"error\":[{}]", Card::Error(e).card(&mut 0,0))),
    }
}

/// Function to print history entry by order for entries without parseable transaction
pub fn print_history_entry_by_order_with_decoding(order: u32, database_name: &str) -> Result<String, ErrorSigner> {
    let entry = get_history_entry_by_order(order, database_name)?;
    Ok(entry.show(|a| {
        match decode_signable_from_history (a, database_name) {
            Ok(b) => format!("{{{}}}", b),
            Err(e) => format!("\"error\":[{}],\"raw_transaction\":\"{}\"", Card::Error(e).card(&mut 0,0), hex::encode(a.transaction()))
        }
    }))
}
