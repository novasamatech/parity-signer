use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use std::convert::TryInto;
use definitions::{error::{ErrorSigner, InputSigner, NotHexSigner, Signer, TransferContent}, helpers::unhex, network_specs::{Verifier, VerifierValue}};
use parser::decoding_commons::get_compact;

pub struct InfoPassedCrypto {
    pub verifier: Verifier,
    pub message: Vec<u8>,
    pub tail: Vec<u8>,
}

pub fn pass_crypto(data_hex: &str, content: TransferContent) -> Result<InfoPassedCrypto, ErrorSigner> {
    
    let data = unhex::<Signer>(data_hex, NotHexSigner::InputContent)?;
    
    match &data_hex[2..4] {
        "00" => {
        // Ed25519 crypto was used by the verifier
            let (pubkey, data) = match data.get(3..35) {
                Some(a) => {
                    let into_pubkey: [u8;32] = a.try_into().expect("fixed size should fit in array");
                    (ed25519::Public::from_raw(into_pubkey), data[35..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            let (message, tail) = cut_data(&data, content)?;
            let (signature, tail) = match tail.get(..64) {
                Some(a) => {
                    let into_signature: [u8;64] = a.try_into().expect("fixed size should fit in array");
                    (ed25519::Signature::from_raw(into_signature), tail[64..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            if ed25519::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Ed25519(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                    tail,
                })
            }
            else {Err(ErrorSigner::Input(InputSigner::BadSignature))}
        },
        "01" => {
        // Sr25519 crypto was used by the verifier
            let (pubkey, data) = match data.get(3..35) {
                Some(a) => {
                    let into_pubkey: [u8;32] = a.try_into().expect("fixed size should fit in array");
                    (sr25519::Public::from_raw(into_pubkey), data[35..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            let (message, tail) = cut_data(&data, content)?;
            let (signature, tail) = match tail.get(..64) {
                Some(a) => {
                    let into_signature: [u8;64] = a.try_into().expect("fixed size should fit in array");
                    (sr25519::Signature::from_raw(into_signature), tail[64..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            if sr25519::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                    tail,
                })
            }
            else {Err(ErrorSigner::Input(InputSigner::BadSignature))}
        },
        "02" => {
        // Ecdsa crypto was used by the verifier
            let (pubkey, data) = match data.get(3..36) {
                Some(a) => {
                    let into_pubkey: [u8;33] = a.try_into().expect("fixed size should fit in array");
                    (ecdsa::Public::from_raw(into_pubkey), data[36..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            let (message, tail) = cut_data(&data, content)?;
            let (signature, tail) = match tail.get(..65) {
                Some(a) => {
                    let into_signature: [u8;65] = a.try_into().expect("fixed size should fit in array");
                    (ecdsa::Signature::from_raw(into_signature), tail[65..].to_vec())
                },
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            if ecdsa::Pair::verify(&signature, &message, &pubkey) {
                let verifier = Verifier(Some(VerifierValue::Standard(MultiSigner::Ecdsa(pubkey))));
                Ok(InfoPassedCrypto {
                    verifier,
                    message,
                    tail,
                })
            }
            else {Err(ErrorSigner::Input(InputSigner::BadSignature))}
        },
        "ff" => {
        // Received info was not signed
            let data = match data.get(3..) {
                Some(a) => a.to_vec(),
                None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
            };
            let (message, tail) = cut_data(&data, content)?;
            let verifier = Verifier(None);
            Ok(InfoPassedCrypto {
                verifier,
                message,
                tail,
            })
        },
        _ => Err(ErrorSigner::Input(InputSigner::EncryptionNotSupported(data_hex[2..4].to_string()))),
    }
}

fn cut_data (data: &[u8], content: TransferContent) -> Result<(Vec<u8>, Vec<u8>), ErrorSigner> {
    let pre_data = match get_compact::<u32>(data) {
        Ok(a) => a,
        Err(_) => return Err(ErrorSigner::Input(InputSigner::TransferContent(content))),
    };
    match content {
        TransferContent::AddSpecs | TransferContent::LoadTypes => {
        // AddSpecs and LoadTypes payloads consist of SCALE encoded Vec<u8> of ContentAddSpecs or ContentLoadTypes correspondingly. Encoding of contents is done to have exact length of data easily accessible (to cut data correctly in case multisignatures are implemented). Signature verifies ContentAddSpecs or ContentLoadTypes correspondingly, WITHOUT the length piece from encoding
            let data_length = pre_data.compact_found as usize;
            match pre_data.start_next_unit {
                Some(start) => {
                    match data.get(start..start+data_length) {
                        Some(a) => Ok((a.to_vec(), data[start+data_length..].to_vec())),
                        None => Err(ErrorSigner::Input(InputSigner::TooShort)),
                    }
                },
                None => Err(ErrorSigner::Input(InputSigner::TooShort)),
            }
        },
        TransferContent::LoadMeta => {
        // LoadMeta payload consists of SCALE encoded Vec<u8> of metadata and [u8;32] genesis hash; compact announces length of metadata vector, 32 is added to include the genesis hash
            let data_length = pre_data.compact_found as usize + 32;
            match pre_data.start_next_unit {
                Some(start) => {
                    match data.get(..start+data_length) {
                        Some(a) => Ok((a.to_vec(), data[start+data_length..].to_vec())),
                        None => Err(ErrorSigner::Input(InputSigner::TooShort)),
                    }
                },
                None => Err(ErrorSigner::Input(InputSigner::TooShort)),
            }
        },
    }
}
