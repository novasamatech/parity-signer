use hex;
use sp_core::{ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use plot_icon::generate_png_scaled_default;

use crate::crypto::Encryption;
use crate::error::{ErrorSigner, ErrorSource, InterfaceSigner};

/// Function to decode hex encoded &str into Vec<u8>,
/// `what` is either of enums (NotHexHot or NotHexSigner) implementing NotHex trait
pub fn unhex<T: ErrorSource>(hex_entry: &str, what: T::NotHex) -> Result<Vec<u8>, T::Error> {
    let hex_entry = {
        if let Some(a) = hex_entry.strip_prefix("0x") {a}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => Err(<T>::hex_to_error(what))
    }
}

/// Function to get public key from MultiSigner
pub fn multisigner_to_public (m: &MultiSigner) -> Vec<u8> {
    match m {
        MultiSigner::Ed25519(a) => a.to_vec(),
        MultiSigner::Sr25519(a) => a.to_vec(),
        MultiSigner::Ecdsa(a) => a.0.to_vec(),
    }
}

/// Function to get encryption from MultiSigner
pub fn multisigner_to_encryption (m: &MultiSigner) -> Encryption {
    match m {
        MultiSigner::Ed25519(_) => Encryption::Ed25519,
        MultiSigner::Sr25519(_) => Encryption::Sr25519,
        MultiSigner::Ecdsa(_) => Encryption::Ecdsa,
    }
}


/// Helper function to print identicon from the multisigner
pub fn make_identicon_from_multisigner(multisigner: &MultiSigner) -> Vec<u8> {
    generate_png_scaled_default(&multisigner_to_public(multisigner))
}

/// Function to get MultiSigner from public key and Encryption
pub fn get_multisigner (public: &[u8], encryption: &Encryption) -> Result<MultiSigner, ErrorSigner> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Interface(InterfaceSigner::PublicKeyLength)),
            };
            Ok(MultiSigner::Ed25519(ed25519::Public::from_raw(into_pubkey)))
        },
        Encryption::Sr25519 => {
            let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Interface(InterfaceSigner::PublicKeyLength)),
            };
            Ok(MultiSigner::Sr25519(sr25519::Public::from_raw(into_pubkey)))
        },
        Encryption::Ecdsa => {
            let into_pubkey: [u8; 33] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorSigner::Interface(InterfaceSigner::PublicKeyLength)),
            };
            Ok(MultiSigner::Ecdsa(ecdsa::Public::from_raw(into_pubkey)))
        },
    }
}

/// Helper function to print id pic for metadata hash.
/// Currently uses identicon, could be changed later.
pub fn pic_meta(meta_hash: &[u8]) -> Vec<u8> {
    generate_png_scaled_default(meta_hash)
}

/// Helper function to print id pic for types data hash.
/// Currently uses identicon, could be changed later.
pub fn pic_types(types_hash: &[u8]) -> Vec<u8> {
    generate_png_scaled_default(types_hash)
}
