use definitions::{constants::EXPORT_FOLDER, metadata::VersionDecoded, network_specs::ChainSpecsToSend, types::TypeEntry};
use meta_reading::{decode_metadata::get_meta_const};
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use std::convert::TryInto;
use qrcode_rtx::transform_into_qr_apng;
use parity_scale_codec::Decode;

use super::parser::{Make, Goal, Crypto, VerifierKind, Msg};

const ALICE_WORDS: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

/// Function to generate signed message.
/// Exact behavior is determined by the keys used.

pub fn make_message (make: Make) -> Result<(), Box<dyn std::error::Error>> {

// check message content for consistency
    let (message, name_stub, msg_type_code) = match make.msg {
        Msg::LoadTypes(vec) => {
            match <Vec<TypeEntry>>::decode(&mut &vec[..]) {
                Ok(_) => {
                    let msg_type_code = "81";
                    (vec, String::from("load_types"), msg_type_code)
                },
                Err(_) => {return Err(Box::from("Provided message has no load_types content."))},
            }
        },
        Msg::LoadMetadata(vec) => {
            if vec.len() < 32 {return Err(Box::from("Provided message too short to be load_metadata content."))}
            let meta = vec[..vec.len()-32].to_vec();
            match get_meta_const(&meta) {
                Ok(version_vector) => {
                    match VersionDecoded::decode(&mut &version_vector[..]) {
                        Ok(version) => {
                            let msg_type_code = "80";
                            (vec, format!("load_metadata_{}V{}", version.specname, version.spec_version), msg_type_code)
                        },
                        Err(_) => {return Err(Box::from("Metadata in provided load_metadata message is damaged. Version could not be decoded."))},
                    }
                },
                Err(e) => {return Err(Box::from(format!("Metadata in provided load_metadata message is damaged. {}", e)))}
            }
        },
        Msg::AddNetwork(vec) => {
            match <(Vec<u8>, ChainSpecsToSend)>::decode(&mut &vec[..]) {
                Ok((meta, network_specs)) => {
                    match get_meta_const(&meta) {
                        Ok(version_vector) => {
                            match VersionDecoded::decode(&mut &version_vector[..]) {
                                Ok(version) => {
                                    if version.specname != network_specs.name {return Err(Box::from("Provided add_network message is damaged. Name is metadata and chainspecs mismatch."))}
                                    let msg_type_code = "c0";
                                    (vec, format!("add_network_{}V{}", version.specname, version.spec_version), msg_type_code)
                                },
                                Err(_) => {return Err(Box::from("Metadata in provided add_network message is damaged. Version could not be decoded."))},
                            }
                        },
                        Err(e) => {return Err(Box::from(format!("Metadata in provided add_network message is damaged. {}", e)))}
                    }
                },
                Err(_) => {return Err(Box::from("Provided message has no add_network content."))},
            }
        },
    };
    
// processing crypto information
    
    let (complete_message, complete_name) = match make.crypto {
        Crypto::Ed25519(v) => {
            let crypto_type_code = "00";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            match v {
                VerifierKind::Alice => {
                    let ed25519_pair = match ed25519::Pair::from_string(ALICE_WORDS, None) {
                        Ok(x) => x,
                        Err(_) => return Err(Box::from("Error generating Alice keys for ed25519 crypto."))
                    };
                    let signature = ed25519_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ed25519_pair.public().to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided verifier public key has wrong length."))},
                    };
                    let into_sign: [u8;64] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided signature has wrong length."))},
                    };
                    let pubkey = ed25519::Public::from_raw(into_pubkey);
                    let sign = ed25519::Signature::from_raw(into_sign);
                    if ed25519::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Box::from("Bad ed25519 signature."))}
                },
            }
        },
        Crypto::Sr25519(v) => {
            let crypto_type_code = "01";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            match v {
                VerifierKind::Alice => {
                    let sr25519_pair = match sr25519::Pair::from_string(ALICE_WORDS, None) {
                        Ok(x) => x,
                        Err(_) => return Err(Box::from("Error generating Alice keys for sr25519 crypto."))
                    };
                    let signature = sr25519_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), sr25519_pair.public().to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided verifier public key has wrong length."))},
                    };
                    let into_sign: [u8;64] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided signature has wrong length."))},
                    };
                    let pubkey = sr25519::Public::from_raw(into_pubkey);
                    let sign = sr25519::Signature::from_raw(into_sign);
                    if sr25519::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Box::from("Bad sr25519 signature."))}
                },
            }
        },
        Crypto::Ecdsa(v) => {
            let crypto_type_code = "02";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            match v {
                VerifierKind::Alice => {
                    let ecdsa_pair = match ecdsa::Pair::from_string(ALICE_WORDS, None) {
                        Ok(x) => x,
                        Err(_) => return Err(Box::from("Error generating Alice keys for ecdsa crypto."))
                    };
                    let signature = ecdsa_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ecdsa_pair.public().0.to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;33] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided verifier public key has wrong length."))},
                    };
                    let into_sign: [u8;65] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Box::from("Provided signature has wrong length."))},
                    };
                    let pubkey = ecdsa::Public::from_raw(into_pubkey);
                    let sign = ecdsa::Signature::from_raw(into_sign);
                    if ecdsa::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.0.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Box::from("Bad ecdsa signature."))}
                },
            }
        },
        Crypto::None => {
            let crypto_type_code = "ff";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            let complete_message = [hex::decode(prelude).expect("known value"), message].concat();
            (complete_message, format!("{}_unverified", name_stub))
        },
    };
    
    let output_name = match make.name {
        Some(a) => format!("{}/{}", EXPORT_FOLDER, a),
        None => format!("{}/{}", EXPORT_FOLDER, complete_name),
    };
    
    match make.goal {
        Goal::Qr => {
            transform_into_qr_apng(&complete_message, &output_name)?;
        },
        Goal::Text => {
            std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message))?;
        },
        Goal::Both => {
            std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message))?;
            transform_into_qr_apng(&complete_message, &output_name)?;
        },
    }
    
    Ok(())
    
}


// fn process_verifier_kind (message: Vec<u8>, v: VerifierKind) -> Result<Vec<u8>>
