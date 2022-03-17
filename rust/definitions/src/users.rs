//! Address key associated non-secret information stored in Signer database
//!
//! Signer database has a tree `ADDRTREE` with [`AddressKey`] in key form
//! as a key and encoded [`AddressDetails`] as a value.
//! [`AddressDetails`] contains non-secret information associated with address key.  
//!
//! `ADDRTREE` is operated mainly from within the Signer.
//!
//! Release and test versions of the cold database are generated on the Active side.

use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_runtime::MultiSigner;

use crate::{
    crypto::Encryption,
    error::{AddressKeySource, ErrorSource, SpecsKeySource},
    helpers::multisigner_to_encryption,
    keyring::{AddressKey, NetworkSpecsKey},
};

/// Address key associated non-secret information stored in Signer database  
///
/// Info that should be available for any address key.  
/// No secrets are stored there.  
#[derive(Decode, Encode, Debug, Clone)]
pub struct AddressDetails {
    /// seed name (as it is known to the Signer device)  
    pub seed_name: String,
    /// derivation path, only with soft (`/`) and hard (`//`) junctions (i.e. no password)  
    pub path: String,
    /// whether the address key has an associated password  
    pub has_pwd: bool,
    /// set of networks, identified through [`NetworkSpecsKey`], that are available
    /// to work with this address key  
    pub network_id: Vec<NetworkSpecsKey>,
    /// encryption algorithm associated with the address key and all its associated networks  
    pub encryption: Encryption,
}

impl AddressDetails {
    /// Gets ([`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html),
    /// [`AddressDetails`]) tuple from [`AddressKey`] and associated value from database `ADDRTREE`.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn process_entry_with_key_checked<T: ErrorSource>(
        address_key: &AddressKey,
        address_details_encoded: IVec,
    ) -> Result<(MultiSigner, Self), T::Error> {
        let multisigner = address_key.multi_signer::<T>(AddressKeySource::AddrTree)?;
        let address_details = match AddressDetails::decode(&mut &address_details_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(<T>::address_details_decoding(address_key.to_owned())),
        };
        if multisigner_to_encryption(&multisigner) != address_details.encryption {
            return Err(<T>::address_details_encryption_mismatch(
                address_key.to_owned(),
                address_details.encryption,
            ));
        }
        for network_specs_key in address_details.network_id.iter() {
            let (_, network_specs_key_encryption) = network_specs_key
                .genesis_hash_encryption::<T>(SpecsKeySource::AddrTree(address_key.to_owned()))?;
            if network_specs_key_encryption != address_details.encryption {
                return Err(<T>::address_details_specs_encryption_mismatch(
                    address_key.to_owned(),
                    network_specs_key.to_owned(),
                ));
            }
        }
        Ok((multisigner, address_details))
    }

    /// Gets ([`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html),
    /// [`AddressDetails`]) tuple from database `ADDRTREE` (key, value) entry.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn process_entry_checked<T: ErrorSource>(
        (address_key_vec, address_details_encoded): (IVec, IVec),
    ) -> Result<(MultiSigner, Self), T::Error> {
        let address_key = AddressKey::from_ivec(&address_key_vec);
        AddressDetails::process_entry_with_key_checked::<T>(&address_key, address_details_encoded)
    }

    /// Gets [`AddressDetails`] from [`AddressKey`] and associated value from database `ADDRTREE`.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn from_entry_with_key_checked<T: ErrorSource>(
        address_key: &AddressKey,
        address_details_encoded: IVec,
    ) -> Result<Self, T::Error> {
        let (_, address_details) = AddressDetails::process_entry_with_key_checked::<T>(
            address_key,
            address_details_encoded,
        )?;
        Ok(address_details)
    }

    /// Checks if the [`AddressDetails`] have empty derivation path (i.e. 
    /// derivation path is empty and there is no password).  
    /// 
    /// Address key in this case is called root key or seed key.  
    pub fn is_root(&self) -> bool {
        (self.path.is_empty()) && (!self.has_pwd)
    }
}
