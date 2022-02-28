use constants::EXPORT_FOLDER;
use definitions::{crypto::{Encryption, SufficientCrypto}, error::{Active, ErrorActive, InputActive}, metadata::MetaValues, qr_transfers::{ContentLoadTypes, ContentLoadMeta, ContentAddSpecs}};
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use qrcode_rtx::make_pretty_qr;

use crate::parser::{Make, Goal, Crypto, Msg};

const ALICE_WORDS: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

/// Function to generate signed message.
/// Exact behavior is determined by the keys used.

pub fn make_message (make: Make) -> Result<(), ErrorActive> {

// check message content for consistency
    let (message_to_verify, message_to_transfer, name_stub, msg_type_code) = match make.msg {
        Msg::LoadTypes(vec) => {
            let content = ContentLoadTypes::from_slice(&vec);
            content.types::<Active>()?;
            (content.to_sign(), content.to_transfer(), String::from("load_types"), "81")
        },
        Msg::LoadMetadata(vec) => {
            let content = ContentLoadMeta::from_slice(&vec);
            let meta = content.meta::<Active>()?;
            match MetaValues::from_slice_metadata(&meta) {
                Ok(meta_values) => (content.to_sign(), content.to_transfer(), format!("load_metadata_{}V{}", meta_values.name, meta_values.version), "80"),
                Err(e) => return Err(ErrorActive::Input(InputActive::FaultyMetadataInPayload(e))),
            }
        },
        Msg::AddSpecs(vec) => {
            let content = ContentAddSpecs::from_slice(&vec);
            let network_specs = content.specs::<Active>()?;
            (content.to_sign(), content.to_transfer(), format!("add_specs_{}-{}", network_specs.name, network_specs.encryption.show()), "c1")
        },
    };
    
// processing crypto information
    
    let (complete_message, complete_name) = match make.crypto {
        Crypto::Alice(encryption) => {
            match encryption {
                Encryption::Ed25519 => {
                    let crypto_type_code = "00";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    let ed25519_pair = ed25519::Pair::from_string(ALICE_WORDS, None).expect("known Alice secret");
                    let signature = ed25519_pair.sign(&message_to_verify[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ed25519_pair.public().to_vec(), message_to_transfer, signature].concat();
                    (complete_message, format!("{}_Alice-ed25519", name_stub))
                },
                Encryption::Sr25519 => {
                    let crypto_type_code = "01";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    let sr25519_pair = sr25519::Pair::from_string(ALICE_WORDS, None).expect("known Alice secret");
                    let signature = sr25519_pair.sign(&message_to_verify[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), sr25519_pair.public().to_vec(), message_to_transfer, signature].concat();
                    (complete_message, format!("{}_Alice-sr25519", name_stub))
                },
                Encryption::Ecdsa => {
                    let crypto_type_code = "02";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    let ecdsa_pair = ecdsa::Pair::from_string(ALICE_WORDS, None).expect("known Alice secret");
                    let signature = ecdsa_pair.sign(&message_to_verify[..]).0.to_vec();
                    let complete_message = [hex::decode(prelude).expect("known value"), ecdsa_pair.public().0.to_vec(), message_to_transfer, signature].concat();
                    (complete_message, format!("{}_Alice-ecdsa", name_stub))
                },
            }
        },
        Crypto::None => {
            let crypto_type_code = "ff";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            let complete_message = [hex::decode(prelude).expect("known value"), message_to_transfer].concat();
            (complete_message, format!("{}_unverified", name_stub))
        },
        Crypto::Sufficient(sufficient_crypto) => {
            match sufficient_crypto {
                SufficientCrypto::Ed25519{public, signature} => {
                    let crypto_type_code = "00";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    if ed25519::Pair::verify(&signature, &message_to_verify, &public) {
                        let complete_message = [hex::decode(prelude).expect("known value"), public.to_vec(), message_to_transfer, signature.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(ErrorActive::Input(InputActive::BadSignature))}
                },
                SufficientCrypto::Sr25519{public, signature} => {
                    let crypto_type_code = "01";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    if sr25519::Pair::verify(&signature, &message_to_verify, &public) {
                        let complete_message = [hex::decode(prelude).expect("known value"), public.to_vec(), message_to_transfer, signature.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(ErrorActive::Input(InputActive::BadSignature))}
                },
                SufficientCrypto::Ecdsa{public, signature} => {
                    let crypto_type_code = "02";
                    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                    if ecdsa::Pair::verify(&signature, &message_to_verify, &public) {
                        let complete_message = [hex::decode(prelude).expect("known value"), public.0.to_vec(), message_to_transfer, signature.0.to_vec()].concat();
                        (complete_message, name_stub)
                    }
                    else {return Err(ErrorActive::Input(InputActive::BadSignature))}
                },
            }
        },
    };
    
    let output_name = match make.name {
        Some(a) => format!("{}/{}", EXPORT_FOLDER, a),
        None => format!("{}/{}", EXPORT_FOLDER, complete_name),
    };
    
    match make.goal {
        Goal::Qr => {
            if let Err(e) = make_pretty_qr(&complete_message, &output_name) {return Err(ErrorActive::Qr(e.to_string()))}
        },
        Goal::Text => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(ErrorActive::Output(e))}
        },
        Goal::Both => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(ErrorActive::Output(e))}
            if let Err(e) = make_pretty_qr(&complete_message, &output_name) {return Err(ErrorActive::Qr(e.to_string()))}
        },
    }
    
    Ok(())
    
}

