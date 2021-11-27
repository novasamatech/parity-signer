use parity_scale_codec_derive;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ed25519, sr25519, ecdsa, crypto::{Ss58Codec, Ss58AddressFormat}};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use crate::crypto::Encryption;
use crate::metadata::NameVersioned;

/// NetworkSpecsKey is the database storage key used to search for 
/// network specs ChainSpecs (COLD database, network specs tree SPECSTREE)
/// network specs ChainSpecsToSend (HOT database, network specs tree SPECSPREPTREE)
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug, Clone)]
pub struct NetworkSpecsKey (Vec<u8>);

/// Enum for decoded NetworkSpecsKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
enum NetworkSpecsKeyContent {
    Ed25519(Vec<u8>),
    Sr25519(Vec<u8>),
    Ecdsa(Vec<u8>),
}

impl NetworkSpecsKey {
    /// Function to generate NetworkSpecsKey from parts: network genesis hash and network encryption
    pub fn from_parts (genesis_hash: &Vec<u8>, encryption: &Encryption) -> Self {
        let network_key_content = match encryption {
            Encryption::Ed25519 => NetworkSpecsKeyContent::Ed25519(genesis_hash.to_vec()),
            Encryption::Sr25519 => NetworkSpecsKeyContent::Sr25519(genesis_hash.to_vec()),
            Encryption::Ecdsa => NetworkSpecsKeyContent::Ecdsa(genesis_hash.to_vec()),
        };
        Self(network_key_content.encode())
    }
    /// Function to transform Vec<u8> into NetworkSpecsKey prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get genesis hash and encryption from the NetworkSpecsKey
    pub fn genesis_hash_encryption (&self) -> Result<(Vec<u8>, Encryption), &'static str> {
        match <NetworkSpecsKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => match a {
                NetworkSpecsKeyContent::Ed25519(b) => Ok((b, Encryption::Ed25519)),
                NetworkSpecsKeyContent::Sr25519(b) => Ok((b, Encryption::Sr25519)),
                NetworkSpecsKeyContent::Ecdsa(b) => Ok((b, Encryption::Ecdsa)),
            },
            Err(_) => return Err("network specs key could not be parced")
        }
    }
    /// Function to get the key that can be used for the database search from the NetworkSpecsKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}


/// VerifierKey is the database storage key used to search for 
/// network verifier information NetworkVerifier (HOT database, network verifiers tree VERIFIERS)
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq)]
pub struct VerifierKey (Vec<u8>);

impl VerifierKey {
    /// Function to generate VerifierKey from network genesis hash (with possibility to add components later on)
    pub fn from_parts (genesis_hash: &Vec<u8>) -> Self {
        Self(genesis_hash.to_vec())
    }
    /// Function to transform Vec<u8> into VerifierKey prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get genesis hash from the VerifierKey
    pub fn genesis_hash (&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to get the key that can be used for the database search from the VerifierKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}


/// AddressKey is the database storage key used to search for
/// address details AddressDetails (HOT database, identities tree ADDRTREE)
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, Debug, PartialEq, Clone)]
pub struct AddressKey (Vec<u8>);

/// Struct for decoded AddressKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct AddressKeyContent (MultiSigner);

