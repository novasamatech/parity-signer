use anyhow;
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::{MultiSigner, MultiSignature};
use constants::ADDRTREE;
use definitions::{crypto::{Encryption, SufficientCrypto}, history::{Event, MetaValuesExport, NetworkSpecsExport, TypesExport}, keyring::AddressKey, metadata::MetaValues, network_specs::VerifierValue};
use parity_scale_codec::Encode;
use std::convert::TryInto;
use db_handling::{db_transactions::TrDbCold, error::NotHex, helpers::{open_db, open_tree, unhex, decode_address_details}, manage_history::{events_to_batch}, prep_messages::{prep_types, prep_load_metadata, prep_network_specs}};
use qrcode_static::png_qr;
use zeroize::Zeroize;

use crate::error::{Error, CryptoError};

pub fn sign_as_address_key (to_sign: &Vec<u8>, address_key: AddressKey, full_address: &str, pwd: Option<&str>) -> anyhow::Result<MultiSignature> {
    match address_key.multi_signer() {
        Ok(MultiSigner::Ed25519(public)) => {
            let ed25519_pair = match ed25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEd25519).show()),
            };
            if public != ed25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            // secret zeroize on drop, https://docs.rs/ed25519-dalek/1.0.1/src/ed25519_dalek/secret.rs.html#43
            let signature = ed25519_pair.sign(&to_sign[..]);
            Ok(MultiSignature::Ed25519(signature))
        },
        Ok(MultiSigner::Sr25519(public)) => {
            let sr25519_pair = match sr25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenSr25519).show()),
            };
            if public != sr25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            // pair zeroize on drop, https://docs.rs/schnorrkel/0.9.1/src/schnorrkel/keys.rs.html#680
            let signature = sr25519_pair.sign(&to_sign[..]);
            Ok(MultiSignature::Sr25519(signature))
        },
        Ok(MultiSigner::Ecdsa(public)) => {
            let ecdsa_pair = match ecdsa::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEcdsa).show()),
            };
            if public != ecdsa_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = ecdsa_pair.sign(&to_sign[..]);
            Ok(MultiSignature::Ecdsa(signature))
        },
        Err(_) => return Err(Error::AddressKeyDecoding.show()),
    }
}

