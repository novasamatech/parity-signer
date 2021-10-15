use constants::{COLD_DB_NAME, HOT_DB_NAME};
use db_handling::{default_cold, default_cold_release, default_hot, metadata::transfer_metadata_to_cold};
use anyhow;

fn main() -> anyhow::Result<()> {
    
//    default_hot()?;
//    default_cold_release()?;
    default_cold()?;
    transfer_metadata_to_cold(HOT_DB_NAME, COLD_DB_NAME)?;
    Ok(())
}

