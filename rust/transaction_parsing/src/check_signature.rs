use crate::error::{Error, Result};
use definitions::{
    error::TransferContent,
    helpers::unhex,
    network_specs::{Verifier, VerifierValue},
};
use parser::decoding_commons::get_compact;
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

pub struct InfoPassedCrypto {
    pub verifier: Verifier,
    pub message: Vec<u8>,
    pub tail: Vec<u8>,
}

pub fn pass_crypto(data_hex: &str, content: TransferContent) -> Result<InfoPassedCrypto> {
    let data = unhex(data_hex)?;

    match &data_hex[2..4] {
        "00" => {
            // `Ed25519` crypto was used by the verifier
            let a = data.get(3..35).ok_or(Error::TooShort)?;
            let into_pubkey: [u8; 32] = a.try_into().expect("fixed size should fit in array");
            let (pubkey, data) = (ed25519::Public::from_raw(into_pubkey), &data[35..]);
            let (message, tail) = cut_data(data, content)?;
            let a = tail.get(..64).ok_or(Error::TooShort)?;
            let into_signature: [u8; 64] = a.try_into().expect("fixed size should fit in array");
            let (signature, tail) = (
                ed25519::Signature::from_raw(into_signature),
                tail[64..].to_vec(),
            );
            ed25519::Pair::verify(&signature, &message, &pubkey)
                .then(|| ())
                .ok_or(Error::BadSignature)?;
            let verifier = Verifier {
                v: Some(VerifierValue::Standard {
                    m: MultiSigner::Ed25519(pubkey),
                }),
            };
            Ok(InfoPassedCrypto {
                verifier,
                message,
                tail,
            })
        }
        "01" => {
            // `Sr25519` crypto was used by the verifier
            let a = data.get(3..35).ok_or(Error::TooShort)?;
            let into_pubkey: [u8; 32] = a.try_into().expect("fixed size should fit in array");
            let (pubkey, data) = (sr25519::Public::from_raw(into_pubkey), &data[35..]);
            let (message, tail) = cut_data(data, content)?;
            let a = tail.get(..64).ok_or(Error::TooShort)?;
            let into_signature: [u8; 64] = a.try_into().expect("fixed size should fit in array");
            let (signature, tail) = (
                sr25519::Signature::from_raw(into_signature),
                tail[64..].to_vec(),
            );
            sr25519::Pair::verify(&signature, &message, &pubkey)
                .then(|| ())
                .ok_or(Error::BadSignature)?;
            let verifier = Verifier {
                v: Some(VerifierValue::Standard {
                    m: MultiSigner::Sr25519(pubkey),
                }),
            };
            Ok(InfoPassedCrypto {
                verifier,
                message,
                tail,
            })
        }
        "02" => {
            // Ecdsa crypto was used by the verifier
            let a = data.get(3..36).ok_or(Error::TooShort)?;
            let into_pubkey: [u8; 33] = a.try_into().expect("fixed size should fit in array");
            let (pubkey, data) = (ecdsa::Public::from_raw(into_pubkey), &data[36..]);
            let (message, tail) = cut_data(data, content)?;
            let a = tail.get(..65).ok_or(Error::TooShort)?;
            let into_signature: [u8; 65] = a.try_into().expect("fixed size should fit in array");
            let (signature, tail) = (
                ecdsa::Signature::from_raw(into_signature),
                tail[65..].to_vec(),
            );
            ecdsa::Pair::verify(&signature, &message, &pubkey)
                .then(|| ())
                .ok_or(Error::BadSignature)?;
            let verifier = Verifier {
                v: Some(VerifierValue::Standard {
                    m: MultiSigner::Ecdsa(pubkey),
                }),
            };
            Ok(InfoPassedCrypto {
                verifier,
                message,
                tail,
            })
        }
        "ff" => {
            // Received info was not signed
            let data = data.get(3..).ok_or(Error::TooShort)?;
            let (message, tail) = cut_data(data, content)?;
            let verifier = Verifier { v: None };
            Ok(InfoPassedCrypto {
                verifier,
                message,
                tail,
            })
        }
        _ => Err(Error::EncryptionNotSupported(data_hex[2..4].to_string())),
    }
}

fn cut_data(data: &[u8], content: TransferContent) -> Result<(Vec<u8>, Vec<u8>)> {
    let pre_data = get_compact::<u32>(data)?;
    match content {
        TransferContent::AddSpecs | TransferContent::LoadTypes => {
            // `AddSpecs` and `LoadTypes` payloads consist of SCALE encoded `Vec<u8>` of `ContentAddSpecs` or `ContentLoadTypes` correspondingly. Encoding of contents is done to have exact length of data easily accessible (to cut data correctly in case multisignatures are implemented). Signature verifies `ContentAddSpecs` or `ContentLoadTypes` correspondingly, WITHOUT the length piece from encoding
            let data_length = pre_data.compact_found as usize;
            let start = pre_data.start_next_unit.ok_or(Error::TooShort)?;
            let a = data
                .get(start..start + data_length)
                .ok_or(Error::TooShort)?;
            Ok((a.to_vec(), data[start + data_length..].to_vec()))
        }
        TransferContent::LoadMeta => {
            // LoadMeta payload consists of SCALE encoded `Vec<u8>` of metadata and `[u8;32]` genesis hash; compact announces length of metadata vector, 32 is added to include the genesis hash
            let data_length = pre_data.compact_found as usize + 32;
            let start = pre_data.start_next_unit.ok_or(Error::TooShort)?;
            let a = data.get(..start + data_length).ok_or(Error::TooShort)?;
            Ok((a.to_vec(), data[start + data_length..].to_vec()))
        }
    }
}
