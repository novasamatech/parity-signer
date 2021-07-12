mod add_network;
    use add_network::add_network;
pub mod cards;
    use cards::Card;
mod constants;
mod check_signature;
mod decoding;
mod error;
    use error::{Error, BadInputData};
mod load_metadata;
    use load_metadata::load_metadata;
mod load_types;
    use load_types::load_types;
mod method;
mod parse_transaction;
    use parse_transaction::parse_transaction;
pub mod test_all_cards;
    use test_all_cards::make_all_cards;
mod tests;
mod utils;
    use utils::purge;
pub mod utils_base58;

/// Payload in hex format as it arrives into handling contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, ** - transaction type),
/// see the standard for details,
/// - actual content (differs between transaction types, could be even empty)
/// actual content is handled individually depending on prelude


fn handle_scanner_input (payload: &str, dbname: &str) -> Result<String, Error> {

    let data_hex = match payload.starts_with("0x") {
        true => &payload[2..],
        false => &payload,
    };
    
    if data_hex.len() < 6 {return Err(Error::BadInputData(BadInputData::TooShort))}
    
    if &data_hex[..2] != "53" {return Err(Error::BadInputData(BadInputData::NotSubstrate))}
    
    purge(dbname)?;
    
    match &data_hex[4..6] {
        "00"|"02" => parse_transaction(data_hex, dbname),
        "80" => load_metadata(data_hex, dbname),
        "81" => load_types(data_hex, dbname),
        "c0" => add_network(data_hex, dbname),
        "f0" => Ok(make_all_cards()),
        _ => return Err(Error::BadInputData(BadInputData::WrongPayloadType)),
    }
}

pub fn produce_output (payload: &str, dbname: &str) -> String {
    match handle_scanner_input (payload, dbname) {
        Ok(out) => out,
        Err(e) => format!("\"error\":[{}]", Card::Error(e).card(0,0)),
    }
}
