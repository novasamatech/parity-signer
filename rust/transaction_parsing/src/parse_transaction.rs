use db_handling::identities::derive_single_key;
use db_handling::{
    db_transactions::{SignContent, TrDbColdSign, TrDbColdSignOne},
    helpers::{get_all_networks, try_get_address_details, try_get_network_specs},
};
use definitions::crypto::Encryption;
use definitions::{
    history::{Entry, Event, SignDisplay},
    keyring::{AddressKey, NetworkSpecsKey},
    navigation::{MEventMaybeDecoded, TransactionCard, TransactionCardSet, TransactionSignAction},
    network_specs::VerifierValue,
    users::AddressDetails,
};
use parser::{cut_method_extensions, decoding_commons::OutputCard, parse_extensions, parse_method};
use sp_core::H256;
use sp_runtime::MultiSigner;
use std::collections::HashMap;

use crate::cards::{make_author_info, Card, Warning};
use crate::dynamic_derivations::dd_transaction_msg_genesis_encryption;
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

pub(crate) fn parse_transaction(database: &sled::Db, data_hex: &str) -> Result<TransactionAction> {
    let (author_multi_signer, call_data, genesis_hash, encryption) =
        multisigner_msg_genesis_encryption(database, data_hex)?;

    let author_address_key = AddressKey::new(author_multi_signer.clone(), Some(genesis_hash));
    let address_details = try_get_address_details(database, &author_address_key)?;
    do_parse_transaction(
        database,
        author_multi_signer,
        &call_data,
        genesis_hash,
        encryption,
        address_details,
    )
}

pub fn parse_dd_transaction(
    database: &sled::Db,
    data_hex: &str,
    seeds: &HashMap<String, String>,
) -> Result<TransactionAction> {
    let (transaction, call_data, genesis_hash, encryption) =
        dd_transaction_msg_genesis_encryption(data_hex)?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash, &encryption);
    let (author_multi_signer, address_details) = derive_single_key(
        database,
        seeds,
        &transaction.derivation_path,
        &transaction.root_multisigner,
        network_specs_key,
    )?;
    do_parse_transaction(
        database,
        author_multi_signer,
        &call_data,
        genesis_hash,
        encryption,
        Some(address_details),
    )
}

fn do_parse_transaction(
    database: &sled::Db,
    author_multi_signer: MultiSigner,
    call_data: &[u8],
    genesis_hash: H256,
    encryption: Encryption,
    address_details: Option<AddressDetails>,
) -> Result<TransactionAction> {
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

    match try_get_network_specs(database, &network_specs_key)? {
        Some(network_specs) => {
            let mut history: Vec<Event> = Vec::new();

            let mut cards_prep = match address_details {
                Some(address_details) => {
                    if address_details.network_id.as_ref() == Some(&network_specs_key) {
                        CardsPrep::SignProceed(address_details, None)
                    } else {
                        let author_card = (Card::Author {
                            author: &author_multi_signer,
                            base58prefix: network_specs.specs.base58prefix,
                            genesis_hash: network_specs.specs.genesis_hash,
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
                        base58prefix: network_specs.specs.base58prefix,
                    })
                    .card(&mut index, indent),
                    Box::new((Card::Warning(Warning::AuthorNotFound)).card(&mut index, indent)),
                ),
            };

            let short_specs = network_specs.specs.short();
            let (method_data, extensions_data) = cut_method_extensions(call_data)?;

            let meta_set = find_meta_set(database, &short_specs)?;
            if meta_set.is_empty() {
                return Err(Error::NoMetadata {
                    name: network_specs.specs.name,
                });
            }
            let mut found_solution = None;
            let mut error_collection = Vec::new();
            let latest_version = meta_set[0].version();
            for (i, x) in meta_set.iter().enumerate() {
                let used_version = x.version();
                let metadata_bundle = bundle_from_meta_set_element(database, x)?;
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
                                        let sign_one = TrDbColdSignOne::generate(
                                            SignContent::Transaction {
                                                method: method_data,
                                                extensions: extensions_data,
                                            },
                                            &network_specs.specs.name,
                                            &address_details.path,
                                            address_details.has_pwd,
                                            &author_multi_signer,
                                            history,
                                        );
                                        let mut sign = TrDbColdSign::from_storage(database, None)?
                                            .unwrap_or_default();
                                        sign.signing_bulk.push(sign_one);
                                        let checksum = sign.store_and_get_checksum(database)?;
                                        let author_info = make_author_info(
                                            &author_multi_signer,
                                            network_specs.specs.base58prefix,
                                            network_specs.specs.genesis_hash,
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
                                            actions: vec![TransactionSignAction {
                                                content,
                                                has_pwd: address_details.has_pwd,
                                                author_info,
                                                network_info: network_specs.clone(),
                                            }],
                                            checksum,
                                        })
                                    }
                                    CardsPrep::ShowOnly(author_card, warning_card) => {
                                        let author = Some(vec![author_card]);
                                        let warning = Some(vec![*warning_card]);
                                        let method = Some(into_cards(&a, &mut index));
                                        let extensions =
                                            Some(into_cards(&extensions_cards, &mut index));
                                        let r = Box::new(TransactionCardSet {
                                            author,
                                            warning,
                                            method,
                                            extensions,
                                            ..Default::default()
                                        });
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
                                            base58prefix: network_specs.specs.base58prefix,
                                            genesis_hash: network_specs.specs.genesis_hash,
                                            address_details: &address_details,
                                        }
                                        .card(&mut index, indent);
                                        let error = Card::Error(e.into()).card(&mut index, indent);
                                        let extensions = into_cards(&extensions_cards, &mut index);
                                        let r = Box::new(TransactionCardSet {
                                            author: Some(vec![author]),
                                            error: Some(vec![error]),
                                            warning,
                                            extensions: Some(extensions),
                                            ..Default::default()
                                        });
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
                                        let r = Box::new(TransactionCardSet {
                                            author,
                                            warning,
                                            error,
                                            extensions,
                                            ..Default::default()
                                        });
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
                    network_name: network_specs.specs.name,
                    errors: error_collection,
                }), // author: [], hint: [], error: []
            }
        }
        None => Err(Error::UnknownNetwork {
            genesis_hash,
            encryption,
        }),
    }
}

