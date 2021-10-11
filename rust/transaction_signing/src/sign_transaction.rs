use anyhow;
use constants::{HISTORY, SIGNTRANS, TRANSACTION};
use definitions::{crypto::Encryption, history::Event, network_specs::Verifier, transactions::{Transaction, SignDisplay}};
use parity_scale_codec::Decode;
use db_handling::{helpers::{open_db, open_tree, flush_db, remove_from_tree}, manage_history::enter_events_into_tree};
use qrcode_static::png_qr_from_string;
use sp_runtime::MultiSigner;

use crate::sign_message::sign_as_address_key;
use crate::error::{Error, ActionFailure, CryptoError};
use crate::helpers::verify_checksum;

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub fn create_signature (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    
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
    let (author_line, encryption) = match <MultiSigner>::decode(&mut &action.address_key[..]) {
        Ok(MultiSigner::Ed25519(public)) => (Verifier::Ed25519(hex::encode(public)).show_card(), Encryption::Ed25519),
        Ok(MultiSigner::Sr25519(public)) => (Verifier::Sr25519(hex::encode(public)).show_card(), Encryption::Sr25519),
        Ok(MultiSigner::Ecdsa(public)) => (Verifier::Ecdsa(hex::encode(public)).show_card(), Encryption::Ecdsa),
        Err(_) => return Err(Error::AddressKeyDecoding.show()),
    };
    
// get full address with derivation path, used for signature preparation
// TODO zeroize
    let full_address = seed_phrase.to_owned() + &action.path;
    
    match sign_as_address_key(&action.transaction, action.address_key, &full_address, pwd) {
        Ok(s) => {
            let hex_signature = hex::encode(s);
            
            remove_from_tree(SIGNTRANS.to_vec(), &transaction)?;
            flush_db(&database)?;
            
            let sign_display = SignDisplay {
                transaction: &hex_signature,
                author_line,
                user_comment,
            }.show();
            events.push(Event::TransactionSigned(sign_display));
            enter_events_into_tree(&history, events)?;
            flush_db(&database)?;
            
            match encryption {
                Encryption::Ed25519 => Ok(format!("00{}", hex_signature)),
                Encryption::Sr25519 => Ok(format!("01{}", hex_signature)),
                Encryption::Ecdsa => Ok(format!("02{}", hex_signature)),
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

pub fn create_signature_png (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    let hex_result = create_signature(seed_phrase, pwd_entry, user_comment, database_name, checksum)?;
    Ok(hex::encode(png_qr_from_string(&hex_result)?))
}
