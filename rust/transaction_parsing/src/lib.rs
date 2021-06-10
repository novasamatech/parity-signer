use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use printing_balance::{PrettyOutput, convert_balance_pretty};
use sled::{Db, Tree, open};
use db_handling::{chainspecs::ChainSpecs, settings::{TypeEntry, SignDb}, users::AddressDetails};
use sp_runtime::generic::Era;
use std::convert::TryInto;

mod constants;
mod utils_base58;
    use utils_base58::{vec_to_base};
mod utils_chainspecs;
    use utils_chainspecs::{find_meta};
mod method;
mod decoding;
    use decoding::{process_as_call, fancy};


/// struct to separate prelude, address, actual method, and extrinsics in transaction string
#[derive(Debug, parity_scale_codec_derive::Decode)]
struct TransactionParts {
    method: Vec<u8>,
    extrinsics: ExtrinsicValues,
    genesis_hash: [u8; 32],
}

/// enum to record author public key depending on crypto used: so far ed25519, sr25519, and ecdsa should be supported
enum AuthorPublicKey {
    Ed25519([u8; 32]),
    Sr25519([u8; 32]),
    Ecdsa([u8; 33]),
}

/// struct to decode extrinsics
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

/// struct to store the output of decoding: "normal" format and fancy easy-into-js format

pub struct DecodingResult {
    pub normal_cards: String,
    pub js_cards: String,
}


/// function to print extrinsics in fancy format
fn print_fancy_extrinsics (index: u32, indent: u32, tip_output: &PrettyOutput, short: &ExtrinsicValues, chain_name: &str) -> String {
    match short.era {
        Era::Immortal => format!("{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", short.nonce)), fancy(index+1, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", tip_output.number, tip_output.units)), fancy(index+2, indent, "tx_spec", &format!("{{\"network\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", chain_name, short.metadata_version, short.tx_version))),
        Era::Mortal(period, phase) => format!("{},{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, short.nonce)), fancy(index+1, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", tip_output.number, tip_output.units)), fancy(index+2, indent, "block_hash", &format!("\"{}\"", hex::encode(short.block_hash))), fancy(index+3, indent, "tx_spec", &format!("{{\"network\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", chain_name, short.metadata_version, short.tx_version))),
    }
}


/// function to parse full transaction
/// transaction format corresponds to what we get from qr code:
/// i.e. it starts with 53****, followed by author address, followed by actual transaction piece,
/// followed by extrinsics, concluded with chain genesis hash

