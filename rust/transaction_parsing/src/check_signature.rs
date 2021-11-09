use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use std::convert::TryInto;
use definitions::network_specs::{Verifier, VerifierValue};

use crate::error::{Error, BadInputData, CryptoError};
use crate::helpers::unhex;

pub struct InfoPassedCrypto {
    pub verifier: Verifier,
    pub message: Vec<u8>,
}

pub fn pass_crypto(data_hex: &str) -> Result<InfoPassedCrypto, Error> {
    
    let data = unhex(&data_hex)?;
    
    match &data_hex[2..4] {
        "00" => {
        // Ed25519 crypto was used by the verifier
        // minimal possible data length is 3 + 32 + 64 (prelude, public key in ed25519, signature in ed25519)
            if data.len() < 99 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = ed25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = ed25519::Signature::from_raw(into_signature);
            if ed25519::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Ed25519(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                })
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "01" => {
        // Sr25519 crypto was used by the verifier
        // minimal possible data length is 3 + 32 + 64 (prelude, public key in sr25519, signature in sr25519)
            if data.len() < 99 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = sr25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = sr25519::Signature::from_raw(into_signature);
            if sr25519::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                })
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "02" => {
        // Ecdsa crypto was used by the verifier
        // minimal possible data length is 3 + 33 + 65 (prelude, public key in ecdsa, network genesis hash, signature in ecdsa)
            if data.len() < 101 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;33] = data[3..36].try_into().expect("fixed size should fit in array");
            let pubkey = ecdsa::Public::from_raw(into_pubkey);
            let message = data[36..data.len()-65].to_vec();
            let into_signature: [u8;65] = data[data.len()-65..].try_into().expect("fixed size should fit in array");
            let signature = ecdsa::Signature::from_raw(into_signature);
            if ecdsa::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Ecdsa(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                })
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "ff" => {
        // Received info was not signed
        // minimal possible data length is 3 (prelude, network genesis hash)
            if data.len() < 3 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let message = data[3..].to_vec();
            let verifier = Verifier(None);
            Ok(InfoPassedCrypto {
                verifier,
                message,
            })
        },
        _ => return Err(Error::BadInputData(BadInputData::CryptoNotSupported))
    }
}
