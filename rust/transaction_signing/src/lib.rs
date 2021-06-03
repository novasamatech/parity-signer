use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use ethsign::{keyfile::Crypto, Protected};
use serde_json;

mod interpretation;
use interpretation::get_info;

/// function to create signatures using js output action line, and user entered pin and password
pub fn create_signature (action_line: &str, pin: &str, pwd_entry: &str) -> Result<String, Box<dyn std::error::Error>> {

// get information needed from action line    
    let action = get_info(action_line)?;

// get words from encoded seed
    let password = Protected::new(pin.as_bytes());
    let pwd = {
        if action.has_pwd {Some(pwd_entry)}
        else {None}
    };
    let seed_to_use: String = serde_json::from_str(&action.seed)?;
    let crypto: Crypto = serde_json::from_str(&seed_to_use)?;
    let decrypted = crypto.decrypt(&password)?;
    let words = String::from_utf8(decrypted)?;
    
// get full line with derivation path, used for signature preparation
    let full_line = format!("{}{}", words, action.path);
    
    match action.crypto {
        "ed25519" => {
            let ed25519_pair = ed25519::Pair::from_string(&full_line, pwd).unwrap();
            let signature = ed25519_pair.sign(&action.transaction[..]);
            Ok(format!("00{}", hex::encode(signature)))
        },
        "sr25519" => {
            let sr25519_pair = sr25519::Pair::from_string(&full_line, pwd).unwrap();
            let signature = sr25519_pair.sign(&action.transaction[..]);
            Ok(format!("01{}", hex::encode(signature)))
        },
        "ecdsa" => {
            let ecdsa_pair = ecdsa::Pair::from_string(&full_line, pwd).unwrap();
            let signature = ecdsa_pair.sign(&action.transaction[..]);
            Ok(format!("02{}", hex::encode(signature)))
        },
        _ => return Err(Box::from("Encryption type not supported"))
    }
}
