use constants::{HOT_DB_NAME, TYLO};
use std::fs;
use db_handling::prep_messages::prep_types;

pub fn gen_types() -> Result<(), Box<dyn std::error::Error>> {
    
    let types_info = prep_types(HOT_DB_NAME)?;
    
    match fs::write(&TYLO, &types_info) {
        Ok(()) => Ok(()),
        Err(e) => {
            let err_text = format!("Problem writing types export file. {}", e);
            let err: Box<dyn std::error::Error> = From::from(err_text);
            return Err(err)
        },
    }
}
