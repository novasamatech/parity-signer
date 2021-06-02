use hex;
use parity_scale_codec::{Decode, Encode};
use printing_balance::{PrettyOutput, convert_balance_pretty};
use sp_runtime::generic::Era;

pub mod constants;
pub mod utils_base58;
    use utils_base58::{arr_to_base, get_id_values};
pub mod utils_chainspecs;
    use utils_chainspecs::{specs_from_genesis_hash, find_meta};
pub mod map_types;
pub mod parse_types;
    use parse_types::generate_type_database;
pub mod method;
pub mod decoding;
    use decoding::{process_as_call, fancy};


/// struct to store three important databases: chain_spec, metadata, and types_info
pub struct DataFiles<'a> {
    pub chain_spec_database: &'a str,
    pub metadata_contents: &'a str,
    pub types_info: &'a str,
    pub identities: &'a str,
}

/// struct to separate prelude, address, actual method, and extrinsics in transaction string
#[derive(Debug, Decode)]
pub struct TransactionParts {
    pub prelude: [u8; 3],
    pub author: [u8; 32],
    pub method: Vec<u8>,
    pub extrinsics: ExtrinsicValues,
    pub genesis_hash: [u8; 32],
}

/// struct to decode extrinsics
#[derive(Debug, Decode)]
pub struct ExtrinsicValues {
    pub era: Era,
#[codec(compact)]
    pub nonce: u64,
#[codec(compact)]
    pub tip: u128,
    pub metadata_version: u32,
    pub tx_version: u32,
    pub genesis_hash: [u8; 32],
    pub block_hash: [u8; 32],
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

pub fn full_run (transaction: &str, datafiles: DataFiles) -> Result<DecodingResult, &'static str> {
    let data_hex = match transaction.starts_with("0x") {
        true => &transaction[2..],
        false => &transaction,
    };
    
    let data = match hex::decode(data_hex) {
        Ok(a) => a,
        Err(_) => return Err("Wrong format of input transaction string."),
    };
    
    let transaction_decoded = match <TransactionParts>::decode(&mut &data[..]) {
        Ok(a) => a,
        Err(_) => return Err("Error separating prelude, author address, method, and extrinsics"),
    };
    
    let short = transaction_decoded.extrinsics;
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

// try to get chain specs from genesis hash
    if transaction_decoded.genesis_hash != short.genesis_hash {return Err("Two different genesis hashes are found.")}
    if let Era::Immortal = short.era {if short.genesis_hash != short.block_hash {return Err("Block hash found to not be equal to genesis hash in immortal transaction.")}}
    
    let genesis_hash = hex::encode(&transaction_decoded.genesis_hash);
    
    match specs_from_genesis_hash(&datafiles.chain_spec_database, &genesis_hash) {
        Ok(chain_specs) => {
            let chain_name = &chain_specs.name;
            let chain_prefix = chain_specs.base58prefix;

        // transform author
            let author = arr_to_base(transaction_decoded.author, chain_prefix);
        // and get id values for author, those also go into action card if action card is created
            let id_values = get_id_values(&author, &datafiles.identities)?;
            
            let mut to_normal = format!("\"author\":{{\"base58\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\"}}", author, id_values.path, id_values.has_pwd, id_values.name);
            let mut to_fancy = fancy(index, indent, "author", &format!("{{\"base58\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\"}}", author, id_values.path, id_values.has_pwd, id_values.name));
            index = index + 1;
            
        // update tip output
            let tip_output = convert_balance_pretty (short.tip, chain_specs.decimals, &chain_specs.unit)?;
            
        // transform extrinsics information for normal output
            let extrinsics_to_normal = match short.era {
                Era::Immortal => format!("\"extrinsics\":{{\"era\":\"Immortal\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version),
                Era::Mortal(period, phase) => format!("\"extrinsics\":{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\",\"tip\":\"{}\",\"units\":\"{}\",\"chain\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\",\"block_hash\":\"{}\"}}", phase, period, short.nonce, tip_output.number, tip_output.units, chain_name, short.metadata_version, short.tx_version, hex::encode(short.block_hash)),
            };
            
        // fetch chain metadata in RuntimeMetadataV12 format
            match find_meta(&chain_name, short.metadata_version, &datafiles.metadata_contents) {
                Ok((meta, ver)) => {
                    if let Some(x) = ver {
                        let warn_normal = format!(",\"warning\":{{\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"}}", short.metadata_version, x);
                        to_normal.push_str(&warn_normal);
                        let warn_fancy = format!("],\"warning\":[{}", fancy(index, indent, "warning", &format!("\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"", short.metadata_version, x)));
                        index = index + 1;
                        to_fancy.push_str(&warn_fancy);
                    }
                    
                // generate type database to be used in decoding
                    let type_database = generate_type_database (&datafiles.types_info);
                    
                // action card preparations
                    let method_output = hex::encode(&(transaction_decoded.method.encode()));
                    let crypto = match &data_hex[2..4] {
                        "00" => "ed25519",
                        "01" => "sr25519",
                        "02" => "Ecdsa",
                        _ => "unknown",
                    };
                    
                // transaction parsing
                    match process_as_call (transaction_decoded.method, &meta, &type_database, index, indent, &chain_specs) {
                        Ok(transaction_parsed) => {
                            let index = transaction_parsed.index;
                            if transaction_parsed.remaining_vector.len() != 0 {return Err("After transaction parsing, some data in transaction vector remained unused.")}

                            let normal_cards = format!("{},{},{}", to_normal, transaction_parsed.decoded_string, extrinsics_to_normal);
                        //transform extrinsics information for fancy output
                            let extrinsics_to_js = print_fancy_extrinsics (index, indent, &tip_output, &short, chain_name);
                        // making action card for js
                            let action = format!("\"action\":{{\"type\":\"sign_transaction\",\"payload\":{{\"author\":\"{}\",\"encrypted_seed\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\",\"network\":\"{}\",\"version\":\"{}\",\"genesis_hash\":\"{}\",\"prefix\":\"{}\",\"transaction\":\"{}\",\"crypto\":\"{}\"}}}}", author, id_values.seed, id_values.path, id_values.has_pwd, id_values.name, chain_name, short.metadata_version, hex::encode(&transaction_decoded.genesis_hash), chain_prefix, method_output, crypto);
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
                        //transform extrinsics information for fancy output
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
                //transform extrinsics information for fancy output
                    let extrinsics_to_js = print_fancy_extrinsics (index, indent, &tip_output, &short, chain_name);
                    let js_cards = format!("{{\"author\":[{}],\"error\":[{}],\"extrinsics\":[{}]}}", to_fancy, error_fancy, extrinsics_to_js);
                    Ok(DecodingResult{
                        normal_cards,
                        js_cards,
                    })
                },
            }
        },
        Err("No matching genesis hash found.") => {
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
        Err(e) => return Err(e),
    }
}


