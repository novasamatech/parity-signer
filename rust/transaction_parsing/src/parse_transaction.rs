use db_handling::{db_transactions::{TrDbColdSign, SignContent}, helpers::{try_get_network_specs, try_get_address_details}};
use definitions::{error::{ErrorSigner, InputSigner, NotFoundSigner, ParserError}, history::{Event, SignDisplay}, keyring::{AddressKey, NetworkSpecsKey}, users::AddressDetails};
use parser::{cut_method_extensions, parse_extensions, parse_method, decoding_commons::OutputCard};

use crate::Action;
use crate::cards::{Card, make_author_info, Warning};
use crate::helpers::{bundle_from_meta_set_element, find_meta_set, multisigner_msg_genesis_encryption, specs_by_name};

/// Transaction payload in hex format as it arrives into parsing program contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, 00 or 02 - transaction type),
/// see the standard for details,
/// - author public key (length depends on cryptography used),
/// - method, extensions, network genesis hash


/// Enum to move around cards in preparatory stage (author details or author card, and warning card)
enum CardsPrep <'a> {
    SignProceed (AddressDetails, Option<Warning <'a>>), 
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

pub (crate) fn parse_transaction (data_hex: &str, dbname: &str) -> Result<Action, ErrorSigner> {

    let (author_multi_signer, parser_data, genesis_hash_vec, encryption) = multisigner_msg_genesis_encryption(data_hex)?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash_vec, &encryption);

// Some(true/false) should be here by the standard; should stay None for now, as currently existing transactions apparently do not comply to standard.
    let optional_mortal_flag = None; /*match &data_hex[4..6] {
        "00" => Some(true), // expect transaction to be mortal
        "02" => Some(false), // expect transaction to be immortal
        _ => unreachable!(),
    };*/

// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

    match try_get_network_specs(&dbname, &network_specs_key)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_multisigner(&author_multi_signer);
            let mut history: Vec<Event> = Vec::new();

            let mut cards_prep = match try_get_address_details(&dbname, &address_key)? {
                Some(address_details) => {
                    if address_details.network_id.contains(&network_specs_key) {CardsPrep::SignProceed(address_details, None)}
                    else {
                        let author_card = (Card::Author{author: &author_multi_signer, base58prefix: network_specs.base58prefix, address_details: &address_details}).card(&mut index, indent);
                        CardsPrep::ShowOnly(author_card, Card::Warning(Warning::NoNetworkID).card(&mut index, indent))
                    }
                },
                None => {
                    CardsPrep::ShowOnly((Card::AuthorPlain{author: &author_multi_signer, base58prefix: network_specs.base58prefix}).card(&mut index, indent),(Card::Warning(Warning::AuthorNotFound)).card(&mut index, indent))
                },
            };

            let short_specs = network_specs.short();
            let (method_data, extensions_data) = match cut_method_extensions(&parser_data) {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Parser(ParserError::SeparateMethodExtensions)),
            };
            
            let meta_set = find_meta_set(&short_specs, &dbname)?;
            if meta_set.is_empty() {return Err(ErrorSigner::Input(InputSigner::NoMetadata{name: network_specs.name}))}
            let mut found_solution = None;
            let mut error_collection: Vec<(u32, ParserError)> = Vec::new();
            let latest_version = meta_set[0].version;
            for (i,x) in meta_set.iter().enumerate() {
                let used_version = x.version;
                let metadata_bundle = bundle_from_meta_set_element(x, &dbname)?;
                match parse_extensions (extensions_data.to_vec(), &metadata_bundle, &short_specs, optional_mortal_flag) {
                    Ok(extensions_cards) => {
                        if i>0 {
                            history.push(Event::Warning(Warning::NewerVersion{used_version, latest_version}.show()));
                            cards_prep = match cards_prep {
                                CardsPrep::SignProceed(address_details, _) => CardsPrep::SignProceed(address_details, Some(Warning::NewerVersion{used_version, latest_version})),
                                CardsPrep::ShowOnly(author_card, warning_card) => CardsPrep::ShowOnly(author_card, format!("{},{}", warning_card, Card::Warning(Warning::NewerVersion{used_version, latest_version}).card(&mut index, indent))),
                            };
                        }
                        match parse_method (method_data.to_vec(), &metadata_bundle, &short_specs) {
                            Ok(a) => {
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(address_details, possible_warning) => {
                                        let sign = TrDbColdSign::generate(SignContent::Transaction{method: method_data, extensions: extensions_data}, &network_specs.name, &address_details.path, address_details.has_pwd, &author_multi_signer, history);
                                        let checksum = sign.store_and_get_checksum (&dbname)?;
                                        let author_info = make_author_info(&author_multi_signer, network_specs.base58prefix, &address_details);
                                        let network_info = format!("\"network_title\":\"{}\",\"network_logo\":\"{}\"", network_specs.title, network_specs.logo);
                                        match possible_warning {
                                            Some(warning) => Some(Action::Sign{content: format!("\"warning\":[{}],\"method\":[{}],\"extensions\":[{}]", Card::Warning(warning).card(&mut index, indent), into_cards(&a, &mut index), into_cards(&extensions_cards, &mut index)), checksum, has_pwd: address_details.has_pwd, author_info, network_info}),
                                            None => Some(Action::Sign{content: format!("\"method\":[{}],\"extensions\":[{}]", into_cards(&a, &mut index), into_cards(&extensions_cards, &mut index)), checksum, has_pwd: address_details.has_pwd, author_info, network_info}),
                                        }
                                    },
                                    CardsPrep::ShowOnly(author_card, warning_card) => Some(Action::Read(format!("\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extensions\":[{}]", author_card, warning_card, into_cards(&a, &mut index), into_cards(&extensions_cards, &mut index))))
                                };
                            },
                            Err(e) => {
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(address_details, possible_warning) => {
                                        match possible_warning {
                                            Some(warning) => Some(Action::Read(format!("\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extensions\":[{}]", Card::Author{author: &author_multi_signer, base58prefix: network_specs.base58prefix, address_details: &address_details}.card(&mut index, indent), Card::Warning(warning).card(&mut index, indent), Card::Error(ErrorSigner::Parser(e)).card(&mut index, indent), into_cards(&extensions_cards, &mut index)))),
                                            None => Some(Action::Read(format!("\"author\":[{}],\"error\":[{}],\"extensions\":[{}]", Card::Author{author: &author_multi_signer, base58prefix: network_specs.base58prefix, address_details: &address_details}.card(&mut index, indent), Card::Error(ErrorSigner::Parser(e)).card(&mut index, indent), into_cards(&extensions_cards, &mut index)))),
                                        }
                                    },
                                    CardsPrep::ShowOnly(author_card, warning_card) => Some(Action::Read(format!("\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extensions\":[{}]", author_card, warning_card, Card::Error(ErrorSigner::Parser(e)).card(&mut index, indent), into_cards(&extensions_cards, &mut index))))
                                };
                            },
                        }
                        break;
                    },
                    Err(e) => error_collection.push((used_version, e)),
                }
            }
            match found_solution {
                Some(a) => Ok(a),
                None => return Err(ErrorSigner::AllExtensionsParsingFailed{network_name: network_specs.name.to_string(), errors: error_collection})
                // author: [], hint: [], error: []
            }
        },
        None => {
        // did not find network with matching genesis hash in database
            let author_card = Card::AuthorPublicKey(&author_multi_signer).card(&mut index, indent);
            let error_card = Card::Error(ErrorSigner::Input(InputSigner::UnknownNetwork{genesis_hash: genesis_hash_vec.to_vec(), encryption})).card(&mut index, indent);
            Ok(Action::Read(format!("\"author\":[{}],\"error\":[{}]", author_card, error_card)))
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

pub (crate) fn decode_signable_from_history (found_signable: &SignDisplay, database_name: &str) -> Result<String, ErrorSigner> {
    
    let (parser_data, network_name, encryption) = found_signable.transaction_network_encryption();
    
    let short_specs = specs_by_name(&network_name, &encryption, &database_name)?.short();
    let meta_set = find_meta_set(&short_specs, &database_name)?;
    if meta_set.is_empty() {return Err(ErrorSigner::NotFound(NotFoundSigner::HistoricalMetadata{name: network_name}))}
    
    let (method_data, extensions_data) = match cut_method_extensions(&parser_data) {
        Ok(a) => a,
        Err(_) => return Err(ErrorSigner::Parser(ParserError::SeparateMethodExtensions)),
    };
    
    let mut found_solution = None;
    let mut error_collection: Vec<(u32, ParserError)> = Vec::new();
    let mut index = 0;
    let indent = 0;
    
    for x in meta_set.iter() {
        let used_version = x.version;
        let metadata_bundle = bundle_from_meta_set_element(x, &database_name)?;
        match parse_extensions (extensions_data.to_vec(), &metadata_bundle, &short_specs, None) {
            Ok(extensions_cards) => {
                match parse_method (method_data, &metadata_bundle, &short_specs) {
                    Ok(a) => {
                        let method = into_cards(&a, &mut index);
                        let extensions = into_cards(&extensions_cards, &mut index);
                        found_solution = Some(format!("\"method\":[{}],\"extensions\":[{}]", method, extensions));
                    },
                    Err(e) => {
                        let method_error = Card::Error(ErrorSigner::Parser(e)).card(&mut index, indent);
                        let extensions = into_cards(&extensions_cards, &mut index);
                        found_solution = Some(format!("\"error\":[{}],\"extensions\":[{}]", method_error, extensions));
                    },
                }
                break;
            },
            Err(e) => error_collection.push((used_version, e)),
        }
    }
    match found_solution {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::AllExtensionsParsingFailed{network_name: network_name.to_string(), errors: error_collection})
    }
}
