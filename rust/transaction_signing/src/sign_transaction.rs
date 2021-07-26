use anyhow;
use sled::{Db, open, Tree};
use definitions::{transactions::SignDb, constants::{TRANSACTION, SIGNTRANS}, users::Encryption};
use parity_scale_codec::Decode;

use super::sign_message::sign_as_address_key;
use super::error::{Error, ActionFailure};

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub fn create_signature (seed_phrase: &str, pwd_entry: &str, dbname: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let action = match transaction.get(SIGNTRANS) {
        Ok(a) => match a {
            Some(x) => match <SignDb>::decode(&mut &x[..]) {
                Ok(b) => b,
                Err(_) => return Err(Error::BadActionDecode(ActionFailure::SignTransaction).show()),
            },
            None => return Err(Error::NoAction(ActionFailure::SignTransaction).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let pwd = {
        if action.has_pwd {Some(pwd_entry)}
        else {None}
    };
    
// get full address with derivation path, used for signature preparation
// TODO zeroize
    let full_address = seed_phrase.to_owned() + &action.path;
    let hex_signature = hex::encode(sign_as_address_key(&action.transaction, action.address_key, &action.encryption, &full_address, pwd)?);
    
    match transaction.remove(SIGNTRANS) {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match action.encryption {
        Encryption::Ed25519 => {
            Ok(format!("00{}", hex_signature))
        },
        Encryption::Sr25519 => {
            Ok(format!("01{}", hex_signature))
        },
        Encryption::Ecdsa => {
            Ok(format!("02{}", hex_signature))
        },
    }
}
