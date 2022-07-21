use blake2_rfc::blake2b::blake2b;
use parity_scale_codec::Encode;
use sp_runtime::MultiSignature;
use zeroize::Zeroize;

use db_handling::db_transactions::{SignContent, TrDbColdSign};
use definitions::error_signer::ErrorSigner;
use qrcode_static::png_qr_from_string;

use crate::sign_message::sign_as_address_key;

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub(crate) fn create_signature(
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: &str,
    checksum: u32,
) -> Result<MultiSignature, ErrorSigner> {
    let sign = TrDbColdSign::from_storage(database_name, checksum)?;
    let pwd = {
        if sign.has_pwd() {
            Some(pwd_entry)
        } else {
            None
        }
    };
    let content_vec = match sign.content() {
        SignContent::Transaction { method, extensions } => {
            // For larger transactions, their hash should be signed instead; this is not
            // implemented upstream so we put it here
            let whole_vec = [method.to_vec(), extensions.to_vec()].concat();
            if whole_vec.len() > 257 {
                blake2b(32, &[], &whole_vec).as_bytes().to_vec()
            } else {
                whole_vec
            }
        }
        SignContent::Message(a) => format!("<Bytes>{}</Bytes>", a).into_bytes(),
    };

    let mut full_address = seed_phrase.to_owned() + &sign.path();
    match sign_as_address_key(&content_vec, &sign.multisigner(), &full_address, pwd) {
        Ok(s) => {
            full_address.zeroize();
            sign.apply(false, user_comment, database_name)?;
            Ok(s.multi_signature())
        }
        Err(e) => {
            full_address.zeroize();
            if let ErrorSigner::WrongPassword = e {
                let checksum = sign.apply(true, user_comment, database_name)?;
                Err(ErrorSigner::WrongPasswordNewChecksum(checksum))
            } else {
                Err(e)
            }
        }
    }
}

pub fn create_signature_png(
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: &str,
    checksum: u32,
) -> Result<Vec<u8>, ErrorSigner> {
    let hex_result = hex::encode(
        create_signature(
            seed_phrase,
            pwd_entry,
            user_comment,
            database_name,
            checksum,
        )?
        .encode(),
    );
    let qr_data = match png_qr_from_string(&hex_result) {
        Ok(a) => a,
        Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
    };
    Ok(qr_data)
}
