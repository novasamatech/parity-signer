//! Record of the Vault exposure to dangerous events
//!
//! Vault potentially dangerous exposures are recorded in `SETTREE` as
//! encoded [`DangerRecord`] under key `DANGER`.
//!
//! Vault should stay offline (i.e. air-gapped) throughout its usage.
//!
//! In case Vault finds itself online, it records this in the database
//! danger record and generates log entry in `HISTORY` tree.
//!
//! [`DangerRecord`] could be reset only by designated reset function, and the
//! fact of the reset is also recorded in the history log.  
use parity_scale_codec::{Decode, Encode};
use sled::IVec;

use crate::error::Result;

/// Danger status in the Vault database
///
/// Indicates if the Vault has a record of unsafe exposure.
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
    /// Infallible, as the validity of the value is not checked.
    pub fn from_ivec(ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }

    /// Get the value of `device_was_online` flag from `DangerRecord`.  
    ///
    /// Could result in error if the `DangerRecord` content is corrupted.  
    pub fn device_was_online(&self) -> Result<bool> {
        Ok(<DecodedDangerRecord>::decode(&mut &self.0[..])?.device_was_online)
    }

    /// Transform `DangerRecord` into `Vec<u8>` to put in the database.  
    pub fn store(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
