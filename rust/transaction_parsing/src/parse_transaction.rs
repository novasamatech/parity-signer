use db_handling::{
    db_transactions::{SignContent, TrDbColdSign},
    helpers::{try_get_address_details, try_get_network_specs},
};
use definitions::{
    history::{Entry, Event, SignDisplay},
    keyring::{AddressKey, NetworkSpecsKey},
    navigation::{MEventMaybeDecoded, TransactionCard, TransactionCardSet},
    network_specs::VerifierValue,
    users::AddressDetails,
};
use parser::{cut_method_extensions, decoding_commons::OutputCard, parse_extensions, parse_method};
use std::path::Path;

use crate::cards::{make_author_info, Card, Warning};
use crate::error::{Error, Result};
use crate::helpers::{
    bundle_from_meta_set_element, find_meta_set, multisigner_msg_genesis_encryption, specs_by_name,
};
use crate::TransactionAction;

/// Transaction payload in hex format as it arrives into parsing program contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, 00 or 02 - transaction type),
/// see the standard for details,
/// - author public key (length depends on cryptography used),
/// - method, extensions, network genesis hash

/// Enum to move around cards in preparatory stage (author details or author card, and warning card)
enum CardsPrep<'a> {
    SignProceed(AddressDetails, Option<Warning<'a>>),
    ShowOnly(TransactionCard, Box<TransactionCard>),
}

/// Function to parse transaction.
/// Attempts to decode the transaction, and if completely successful,
/// produces a set of cards to print the transaction content,
/// and an action card `sign_transaction` with database entry to be used to
/// actually sign the transaction later if approved.
/// Transaction format corresponds to what we get from qr code:
/// i.e. it starts with 53****, followed by author address, followed by actual transaction piece,
/// followed by extrinsics, concluded with chain genesis hash

