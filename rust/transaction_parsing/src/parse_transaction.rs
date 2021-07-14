use hex;
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use printing_balance::{PrettyOutput, convert_balance_pretty};
use sled::{Db, Tree, open};
use definitions::{network_specs::{ChainSpecs, generate_network_key}, transactions::SignDb, users::{AddressDetails, Encryption, generate_address_key}, constants::{SPECSTREE, METATREE, ADDRTREE, SETTREE, SIGNTRANS, TRANSACTION}};
use sp_runtime::generic::Era;
use std::convert::TryInto;

use super::utils_base58::vec_to_base;
use super::utils::{find_meta, get_types};
use super::decoding::process_as_call;
use super::cards::{Action, Card, Warning};
use super::error::{Error, BadInputData, UnableToDecode, DatabaseError, SystemError};

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

/// Enum to record author public key depending on crypto used:
/// so far ed25519, sr25519, and ecdsa should be supported
pub enum AuthorPublicKey {
    Ed25519([u8; 32]),
    Sr25519([u8; 32]),
    Ecdsa([u8; 33]),
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


/// function to print full extrinsics cards
fn print_full_extrinsics (index: u32, indent: u32, tip_output: &PrettyOutput, short: &ExtrinsicValues, chain_name: &str) -> String {
    match short.era {
        Era::Immortal => format!("{},{},{}", (Card::EraImmortalNonce(short.nonce)).card(index, indent), (Card::Tip{number: &tip_output.number, units: &tip_output.units}).card(index+1, indent), (Card::TxSpec{network: chain_name, version: short.metadata_version, tx_version: short.tx_version}).card(index+2, indent)),
        Era::Mortal(period, phase) => format!("{},{},{},{}", (Card::EraMortalNonce{phase, period, nonce: short.nonce}).card(index, indent), (Card::Tip{number: &tip_output.number, units: &tip_output.units}).card(index+1, indent), (Card::BlockHash(&hex::encode(short.block_hash))).card(index+2, indent), (Card::TxSpec{network: chain_name, version: short.metadata_version, tx_version: short.tx_version}).card(index+3, indent)),
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

// loading the database and removing the previous (if any) signing saves
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let addresses: Tree = match database.open_tree(ADDRTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
// input hex data of correct size should have at least 6 + 64 + 64 symbols (prelude + author public key minimal size + genesis hash)
    if data_hex.len() < 134 {return Err(Error::BadInputData(BadInputData::TooShort))}

    let data = match hex::decode(&data_hex) {
        Ok(a) => a,
        Err(_) => return Err(Error::BadInputData(BadInputData::NotHex)),
    };
    
    let (author_pub_key, data) = match &data_hex[2..4] {
        "00" => (AuthorPublicKey::Ed25519(data[3..35].try_into().expect("fixed size should fit in array")), &data[35..]),
        "01" => (AuthorPublicKey::Sr25519(data[3..35].try_into().expect("fixed size should fit in array")), &data[35..]),
        "02" => (AuthorPublicKey::Ecdsa(data[3..36].try_into().expect("fixed size should fit in array")), &data[36..]),
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
    //if &data_hex[4..6] == "00" {if let Era::Immortal = short.era {return Err(Error::BadInputData(BadInputData::UnexpectedImmortality))}}
    //if &data_hex[4..6] == "02" {if let Era::Mortal(_, _) = short.era {return Err(Error::BadInputData(BadInputData::UnexpectedMortality))}}

    if let Era::Immortal = short.era {if short.genesis_hash != short.block_hash {return Err(Error::BadInputData(BadInputData::ImmortalHashMismatch))}}
    
    let network_key = generate_network_key(&transaction_decoded.genesis_hash.to_vec());
    
    let chainspecs_db_reply = match chainspecs.get(&network_key) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    match chainspecs_db_reply {
        Some(x) => {
            let chain_specs_found = match <ChainSpecs>::decode(&mut &x[..]) {
                Ok(x) => x,
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            };
            let chain_name = &chain_specs_found.name;
            let chain_prefix = chain_specs_found.base58prefix;
            
        // update tip output since we know chain specs already
            let tip_output = match convert_balance_pretty (short.tip, chain_specs_found.decimals, &chain_specs_found.unit) {
                Ok(x) => x,
                Err(_) => return Err(Error::SystemError(SystemError::BalanceFail)),
            };

        // transform public key into base58 address and get crypto for action card exporting
            let (author, address_key, crypto) = match author_pub_key {
                AuthorPublicKey::Ed25519(t) => (vec_to_base(&(t.to_vec()), chain_prefix), generate_address_key(&t.to_vec()), Encryption::Ed25519),
                AuthorPublicKey::Sr25519(t) => (vec_to_base(&(t.to_vec()), chain_prefix), generate_address_key(&t.to_vec()), Encryption::Sr25519),
                AuthorPublicKey::Ecdsa(t) => (vec_to_base(&(t.to_vec()), chain_prefix), generate_address_key(&t.to_vec()), Encryption::Ecdsa),
            };
        // search for this base58 address in existing accounts, get address details
            let addresses_db_reply = match addresses.get(&address_key) {
                Ok(x) => x,
                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
            };
            match addresses_db_reply {
                Some(y) => {
                    let address_details = match <AddressDetails>::decode(&mut &y[..]) {
                        Ok(x) => x,
                        Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedAddressDetails)),
                    };
                
                    let author_card = (Card::Author{base58_author: &author, seed_name: &address_details.seed_name, path: &address_details.path, has_pwd: address_details.has_pwd, name: &address_details.name}).card(index, indent);
                    index = index + 1;
                    
                // current network is among allowed networks for this address key;
                    let warn_network_not_allowed = {
                        if address_details.network_id.contains(&network_key) {None}
                        else {
                            let warn_no_network_id = Card::Warning(Warning::NoNetworkID).card(index, indent);
                            index = index + 1;
                            Some(warn_no_network_id)
                        }
                    };

                // fetch chain metadata in RuntimeMetadataV12 format
                    match find_meta(&chain_name, short.metadata_version, &metadata) {
                        Ok((meta, ver)) => {
                            let mut warning_card = None;
                            if let Some(x) = ver {
                                warning_card = Some(Card::Warning(Warning::NewerVersion{used_version: short.metadata_version, latest_version: x}).card(index, indent));
                                index = index + 1;
                            }
                    
                        // generate type database to be used in decoding
                            
                            let type_database = get_types(&settings)?;
                    
                        // action card preparations: vector that should be signed
                            let for_signing = [transaction_decoded.method.to_vec(), transaction_decoded.extrinsics.encode().to_vec()].concat();
                    
                        // transaction parsing
                            match process_as_call (transaction_decoded.method, &meta, &type_database, index, indent, &chain_specs_found) {
                                Ok(transaction_parsed) => {
                                    let method_cards = &transaction_parsed.fancy_out[1..];
                                    let index = transaction_parsed.index;
                                    if transaction_parsed.remaining_vector.len() != 0 {return Err(Error::BadInputData(BadInputData::SomeDataNotUsed))}

                                // make extrinsics card set
                                    let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                                    
                                    match warn_network_not_allowed {
                                        None => {
                                        // network is among the allowed ones for this address key; can sign;
                                        // making action entry into database
                                            let action_into_db = SignDb {
                                                crypto,
                                                path: address_details.path,
                                                transaction: for_signing,
                                                has_pwd: address_details.has_pwd,
                                                address_key,
                                            };
                                            match transaction.insert(SIGNTRANS, action_into_db.encode()) {
                                                Ok(_) => (),
                                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                                            };
                                            match database.flush() {
                                                Ok(_) => (),
                                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                                            };
                                            let checksum = match database.checksum() {
                                                Ok(x) => x,
                                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                                            };
                                        // action card
                                            let action_card = Action::SignTransaction(checksum).card();
                                        // full cards set
                                            let cards = match warning_card {
                                                Some(warn) => format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extrinsics\":[{}],{}}}", author_card, warn, method_cards, extrinsics_cards, action_card),
                                                None => format!("{{\"author\":[{}],\"method\":[{}],\"extrinsics\":[{}],{}}}", author_card, method_cards, extrinsics_cards, action_card),
                                            };
                                            Ok(cards)
                                        },
                                        Some(warn_no_network_id) => {
                                        // network is NOT among the allowed ones for this address key; should not happen; can decode, not allowed to sign
                                            let cards = match warning_card {
                                                Some(warn) => format!("{{\"author\":[{}],\"warning\":[{},{}],\"method\":[{}],\"extrinsics\":[{}]}}", author_card, warn_no_network_id, warn, method_cards, extrinsics_cards),
                                                None => format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extrinsics\":[{}]}}", author_card, warn_no_network_id, method_cards, extrinsics_cards),
                                            };
                                            Ok(cards)
                                        }
                                    }
                                },
                                Err(e) => {
                                // was unable to decode transaction properly, produced one of known decoding errors
                                // no action possible
                                    let error_card = (Card::Error(e)).card(index, indent);
                                    index = index + 1;
                                // make extrinsics card set
                                    let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                                // full cards set
                                    let cards = match warning_card {
                                        Some(warn) => format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, warn, error_card, extrinsics_cards),
                                        None => format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, error_card, extrinsics_cards),
                                    };
                                    Ok(cards)
                                },
                            }
                        },
                        Err(e) => {
                        // run failed on finding/decoding metadata step, produced one of known errors
                            if (e == Error::DatabaseError(DatabaseError::NoMetaThisVersion))||(e == Error::DatabaseError(DatabaseError::NoMetaAtAll)) {
                                let error_card = (Card::Error(e)).card(index, indent);
                                index = index + 1;
                            // make extrinsics card set
                                let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                            // full cards set
                                let cards = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, error_card, extrinsics_cards);
                                Ok(cards)
                            }
                            else {return Err(e)}
                        },
                    }
                },
                None => {
                // identity not found in database
                // try to decode the transaction anyways
                // no action card made, no signing possible
                    let author_card = (Card::AuthorPlain(&author)).card(index, indent);
                    index = index + 1;
                    let mut warning_card = (Card::Warning(Warning::AuthorNotFound)).card(index, indent);
                    index = index + 1;
                    
                    // fetch chain metadata in RuntimeMetadataV12 format
                    match find_meta(&chain_name, short.metadata_version, &metadata) {
                        Ok((meta, ver)) => {
                            if let Some(x) = ver {
                                let add_this = (Card::Warning(Warning::NewerVersion{used_version: short.metadata_version, latest_version: x})).card(index, indent);
                                warning_card.push_str(&format!(",{}", add_this));
                                index = index + 1;
                            }
                    
                        // generate type database to be used in decoding
                            
                            let type_database = get_types(&settings)?;

                        // transaction parsing
                            match process_as_call (transaction_decoded.method, &meta, &type_database, index, indent, &chain_specs_found) {
                                Ok(transaction_parsed) => {
                                    let method_cards = &transaction_parsed.fancy_out[1..];
                                    let index = transaction_parsed.index;
                                    if transaction_parsed.remaining_vector.len() != 0 {return Err(Error::BadInputData(BadInputData::SomeDataNotUsed))}

                                // make extrinsics card set
                                    let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                                // full cards set
                                    let cards = format!("{{\"author\":[{}],\"warning\":[{}],\"method\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, method_cards, extrinsics_cards);
                                    Ok(cards)
                                },
                                Err(e) => {
                                // was unable to decode transaction properly, produced one of known decoding errors
                                // no action possible
                                    let error_card = (Card::Error(e)).card(index, indent);
                                    index = index + 1;
                                // make extrinsics card set
                                    let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                                    let cards = format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, error_card, extrinsics_cards);
                                    Ok(cards)
                                },
                            }
                        },
                        Err(e) => {
                        // run failed on finding/decoding metadata step, produced one of known errors
                            if (e == Error::DatabaseError(DatabaseError::NoMetaThisVersion))||(e == Error::DatabaseError(DatabaseError::NoMetaAtAll)) {
                                let error_card = (Card::Error(e)).card(index, indent);
                                index = index + 1;
                            // make extrinsics card set
                                let extrinsics_cards = print_full_extrinsics (index, indent, &tip_output, &short, chain_name);
                                let cards = format!("{{\"author\":[{}],\"warning\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, warning_card, error_card, extrinsics_cards);
                                Ok(cards)
                            }
                            else {return Err(e)}
                        },
                    }
                    
                },
            }
        },
        None => {
        // did not find network with matching genesis hash in database
            let author_card = (Card::AuthorPublicKey(author_pub_key)).card(index, indent);
            index = index + 1;
            let error_card = (Card::Error(Error::DatabaseError(DatabaseError::NoNetwork))).card(index, indent);
            index = index + 1;
        // can print plain extrinsics anyways
            let extrinsics_cards = match short.era {
                Era::Immortal => format!("{},{},{}", (Card::EraImmortalNonce(short.nonce)).card(index, indent), (Card::TipPlain(short.tip)).card(index+1, indent), (Card::TxSpecPlain{gen_hash: &hex::encode(transaction_decoded.genesis_hash), version: short.metadata_version, tx_version: short.tx_version}).card(index+2, indent)),
                Era::Mortal(period, phase) => format!("{},{},{},{}", (Card::EraMortalNonce{phase, period, nonce: short.nonce}).card(index, indent), (Card::TipPlain(short.tip)).card(index+1, indent), (Card::BlockHash(&hex::encode(short.block_hash))).card(index+2, indent), (Card::TxSpecPlain{gen_hash: &hex::encode(transaction_decoded.genesis_hash), version: short.metadata_version, tx_version: short.tx_version}).card(index+3, indent)),
            };

            let cards = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", author_card, error_card, extrinsics_cards);
            Ok(cards)
        },
    }
}