/// Function to generate signature for some message for given public key
pub fn sign_message (public_key: &str, encryption: &Encryption, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<MultiSignature> {
    
    let address_key = match AddressKey::from_parts(&unhex(public_key, NotHex::PublicKey)?, encryption) {
        Ok(a) => a,
        Err(e) => return Err(Error::AddressKeyGeneration(e.to_string()).show()),
    };
    let address_details = {
        let database = open_db(database_name)?;
        let identities = open_tree(&database, ADDRTREE)?;
        match identities.get(&address_key.key()) {
            Ok(Some(address_details_encoded)) => decode_address_details(address_details_encoded)?,
            Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        }
    };
    let pwd = {
        if address_details.has_pwd {Some(pwd_entry)}
        else {None}
    };
    let mut full_address = seed_phrase.to_owned() + &address_details.path;
    match sign_as_address_key(to_sign, address_key, &full_address, pwd) {
        Ok(a) => {
            full_address.zeroize();
            Ok(a)
        },
        Err(e) => {
            full_address.zeroize();
            Err(e)
        },
    }
}


/// Function to generate `sufficient crypto line` for given public key
fn sufficient_crypto (public_key: &str, encryption: &Encryption, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<SufficientCrypto> {
    let unhex_public_key = unhex(public_key, NotHex::PublicKey)?;
    let address_key = match AddressKey::from_parts(&unhex_public_key, encryption) {
        Ok(a) => a,
        Err(e) => return Err(Error::AddressKeyGeneration(e.to_string()).show()),
    };
    let address_details = {
        let database = open_db(database_name)?;
        let identities = open_tree(&database, ADDRTREE)?;
        match identities.get(&address_key.key()) {
            Ok(Some(address_details_encoded)) => decode_address_details(address_details_encoded)?,
            Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        }
    };
    if encryption != &address_details.encryption {return Err(Error::EncryptionMismatch.show())}
    let pwd = {
        if address_details.has_pwd {Some(pwd_entry)}
        else {None}
    };
    let mut full_address = seed_phrase.to_owned() + &address_details.path;
    let signature = match sign_as_address_key(to_sign, address_key, &full_address, pwd) {
        Ok(a) => {
            full_address.zeroize();
            a
        },
        Err(e) => {
            full_address.zeroize();
            return Err(e)
        },
    };
    match signature {
        MultiSignature::Ed25519(signature) => Ok(SufficientCrypto::Ed25519 {public_key: sp_core::ed25519::Public::from_raw(unhex_public_key.try_into().expect("just checked the length")), signature}),
        MultiSignature::Sr25519(signature) => Ok(SufficientCrypto::Sr25519 {public_key: sp_core::sr25519::Public::from_raw(unhex_public_key.try_into().expect("just checked the length")), signature}),
        MultiSignature::Ecdsa(signature) => Ok(SufficientCrypto::Ecdsa {public_key: sp_core::ecdsa::Public::from_raw(unhex_public_key.try_into().expect("just checked the length")), signature}),
    }
}

/// Helper function to generate verifier_line from known SufficientCrypto
fn get_verifier_value(s: &SufficientCrypto) -> VerifierValue {
    match s {
        SufficientCrypto::Ed25519 {public_key, signature: _} => VerifierValue::Standard(MultiSigner::Ed25519(public_key.to_owned())),
        SufficientCrypto::Sr25519 {public_key, signature:_} => VerifierValue::Standard(MultiSigner::Sr25519(public_key.to_owned())),
        SufficientCrypto::Ecdsa {public_key, signature: _} => VerifierValue::Standard(MultiSigner::Ecdsa(public_key.to_owned())),
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_types message
pub fn sufficient_crypto_load_types (public_key: &str, encryption_str: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    let encryption = parse_encryption(encryption_str)?;
    let types_content = prep_types(database_name)?;
    match sufficient_crypto (public_key, &encryption, &types_content.store(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch(&database_name, vec![Event::TypesSigned(TypesExport::get(&types_content, &get_verifier_value(&s)))])?)
                .apply(&database_name)?;
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                TrDbCold::new()
                    .set_history(events_to_batch(&database_name, vec![Event::WrongPassword])?)
                    .apply(&database_name)?;
            }
            return Err(e)
        },
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_metadata message
pub fn sufficient_crypto_load_metadata (network_name: &str, network_version: u32, public_key: &str, encryption_str: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    let encryption = parse_encryption(encryption_str)?;
    let load_meta_content = prep_load_metadata(network_name, network_version, database_name)?;
    match sufficient_crypto (public_key, &encryption, &load_meta_content.store(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let meta = load_meta_content.meta().expect("just checked in prep_load_metadata function");
            TrDbCold::new()
                .set_history(events_to_batch(&database_name, vec![Event::MetadataSigned(MetaValuesExport::get(&MetaValues{name: network_name.to_string(), version: network_version, meta}, &get_verifier_value(&s)))])?)
                .apply(&database_name)?;
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                TrDbCold::new()
                    .set_history(events_to_batch(&database_name, vec![Event::WrongPassword])?)
                    .apply(&database_name)?;
            }
            return Err(e)
        },
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for add_specs message
pub fn sufficient_crypto_add_specs (network_specs_key_hex: &str, public_key: &str, encryption_str: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    let encryption = parse_encryption(encryption_str)?;
    let add_specs_content = prep_network_specs(network_specs_key_hex, database_name)?;
    match sufficient_crypto (public_key, &encryption, &add_specs_content.store(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let specs = add_specs_content.specs().expect("just checked in prep_network_specs function");
            TrDbCold::new()
                .set_history(events_to_batch(&database_name, vec![Event::NetworkSpecsSigned(NetworkSpecsExport::get(&specs, &get_verifier_value(&s)))])?)
                .apply(&database_name)?;
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                TrDbCold::new()
                    .set_history(events_to_batch(&database_name, vec![Event::WrongPassword])?)
                    .apply(&database_name)?;
            }
            return Err(e)
        },
    }
}

fn parse_encryption (encryption: &str) -> anyhow::Result<Encryption> {
    match encryption {
        "ed25519" => Ok(Encryption::Ed25519),
        "sr25519" => Ok(Encryption::Sr25519),
        "ecdsa" => Ok(Encryption::Ecdsa),
        _ => return Err(Error::CryptoNotSupported.show()),
    }
}
