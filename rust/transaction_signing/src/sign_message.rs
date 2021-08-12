use anyhow;
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use definitions::{constants::ADDRTREE, history::Event, metadata::{MetaValuesDisplay, VerifiedMetaValuesDisplay, NetworkDisplay}, network_specs::Verifier, types::TypesUpdate, users::{AddressKey, Encryption, generate_address_key, SufficientCrypto}};
use parity_scale_codec::Encode;
use std::convert::TryInto;
use db_handling::{prep_messages::{prep_types, prep_load_metadata, prep_add_network_versioned, prep_add_network_latest}, error::NotHex, helpers::{open_db, open_tree, unhex, decode_address_details}, manage_history::enter_events};
use blake2_rfc::blake2b::blake2b;

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
fn sufficient_crypto (public_key: &str, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<SufficientCrypto> {
    
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
            Ok(sufficient_crypto)
        },
        Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
    
}

/// Helper function to generate verifier_line from known SufficientCrypto
fn get_verifier_line(s: &SufficientCrypto) -> String {
    let verifier = match s {
        &SufficientCrypto::Ed25519 {public_key, signature: _} => Verifier::Ed25519(hex::encode(public_key)),
        &SufficientCrypto::Sr25519 {public_key, signature:_} => Verifier::Sr25519(hex::encode(public_key)),
        &SufficientCrypto::Ecdsa {public_key, signature: _} => Verifier::Ecdsa(hex::encode(public_key)),
    };
    verifier.show_card()
}

/// Function to generate `sufficient crypto line` for load_types message;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_types (public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_types(database_name)?; // encoded types info
    match sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let types_update_show = TypesUpdate {
                types_hash: hex::encode(blake2b(32, &[], &to_sign).as_bytes()),
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedTypes(types_update_show)])?;
            Ok(hex::encode(s.encode()))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate `sufficient crypto line` for load_metadata message;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_metadata (network_name: &str, network_version: u32, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_load_metadata(network_name, network_version, database_name)?; // metadata and genesis hash concatenated
    match sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let verified_meta_values_display = VerifiedMetaValuesDisplay {
                name: &network_name,
                version: network_version,
                meta_hash: &hex::encode(blake2b(32, &[], &to_sign[..to_sign.len()-32]).as_bytes()),
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedLoadMetadata(verified_meta_values_display)])?;
            Ok(hex::encode(s.encode()))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate `sufficient crypto line` for add_network message using latest available version for this network;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_latest (network_name: &str, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let prep_add_network = prep_add_network_latest(network_name, database_name)?;
    let to_sign = [prep_add_network.meta.encode(), prep_add_network.network_specs.encode()].concat();
    match sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let network_display = NetworkDisplay {
                meta_values: MetaValuesDisplay {
                    name: &network_name,
                    version: prep_add_network.version,
                    meta_hash: &hex::encode(blake2b(32, &[], &prep_add_network.meta).as_bytes()),
                },
                network_specs: &prep_add_network.network_specs,
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedAddNetwork(network_display)])?;
            Ok(hex::encode(s.encode()))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate `sufficient crypto line` for add_network message using given version;
/// `sufficient crypto line` is in hex format and consists of
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_versioned (network_name: &str, network_version: u32, public_key: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let prep_add_network = prep_add_network_versioned(network_name, network_version, database_name)?;
    let to_sign = [prep_add_network.meta.encode(), prep_add_network.network_specs.encode()].concat();
    
    match sufficient_crypto (public_key, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let network_display = NetworkDisplay {
                meta_values: MetaValuesDisplay {
                    name: &network_name,
                    version: network_version,
                    meta_hash: &hex::encode(blake2b(32, &[], &prep_add_network.meta).as_bytes()),
                },
                network_specs: &prep_add_network.network_specs,
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedAddNetwork(network_display)])?;
            Ok(hex::encode(s.encode()))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
    
}
