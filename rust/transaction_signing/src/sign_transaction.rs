use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sled::{Db, open, Tree};
use definitions::{transactions::SignDb, constants::{TRANSACTION, SIGNTRANS}, users::Encryption};
use parity_scale_codec::Decode;
use std::convert::TryInto;


/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub fn create_signature (seed_phrase: &str, pwd_entry: &str, dbname: &str, checksum: u32) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    let action = match transaction.get(SIGNTRANS)? {
        Some(x) => {<SignDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved transaction found."))}
    };
    
    let pwd = {
        if action.has_pwd {Some(pwd_entry)}
        else {None}
    };
    
// get full address with derivation path, used for signature preparation
// TODO zeroize
    let full_address = seed_phrase.to_owned() + &action.path;
    
    match action.crypto {
        Encryption::Ed25519 => {
            let ed25519_pair = match ed25519::Pair::from_string(&full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ed25519 crypto."))
            };
            let into_key: [u8; 32] = match action.address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Public key not compatible with ed25519 crypto.")),
            };
            let key = ed25519::Public::from_raw(into_key);
            if key != ed25519_pair.public() {return Err(Box::from("Wrong password."))}
            let signature = ed25519_pair.sign(&action.transaction[..]);
            transaction.remove(SIGNTRANS)?;
            database.flush()?;
            Ok(format!("00{}", hex::encode(signature)))
        },
        Encryption::Sr25519 => {
            let sr25519_pair = match sr25519::Pair::from_string(&full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for sr25519 crypto."))
            };
            let into_key: [u8; 32] = match action.address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Public key not compatible with sr25519 crypto.")),
            };
            let key = sr25519::Public::from_raw(into_key);
            if key != sr25519_pair.public() {return Err(Box::from("Wrong password."))}
            let signature = sr25519_pair.sign(&action.transaction[..]);
            transaction.remove(SIGNTRANS)?;
            database.flush()?;
            Ok(format!("01{}", hex::encode(signature)))
        },
        Encryption::Ecdsa => {
            let ecdsa_pair = match ecdsa::Pair::from_string(&full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Box::from("Error generating keys for ecdsa crypto."))
            };
            let into_key: [u8; 33] = match action.address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Public key not compatible with ecdsa crypto.")),
            };
            let key = ecdsa::Public::from_raw(into_key);
            if key != ecdsa_pair.public() {return Err(Box::from("Wrong password."))}
            let signature = ecdsa_pair.sign(&action.transaction[..]);
            transaction.remove(SIGNTRANS)?;
            database.flush()?;
            Ok(format!("02{}", hex::encode(signature)))
        },
    }
}
