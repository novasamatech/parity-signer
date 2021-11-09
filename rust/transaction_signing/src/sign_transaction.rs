use anyhow;
use db_handling::db_transactions::{TrDbColdSign, SignContent};
use parity_scale_codec::Encode;
use qrcode_static::png_qr_from_string;
use zeroize::Zeroize;

use crate::sign_message::sign_as_address_key;
use crate::error::{Error, CryptoError};

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub (crate) fn create_signature (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    let sign = TrDbColdSign::from_storage(&database_name, checksum)?;
    let pwd = {
        if sign.has_pwd() {Some(pwd_entry)}
        else {None}
    };
    let content_vec = match sign.content() {
        SignContent::Transaction{method, extensions} => [method.to_vec(), extensions.to_vec()].concat(),
        SignContent::Message(a) => a.encode(),
    };
    let mut full_address = seed_phrase.to_owned() + &sign.path();
    match sign_as_address_key(&content_vec, sign.address_key(), &full_address, pwd) {
        Ok(s) => {
            full_address.zeroize();
            sign.apply(false, user_comment, &database_name)?;
            Ok(hex::encode(s.encode()))
        },
        Err(e) => {
            full_address.zeroize();
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                sign.apply(true, user_comment, &database_name)?;
            }
            return Err(e)
        },
    }
}

pub fn create_signature_png (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> anyhow::Result<String> {
    let hex_result = create_signature(seed_phrase, pwd_entry, user_comment, database_name, checksum)?;
    Ok(hex::encode(png_qr_from_string(&hex_result)?))
}
