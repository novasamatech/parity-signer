use sled::Db;
use anyhow;

use crate::error::Error;

/// Function to verify checksum from action line
pub fn verify_checksum (database: &Db, checksum: u32) -> anyhow::Result<()> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    Ok(())
}

