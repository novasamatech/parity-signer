use parity_scale_codec_derive;
use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_core::crypto::{Ss58Codec, Ss58AddressFormat};
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;
use crate::error::{AddressKeySource, DatabaseActive, ErrorActive, ErrorSigner, ErrorSource, KeyDecodingActive, NotHexSigner, Signer, SpecsKeySource};
use crate::helpers::{get_multisigner, unhex};

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
    /// Function to transform IVec (database key) into NetworkSpecsKey
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }
    /// Function to transform hex entered key into NetworkSpecsKey
    pub fn from_hex(hex_line: &str) -> Result<Self, ErrorSigner> {
        Ok(Self(unhex::<Signer>(hex_line, NotHexSigner::NetworkSpecsKey{input: hex_line.to_string()})?))
    }
    /// Function to get genesis hash and encryption from the NetworkSpecsKey
    pub fn genesis_hash_encryption<T: ErrorSource>(&self, source: SpecsKeySource<T>) -> Result<(Vec<u8>, Encryption), T::Error> {
        match <NetworkSpecsKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => match a {
                NetworkSpecsKeyContent::Ed25519(b) => Ok((b, Encryption::Ed25519)),
                NetworkSpecsKeyContent::Sr25519(b) => Ok((b, Encryption::Sr25519)),
                NetworkSpecsKeyContent::Ecdsa(b) => Ok((b, Encryption::Ecdsa)),
            },
            Err(_) => return Err(<T>::specs_key_to_error(self, source))
        }
    }
    /// Function to get the key that can be used for the database search from the NetworkSpecsKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}


/// VerifierKey is the database storage key used to search for 
/// network verifier information NetworkVerifier (HOT database, network verifiers tree VERIFIERS)
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, Debug, Clone, PartialEq)]
pub struct VerifierKey (Vec<u8>);

impl VerifierKey {
    /// Function to generate VerifierKey from network genesis hash (with possibility to add components later on)
    pub fn from_parts (genesis_hash: &Vec<u8>) -> Self {
        Self(genesis_hash.to_vec())
    }
    /// Function to transform Vec<u8> into VerifierKey prior to processing
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
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
    /// Function to generate AddressKey from corresponding MultiSigner value
    pub fn from_multisigner (multisigner: &MultiSigner) -> Self {
        Self(AddressKeyContent(multisigner.to_owned()).encode())
    }
    /// Function to generate AddressKey from parts: public key vector and network encryption
    pub fn from_parts (public: &Vec<u8>, encryption: &Encryption) -> Result<Self, ErrorSigner> {
        let multisigner = get_multisigner(public, encryption)?;
        Ok(Self::from_multisigner(&multisigner))
    }
    /// Function to transform IVec into AddressKey
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }
    /// Function to transform Vec<u8> into AddressKey
    pub fn from_hex (hex_address_key: &str) -> Result<Self, ErrorSigner> {
        Ok(Self(unhex::<Signer>(hex_address_key, NotHexSigner::AddressKey{input: hex_address_key.to_string()})?))
    }
    /// Function to get public key and encryption from the AddressKey
    pub fn public_key_encryption<T: ErrorSource>(&self, source: AddressKeySource<T>) -> Result<(Vec<u8>, Encryption), T::Error> {
        match &self.multi_signer(source)? {
            MultiSigner::Ed25519(b) => Ok((b.to_vec(), Encryption::Ed25519)),
            MultiSigner::Sr25519(b) => Ok((b.to_vec(), Encryption::Sr25519)),
            MultiSigner::Ecdsa(b) => Ok((b.0.to_vec(), Encryption::Ecdsa)),
        }
    }
    /// Function to get MultiSigner from the AddressKey
    pub fn multi_signer<T: ErrorSource>(&self, source: AddressKeySource<T>) -> Result<MultiSigner, T::Error> {
        match <AddressKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.0),
            Err(_) => return Err(<T>::address_key_to_error(&self, source)),
        }
    }
    /// Function to get the key that can be used for the database search from the NetworkSpecsKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

pub fn print_multisigner_as_base58 (multi_signer: &MultiSigner, optional_prefix: Option<u16>) -> String {
    match optional_prefix {
        Some(base58prefix) => {
            let version_for_base58 = Ss58AddressFormat::Custom(base58prefix);
            match multi_signer {
                MultiSigner::Ed25519(pubkey) => pubkey.to_ss58check_with_version(version_for_base58),
                MultiSigner::Sr25519(pubkey) => pubkey.to_ss58check_with_version(version_for_base58),
                MultiSigner::Ecdsa(pubkey) => pubkey.to_ss58check_with_version(version_for_base58),
            }
        },
        None => match multi_signer {
            MultiSigner::Ed25519(pubkey) => pubkey.to_ss58check(),
            MultiSigner::Sr25519(pubkey) => pubkey.to_ss58check(),
            MultiSigner::Ecdsa(pubkey) => pubkey.to_ss58check(),
        }
    }
}

/// MetaKey is the database storage key used to search for 
/// metadata entries (COLD and HOT databases, metadata tree METATREE)
#[derive(Debug, Clone)]
pub struct MetaKey (Vec<u8>);

/// Struct for decoded MetaKey content
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct MetaKeyContent {
    name: String,
    version: u32,
}

impl MetaKey {
    /// Function to generate MetaKey from parts: network genesis hash and network encryption
    pub fn from_parts (name: &str, version: u32) -> Self {
        let meta_key_content = MetaKeyContent {name: name.to_string(), version};
        Self(meta_key_content.encode())
    }
    /// Function to transform Vec<u8> into MetaKey prior to processing
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }
    /// Function to get genesis hash and encryption from the MetaKey
    pub fn name_version<T: ErrorSource>(&self) -> Result<(String, u32), T::Error> {
        match <MetaKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.name, a.version)),
            Err(_) => return Err(<T>::meta_key_to_error(self)),
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
#[derive(Debug)]
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
#[derive(Debug, Clone)]
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
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }
    /// Function to get the network title from the AddressBookKey
    pub fn title (&self) -> Result<String, ErrorActive> {
        match <AddressBookKeyContent>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.0),
            Err(_) => return Err(ErrorActive::Database(DatabaseActive::KeyDecoding(KeyDecodingActive::AddressBookKey(self.to_owned()))))
        }
    }
    /// Function to get the key that can be used for the database search from the AddressBookKey
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{Active, SpecsKeySource, ExtraSpecsKeySourceSigner};
    
    #[test]
    fn error_in_network_specs_key() {
        let network_specs_key_hex = "0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_hex(network_specs_key_hex).unwrap();
        let error = network_specs_key.genesis_hash_encryption::<Signer>(SpecsKeySource::Extra(ExtraSpecsKeySourceSigner::Interface)).unwrap_err();
        let error_print = <Signer>::show(&error);
        let expected_error_print = "Error on the interface. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e passed through the interface.";
        assert!( error_print == expected_error_print, "Received: \n{}", error_print);
        let error = network_specs_key.genesis_hash_encryption::<Signer>(SpecsKeySource::SpecsTree).unwrap_err();
        let error_print = <Signer>::show(&error);
        let expected_error_print = "Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from the database.";
        assert!( error_print == expected_error_print, "Received: \n{}", error_print);
        let error = network_specs_key.genesis_hash_encryption::<Active>(SpecsKeySource::SpecsTree).unwrap_err();
        let error_print = <Active>::show(&error);
        let expected_error_print = "Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from the database.";
        assert!( error_print == expected_error_print, "Received: \n{}", error_print);
    }
}

