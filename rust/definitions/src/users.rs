use sled::IVec;
use parity_scale_codec::Decode;
use parity_scale_codec_derive;
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;
use crate::error::{AddressKeySource, ErrorSigner, ErrorSource, SpecsKeySource};
use crate::helpers::{multisigner_to_public, multisigner_to_encryption};
use crate::keyring::{AddressKey, NetworkSpecsKey, print_multisigner_as_base58};

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

    pub fn print (&self, multisigner: &MultiSigner, optional_prefix: Option<u16>) -> Result<String, ErrorSigner> {
        let base58print = print_multisigner_as_base58(multisigner, optional_prefix);
        Ok(format!("\"public_key\":\"{}\",\"encryption\":\"{}\",\"ss58\":\"{}\",\"path\":\"{}\",\"has_password\":\"{}\",\"seed_name\":\"{}\"", hex::encode(multisigner_to_public(multisigner)), &self.encryption.show(), base58print, &self.path, &self.has_pwd, &self.seed_name))
    }
    
    pub fn process_entry_with_key_checked<T: ErrorSource>(address_key: &AddressKey, address_details_encoded: IVec) -> Result<(MultiSigner, Self), T::Error> {
        let multisigner = address_key.multi_signer::<T>(AddressKeySource::AddrTree)?;
        let address_details = match AddressDetails::decode(&mut &address_details_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(<T>::address_details_decoding(address_key.to_owned())),
        };
        if multisigner_to_encryption(&multisigner) != address_details.encryption {return Err(<T>::address_details_encryption_mismatch(address_key.to_owned(), address_details.encryption))}
        for network_specs_key in address_details.network_id.iter() {
            let (_, network_specs_key_encryption) = network_specs_key.genesis_hash_encryption::<T>(SpecsKeySource::AddrTree(address_key.to_owned()))?;
            if network_specs_key_encryption != address_details.encryption {return Err(<T>::address_details_specs_encryption_mismatch(address_key.to_owned(), network_specs_key.to_owned()))}
        }
        Ok((multisigner, address_details))
    }
    
    pub fn process_entry_checked<T: ErrorSource>((address_key_vec, address_details_encoded): (IVec, IVec)) -> Result<(MultiSigner, Self), T::Error> {
        let address_key = AddressKey::from_ivec(&address_key_vec);
        AddressDetails::process_entry_with_key_checked::<T>(&address_key, address_details_encoded)
    }
    
    pub fn from_entry_with_key_checked<T: ErrorSource>(address_key: &AddressKey, address_details_encoded: IVec) -> Result<Self, T::Error> {
        let (_, address_details) = AddressDetails::process_entry_with_key_checked::<T>(address_key, address_details_encoded)?;
        Ok(address_details)
    }
    pub fn is_root(&self) -> bool {
        (self.path.is_empty())&&(!self.has_pwd)
    }
    
}
