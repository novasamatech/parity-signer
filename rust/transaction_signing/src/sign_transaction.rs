use anyhow;
use definitions::{transactions::SignDb, constants::{TRANSACTION, SIGNTRANS}, users::Encryption};
use parity_scale_codec::Decode;
use db_handling::helpers::{open_db, open_tree, flush_db, remove_from_tree};

use crate::sign_message::sign_as_address_key;
use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub fn create_signature (seed_phrase: &str, pwd_entry: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let action = match transaction.get(SIGNTRANS) {
        Ok(Some(x)) => match <SignDb>::decode(&mut &x[..]) {
            Ok(a) => a,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::SignTransaction).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::SignTransaction).show()),
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
    
    remove_from_tree(SIGNTRANS.to_vec(), &transaction)?;
    flush_db(&database)?;
    
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
