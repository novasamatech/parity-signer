use db_handling::db_transactions::{TrDbColdSign, SignContent};
use definitions::{history::Event, keyring::{AddressKey}, users::AddressDetails};
use parser::{parse_set, error::ParserError, decoding_commons::OutputCard};

use crate::cards::{Action, Card, Warning};
use crate::error::{Error, DatabaseError};
use crate::helpers::{author_encryption_msg_genesis, checked_address_details, checked_network_specs, find_meta_set, bundle_from_meta_set_element, sign_store_and_get_checksum};

/// Transaction payload in hex format as it arrives into parsing program contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, 00 or 02 - transaction type),
/// see the standard for details,
/// - author public key (length depends on cryptography used),
/// - method, extensions, network genesis hash


/// Enum to move around cards in preparatory stage (author_card and warning_card)
enum CardsPrep {
    SignProceed (String, Option<String>, AddressDetails), 
    ShowOnly (String, String),
}

/// Function to parse transaction.
/// Attempts to decode the transaction, and if completely successful,
/// produces a set of cards to print the transaction content,
/// and an action card "sign_transaction" with database entry to be used to
/// actually sign the transaction later if approved.
/// Transaction format corresponds to what we get from qr code:
/// i.e. it starts with 53****, followed by author address, followed by actual transaction piece,
/// followed by extrinsics, concluded with chain genesis hash

pub fn parse_transaction (data_hex: &str, dbname: &str) -> Result<String, Error> {

    let (author_public_key, encryption, parser_data, network_specs_key) = author_encryption_msg_genesis(data_hex)?;

// this should be here by the standard; should stay commented for now, since the test transactions apparently do not comply to standard.
    let optional_mortal_flag = None; /*match &data_hex[4..6] {
        "00" => Some(true), // expect transaction to be mortal
        "02" => Some(false), // expect transaction to be immortal
        _ => unreachable!(),
    };*/

// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

    match checked_network_specs(&network_specs_key, &dbname)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_parts(&author_public_key, &encryption).expect("already matched encryption type and author public key length, should always work");
            let author = address_key.print_as_base58(&encryption, Some(network_specs.base58prefix)).expect("just generated address_key, should always work");
            let mut history: Vec<Event> = Vec::new();

            let mut cards_prep = match checked_address_details(&address_key, &dbname)? {
                Some(address_details) => {
                    let author_card = (Card::Author{base58_author: &author, seed_name: &address_details.seed_name, path: &address_details.path, has_pwd: address_details.has_pwd, name: &address_details.name}).card(&mut index, indent);
                    if address_details.network_id.contains(&network_specs_key) {CardsPrep::SignProceed(author_card, None, address_details)}
                    else {CardsPrep::ShowOnly(author_card, Card::Warning(Warning::NoNetworkID).card(&mut index, indent))}
                }
                None => {
                    CardsPrep::ShowOnly((Card::AuthorPlain(&author)).card(&mut index, indent),(Card::Warning(Warning::AuthorNotFound)).card(&mut index, indent))
                }
            };

            let short_specs = network_specs.short();
            let meta_set = find_meta_set(&network_specs.name, &dbname)?;
            if meta_set.len() == 0 {return Err(Error::DatabaseError(DatabaseError::NoMetaAtAll))}
            let mut found_solution = None;
            let mut error_collection: Vec<(String, u32, ParserError)> = Vec::new();
            let latest_version = meta_set[0].version;
            for (i,x) in meta_set.iter().enumerate() {
                let used_version = x.version;
                match parse_set(&parser_data, &bundle_from_meta_set_element(x, &dbname)?, &short_specs, optional_mortal_flag) {
                    Ok((method_cards_result, extensions_cards)) => {
                        if i>0 {
                            history.push(Event::Warning(Warning::NewerVersion{used_version, latest_version}.show()));
                            let add = Card::Warning(Warning::NewerVersion{used_version, latest_version}).card(&mut index, indent);
                            cards_prep = match cards_prep {
                                CardsPrep::SignProceed(author_card, _, address_details) => CardsPrep::SignProceed(author_card, Some(add), address_details),
                                CardsPrep::ShowOnly(author_card, warning_card) => CardsPrep::ShowOnly(author_card, format!("{},{}", warning_card, add)),
                            };
                        }
                        match method_cards_result {
                            Ok(a) => {
                                let method = into_cards(&a, &mut index);
                                let extensions = into_cards(&extensions_cards, &mut index);
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(author_card, possible_warning, address_details) => {
                                        let sign = TrDbColdSign::generate(SignContent::Transaction(parser_data), &network_specs.name, &address_details.path, address_details.has_pwd, &address_key, history);
                                        let checksum = sign_store_and_get_checksum (sign, &dbname)?;
                                        let action_card = Action::Sign(checksum).card();
                                        match possible_warning {
                                            Some(warning_card) => Some(format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extensions\":[{}],{}}}", author_card, warning_card, method, extensions, action_card)),
                                            None => Some(format!("{{\"author\":[{}],\"method\":[{}],\"extensions\":[{}],{}}}", author_card, method, extensions, action_card)),
                                        }
                                    },
                                    CardsPrep::ShowOnly(author_card, warning_card) => Some(format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extensions\":[{}]}}", author_card, warning_card, method, extensions))
                                };
                            },
                            Err(e) => {
                                let method_error = Card::Error(Error::Parser(e)).card(&mut index, indent);
                                let extensions = into_cards(&extensions_cards, &mut index);
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(author_card, possible_warning, _) => {
                                        match possible_warning {
                                            Some(warning_card) => Some(format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extensions\":[{}]}}", author_card, warning_card, method_error, extensions)),
                                            None => Some(format!("{{\"author\":[{}],\"error\":[{}],\"extensions\":[{}]}}", author_card, method_error, extensions)),
                                        }
                                    },
                                    CardsPrep::ShowOnly(author_card, warning_card) => Some(format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extensions\":[{}]}}", author_card, warning_card, method_error, extensions))
                                };
                            },
                        }
                        break;
                    },
                    Err(e) => error_collection.push((network_specs.name.to_string(), used_version, e)),
                }
            }
            match found_solution {
                Some(a) => Ok(a),
                None => return Err(Error::AllParsingFailed(error_collection))
            }
        },
        None => {
        // did not find network with matching genesis hash in database
            let author_card = (Card::AuthorPublicKey{author_public_key, encryption}).card(&mut index, indent);
            let error_card = (Card::Error(Error::DatabaseError(DatabaseError::NoNetwork))).card(&mut index, indent);
            Ok(format!("{{\"author\":[{}],\"error\":[{}]}}", author_card, error_card))
        },
    }
}

fn into_cards (set: &Vec<OutputCard>, index: &mut u32) -> String {
    let mut out = String::new();
    for (i, x) in set.iter().enumerate() {
        if i>0 {out.push_str(",");}
        out.push_str(&Card::ParserCard(&x.card).card(index, x.indent));
    }
    out
}