fn into_cards(set: &[OutputCard], index: &mut u32) -> Vec<TransactionCard> {
    set.iter()
        .map(|card| Card::ParserCard(&card.card).card(index, card.indent))
        .collect()
}

pub fn entry_to_transactions_with_decoding(
    database: &sled::Db,
    entry: Entry,
) -> Result<Vec<MEventMaybeDecoded>> {
    let mut res = Vec::new();

    // TODO: insanely bad code.
    for event in entry.events.into_iter() {
        let (verifier_details, signed_by, decoded) = match event {
            Event::TransactionSigned { ref sign_display }
            | Event::TransactionSignError { ref sign_display } => {
                let VerifierValue::Standard { ref m } = sign_display.signed_by;
                let network = get_all_networks(database)?
                    .iter()
                    .find(|network| sign_display.network_name == network.specs.name)
                    .cloned()
                    .unwrap();

                let address_key = AddressKey::new(m.clone(), Some(network.specs.genesis_hash));
                let verifier_details = Some(sign_display.signed_by.show_card());

                if let Some(address_details) = try_get_address_details(database, &address_key)? {
                    let mut specs_found = None;
                    let id = &address_details.network_id;
                    if let Some(id) = &id {
                        let specs = try_get_network_specs(database, id)?;
                        if let Some(ordered_specs) = specs {
                            if ordered_specs.specs.name == sign_display.network_name {
                                specs_found = Some(ordered_specs);
                            }
                        }
                    }

                    if let Some(specs_found) = specs_found {
                        (
                            verifier_details,
                            Some(make_author_info(
                                m,
                                specs_found.specs.base58prefix,
                                specs_found.specs.genesis_hash,
                                &address_details,
                            )),
                            Some(decode_signable_from_history(database, sign_display)?),
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

pub(crate) fn decode_signable_from_history(
    database: &sled::Db,
    found_signable: &SignDisplay,
) -> Result<TransactionCardSet> {
    let (parser_data, network_name, encryption) = found_signable.transaction_network_encryption();

    let short_specs = specs_by_name(database, &network_name, &encryption)?.short();
    let meta_set = find_meta_set(database, &short_specs)?;
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
        let metadata_bundle = bundle_from_meta_set_element(database, x)?;

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
