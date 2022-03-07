use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "signer")]
use sled::IVec;

#[cfg(feature = "signer")]
use crate::error_signer::{ErrorSigner, DatabaseSigner, EntryDecodingSigner};

/// Danger status in the Signer database
///
/// Indicates if the Signer has a record of unsafe exposure.
pub struct DangerRecord (Vec<u8>);

#[derive(Decode, Encode)]
struct DecodedDangerRecord {
    device_was_online: bool,
}

impl DangerRecord {
    /// Function to set danger record to `safe`
    pub fn safe () -> Self {
        Self (
            DecodedDangerRecord {
                device_was_online: false,
            }.encode()
        )
    }
    /// Function to set danger record to `not_safe`
    #[cfg(feature = "signer")]
    pub fn not_safe () -> Self {
        Self (
            DecodedDangerRecord {
                device_was_online: true,
            }.encode()
        )
    }
    /// Function to get danger record from the corresponding database key
    #[cfg(feature = "signer")]
    pub fn from_ivec (ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }
    /// Function to get `device_was_online` flag
    #[cfg(feature = "signer")]
    pub fn device_was_online (&self) -> Result<bool, ErrorSigner>  {
        match <DecodedDangerRecord>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.device_was_online),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::DangerStatus))),
        }
    }
    /// Function to prepare the danger record information into storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

