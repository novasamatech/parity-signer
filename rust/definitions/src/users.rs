//! Address key associated non-secret information stored in Vault database
//!
//! Vault database has a tree `ADDRTREE` with [`AddressKey`] in key form
//! as a key and encoded [`AddressDetails`] as a value.
//! [`AddressDetails`] contains non-secret information associated with address key.  
//!
//! `ADDRTREE` is operated mainly from within the Vault.
//!
//! Release and test versions of the cold database are generated on the Active side.

use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_runtime::MultiSigner;

use crate::{
    crypto::Encryption,
    error::{Error, Result},
    helpers::multisigner_to_encryption,
    keyring::{AddressKey, NetworkSpecsKey},
};

/// Address key associated non-secret information stored in Vault database
///
/// Info that should be available for any address key.  
/// No secrets are stored there.  
#[derive(Decode, PartialEq, Eq, Encode, Debug, Clone)]
pub struct AddressDetails {
    /// seed name (as it is known to the Vault device)
    pub seed_name: String,

    /// derivation path, only with soft (`/`) and hard (`//`) junctions (i.e. no password)  
    pub path: String,

    /// whether the address key has an associated password  
    pub has_pwd: bool,

    /// set of networks, identified through [`NetworkSpecsKey`], that are available
    /// to work with this address key  
    pub network_id: Option<NetworkSpecsKey>,

    /// encryption algorithm associated with the address key and all its associated networks  
    pub encryption: Encryption,

    /// address, or its parent address, had or could have secret exposed
    pub secret_exposed: bool,
}

impl AddressDetails {
    /// Gets ([`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html),
    /// [`AddressDetails`]) tuple from [`AddressKey`] and associated value from
    /// database tree `ADDRTREE`.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn process_entry_with_key_checked(
        address_key: &AddressKey,
        address_details_encoded: IVec,
    ) -> Result<(MultiSigner, Self)> {
        let multisigner = address_key.multi_signer().clone();
        let address_details = AddressDetails::decode(&mut &address_details_encoded[..])?;
        if multisigner_to_encryption(&multisigner) != address_details.encryption
            && multisigner_to_encryption(&multisigner) != Encryption::Ecdsa
            && address_details.encryption != Encryption::Ethereum
        {
            return Err(Error::EncryptionMismatch {
                address_key: address_key.to_owned(),
                encryption: address_details.encryption,
            });
        }
        let network_specs_key = &address_details.network_id;
        if let Some(network_specs_key) = network_specs_key {
            let (_, network_specs_key_encryption) = network_specs_key.genesis_hash_encryption()?;
            if network_specs_key_encryption != address_details.encryption {
                return Err(Error::EncryptionMismatch {
                    address_key: address_key.to_owned(),
                    encryption: address_details.encryption,
                });
            }
        }
        Ok((multisigner, address_details))
    }

    /// Gets ([`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html),
    /// [`AddressDetails`]) tuple from database tree `ADDRTREE` (key, value) entry.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn process_entry_checked(
        (address_key_vec, address_details_encoded): (IVec, IVec),
    ) -> Result<(MultiSigner, Self)> {
        let address_key = AddressKey::from_ivec(&address_key_vec)?;
        AddressDetails::process_entry_with_key_checked(&address_key, address_details_encoded)
    }

    /// Gets [`AddressDetails`] from [`AddressKey`] and associated value from
    /// database tree `ADDRTREE`.  
    ///
    /// Checks that there is no encryption mismatch.
    pub fn from_entry_with_key_checked(
        address_key: &AddressKey,
        address_details_encoded: IVec,
    ) -> Result<Self> {
        let (_, address_details) =
            AddressDetails::process_entry_with_key_checked(address_key, address_details_encoded)?;
        Ok(address_details)
    }

    /// Checks if the [`AddressDetails`] have empty derivation path (i.e.
    /// derivation path is empty and there is no password).  
    ///
    /// Address key in this case is called root key or seed key.  
    pub fn is_root(&self) -> bool {
        self.path.is_empty() && !self.has_pwd && self.network_id.is_none()
    }
}
