//! Record of the Signer exposure to dangerous events  
//!
//! Signer potentially dangerous exposures are recorded in `SETTREE` as
//! encoded [`DangerRecord`] under key `DANGER`.
//!
//! Signer should stay offline (i.e. air-gapped) throughout its usage.
//!
//! In case Signer finds itself online, it records this in the database
//! danger record and generates log entry in `HISTORY` tree.
//!
//! [`DangerRecord`] could be reset only by designated reset function, and the
//! fact of the reset is also recorded in the history log.  
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "signer")]
use sled::IVec;

#[cfg(feature = "signer")]
use crate::error_signer::{DatabaseSigner, EntryDecodingSigner, ErrorSigner};

/// Danger status in the Signer database
///
/// Indicates if the Signer has a record of unsafe exposure.
pub struct DangerRecord(Vec<u8>);

/// Decoded `DangerRecord` content, struct with boolean fields corresponding
/// to different exposure modes inside  
///
/// Currently contains only `device_was_online` flag
#[derive(Decode, Encode)]
struct DecodedDangerRecord {
    device_was_online: bool,
}

impl DangerRecord {
    /// Set danger record to "safe".  
    ///
    /// Switch all exposure flags to `false`.  
    pub fn safe() -> Self {
        Self(
            DecodedDangerRecord {
                device_was_online: false,
            }
            .encode(),
        )
    }

    /// Set `device_was_online` exposure flag to `true`.  
    ///
    /// Having `device_was_online` flag `true` makes danger record "not safe".  
    #[cfg(feature = "signer")]
    pub fn set_was_online() -> Self {
        Self(
            DecodedDangerRecord {
                device_was_online: true,
            }
            .encode(),
        )
    }

    /// Get `DangerRecord` with content from the encoded value,
    /// as it is stored in the database.  
    ///
    /// Unfallible, as the validity of the value is not checked.  
    #[cfg(feature = "signer")]
    pub fn from_ivec(ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }

    /// Get the value of `device_was_online` flag from `DangerRecord`.  
    ///
    /// Could result in error if the `DangerRecord` content is corrupted.  
    #[cfg(feature = "signer")]
    pub fn device_was_online(&self) -> Result<bool, ErrorSigner> {
        match <DecodedDangerRecord>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.device_was_online),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(
                EntryDecodingSigner::DangerStatus,
            ))),
        }
    }

    /// Transform `DangerRecord` into `Vec<u8>` to put in the database.  
    pub fn store(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
