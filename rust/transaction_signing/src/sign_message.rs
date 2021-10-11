use anyhow;
use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use constants::ADDRTREE;
use definitions::{crypto::{Encryption, SufficientCrypto}, history::Event, metadata::{MetaValuesDisplay, VerifiedMetaValuesDisplay, NetworkDisplay}, network_specs::Verifier, types::TypesUpdate, users::{AddressKey, generate_address_key}};
use parity_scale_codec::{Decode, Encode};
use std::convert::TryInto;
use db_handling::{prep_messages::{prep_types, prep_load_metadata, prep_add_network_versioned, prep_add_network_latest}, error::NotHex, helpers::{open_db, open_tree, unhex, decode_address_details}, manage_history::enter_events};
use blake2_rfc::blake2b::blake2b;
use qrcode_static::png_qr;

use crate::error::{Error, CryptoError};

pub fn sign_as_address_key (to_sign: &Vec<u8>, address_key: AddressKey, full_address: &str, pwd: Option<&str>) -> anyhow::Result<Vec<u8>> {
    
    match <MultiSigner>::decode(&mut &address_key[..]) {
        Ok(MultiSigner::Ed25519(public)) => {
            let ed25519_pair = match ed25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEd25519).show()),
            };
            if public != ed25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = ed25519_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
        Ok(MultiSigner::Sr25519(public)) => {
            let sr25519_pair = match sr25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenSr25519).show()),
            };
            if public != sr25519_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = sr25519_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
        Ok(MultiSigner::Ecdsa(public)) => {
            let ecdsa_pair = match ecdsa::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(_) => return Err(Error::CryptoError(CryptoError::KeyGenEcdsa).show()),
            };
            if public != ecdsa_pair.public() {return Err(Error::CryptoError(CryptoError::WrongPassword).show())}
            let signature = ecdsa_pair.sign(&to_sign[..]);
            Ok(signature.0.to_vec())
        },
        Err(_) => return Err(Error::AddressKeyDecoding.show()),
    }
}

/// Function to generate signature for some message for given public key
pub fn sign_message (public_key: &str, encryption: Encryption, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<Vec<u8>> {
    
    let address_key = match generate_address_key(&unhex(public_key, NotHex::PublicKey)?, encryption) {
        Ok(a) => a,
        Err(e) => return Err(Error::AddressKeyGeneration(e.to_string()).show()),
    };
    
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
            sign_as_address_key(to_sign, address_key, &full_address, pwd)
        },
        Ok(None) => return Err(Error::AddressDetailsNotFound.show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}


/// Function to generate `sufficient crypto line` for given public key
fn sufficient_crypto (public_key: &str, encryption: Encryption, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<SufficientCrypto> {
    
    let unhex_public_key = unhex(public_key, NotHex::PublicKey)?;
    let address_key = match generate_address_key(&unhex_public_key, encryption) {
        Ok(a) => a,
        Err(e) => return Err(Error::AddressKeyGeneration(e.to_string()).show()),
    };
    
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    
    match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
            let address_details = decode_address_details(address_details_encoded)?;
            if encryption != address_details.encryption {return Err(Error::EncryptionMismatch.show())}
            let pwd = {
                if address_details.has_pwd {Some(pwd_entry)}
                else {None}
            };
        // get full address with derivation path, used for signature preparation
        // TODO zeroize
            let full_address = seed_phrase.to_owned() + &address_details.path;
            let signature = sign_as_address_key(to_sign, address_key, &full_address, pwd)?;
        
            let sufficient_crypto = match encryption {
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

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_types message;
/// `sufficient_crypto` consists of:
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_types (public_key: &str, encryption: Encryption, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_types(database_name)?.store(); // encoded types info
    match sufficient_crypto (public_key, encryption, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let types_update_show = TypesUpdate {
                types_hash: hex::encode(blake2b(32, &[], &to_sign).as_bytes()),
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedTypes(types_update_show)])?;
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_metadata message;
/// `sufficient_crypto` consists of:
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_load_metadata (network_name: &str, network_version: u32, public_key: &str, encryption: Encryption, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let to_sign = prep_load_metadata(network_name, network_version, database_name)?; // metadata and genesis hash concatenated
    match sufficient_crypto (public_key, encryption, &to_sign, database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            let verified_meta_values_display = VerifiedMetaValuesDisplay {
                name: &network_name,
                version: network_version,
                meta_hash: &hex::encode(blake2b(32, &[], &to_sign[..to_sign.len()-32]).as_bytes()),
                verifier_line: get_verifier_line(&s),
            }.show();
            enter_events(database_name, vec![Event::SignedLoadMetadata(verified_meta_values_display)])?;
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for add_network message
/// using latest available version for this network;
/// `sufficient_crypto` consists of:
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_latest (network_name: &str, public_key: &str, encryption: Encryption, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let prep_add_network = prep_add_network_latest(network_name, database_name)?;
    let to_sign = [prep_add_network.meta.encode(), prep_add_network.network_specs.encode()].concat();
    match sufficient_crypto (public_key, encryption, &to_sign, database_name, seed_phrase, pwd_entry) {
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
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
}


/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for add_network message using given version;
/// `sufficient_crypto` consists of:
/// ** two first symbols denoting encryption algorithm used 00 for ed25519, 01 for sr25519, 02 for ecdsa
/// <public_key_in_hex> - length depends on encryption algorithm
/// <signature_in_hex> - length depends on encryption algorithm

pub fn sufficient_crypto_add_network_versioned (network_name: &str, network_version: u32, public_key: &str, encryption: Encryption, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    
    let prep_add_network = prep_add_network_versioned(network_name, network_version, database_name)?;
    let to_sign = [prep_add_network.meta.encode(), prep_add_network.network_specs.encode()].concat();
    
    match sufficient_crypto (public_key, encryption, &to_sign, database_name, seed_phrase, pwd_entry) {
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
            let qr_data = png_qr(&s.encode())?;
            Ok(hex::encode(qr_data))
        },
        Err(e) => {
            if e.to_string() == Error::CryptoError(CryptoError::WrongPassword).show().to_string() {
                enter_events(database_name, vec![Event::Error(e.to_string())])?;
            }
            return Err(e)
        },
    }
    
}
