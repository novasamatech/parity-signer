use constants::{HOT_DB_NAME, TYLO};
use db_handling::prep_messages::prep_types;

use crate::error::Error;

pub fn gen_types() -> anyhow::Result<()> {
    
    let content = prep_types(HOT_DB_NAME)?;
    match content.write(TYLO) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InputOutputError(e).show()),
    }
}
