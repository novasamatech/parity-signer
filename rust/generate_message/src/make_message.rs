//! Complete generating update QR code or test string, signed or unsigned
use constants::{ALICE_SEED_PHRASE, EXPORT_FOLDER};
use definitions::{
    crypto::{Encryption, SufficientCrypto},
    metadata::MetaValues,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta, ContentLoadTypes},
};
use qrcode_rtx::make_pretty_qr;
use sp_core::{ecdsa, ed25519, sr25519, Pair};

use crate::error::{Error, Result};
use crate::parser::{Crypto, Goal, Make, Msg};

/// Alice seed phrase and derivation `//Alice`, for making updates signed with
/// test verifier.
fn alice_secret() -> String {
    [ALICE_SEED_PHRASE, "//Alice"].concat()
}

/// Generate and export update based on keys used in command line.
///
/// Checks that message content is the one expected and the signature is valid
/// for provided public key and [`Encryption`]. Generates prelude and assembles
/// complete update message, then exports it as a QR code or text file with
/// hex-encoded bytes.
pub fn make_message(make: Make) -> Result<()> {
    let vec = make.payload()?;
    // check message content for consistency
    //
    // note that bytes signed and bytes added into concatenated update are not
    // necessarily the same
    let (message_to_verify, message_to_transfer, name_stub, msg_type_code) = match make.msg {
        Msg::LoadTypes => {
            let content = ContentLoadTypes::from_slice(&vec);
            content.types()?;
            (
                content.to_sign(),
                content.to_transfer(),
                String::from("load_types"),
                "81",
            )
        }
        Msg::LoadMetadata => {
            let content = ContentLoadMeta::from_slice(&vec);
            let meta = content.meta()?;
            let meta_values = MetaValues::from_slice_metadata(&meta)?;
            (
                content.to_sign(),
                content.to_transfer(),
                format!("load_metadata_{}V{}", meta_values.name, meta_values.version),
                "80",
            )
        }
        Msg::AddSpecs => {
            let content = ContentAddSpecs::from_slice(&vec);
            let network_specs = content.specs()?;
            (
                content.to_sign(),
                content.to_transfer(),
                format!(
                    "add_specs_{}-{}",
                    network_specs.name,
                    network_specs.encryption.show()
                ),
                "c1",
            )
        }
    };

    // adding signature (if any) and finalize the message and filename
    let (complete_message, complete_name) = match make.crypto()? {
        // verifier is Alice, make signature here
        Crypto::Alice { e: encryption } => match encryption {
            Encryption::Ed25519 => {
                let crypto_type_code = "00";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                let ed25519_pair =
                    ed25519::Pair::from_string(&alice_secret(), None).expect("known Alice secret");
                let signature = ed25519_pair.sign(&message_to_verify[..]).0.to_vec();
                let complete_message = [
                    hex::decode(prelude).expect("known value"),
                    ed25519_pair.public().to_vec(),
                    message_to_transfer,
                    signature,
                ]
                .concat();
                (complete_message, format!("{}_Alice-ed25519", name_stub))
            }
            Encryption::Sr25519 => {
                let crypto_type_code = "01";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                let sr25519_pair =
                    sr25519::Pair::from_string(&alice_secret(), None).expect("known Alice secret");
                let signature = sr25519_pair.sign(&message_to_verify[..]).0.to_vec();
                let complete_message = [
                    hex::decode(prelude).expect("known value"),
                    sr25519_pair.public().to_vec(),
                    message_to_transfer,
                    signature,
                ]
                .concat();
                (complete_message, format!("{}_Alice-sr25519", name_stub))
            }
            Encryption::Ecdsa => {
                let crypto_type_code = "02";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                let ecdsa_pair =
                    ecdsa::Pair::from_string(&alice_secret(), None).expect("known Alice secret");
                let signature = ecdsa_pair.sign(&message_to_verify[..]).0.to_vec();
                let complete_message = [
                    hex::decode(prelude).expect("known value"),
                    ecdsa_pair.public().0.to_vec(),
                    message_to_transfer,
                    signature,
                ]
                .concat();
                (complete_message, format!("{}_Alice-ecdsa", name_stub))
            }
        },

        // no verifier
        Crypto::None => {
            let crypto_type_code = "ff";
            let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
            let complete_message = [
                hex::decode(prelude).expect("known value"),
                message_to_transfer,
            ]
            .concat();
            (complete_message, format!("{}_unverified", name_stub))
        }

        // real verifier with real signature: check that signature is valid
        // first
        Crypto::Sufficient { s } => match s {
            SufficientCrypto::Ed25519 { public, signature } => {
                let crypto_type_code = "00";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                if ed25519::Pair::verify(&signature, &message_to_verify, &public) {
                    let complete_message = [
                        hex::decode(prelude).expect("known value"),
                        public.to_vec(),
                        message_to_transfer,
                        signature.0.to_vec(),
                    ]
                    .concat();
                    (complete_message, name_stub)
                } else {
                    return Err(Error::BadSignature);
                }
            }
            SufficientCrypto::Sr25519 { public, signature } => {
                let crypto_type_code = "01";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                if sr25519::Pair::verify(&signature, &message_to_verify, &public) {
                    let complete_message = [
                        hex::decode(prelude).expect("known value"),
                        public.to_vec(),
                        message_to_transfer,
                        signature.0.to_vec(),
                    ]
                    .concat();
                    (complete_message, name_stub)
                } else {
                    return Err(Error::BadSignature);
                }
            }
            SufficientCrypto::Ecdsa { public, signature } => {
                let crypto_type_code = "02";
                let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
                if ecdsa::Pair::verify(&signature, &message_to_verify, &public) {
                    let complete_message = [
                        hex::decode(prelude).expect("known value"),
                        public.0.to_vec(),
                        message_to_transfer,
                        signature.0.to_vec(),
                    ]
                    .concat();
                    (complete_message, name_stub)
                } else {
                    return Err(Error::BadSignature);
                }
            }
        },
    };

    let output_name = match make.name {
        Some(a) => format!("{}", a.to_string_lossy()),
        None => format!("{}/{}", EXPORT_FOLDER, complete_name),
    };

    match make.goal {
        Goal::Qr => {
            make_pretty_qr(&complete_message, &output_name).map_err(Error::Qr)?;
        }
        Goal::Text => {
            std::fs::write(
                &format!("{}.txt", output_name),
                &hex::encode(&complete_message),
            )?;
        }
        Goal::Both => {
            std::fs::write(
                &format!("{}.txt", output_name),
                &hex::encode(&complete_message),
            )?;
            make_pretty_qr(&complete_message, &output_name).map_err(Error::Qr)?;
        }
    }

    Ok(())
}
