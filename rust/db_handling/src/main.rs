//use constants::{COLD_DB_NAME, HOT_DB_NAME};
use db_handling::{/*default_cold, */default_cold_release, default_hot/*, metadata::transfer_metadata*/};
use anyhow;

fn main() -> anyhow::Result<()> {
    
    default_hot()?;
//    default_cold_release()?;
//    transfer_metadata(HOT_DB_NAME, COLD_DB_NAME)?;
    Ok(())
}

