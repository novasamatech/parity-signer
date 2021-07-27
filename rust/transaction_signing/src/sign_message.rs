use anyhow;
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use definitions::{constants::ADDRTREE, users::{AddressKey, Encryption, generate_address_key, SufficientCrypto}};
use parity_scale_codec::Encode;
use std::convert::TryInto;
use db_handling::{prep_messages::{prep_types, prep_load_metadata, prep_add_network_versioned, prep_add_network_latest}, error::NotHex, helpers::{open_db, open_tree, unhex, decode_address_details}};

use crate::error::{Error, CryptoError};

pub fn sign_as_address_key (to_sign: &Vec<u8>, address_key: AddressKey, encryption: &Encryption, full_address: &str, pwd: Option<&str>) -> anyhow::Result<Vec<u8>> {
    
    match encryption {
        Encryption::Ed25519 => {
            let ed25519_pair = match ed25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEd25519).show()),
            };
            let into_key: [u8; 32] = match address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyFormatEd25519).show()),
            };
            let key = ed25519::Public::from_raw(into_key);
            if key != ed25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = ed25519_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
        Encryption::Sr25519 => {
            let sr25519_pair = match sr25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenSr25519).show()),
            };
            let into_key: [u8; 32] = match address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyFormatSr25519).show()),
            };
            let key = sr25519::Public::from_raw(into_key);
            if key != sr25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = sr25519_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
        Encryption::Ecdsa => {
            let ecdsa_pair = match ecdsa::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEcdsa).show()),
            };
            let into_key: [u8; 33] = match address_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyFormatEcdsa).show()),
            };
            let key = ecdsa::Public::from_raw(into_key);
            if key != ecdsa_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = ecdsa_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
    }
}

/// Function to generate signature for some message for given public key
pub fn sign_message (public_key: &str, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<Vec<u8>> {
    
    let address_key = generate_address_key(&unhex(public_key, NotHex::PublicKey)?);
    
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    
    match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
            let address_details = decode_address_details(address_details_encoded)?;
            let pwd = {
                if address_details.has_pwd {Some(pwd_entry)}
                else {None}
            };
        // get full address with derivation path, used for signature preparation
        // TODO zeroize
            let full_address = seed_phrase.to_owned() + &address_details.path;
            sign_as_address_key(to_sign, address_key, &address_details.encryption, &full_address, pwd)
        },
        Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}


/// Function to generate `sufficient crypto line` for given public key
pub fn sufficient_crypto (public_key: &str, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let unhex_public_key = unhex(public_key, NotHex::PublicKey)?;
    let address_key = generate_address_key(&unhex_public_key);
    
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    
    match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
            let address_details = decode_address_details(address_details_encoded)?;
            let pwd = {
                if address_details.has_pwd {Some(pwd_entry)}
                else {None}
            };
        // get full address with derivation path, used for signature preparation
        // TODO zeroize
            let full_address = seed_phrase.to_owned() + &address_details.path;
            let signature = sign_as_address_key(to_sign, address_key, &address_details.encryption, &full_address, pwd)?;
        
            let sufficient_crypto = match address_details.encryption {
                Encryption::Ed25519 => SufficientCrypto::Ed25519 {public_key: unhex_public_key.try_into().expect("just checked the length"), signature: signature.try_into().expect("just generated, the length is correct")},
                Encryption::Sr25519 => SufficientCrypto::Sr25519 {public_key: unhex_public_key.try_into().expect("just checked the length"), signature: signature.try_into().expect("just generated, the length is correct")},
                Encryption::Ecdsa => SufficientCrypto::Ecdsa {public_key: unhex_public_key.try_into().expect("just checked the length"), signature: signature.try_into().expect("just generated, the length is correct")},
            };
            Ok(hex::encode(sufficient_crypto.encode()))    
        },
        Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
    
}

/// Function to generate `sufficient crypto line` for load_types message;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_types (public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_types(database_name)?;
    sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry)
    
}


/// Function to generate `sufficient crypto line` for load_metadata message;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_metadata (network_name: &str, network_version: u32, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_load_metadata(network_name, network_version, database_name)?;
    sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry)
    
}


/// Function to generate `sufficient crypto line` for add_network message using latest available version for this network;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_latest (network_name: &str, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_add_network_latest(network_name, database_name)?;
    sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry)
    
}


/// Function to generate `sufficient crypto line` for add_network message using given version;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_versioned (network_name: &str, network_version: u32, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_add_network_versioned(network_name, network_version, database_name)?;
    sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry)
    
}
