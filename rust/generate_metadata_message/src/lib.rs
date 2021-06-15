use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};

pub enum CryptoUsed <'a> {
    None,
    Ed25519 {pwd: Option<&'a str>, full_line: String},
    Sr25519 {pwd: Option<&'a str>, full_line: String},
    Ecdsa {pwd: Option<&'a str>, full_line: String},
}

pub fn create_metadata_transfer <'a> (meta: String, genesis_hash: String, crypto_used: CryptoUsed <'a>) -> Result<String, Box<dyn std::error::Error>> {
    
    let meta_vector = hex::decode(&meta)?;
    let genesis_hash_vector = hex::decode(&genesis_hash)?;
    let vector_to_sign = [meta_vector, genesis_hash_vector].concat();
    
    match crypto_used {
        CryptoUsed::None => {
            Ok(format!("53ff80{}{}", meta, genesis_hash))
        },
        CryptoUsed::Ed25519 {pwd, full_line} => {
            let ed25519_pair = match ed25519::Pair::from_string(&full_line, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ed25519 crypto."))
            };
            let signature = ed25519_pair.sign(&vector_to_sign[..]);
            Ok(format!("530080{}{}{}{}", hex::encode(ed25519_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
        CryptoUsed::Sr25519 {pwd, full_line} => {
            let sr25519_pair = match sr25519::Pair::from_string(&full_line, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for sr25519 crypto."))
            };
            let signature = sr25519_pair.sign(&vector_to_sign[..]);
            Ok(format!("530180{}{}{}{}", hex::encode(sr25519_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
        CryptoUsed::Ecdsa {pwd, full_line} => {
            let ecdsa_pair = match ecdsa::Pair::from_string(&full_line, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ecdsa crypto."))
            };
            let signature = ecdsa_pair.sign(&vector_to_sign[..]);
            Ok(format!("530280{}{}{}{}", hex::encode(ecdsa_pair.public()), meta, genesis_hash, hex::encode(signature)))
        },
    }
}
