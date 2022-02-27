use blake2_rfc::blake2b::blake2b;
use db_handling::db_transactions::{TrDbColdSign, SignContent};
use definitions::error::ErrorSigner;
use parity_scale_codec::Encode;
use qrcode_static::png_qr_from_string;
use sp_runtime::MultiSignature;
use zeroize::Zeroize;

use crate::sign_message::sign_as_address_key;

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub (crate) fn create_signature (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> Result<MultiSignature, ErrorSigner> {
    let sign = TrDbColdSign::from_storage(database_name, checksum)?;
    let pwd = {
        if sign.has_pwd() {Some(pwd_entry)}
        else {None}
    };
    let content_vec = match sign.content() {
        SignContent::Transaction{method, extensions} => [method.to_vec(), extensions.to_vec()].concat(),
        SignContent::Message(a) => a.encode(),
    };
    
// For larger transactions, their hash should be signed instead; this is not implemented
// upstream so we put it here
    let content_vec = {
        if content_vec.len() > 257 {blake2b(32, &[], &content_vec).as_bytes().to_vec()}
        else {content_vec}
    };
    let mut full_address = seed_phrase.to_owned() + &sign.path();
    match sign_as_address_key(&content_vec, &sign.multisigner(), &full_address, pwd) {
        Ok(s) => {
            full_address.zeroize();
            sign.apply(false, user_comment, database_name)?;
            Ok(s.get_multi_signature())
        },
        Err(e) => {
            full_address.zeroize();
            if let ErrorSigner::WrongPassword = e {
                let checksum = sign.apply(true, user_comment, database_name)?;
                Err(ErrorSigner::WrongPasswordNewChecksum(checksum))
            }
            else {Err(e)}
        },
    }
}

pub fn create_signature_png (seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str, checksum: u32) -> Result<String, ErrorSigner> {
    let hex_result = hex::encode(create_signature(seed_phrase, pwd_entry, user_comment, database_name, checksum)?.encode());
    let qr_data = match png_qr_from_string(&hex_result) {
        Ok(a) => a,
        Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
    };
    Ok(hex::encode(qr_data))
}