pub(crate) fn parse_transaction<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let (author_multi_signer, parser_data, genesis_hash, encryption) =
        multisigner_msg_genesis_encryption(data_hex)?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash, &encryption);

    // Some(true/false) should be here by the standard; should stay None for now, as currently existing transactions apparently do not comply to standard.
    let optional_mortal_flag = None; /*match &data_hex[4..6] {
                                         "00" => Some(true), // expect transaction to be mortal
                                         "02" => Some(false), // expect transaction to be immortal
                                         _ => unreachable!(),
                                     };*/

    // initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

    match try_get_network_specs(&db_path, &network_specs_key)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_multisigner(&author_multi_signer);
            let mut history: Vec<Event> = Vec::new();

            let mut cards_prep = match try_get_address_details(&db_path, &address_key)? {
                Some(address_details) => {
                    if address_details.network_id.contains(&network_specs_key) {
                        CardsPrep::SignProceed(address_details, None)
                    } else {
                        let author_card = (Card::Author {
                            author: &author_multi_signer,
                            base58prefix: network_specs.base58prefix,
                            address_details: &address_details,
                        })
                        .card(&mut index, indent);
                        CardsPrep::ShowOnly(
                            author_card,
                            Box::new(Card::Warning(Warning::NoNetworkID).card(&mut index, indent)),
                        )
                    }
                }
                None => CardsPrep::ShowOnly(
                    (Card::AuthorPlain {
                        author: &author_multi_signer,
                        base58prefix: network_specs.base58prefix,
                    })
                    .card(&mut index, indent),
                    Box::new((Card::Warning(Warning::AuthorNotFound)).card(&mut index, indent)),
                ),
            };

            let short_specs = network_specs.short();
            let (method_data, extensions_data) = match cut_method_extensions(&parser_data) {
                Ok(a) => a,
                Err(_) => return Err(Error::SeparateMethodExtensions),
            };

            let meta_set = find_meta_set(&short_specs, &db_path)?;
            if meta_set.is_empty() {
                return Err(Error::NoMetadata {
                    name: network_specs.name,
                });
            }
            let mut found_solution = None;
            let mut error_collection = Vec::new();
            let latest_version = meta_set[0].version();
            for (i, x) in meta_set.iter().enumerate() {
                let used_version = x.version();
                let metadata_bundle = bundle_from_meta_set_element(x, &db_path)?;
                match parse_extensions(
                    extensions_data.to_vec(),
                    &metadata_bundle,
                    &short_specs,
                    optional_mortal_flag,
                ) {
                    Ok(extensions_cards) => {
                        if i > 0 {
                            history.push(Event::Warning {
                                warning: Warning::NewerVersion {
                                    used_version,
                                    latest_version,
                                }
                                .show(),
                            });
                            cards_prep = match cards_prep {
                                CardsPrep::SignProceed(address_details, _) => {
                                    CardsPrep::SignProceed(
                                        address_details,
                                        Some(Warning::NewerVersion {
                                            used_version,
                                            latest_version,
                                        }),
                                    )
                                }
                                CardsPrep::ShowOnly(author_card, _warning_card) => {
                                    CardsPrep::ShowOnly(
                                        author_card,
                                        //warning_card,
                                        Box::new(
                                            Card::Warning(Warning::NewerVersion {
                                                used_version,
                                                latest_version,
                                            })
                                            .card(&mut index, indent),
                                        ),
                                    )
                                }
                            };
                        }
                        match parse_method(method_data.to_vec(), &metadata_bundle, &short_specs) {
                            Ok(a) => {
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(address_details, possible_warning) => {
                                        let sign = TrDbColdSign::generate(
                                            SignContent::Transaction {
                                                method: method_data,
                                                extensions: extensions_data,
                                            },
                                            &network_specs.name,
                                            &address_details.path,
                                            address_details.has_pwd,
                                            &author_multi_signer,
                                            history,
                                        );
                                        let checksum = sign.store_and_get_checksum(&db_path)?;
                                        let author_info = make_author_info(
                                            &author_multi_signer,
                                            network_specs.base58prefix,
                                            &address_details,
                                        );
                                        let warning = possible_warning
                                            .map(|w| Card::Warning(w).card(&mut index, indent))
                                            .map(|w| vec![w]);
                                        let method = into_cards(&a, &mut index);
                                        let extensions = into_cards(&extensions_cards, &mut index);
                                        let content = TransactionCardSet {
                                            warning,
                                            method: Some(method),
                                            extensions: Some(extensions),
                                            ..Default::default()
                                        };
                                        Some(TransactionAction::Sign {
                                            content,
                                            checksum,
                                            has_pwd: address_details.has_pwd,
                                            author_info,
                                            network_info: network_specs.clone(),
                                        })
                                    }
                                    CardsPrep::ShowOnly(author_card, warning_card) => {
                                        let author = Some(vec![author_card]);
                                        let warning = Some(vec![*warning_card]);
                                        let method = Some(into_cards(&a, &mut index));
                                        let extensions =
                                            Some(into_cards(&extensions_cards, &mut index));
                                        let r = TransactionCardSet {
                                            author,
                                            warning,
                                            method,
                                            extensions,
                                            ..Default::default()
                                        };
                                        Some(TransactionAction::Read { r })
                                    }
                                };
                            }
                            Err(e) => {
                                found_solution = match cards_prep {
                                    CardsPrep::SignProceed(address_details, possible_warning) => {
                                        let warning = possible_warning
                                            .map(|w| Card::Warning(w).card(&mut index, indent))
                                            .map(|w| vec![w]);
                                        let author = Card::Author {
                                            author: &author_multi_signer,
                                            base58prefix: network_specs.base58prefix,
                                            address_details: &address_details,
                                        }
                                        .card(&mut index, indent);
                                        let error = Card::Error(e.into()).card(&mut index, indent);
                                        let extensions = into_cards(&extensions_cards, &mut index);
                                        let r = TransactionCardSet {
                                            author: Some(vec![author]),
                                            error: Some(vec![error]),
                                            warning,
                                            extensions: Some(extensions),
                                            ..Default::default()
                                        };
                                        Some(TransactionAction::Read { r })
                                    }
                                    CardsPrep::ShowOnly(author_card, warning_card) => {
                                        let author = Some(vec![author_card]);
                                        let warning = Some(vec![*warning_card]);
                                        let error = Some(vec![
                                            Card::Error(e.into()).card(&mut index, indent)
                                        ]);
                                        let extensions =
                                            Some(into_cards(&extensions_cards, &mut index));
                                        let r = TransactionCardSet {
                                            author,
                                            warning,
                                            error,
                                            extensions,
                                            ..Default::default()
                                        };
                                        Some(TransactionAction::Read { r })
                                    }
                                };
                            }
                        }
                        break;
                    }
                    Err(e) => error_collection.push((used_version, e)), // TODO output transaction author info
                }
            }
            match found_solution {
                Some(a) => Ok(a),
                None => Err(Error::AllExtensionsParsingFailed {
                    network_name: network_specs.name,
                    errors: error_collection,
                }), // author: [], hint: [], error: []
            }
        }
        None => {
            // did not find network with matching genesis hash in database
            let author_card = Card::AuthorPublicKey(&author_multi_signer).card(&mut index, indent);
            let error_card = Card::Error(Error::UnknownNetwork {
                genesis_hash,
                encryption,
            })
            .card(&mut index, indent);
            let author = Some(vec![author_card]);
            let error = Some(vec![error_card]);

            let r = TransactionCardSet {
                author,
                error,
                ..Default::default()
            };

            Ok(TransactionAction::Read { r })
        }
    }
}