pub fn full_run (transaction: &str, dbname: &str) -> Result<DecodingResult, Box<dyn std::error::Error>> {
    let data_hex = match transaction.starts_with("0x") {
        true => &transaction[2..],
        false => &transaction,
    };
    
    if &data_hex[..2] != "53" {return Err(Box::from("Only Substrate transactions are supported."))}
    
    let data = match hex::decode(&data_hex) {
        Ok(a) => a,
        Err(_) => return Err(Box::from("Wrong format of input transaction string.")),
    };
    
    let (author_pub_key, data) = match &data_hex[2..4] {
        "00" => (AuthorPublicKey::Ed25519(data[3..35].try_into().unwrap()), &data[35..]),
        "01" => (AuthorPublicKey::Sr25519(data[3..35].try_into().unwrap()), &data[35..]),
        "02" => (AuthorPublicKey::Ecdsa(data[3..36].try_into().unwrap()), &data[36..]),
        _ => return Err(Box::from("Crypto type not supported."))
    };
    
    let transaction_decoded = match <TransactionParts>::decode(&mut &data[..]) {
        Ok(a) => a,
        Err(_) => return Err(Box::from("Error separating method and extrinsics")),
    };
    
    let short = &transaction_decoded.extrinsics;
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

// try to get chain specs from genesis hash
    if transaction_decoded.genesis_hash != short.genesis_hash {return Err(Box::from("Two different genesis hashes are found."))}

// this should be here by the standard; should stay commented for now, since the test transactions apparently do not comply to standard.
    /*match &data_hex[4..6] {
        "00" => {
            if let Era::Immortal = short.era {return Err(Box::from("Expected mortal transaction because of prelude. Got immortal one on decoding."))}
        },
        "02" => {
            if let Era::Mortal(_, _) = short.era {return Err(Box::from("Expected immortal transaction because of prelude. Got mortal one on decoding."))}
        },
        _ => return Err(Box::from("Payload type not supported")),
    }*/

    if let Era::Immortal = short.era {if short.genesis_hash != short.block_hash {return Err(Box::from("Block hash found to not be equal to genesis hash in immortal transaction."))}}
    
    let database: Db = open(dbname)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    let metadata: Tree = database.open_tree(b"metadata")?;
    let addresses: Tree = database.open_tree(b"addresses")?;
    let settings: Tree = database.open_tree(b"settings")?;
    
    match chainspecs.get(transaction_decoded.genesis_hash)? {
        Some(x) => {
            let chain_specs_found = <ChainSpecs>::decode(&mut &x[..])?;
            let chain_name = &chain_specs_found.name;
            let chain_prefix = chain_specs_found.base58prefix;

        // transform public key into base58 address and get crypto for action card exporting
            let (author, crypto) = match author_pub_key {
                AuthorPublicKey::Ed25519(t) => (vec_to_base(&(t.to_vec()), chain_prefix), "ed25519"),
                AuthorPublicKey::Sr25519(t) => (vec_to_base(&(t.to_vec()), chain_prefix), "sr25519"),
                AuthorPublicKey::Ecdsa(t) => (vec_to_base(&(t.to_vec()), chain_prefix), "ecdsa"),
            };
        // search for this base58 address in existing accounts, get address details
            match addresses.get(author.encode())? {
                Some(y) => {
                    
                    let id_values = <AddressDetails>::decode(&mut &y[..])?;
                    let mut to_normal = format!("\"author\":{{\"base58\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\"}}", author, id_values.path, id_values.has_pwd, id_values.name);
                    let mut to_fancy = fancy(index, indent, "author", &format!("{{\"base58\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\"}}", author, id_values.path, id_values.has_pwd, id_values.name));
                    index = index + 1;
            
                // update tip output
                    let tip_output = convert_balance_pretty (short.tip, chain_specs_found.decimals, &chain_specs_found.unit)?;
            
                // transform extrinsics information for normal output
                    let extrinsics_to_normal = match short.era {
                        Era::Immortal => format!("\"extrinsics\":{{\"era\":\"Immortal\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version),
                        Era::Mortal(period, phase) => format!("\"extrinsics\":{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\",\"block_hash\":\"{}\"}}", phase, period, short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version, hex::encode(short.block_hash)),
                    };
            
                // fetch chain metadata in RuntimeMetadataV12 format
                    match find_meta(&chain_name, short.metadata_version, &metadata) {
                        Ok((meta, ver)) => {
                            if let Some(x) = ver {
                                let warn_normal = format!(",\"warning\":{{\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"}}", short.metadata_version, x);
                                to_normal.push_str(&warn_normal);
                                let warn_fancy = format!("],\"warning\":[{}", fancy(index, indent, "warning", &format!("\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"", short.metadata_version, x)));
                                index = index + 1;
                                to_fancy.push_str(&warn_fancy);
                            }
                    
                        // generate type database to be used in decoding
                            
                            let type_database = match settings.get(String::from("types").encode())? {
                                Some(a) => <Vec<TypeEntry>>::decode(&mut &a[..])?,
                                None => {return Err(Box::from("No types info found in database."))}
                            };
                    
                        // action card preparations
                            let extrinsics_to_sign = &transaction_decoded.extrinsics.encode();
                            let prep_to_sign = [transaction_decoded.method.to_vec(), extrinsics_to_sign.to_vec()].concat();
                    
                        // transaction parsing
                            match process_as_call (transaction_decoded.method, &meta, &type_database, index, indent, &chain_specs_found) {
                                Ok(transaction_parsed) => {
                                    let index = transaction_parsed.index;
                                    if transaction_parsed.remaining_vector.len() != 0 {return Err(Box::from("After transaction parsing, some data in transaction vector remained unused."))}

                                    let normal_cards = format!("{},{},{}", to_normal, transaction_parsed.decoded_string, extrinsics_to_normal);
                                // transform extrinsics information for fancy output
                                    let extrinsics_to_js = print_fancy_extrinsics (index, indent, &tip_output, &short, chain_name);
                                // making action entry into database
                                    let action_into_db = SignDb {
                                        crypto: crypto.to_string(),
                                        path: id_values.path,
                                        name_for_seed: id_values.name_for_seed,
                                        transaction: prep_to_sign,
                                        has_pwd: id_values.has_pwd,
                                        author_base58: author,
                                    };
                                    settings.insert(b"sign_transaction", action_into_db.encode())?;
                                    database.flush()?;
                                    let checksum = database.checksum()?;
                                // making action card for js
                                    let action = format!("\"action\":{{\"type\":\"sign_transaction\",\"payload\":{{\"checksum\":\"{}\",\"has_password\":\"{}\"}}}}", checksum, id_values.has_pwd);
                                    let js_cards = format!("{{\"author\":[{}],\"method\":[{}],\"extrinsics\":[{}],{}}}", to_fancy, &transaction_parsed.fancy_out[1..], extrinsics_to_js, action);
                                    Ok(DecodingResult{
                                        normal_cards,
                                        js_cards,
                                    })
                                },
                                Err(e) => {
                                    let mut err = String::from("Unable to decode the transaction.");
                                    if e == "Could not interpret the type." {
                                        err.push_str(" Unknown types encountered.")
                                    }
                                    let error_normal = format!("\"error\":{{\"{}\"}}", err);
                                    let error_fancy = fancy(index, indent, "error", &format!("\"{}\"", e));
                                    index = index + 1;
                                    let normal_cards = format!("{},{},{}", to_normal, error_normal, extrinsics_to_normal);
                                // transform extrinsics information for fancy output
                                    let extrinsics_to_js = print_fancy_extrinsics (index, indent, &tip_output, &short, chain_name);
                                    let js_cards = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", to_fancy, error_fancy, extrinsics_to_js);
                                    Ok(DecodingResult{
                                        normal_cards,
                                        js_cards,
                                    })
                                },
                            }
                        },
                        Err(e) => {
                            let error_normal = format!("\"error\":{{\"{}\"}}", e);
                            let error_fancy = fancy(index, indent, "error", &format!("\"{}\"", e));
                            index = index + 1;
                            let normal_cards = format!("{},{},{}", to_normal, error_normal, extrinsics_to_normal);
                        // transform extrinsics information for fancy output
                            let extrinsics_to_js = print_fancy_extrinsics (index, indent, &tip_output, &short, chain_name);
                            let js_cards = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", to_fancy, error_fancy, extrinsics_to_js);
                            Ok(DecodingResult{
                                normal_cards,
                                js_cards,
                            })
                        },
                    }
                },
                None => {return Err(Box::from("Transaction made by account not listed in current account database. Please add the account."))}
            }
        },
        None => {
            let error_normal = String::from("\"error\":{\"Network not found. Please add the network.\"}");
            let error_fancy = fancy(index, indent, "error", "\"Network not found. Please add the network.\"");
            index = index + 1;
            // extrinsics information
            let extrinsics_to_normal = match short.era {
                Era::Immortal => format!("\"extrinsics\":{{\"era\":\"Immortal\",\"nonce\":\"{}\",\"tip\":\"{}\",\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", short.nonce, short.tip, hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version),
                Era::Mortal(period, phase) => format!("\"extrinsics\":{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\",\"tip\":\"{}\",\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\",\"block_hash\":\"{}\"}}", phase, period, short.nonce, short.tip, hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version, hex::encode(short.block_hash)),
            };
            let extrinsics_to_js = match short.era {
                Era::Immortal => format!("{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", short.nonce)), fancy(index+1, indent, "tip_plain", &format!("\"{}\"", short.tip)), fancy(index+2, indent, "tx_spec_plain", &format!("{{\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version))),
                Era::Mortal(period, phase) => format!("{},{},{},{}", fancy(index, indent, "era_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, short.nonce)), fancy(index+1, indent, "tip_plain", &format!("\"{}\"", short.tip)), fancy(index+2, indent, "block_hash", &format!("\"{}\"", hex::encode(short.block_hash))), fancy(index+3, indent, "tx_spec_plain", &format!("{{\"chain_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", hex::encode(transaction_decoded.genesis_hash), short.metadata_version, short.tx_version))),
            };
            let normal_cards = format!("{},{}", error_normal, extrinsics_to_normal);
            let js_cards = format!("{{\"error\":[{}],\"extrinsics\":[{}]}}", error_fancy, extrinsics_to_js);
            Ok(DecodingResult{
                normal_cards,
                js_cards,
            })
        },
    }
}


