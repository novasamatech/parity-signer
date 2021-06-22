use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use meta_reading::{address_book::get_default_address_book, decode_metadata::decode_version, fetch_metadata::fetch_info};
use qrcode_rtx::transform_into_qr_apng;

pub enum CryptoUsed <'a> {
    None,
    Ed25519 {pwd: Option<&'a str>, full_line: String},
    Sr25519 {pwd: Option<&'a str>, full_line: String},
    Ecdsa {pwd: Option<&'a str>, full_line: String},
}


/// Function to create hex string from metadata as &str, genesis hash as &str, and crypto info

pub fn create_metadata_transfer_string <'a> (meta: &'a str, genesis_hash: &'a str, crypto_used: &'a CryptoUsed <'a>) -> Result<String, Box<dyn std::error::Error>> {
    
    let meta = match &meta[..2] {
        "0x" => &meta[2..],
        _ => &meta[..],
    };
    
    let genesis_hash = match &genesis_hash[..2] {
        "0x" => &genesis_hash[2..],
        _ => &genesis_hash[..],
    };
    
    let meta_vector = hex::decode(meta)?;
    let genesis_hash_vector = hex::decode(genesis_hash)?;
    let vector_to_sign = [meta_vector, genesis_hash_vector].concat();
    
    match crypto_used {
        CryptoUsed::None => {
            Ok(format!("53ff80{}{}", meta, genesis_hash))
        },
        CryptoUsed::Ed25519 {pwd, full_line} => {
            let ed25519_pair = match ed25519::Pair::from_string(&full_line, *pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ed25519 crypto."))
            };
            let signature = ed25519_pair.sign(&vector_to_sign[..]);
            Ok(format!("530080{}{}{}{}", hex::encode(ed25519_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
        CryptoUsed::Sr25519 {pwd, full_line} => {
            let sr25519_pair = match sr25519::Pair::from_string(&full_line, *pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for sr25519 crypto."))
            };
            let signature = sr25519_pair.sign(&vector_to_sign[..]);
            Ok(format!("530180{}{}{}{}", hex::encode(sr25519_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
        CryptoUsed::Ecdsa {pwd, full_line} => {
            let ecdsa_pair = match ecdsa::Pair::from_string(&full_line, *pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ecdsa crypto."))
            };
            let signature = ecdsa_pair.sign(&vector_to_sign[..]);
            Ok(format!("530280{}{}{}{}", hex::encode(ecdsa_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
    }
}


/// Function to create a file with fountain qr code from metadata as &str,
/// genesis hash as &str, and crypto info

pub fn create_metadata_transfer_qr <'a> (meta: &'a str, genesis_hash: &'a str, crypto_used: &'a CryptoUsed <'a>) -> Result<(), Box<dyn std::error::Error>> {
    let output_name = make_output_name(&meta)?;
    let input_hex = create_metadata_transfer_string (meta, genesis_hash, crypto_used)?;
    let input = hex::decode(&input_hex).expect("Just created the proper hex string. Is always decodeable.");
    transform_into_qr_apng(&input, &output_name)
}


/// Function to create a file with hex string and a file with fountain qr code
/// from metadata as &str, genesis hash as &str, and crypto info

pub fn create_qr_and_string_file <'a> (meta: &'a str, genesis_hash: &'a str, crypto_used: &'a CryptoUsed <'a>) -> Result<(), Box<dyn std::error::Error>> {
    let output_name = make_output_name(&meta)?;
    let input_hex = create_metadata_transfer_string (meta, genesis_hash, crypto_used)?;
//    println!("{}", input_hex);
    std::fs::write(&format!("{}.txt", output_name), &input_hex)?;
    let input = hex::decode(&input_hex).expect("Just created the proper hex string. Is always decodeable.");
    transform_into_qr_apng(&input, &output_name)?;
    Ok(())
}


/// function to generate a filename

fn make_output_name (meta: &str) -> Result<String, Box<dyn std::error::Error>> {
    let version_info = decode_version(meta)?;
    Ok(format!("tests/network_metadata_{}V{}", version_info.name, version_info.version))
}


/// Function to read default address book from metadata_reading crate,
/// fetch fresh metadata and genesis hash for each network, 
/// and generate qr codes and string files

pub fn generate_defaults <'a> (crypto_used: &'a CryptoUsed <'a>) -> Result<(), Box<dyn std::error::Error>> {
    let address_book = get_default_address_book();
    for x in address_book.iter() {
        let new_info = fetch_info(x.address)?;
        let genesis_hash_fetched = match &new_info.genesis_hash[..2] {
            "0x" => hex::decode(&new_info.genesis_hash[2..])?,
            _ => hex::decode(&new_info.genesis_hash)?,
        };
        if genesis_hash_fetched != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
        create_qr_and_string_file (&new_info.meta, &new_info.genesis_hash, crypto_used)?;
    }
    Ok(())
}
