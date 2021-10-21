use hex;
use frame_metadata::RuntimeMetadata;
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use printing_balance::{PrettyOutput, convert_balance_pretty};
use db_handling::db_transactions::TrDbColdSign;
use definitions::{crypto::Encryption, history::Event, keyring::{AddressKey, NetworkSpecsKey}, users::AddressDetails};
use sp_runtime::generic::Era;

use crate::cards::{Action, Card, Warning};
use crate::decoding_older::process_as_call;
use crate::decoding_sci::decoding_sci_entry_point;
use crate::error::{Error, BadInputData, UnableToDecode, DatabaseError, SystemError};
use crate::helpers::{checked_address_details, checked_network_specs, unhex, find_meta, get_types, sign_store_and_get_checksum};
use crate::method::OlderMeta;

/// Transaction payload in hex format as it arrives into parsing program contains following elements:
/// - prelude, length 6 symbols ("53" stands for substrate, ** - crypto type, 00 or 02 - transaction type),
/// see the standard for details,
/// - author public key (length depends on cryptography used),
/// - method, extrinsics, network genesis hash


/// Struct to decode method, extrinsics, and genesis hash from transaction Vec<u8>
#[derive(Debug, parity_scale_codec_derive::Decode)]
struct TransactionParts {
    method: Vec<u8>,
    extrinsics: ExtrinsicValues,
    genesis_hash: [u8; 32],
}


/// Struct to decode extrinsics
#[derive(Debug, parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct ExtrinsicValues {
    era: Era,
#[codec(compact)]
    nonce: u64,
#[codec(compact)]
    tip: u128,
    metadata_version: u32,
    tx_version: u32,
    genesis_hash: [u8; 32],
    block_hash: [u8; 32],
}

/// Enum to move around cards in preparatory stage (author_card and warning_card)
enum CardsPrep {
    SignProceed (String, Option<String>, AddressDetails), 
    ShowOnly (String, String),
}

