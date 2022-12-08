use std::path::Path;

use definitions::crypto::Encryption;
use parity_scale_codec::Encode;
use sp_core::blake2_256;
use sp_runtime::MultiSignature;
use zeroize::Zeroize;

use db_handling::db_transactions::{SignContent, TrDbColdSign};

use crate::sign_message::sign_as_address_key;
use crate::{Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureType {
    Transaction,
    Message,
}

pub struct SignatureAndChecksum {
    signature_type: SignatureType,
    signature: MultiSignature,
    new_checksum: u32,
}

impl SignatureAndChecksum {
    pub fn new_checksum(&self) -> u32 {
        self.new_checksum
    }

    pub fn signature(&self) -> &MultiSignature {
        &self.signature
    }

    pub fn signature_type(&self) -> SignatureType {
        self.signature_type
    }
}

impl ToString for SignatureAndChecksum {
    fn to_string(&self) -> String {
        match self.signature_type {
            SignatureType::Transaction => hex::encode(self.signature.encode()),
            SignatureType::Message => match &self.signature {
                MultiSignature::Ed25519(a) => hex::encode(a),
                MultiSignature::Sr25519(a) => hex::encode(a),
                MultiSignature::Ecdsa(a) => hex::encode(a),
            },
        }
    }
}

/// Function to create signatures using RN output action line, and user entered pin and password.
/// Also needs database name to fetch saved transaction and key.
pub fn create_signature<P: AsRef<Path>>(
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: P,
    checksum: u32,
    idx: usize,
    encryption: Encryption,
) -> Result<SignatureAndChecksum> {
    let sign = TrDbColdSign::from_storage(&database_name, Some(checksum))?
        .ok_or(db_handling::Error::Sign)?;
    let pwd = {
        if sign.signing_bulk[idx].has_pwd() {
            Some(pwd_entry)
        } else {
            None
        }
    };
    let content = sign.signing_bulk[idx].content().to_owned();
    let content_vec = match &content {
        SignContent::Transaction { method, extensions } => {
            [method.to_vec(), extensions.to_vec()].concat()
        }
        SignContent::Message(a) => a.as_bytes().to_vec(),
    };

    // For larger transactions, their hash should be signed instead; this is not implemented
    // upstream so we put it here
    let content_vec = match &content {
        SignContent::Transaction {
            method: _,
            extensions: _,
        } if content_vec.len() > 257 => blake2_256(&content_vec).to_vec(),
        _ => content_vec,
    };
    let mut full_address = seed_phrase.to_owned() + &sign.signing_bulk[idx].path();
    let signature = match sign_as_address_key(
        &content_vec,
        &sign.signing_bulk[idx].multisigner(),
        &full_address,
        pwd,
        encryption,
    ) {
        Ok(s) => {
            full_address.zeroize();
            let c = sign.apply(false, user_comment, idx, database_name)?;
            Ok((s.multi_signature(), c))
        }
        Err(e) => {
            full_address.zeroize();
            if let Error::WrongPassword = e {
                let checksum = sign.apply(true, user_comment, idx, database_name)?;
                Err(Error::WrongPasswordNewChecksum(checksum))
            } else {
                Err(e)
            }
        }
    }?;

    let signature_type = match &content {
        SignContent::Transaction {
            method: _,
            extensions: _,
        } => SignatureType::Transaction,
        SignContent::Message(_) => SignatureType::Message,
    };
    Ok(SignatureAndChecksum {
        signature_type,
        signature: signature.0,
        new_checksum: signature.1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::ALICE_SEED_PHRASE;
    use db_handling::cold_default::populate_cold;
    use definitions::navigation::TransactionAction;
    use definitions::network_specs::Verifier;
    use sp_core::crypto::AccountId32;

    use sp_runtime::traits::Verify;
    use tempfile::tempdir;
    use transaction_parsing::produce_output;

    #[test]
    fn sign_long_msg() {
        let tmp_dir = tempdir().unwrap();
        populate_cold(tmp_dir.path(), Verifier { v: None }).unwrap();
        let message = format!("<Bytes>{}bbb</Bytes>", "a".repeat(256));
        let line = format!("530103d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d{}e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", hex::encode(&message));
        let output = produce_output(&line, tmp_dir.path());
        let public = sp_core::sr25519::Public::try_from(
            hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
                .unwrap()
                .as_ref(),
        )
        .unwrap();
        if let TransactionAction::Sign {
            actions: _,
            checksum,
        } = output
        {
            let signature = create_signature(
                ALICE_SEED_PHRASE,
                "",
                "",
                tmp_dir.path(),
                checksum,
                0,
                Encryption::Sr25519,
            )
            .unwrap();
            assert!(signature
                .signature
                .verify(message.as_bytes(), &AccountId32::new(public.0)));
        } else {
            panic!("Wrong action: {:?}", output)
        }
    }
}
