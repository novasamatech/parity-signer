use parity_scale_codec_derive::{Decode, Encode};
use sp_core::{ed25519, sr25519, ecdsa, crypto::{Ss58Codec, Ss58AddressFormat}};
use std::convert::TryInto;

use super::network_specs::NetworkKey;


/// Type of encryption; only allow supported types here - compile-time check for that is happening
/// here.
//TODO: check if it is redundant
//Could not be replaced by sp_core::...::CRYPTO_ID as that doesn't do anything at compile time
#[derive(Clone, Copy, PartialEq, Debug, Decode, Encode)]
pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

/// Struct associated with public address that has secret key available
#[derive(Decode, Encode, Debug)]
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


/// Network identifier, used to search for network specs in the database
/// At this moment, vector made from public key
pub type AddressKey = Vec<u8>;

/// Generate address key from minimal amount of information
pub fn generate_address_key (public: &Vec<u8>) -> AddressKey {
    public.to_vec()
}


/// Function to make base58 address for known public address with known encryption;
/// if base58prefix is provided, generates custom Ss58AddressFormat,
/// if not, uses default

pub fn print_as_base58 (address_key: &Vec<u8>, encryption: Encryption, optional_prefix: Option<u16>) -> Result<String, &'static str> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8; 32] = match address_key.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Address key length does not match address details encryption."),
            };
            let pubkey = ed25519::Public::from_raw(into_pubkey);
            match optional_prefix {
                Some(base58prefix) => {
                    let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                    Ok(pubkey.to_ss58check_with_version(version_for_base58))
                },
                None => Ok(pubkey.to_ss58check()),
            }
        },
        Encryption::Sr25519 => {
            let into_pubkey: [u8; 32] = match address_key.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Address key length does not match address details encryption."),
            };
            let pubkey = sr25519::Public::from_raw(into_pubkey);
            match optional_prefix {
                Some(base58prefix) => {
                    let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                    Ok(pubkey.to_ss58check_with_version(version_for_base58))
                },
                None => Ok(pubkey.to_ss58check()),
            }
        },
        Encryption::Ecdsa => {
            let into_pubkey: [u8; 33] = match address_key.to_vec().try_into() {
                Ok(a) => a,
                Err(_) => return Err("Address key length does not match address details encryption."),
            };
            let pubkey = ecdsa::Public::from_raw(into_pubkey);
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
