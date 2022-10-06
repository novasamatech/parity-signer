use definitions::crypto::Encryption;
use parity_scale_codec::Encode;
use sp_core::blake2_256;
use sp_runtime::MultiSignature;
use zeroize::Zeroize;

use db_handling::db_transactions::{SignContent, TrDbColdSign};
use qrcode_static::{png_qr_from_string, DataType};

use crate::sign_message::sign_as_address_key;
use crate::{Error, Result};

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.

pub(crate) fn create_signature(
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: &str,
    checksum: u32,
    encryption: Encryption,
) -> Result<String> {
    let sign = TrDbColdSign::from_storage(database_name, checksum)?;
    let pwd = {
        if sign.has_pwd() {
            Some(pwd_entry)
        } else {
            None
        }
    };
    let content = sign.content().to_owned();
    let content_vec = match &content {
        SignContent::Transaction { method, extensions } => {
            [method.to_vec(), extensions.to_vec()].concat()
        }
        SignContent::Message(a) => a.as_bytes().to_vec(),
    };

    // For larger transactions, their hash should be signed instead; this is not implemented
    // upstream so we put it here
    let content_vec = {
        if content_vec.len() > 257 {
            blake2_256(&content_vec).to_vec()
        } else {
            content_vec
        }
    };
    let mut full_address = seed_phrase.to_owned() + &sign.path();
    let signature = match sign_as_address_key(
        &content_vec,
        &sign.multisigner(),
        &full_address,
        pwd,
        encryption,
    ) {
        Ok(s) => {
            full_address.zeroize();
            sign.apply(false, user_comment, database_name)?;
            Ok(s.multi_signature())
        }
        Err(e) => {
            full_address.zeroize();
            if let Error::WrongPassword = e {
                let checksum = sign.apply(true, user_comment, database_name)?;
                Err(Error::WrongPasswordNewChecksum(checksum))
            } else {
                Err(e)
            }
        }
    }?;

    let encoded = match &content {
        SignContent::Transaction {
            method: _,
            extensions: _,
        } => hex::encode(signature.encode()),
        SignContent::Message(_) => match signature {
            MultiSignature::Sr25519(a) => hex::encode(a),
            MultiSignature::Ed25519(a) => hex::encode(a),
            MultiSignature::Ecdsa(a) => hex::encode(a),
        },
    };
    Ok(encoded)
}

pub fn create_signature_png(
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: &str,
    checksum: u32,
    encryption: Encryption,
) -> Result<Vec<u8>> {
    let hex_result = create_signature(
        seed_phrase,
        pwd_entry,
        user_comment,
        database_name,
        checksum,
        encryption,
    )?;
    let qr_data = png_qr_from_string(&hex_result, DataType::Regular)?;
    Ok(qr_data)
}
