use anyhow;
use definitions::{constants::{HISTORY, SIGNTRANS, TRANSACTION}, history::Event, network_specs::Verifier, transactions::{Transaction, SignDisplay}, users::Encryption};
use parity_scale_codec::Decode;
use db_handling::{helpers::{open_db, open_tree, flush_db, remove_from_tree}, manage_history::enter_events_into_tree};
use qrcode_static::png_qr_from_hex;

use crate::sign_message::sign_as_address_key;
use crate::error::{Error, ActionFailure, CryptoError};
use crate::helpers::verify_checksum;

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub fn create_signature (seed_phrase: &str, pwd_entry: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let history = open_tree(&database, HISTORY)?;
    
    let action = match transaction.get(SIGNTRANS) {
        Ok(Some(encoded_action)) => match <Transaction>::decode(&mut &encoded_action[..]) {
            Ok(Transaction::Sign(x)) => x,
            Ok(_) => return Err(Error::NoAction(ActionFailure::SignTransaction).show()),
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::SignTransaction).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::SignTransaction).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let pwd = {
        if action.has_pwd {Some(pwd_entry)}
        else {None}
    };
    
    let mut events = action.history;
    let hex_key = hex::encode(&action.address_key);
    
// get full address with derivation path, used for signature preparation
// TODO zeroize
    let full_address = seed_phrase.to_owned() + &action.path;
    
    match sign_as_address_key(&action.transaction, action.address_key, &action.encryption, &full_address, pwd) {
        Ok(s) => {
            let hex_signature = hex::encode(s);
            
            remove_from_tree(SIGNTRANS.to_vec(), &transaction)?;
            flush_db(&database)?;
    
            match action.encryption {
                Encryption::Ed25519 => {
                    let sign_display = SignDisplay {
                        transaction: &hex_signature,
                        author_line: Verifier::Ed25519(hex_key).show_card(),
                    }.show();
                    events.push(Event::TransactionSigned(sign_display));
                    enter_events_into_tree(&history, events)?;
                    flush_db(&database)?;
                    
                    Ok(format!("00{}", hex_signature))
                },
                Encryption::Sr25519 => {
                    let sign_display = SignDisplay {
                        transaction: &hex_signature,
                        author_line: Verifier::Sr25519(hex_key).show_card(),
                    }.show();
                    events.push(Event::TransactionSigned(sign_display));
                    enter_events_into_tree(&history, events)?;
                    flush_db(&database)?;
                    
                    Ok(format!("01{}", hex_signature))
                },
                Encryption::Ecdsa => {
                    let sign_display = SignDisplay {
                        transaction: &hex_signature,
                        author_line: Verifier::Ecdsa(hex_key).show_card(),
                    }.show();
                    events.push(Event::TransactionSigned(sign_display));
                    enter_events_into_tree(&history, events)?;
                    flush_db(&database)?;
                    
                    Ok(format!("02{}", hex_signature))
                },
            }
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                events.push(Event::Error(e.to_string()));
                enter_events_into_tree(&history, events)?;
                flush_db(&database)?;
            }
            return Err(e)
        },
    }
}

pub fn create_signature_png (seed_phrase: &str, pwd_entry: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    let hex_result = create_signature(seed_phrase, pwd_entry, database_name, checksum)?;
    Ok(hex::encode(png_qr_from_hex(&hex_result)?))
}
