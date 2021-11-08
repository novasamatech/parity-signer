use constants::EXPORT_FOLDER;
use definitions::{crypto::Encryption, metadata::VersionDecoded, qr_transfers::{ContentLoadTypes, ContentLoadMeta, ContentAddNetwork, ContentAddSpecs}};
use meta_reading::decode_metadata::get_meta_const;
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use std::convert::TryInto;
use qrcode_rtx::transform_into_qr_apng;
use parity_scale_codec::Decode;
use anyhow;

use crate::parser::{Make, Goal, Crypto, VerifierKind, Msg};
use crate::error::Error;

const ALICE_WORDS: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

/// Function to generate signed message.
/// Exact behavior is determined by the keys used.

pub fn make_message (make: Make) -> anyhow::Result<()> {

// check message content for consistency
    let (message, name_stub, msg_type_code) = match make.msg {
        Msg::LoadTypes(vec) => {
            match ContentLoadTypes::from_vec(&vec).types() {
                Ok(_) => (vec, String::from("load_types"), "81"),
                Err(_) => {return Err(Error::NotLoadTypes.show())},
            }
        },
        Msg::LoadMetadata(vec) => {
            match ContentLoadMeta::from_vec(&vec).meta() {
                Ok(meta) => {
                    match get_meta_const(&meta) {
                        Ok(version_vector) => {
                            match VersionDecoded::decode(&mut &version_vector[..]) {
                                Ok(version) => (vec, format!("load_metadata_{}V{}", version.specname, version.spec_version), "80"),
                                Err(_) => {return Err(Error::DamagedMetadata.show())},
                            }
                        },
                        Err(_) => {return Err(Error::DamagedMetadata.show())},
                    }
                },
                Err(_) => {return Err(Error::NotLoadMetadata.show())},
            }
        },
        Msg::AddNetwork(vec) => {
            match ContentAddNetwork::from_vec(&vec).meta_specs() {
                Ok((meta, network_specs)) => {
                    match get_meta_const(&meta) {
                        Ok(version_vector) => {
                            match VersionDecoded::decode(&mut &version_vector[..]) {
                                Ok(version) => {
                                    if version.specname != network_specs.name {return Err(Error::MessageNameMismatch{name_meta: version.specname, name_specs: network_specs.name}.show())}
                                    (vec, format!("add_network_{}V{}", version.specname, version.spec_version), "c0")
                                },
                                Err(_) => {return Err(Error::DamagedMetadata.show())},
                            }
                        },
                        Err(_) => {return Err(Error::DamagedMetadata.show())},
                    }
                },
                Err(_) => {return Err(Error::NotAddNetwork.show())},
            }
        },
        Msg::AddSpecs(vec) => {
            match ContentAddSpecs::from_vec(&vec).specs() {
                Ok(network_specs) => (vec, format!("add_specs_{}", network_specs.name), "c1"),
                Err(_) => {return Err(Error::NotAddSpecs.show())},
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
                        Err(_) => return Err(Error::AliceKey(Encryption::Ed25519).show()),
                    };
                    let signature = ed25519_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ed25519_pair.public().to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthPublicKey.show())},
                    };
                    let into_sign: [u8;64] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthSignature.show())},
                    };
                    let pubkey = ed25519::Public::from_raw(into_pubkey);
                    let sign = ed25519::Signature::from_raw(into_sign);
                    if ed25519::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Error::BadSignature(Encryption::Ed25519).show())}
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
                        Err(_) => return Err(Error::AliceKey(Encryption::Sr25519).show()),
                    };
                    let signature = sr25519_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), sr25519_pair.public().to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthPublicKey.show())},
                    };
                    let into_sign: [u8;64] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthSignature.show())},
                    };
                    let pubkey = sr25519::Public::from_raw(into_pubkey);
                    let sign = sr25519::Signature::from_raw(into_sign);
                    if sr25519::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Error::BadSignature(Encryption::Sr25519).show())}
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
                        Err(_) => return Err(Error::AliceKey(Encryption::Ecdsa).show()),
                    };
                    let signature = ecdsa_pair.sign(&message[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ecdsa_pair.public().0.to_vec(), message, signature].concat();
                    (complete_message, format!("{}_Alice", name_stub))
                },
                VerifierKind::Normal {verifier_public_key, signature} => {
                    let into_pubkey: [u8;33] = match verifier_public_key.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthPublicKey.show())},
                    };
                    let into_sign: [u8;65] = match signature.try_into() {
                        Ok(a) => a,
                        Err(_) => {return Err(Error::WrongLengthSignature.show())},
                    };
                    let pubkey = ecdsa::Public::from_raw(into_pubkey);
                    let sign = ecdsa::Signature::from_raw(into_sign);
                    if ecdsa::Pair::verify(&sign, &message, &pubkey) {
                        let complete_message = [hex::decode(prelude).expect("known value"), pubkey.0.to_vec(), message, sign.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(Error::BadSignature(Encryption::Ecdsa).show())}
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
            if let Err(e) = transform_into_qr_apng(&complete_message, &output_name) {return Err(Error::Qr(e.to_string()).show())}
        },
        Goal::Text => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(Error::InputOutputError(e.to_string()).show())}
        },
        Goal::Both => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(Error::InputOutputError(e.to_string()).show())}
            if let Err(e) = transform_into_qr_apng(&complete_message, &output_name) {return Err(Error::Qr(e.to_string()).show())}
        },
    }
    
    Ok(())
    
}

