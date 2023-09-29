//! Record of the Vault database schema version
use constants::LIVE_SCHEMA_VERSION;
use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use std::ops::Deref;

use crate::error::Result;

type SchemaVersionType = u32;
#[derive(Decode, Encode)]
pub struct SchemaVersion(SchemaVersionType);

impl SchemaVersion {
    /// Get the current schema version
    pub fn current() -> Self {
        Self(LIVE_SCHEMA_VERSION)
    }

    /// Get `SchemaVersion` with content from the encoded value,
    /// as it is stored in the database.
    pub fn from_ivec(ivec: &IVec) -> Result<Self> {
        Ok(Self(SchemaVersionType::decode(&mut &ivec[..])?))
    }

    pub fn store_current() -> Vec<u8> {
        Self::current().encode()
    }
}

impl Deref for SchemaVersion {
    type Target = SchemaVersionType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