impl AddressKey {
    /// Function to generate NetworkSpecsKey from parts: network genesis hash and network encryption
    pub fn from_parts (public: &Vec<u8>, encryption: &Encryption) -> Result<Self, &'static str> {
        let address_key_content = match encryption {
            Encryption::Ed25519 => {
                let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                    Ok(a) => a,
                    Err(_) => return Err("Public key length does not match encryption."),
                };
                let pubkey = ed25519::Public::from_raw(into_pubkey);
                AddressKeyContent(MultiSigner::Ed25519(pubkey))
            },
            Encryption::Sr25519 => {
                let into_pubkey: [u8; 32] = match public.to_vec().try_into() {
                    Ok(a) => a,
                    Err(_) => return Err("Public key length does not match encryption."),
                };
                let pubkey = sr25519::Public::from_raw(into_pubkey);
                AddressKeyContent(MultiSigner::Sr25519(pubkey))
            },
            Encryption::Ecdsa => {
                let into_pubkey: [u8; 33] = match public.to_vec().try_into() {
                    Ok(a) => a,
                    Err(_) => return Err("Public key length does not match encryption."),
                };
                let pubkey = ecdsa::Public::from_raw(into_pubkey);
                AddressKeyContent(MultiSigner::Ecdsa(pubkey))
            },
        };
        Ok(Self(address_key_content.encode()))
    }
    /// Function to transform Vec<u8> into AddressKey prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get public key and encryption from the AddressKey
    pub fn public_key_encryption (&self) -> Result<(Vec<u8>, Encryption), &'static str> {
        match &self.multi_signer()? {
            MultiSigner::Ed25519(b) => Ok((b.to_vec(), Encryption::Ed25519)),
            MultiSigner::Sr25519(b) => Ok((b.to_vec(), Encryption::Sr25519)),
            MultiSigner::Ecdsa(b) => Ok((b.0.to_vec(), Encryption::Ecdsa)),
        }
    }
    /// Function to get MultiSigner from the AddressKey
    pub fn multi_signer (&self) -> Result<MultiSigner, &'static str> {
        match <AddressKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.0),
            Err(_) => return Err("address key could not be parced")
        }
    }
    /// Function to get the key that can be used for the database search from the NetworkSpecsKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
    /// Function to make base58 address for known public address with known encryption;
    /// if base58prefix is provided, generates custom Ss58AddressFormat,
    /// if not, uses default
    pub fn print_as_base58 (&self, encryption: &Encryption, optional_prefix: Option<u16>) -> Result<String, &'static str> {
        let multi_signer = &self.multi_signer()?;
        match multi_signer {
            MultiSigner::Ed25519(pubkey) => {
                if encryption != &Encryption::Ed25519 {return Err("Encryption algorithm mismatch")}
                match optional_prefix {
                    Some(base58prefix) => {
                        let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                        Ok(pubkey.to_ss58check_with_version(version_for_base58))
                    },
                    None => Ok(pubkey.to_ss58check()),
                }
            },
            MultiSigner::Sr25519(pubkey) => {
                if encryption != &Encryption::Sr25519 {return Err("Encryption algorithm mismatch")}
                match optional_prefix {
                    Some(base58prefix) => {
                        let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
                        Ok(pubkey.to_ss58check_with_version(version_for_base58))
                    },
                    None => Ok(pubkey.to_ss58check()),
                }
            },
            MultiSigner::Ecdsa(pubkey) => {
                if encryption != &Encryption::Ecdsa {return Err("Encryption algorithm mismatch")}
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
}


/// MetaKey is the database storage key used to search for 
/// metadata entries (COLD and HOT databases, metadata tree METATREE)
pub struct MetaKey (Vec<u8>);

/// Struct for decoded MetaKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct MetaKeyContent (NameVersioned);

impl MetaKey {
    /// Function to generate MetaKey from parts: network genesis hash and network encryption
    pub fn from_parts (name: &str, version: u32) -> Self {
        let meta_key_content = MetaKeyContent (NameVersioned {name: name.to_string(), version});
        Self(meta_key_content.encode())
    }
    /// Function to transform Vec<u8> into MetaKey prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get genesis hash and encryption from the MetaKey
    pub fn name_version (&self) -> Result<(String, u32), &'static str> {
        match <MetaKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.0.name, a.0.version)),
            Err(_) => return Err("metadata key could not be parced")
        }
    }
    /// Function to get the key that can be used for the database search from the MetaKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// MetaKeyPrefix is the prefix of database storage key used to search for 
/// metadata entries (COLD and HOT databases, metadata tree METATREE)
/// prefix is derived from network name alone, and is intended to find all metadata entries,
/// i.e. all versions available
pub struct MetaKeyPrefix (Vec<u8>);

/// Struct for decoded MetaKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct MetaKeyPrefixContent (String);

impl MetaKeyPrefix {
    /// Function to generate MetaKeyPrefix from parts: network genesis hash and network encryption
    pub fn from_name (name: &str) -> Self {
        let meta_key_prefix_content = MetaKeyPrefixContent (name.to_string());
        Self(meta_key_prefix_content.encode())
    }
    /// Function to get the key that can be used for the database search from the MetaKeyPrefix
    pub fn prefix(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// AddressBookKey is the database storage key used to search for 
/// address book entries (HOT database, address book tree ADDRESS_BOOK)
pub struct AddressBookKey (Vec<u8>);

/// Struct for decoded MetaKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct AddressBookKeyContent (String);

impl AddressBookKey {
    /// Function to generate AddressBookKey from parts: network genesis hash and network encryption
    pub fn from_title (title: &str) -> Self {
        let address_book_key_content = AddressBookKeyContent (title.to_string());
        Self(address_book_key_content.encode())
    }
    /// Function to transform Vec<u8> into AddressBookKey prior to processing
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get genesis hash and encryption from the AddressBookKey
    pub fn title (&self) -> Result<String, &'static str> {
        match <AddressBookKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.0),
            Err(_) => return Err("address book key could not be parced")
        }
    }
    /// Function to get the key that can be used for the database search from the AddressBookKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

