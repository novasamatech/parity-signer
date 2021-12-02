use sled::IVec;
use parity_scale_codec::Decode;
use parity_scale_codec_derive;
use zeroize::Zeroize;

use crate::crypto::Encryption;
use crate::error::{AddressKeySource, DatabaseSigner, ErrorSigner, ErrorSource, MismatchSigner, SpecsKeySource};
use crate::keyring::{AddressKey, NetworkSpecsKey};

/// Struct associated with public address that has secret key available
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, Debug, Clone)]
pub struct AddressDetails {
    pub seed_name: String,
    pub path: String,
    pub has_pwd: bool,
    pub network_id: Vec<NetworkSpecsKey>,
    pub encryption: Encryption,
}

impl AddressDetails {

    pub fn print (&self, address_key: &AddressKey, optional_prefix: Option<u16>) -> Result<String, ErrorSigner> {
        let (public_key, encryption, base58print) = address_key.public_encryption_base58(optional_prefix, AddressKeySource::AddrTree)?;
        if encryption != self.encryption {return Err(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::AddressDetailsEncryption{key: address_key.to_owned(), encryption: self.encryption.to_owned()})))}
        Ok(format!("\"public_key\":\"{}\",\"encryption\":\"{}\",\"ss58\":\"{}\",\"path\":\"{}\",\"has_password\":\"{}\",\"seed_name\":\"{}\"", hex::encode(public_key), &self.encryption.show(), base58print, &self.path, &self.has_pwd, &self.seed_name))
    }
    
    pub fn process_entry_with_key_checked<T: ErrorSource>(address_key: &AddressKey, address_details_encoded: IVec) -> Result<(Encryption, Vec<u8>, Self), T::Error> {
        let (public_key, encryption) = address_key.public_key_encryption::<T>(AddressKeySource::AddrTree)?;
        let address_details = match AddressDetails::decode(&mut &address_details_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(<T>::address_details_decoding(address_key.to_owned())),
        };
        if encryption != address_details.encryption {return Err(<T>::address_details_encryption_mismatch(address_key.to_owned(), address_details.encryption))}
        for network_specs_key in address_details.network_id.iter() {
            let (_, network_specs_key_encryption) = network_specs_key.genesis_hash_encryption::<T>(SpecsKeySource::AddrTree(address_key.to_owned()))?;
            if network_specs_key_encryption != address_details.encryption {return Err(<T>::address_details_specs_encryption_mismatch(address_key.to_owned(), network_specs_key.to_owned()))}
        }
        Ok((encryption, public_key, address_details))
    }
    
    pub fn process_entry_checked<T: ErrorSource>((address_key_vec, address_details_encoded): (IVec, IVec)) -> Result<(Encryption, Vec<u8>, Self), T::Error> {
        let address_key = AddressKey::from_ivec(&address_key_vec);
        AddressDetails::process_entry_with_key_checked::<T>(&address_key, address_details_encoded)
    }
    
    pub fn from_entry_with_key_checked<T: ErrorSource>(address_key: &AddressKey, address_details_encoded: IVec) -> Result<Self, T::Error> {
        let (_, _, address_details) = AddressDetails::process_entry_with_key_checked::<T>(&address_key, address_details_encoded)?;
        Ok(address_details)
    }
    
}

/// Struct to move seed around
#[derive(PartialEq, Debug, Zeroize)]
#[zeroize(drop)]
pub struct SeedObject {
    pub seed_name: String,
    pub seed_phrase: String,
    pub encryption: Encryption,
}