fn into_cards(set: &[OutputCard], index: &mut u32) -> Vec<TransactionCard> {
    set.iter()
        .map(|card| Card::ParserCard(&card.card).card(index, card.indent))
        .collect()
}

pub fn entry_to_transactions_with_decoding<P>(
    entry: Entry,
    db_path: P,
) -> Result<Vec<MEventMaybeDecoded>>
where
    P: AsRef<Path>,
{
    let mut res = Vec::new();

    // TODO: insanely bad code.
    for event in entry.events.into_iter() {
        let (verifier_details, signed_by, decoded) = match event {
            Event::TransactionSigned { ref sign_display }
            | Event::TransactionSignError { ref sign_display } => {
                let VerifierValue::Standard { ref m } = sign_display.signed_by;
                let address_key = AddressKey::from_multisigner(m);
                let verifier_details = Some(sign_display.signed_by.show_card());

                if let Some(address_details) = try_get_address_details(&db_path, &address_key)? {
                    let mut specs_found = None;
                    for id in &address_details.network_id {
                        let specs = try_get_network_specs(&db_path, id)?;
                        if let Some(specs) = specs {
                            if specs.name == sign_display.network_name {
                                specs_found = Some(specs);
                            }
                        }
                    }

                    if let Some(specs_found) = specs_found {
                        (
                            verifier_details,
                            Some(make_author_info(
                                m,
                                specs_found.base58prefix,
                                &address_details,
                            )),
                            Some(decode_signable_from_history(sign_display, &db_path)?),
                        )
                    } else {
                        (verifier_details, None, None)
                    }
                } else {
                    (verifier_details, None, None)
                }
            }
            _ => (None, None, None),
        };
        res.push(MEventMaybeDecoded {
            event,
            decoded,
            signed_by,
            verifier_details,
        });
    }

    Ok(res)
}

pub(crate) fn decode_signable_from_history<P>(
    found_signable: &SignDisplay,
    db_path: P,
) -> Result<TransactionCardSet>
where
    P: AsRef<Path>,
{
    let (parser_data, network_name, encryption) = found_signable.transaction_network_encryption();

    let short_specs = specs_by_name(&network_name, &encryption, &db_path)?.short();
    let meta_set = find_meta_set(&short_specs, &db_path)?;
    if meta_set.is_empty() {
        return Err(Error::HistoricalMetadata { name: network_name });
    }

    let (method_data, extensions_data) = cut_method_extensions(&parser_data)?;

    let mut found_solution = None;
    let mut error_collection = Vec::new();
    let mut index = 0;
    let indent = 0;

    for x in meta_set.iter() {
        let used_version = x.version();
        let metadata_bundle = bundle_from_meta_set_element(x, &db_path)?;

        match parse_extensions(
            extensions_data.to_vec(),
            &metadata_bundle,
            &short_specs,
            None,
        ) {
            Ok(extensions_cards) => {
                match parse_method(method_data, &metadata_bundle, &short_specs) {
                    Ok(a) => {
                        let method = into_cards(&a, &mut index);
                        let extensions = into_cards(&extensions_cards, &mut index);
                        found_solution = Some(TransactionCardSet {
                            method: Some(method),
                            extensions: Some(extensions),
                            ..Default::default()
                        });
                    }
                    Err(e) => {
                        let error = Card::Error(e.into()).card(&mut index, indent);
                        let extensions = Some(into_cards(&extensions_cards, &mut index));
                        found_solution = Some(TransactionCardSet {
                            error: Some(vec![error]),
                            extensions,
                            ..Default::default()
                        });
                    }
                }
                break;
            }
            Err(e) => error_collection.push((used_version, e)),
        }
    }
    match found_solution {
        Some(a) => Ok(a),
        None => Err(Error::AllExtensionsParsingFailed {
            network_name,
            errors: error_collection,
        }),
    }
}
