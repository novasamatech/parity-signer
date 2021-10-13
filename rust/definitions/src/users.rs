use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use sp_core::{ed25519, sr25519, ecdsa, crypto::{Ss58Codec, Ss58AddressFormat}};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use crate::crypto::Encryption;
use crate::network_specs::NetworkKey;

/// Struct associated with public address that has secret key available
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, Debug)]
pub struct AddressDetails {
    pub seed_name: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_id: Vec<NetworkKey>,
    pub encryption: Encryption,
}

/// Struct to move seed around
/// TODO: zeroize somehow
#[derive(PartialEq, Debug)]
pub struct SeedObject {
    pub seed_name: String,
    pub seed_phrase: String,
    pub encryption: Encryption,
}


/// Struct to store history entry for identity action
pub struct IdentityHistory <'a> {
    pub seed_name: &'a str,
    pub encryption: Encryption,
    pub public_key: &'a str,
    pub path: &'a str,
    pub network_genesis_hash: &'a str,
}

impl <'a> IdentityHistory <'a> {
    pub fn show(&self) -> String {
        format!("\"seed_name\":\"{}\",\"encryption\":\"{}\",\"public_key\":\"{}\",\"path\":\"{}\",\"network_genesis_hash\":\"{}\"", &self.seed_name, &self.encryption.show(), &self.public_key, &self.path, &self.network_genesis_hash)
    }
}


/// Network identifier, used to search for network specs in the database
/// At this moment, vector made from public key
pub type AddressKey = Vec<u8>;

/// Generate address key from minimal amount of information
pub fn generate_address_key (public: &Vec<u8>, encryption: Encryption) -> Result<AddressKey, &'static str> {
    let out = match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Public key length does not match encryption."),
            };
            let pubkey = ed25519::Public::from_raw(into_pubkey);
            MultiSigner::Ed25519(pubkey)
        },
        Encryption::Sr25519 => {
            let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Public key length does not match encryption."),
            };
            let pubkey = sr25519::Public::from_raw(into_pubkey);
            MultiSigner::Sr25519(pubkey)
        },
        Encryption::Ecdsa => {
            let into_pubkey: [u8; 33] = match public.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Public key length does not match encryption."),
            };
            let pubkey = ecdsa::Public::from_raw(into_pubkey);
            MultiSigner::Ecdsa(pubkey)
        },
    }.encode();
    Ok(out)
}


/// Function to make base58 address for known public address with known encryption;
/// if base58prefix is provided, generates custom Ss58AddressFormat,
/// if not, uses default

pub fn print_as_base58 (address_key: &AddressKey, encryption: Encryption, optional_prefix: Option<u16>) -> Result<String, &'static str> {
    let address_key_decoded = match <MultiSigner>::decode(&mut &address_key[..]) {
        Ok(a) => a,
        Err(_) => return Err("Error decoding MultiSigner address key."),
    };
    match address_key_decoded {
        MultiSigner::Ed25519(pubkey) => {
            if encryption != Encryption::Ed25519 {return Err("Encryption algorithm mismatch")}
            match optional_prefix {
                Some(base58prefix) => {
                    let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                    Ok(pubkey.to_ss58check_with_version(version_for_base58))
                },
                None => Ok(pubkey.to_ss58check()),
            }
        },
        MultiSigner::Sr25519(pubkey) => {
            if encryption != Encryption::Sr25519 {return Err("Encryption algorithm mismatch")}
            match optional_prefix {
                Some(base58prefix) => {
                    let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                    Ok(pubkey.to_ss58check_with_version(version_for_base58))
                },
                None => Ok(pubkey.to_ss58check()),
            }
        },
        MultiSigner::Ecdsa(pubkey) => {
            if encryption != Encryption::Ecdsa {return Err("Encryption algorithm mismatch")}
            match optional_prefix {
                Some(base58prefix) => {
                    let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                    Ok(pubkey.to_ss58check_with_version(version_for_base58))
                },
                None => Ok(pubkey.to_ss58check()),
            }
        },
    }
}