/// function to print full extrinsics cards
fn print_full_extrinsics (index: &mut u32, indent: u32, tip_output: &PrettyOutput, short: &ExtrinsicValues, chain_name: &str) -> String {
    match short.era {
        Era::Immortal => format!("{},{},{}", (Card::EraImmortalNonce(short.nonce)).card(index, indent), (Card::Tip{number: &tip_output.number, units: &tip_output.units}).card(index, indent), (Card::TxSpec{network: chain_name, version: short.metadata_version, tx_version: short.tx_version}).card(index, indent)),
        Era::Mortal(period, phase) => format!("{},{},{},{}", (Card::EraMortalNonce{phase, period, nonce: short.nonce}).card(index, indent), (Card::Tip{number: &tip_output.number, units: &tip_output.units}).card(index, indent), (Card::BlockHash(&hex::encode(short.block_hash))).card(index, indent), (Card::TxSpec{network: chain_name, version: short.metadata_version, tx_version: short.tx_version}).card(index, indent)),
    }
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
    
// input hex data of correct size should have at least 6 + 64 + 64 symbols (prelude + author public key minimal size + genesis hash)
    if data_hex.len() < 134 {return Err(Error::BadInputData(BadInputData::TooShort))}

    let data = unhex(&data_hex)?;
    
    let (author_public_key, encryption, data) = match &data_hex[2..4] {
        "00" => (data[3..35].to_vec(), Encryption::Ed25519, &data[35..]),
        "01" => (data[3..35].to_vec(), Encryption::Sr25519, &data[35..]),
        "02" => (data[3..36].to_vec(), Encryption::Ecdsa, &data[36..]),
        _ => return Err(Error::BadInputData(BadInputData::CryptoNotSupported))
    };
    
    let transaction_decoded = match <TransactionParts>::decode(&mut &data[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::UnableToDecode(UnableToDecode::MethodAndExtrinsicsFailure)),
    };
    
    let short = &transaction_decoded.extrinsics;
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

// try to get chain specs from genesis hash
    if transaction_decoded.genesis_hash != short.genesis_hash {return Err(Error::BadInputData(BadInputData::GenesisHashMismatch))}

// this should be here by the standard; should stay commented for now, since the test transactions apparently do not comply to standard.
//    if &data_hex[4..6] == "00" {if let Era::Immortal = short.era {return Err(Error::BadInputData(BadInputData::UnexpectedImmortality))}}
//    if &data_hex[4..6] == "02" {if let Era::Mortal(_, _) = short.era {return Err(Error::BadInputData(BadInputData::UnexpectedMortality))}}

    if let Era::Immortal = short.era {if short.genesis_hash != short.block_hash {return Err(Error::BadInputData(BadInputData::ImmortalHashMismatch))}}
    
    let network_specs_key = NetworkSpecsKey::from_parts(&transaction_decoded.genesis_hash.to_vec(), &encryption);
    
    match checked_network_specs(&network_specs_key, &dbname)? {
        Some(chain_specs_found) => {
            let chain_name = &chain_specs_found.name;
            let chain_prefix = chain_specs_found.base58prefix;
            
        // update tip output since we know chain specs already
            let tip_output = match convert_balance_pretty (&short.tip.to_string(), chain_specs_found.decimals, &chain_specs_found.unit) {
                Ok(x) => x,
                Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
            };

        // check that the network is compatible with provided encryption
            if encryption != chain_specs_found.encryption {return Err(Error::BadInputData(BadInputData::EncryptionMismatch))}
            
            let address_key = AddressKey::from_parts(&author_public_key, &encryption).expect("already matched encryption type and author public key length, should always work");
            let author = address_key.print_as_base58(&encryption, Some(chain_prefix)).expect("just generated address_key, should always work");let mut history: Vec<Event> = Vec::new();
        // search for this address key in existing accounts, get address details
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
        // preparing info that should be signed in case of success
            let for_signing = [transaction_decoded.method.to_vec(), transaction_decoded.extrinsics.encode().to_vec()].concat();
            let index_backup = index;
        // fetch network metadata as RuntimeMetadata
            let method_cards_result = match find_meta(&chain_name, short.metadata_version, &dbname) {
                Ok((meta, ver)) => {
                    if let Some(x) = ver {
                        history.push(Event::Warning(Warning::NewerVersion{used_version: short.metadata_version, latest_version: x}.show()));
                        let add = Card::Warning(Warning::NewerVersion{used_version: short.metadata_version, latest_version: x}).card(&mut index, indent);
                        cards_prep = match cards_prep {
                            CardsPrep::SignProceed(author_card, _, address_details) => CardsPrep::SignProceed(author_card, Some(add), address_details),
                            CardsPrep::ShowOnly(author_card, warning_card) => CardsPrep::ShowOnly(author_card, format!("{},{}", warning_card, add)),
                        };
                    }
                // transaction parsing
                    match meta {
                        RuntimeMetadata::V12(_)|RuntimeMetadata::V13(_) => {
                            let older_meta = match meta {
                                RuntimeMetadata::V12(meta_v12) => {OlderMeta::V12(meta_v12)},
                                RuntimeMetadata::V13(meta_v13) => {OlderMeta::V13(meta_v13)},
                                _ => unreachable!(),
                            };
                            // get types to be used in decoding
                            let type_database = get_types(&dbname)?;
                            match process_as_call (transaction_decoded.method, &older_meta, &type_database, &mut index, indent, &chain_specs_found) {
                                Ok(transaction_parsed) => {
                                    if transaction_parsed.remaining_vector.len() != 0 {Err(Error::BadInputData(BadInputData::SomeDataNotUsed))}
                                    else {Ok(transaction_parsed.fancy_out[1..].to_string())}
                                },
                                Err(e) => Err(e),
                            }
                        },
                        RuntimeMetadata::V14(meta_v14) => {
                             match decoding_sci_entry_point (transaction_decoded.method, &meta_v14, &mut index, indent, &chain_specs_found) {
                                Ok(transaction_parsed) => {
                                    if transaction_parsed.remaining_vector.len() != 0 {Err(Error::BadInputData(BadInputData::SomeDataNotUsed))}
                                    else {Ok(transaction_parsed.fancy_out)}
                                },
                                Err(e) => Err(e),
                            }
                        },
                        _ => return Err(Error::SystemError(SystemError::MetaVersionBelow12)),
                    }
                },
                Err(e) => {
                // run failed on finding/decoding metadata step, produced one of known errors
                    if (e == Error::DatabaseError(DatabaseError::NoMetaThisVersion))||(e == Error::DatabaseError(DatabaseError::NoMetaAtAll)) {Err(e)}
                    else {return Err(e)}
                },
            };
            match method_cards_result {
                Ok(method_cards) => {
                    let extrinsics_cards = print_full_extrinsics (&mut index, indent, &tip_output, &short, chain_name);
                    match cards_prep {
                        CardsPrep::SignProceed(author_card, possible_warning, address_details) => {
                            let sign = TrDbColdSign::generate(&for_signing, &address_details.path, address_details.has_pwd, &address_key, history);
                            let checksum = sign_store_and_get_checksum (sign, &dbname)?;
                            let action_card = Action::Sign(checksum).card();
                            match possible_warning {
                                Some(warning_card) => Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extrinsics\":[{}],{}}}", author_card, warning_card, method_cards, extrinsics_cards, action_card)),
                                None => Ok(format!("{{\"author\":[{}],\"method\":[{}],\"extrinsics\":[{}],{}}}", author_card, method_cards, extrinsics_cards, action_card))
                            }
                        },
                        CardsPrep::ShowOnly(author_card, warning_card) => {
                            Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, method_cards, extrinsics_cards))
                        },
                    }
                },
                Err(e) => {
                    index = index_backup;
                    let error_card = (Card::Error(e)).card(&mut index, indent);
                    let extrinsics_cards = print_full_extrinsics (&mut index, indent, &tip_output, &short, chain_name);
                    match cards_prep {
                        CardsPrep::SignProceed(author_card, possible_warning, _) => {
                            match possible_warning {
                                Some(warning_card) => Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, error_card, extrinsics_cards)),
                                None => Ok(format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, error_card, extrinsics_cards)),
                            }
                        },
                        CardsPrep::ShowOnly(author_card, warning_card) => {
                            Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, error_card, extrinsics_cards))
                        },
                    }
                },
            }
        },
        None => {
        // did not find network with matching genesis hash in database
            let author_card = (Card::AuthorPublicKey{author_public_key, encryption}).card(&mut index, indent);
            let error_card = (Card::Error(Error::DatabaseError(DatabaseError::NoNetwork))).card(&mut index, indent);
        // can print plain extrinsics anyways
            let extrinsics_cards = match short.era {
                Era::Immortal => format!("{},{},{}", (Card::EraImmortalNonce(short.nonce)).card(&mut index, indent), (Card::TipPlain(short.tip)).card(&mut index, indent), (Card::TxSpecPlain{gen_hash: &hex::encode(transaction_decoded.genesis_hash), version: short.metadata_version, tx_version: short.tx_version}).card(&mut index, indent)),
                Era::Mortal(period, phase) => format!("{},{},{},{}", (Card::EraMortalNonce{phase, period, nonce: short.nonce}).card(&mut index, indent), (Card::TipPlain(short.tip)).card(&mut index, indent), (Card::BlockHash(&hex::encode(short.block_hash))).card(&mut index, indent), (Card::TxSpecPlain{gen_hash: &hex::encode(transaction_decoded.genesis_hash), version: short.metadata_version, tx_version: short.tx_version}).card(&mut index, indent)),
            };
            Ok(format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, error_card, extrinsics_cards))
        },
    }
}
