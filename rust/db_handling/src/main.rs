use std::fs;
use db_handling::{DataFiles, default_cold, default_hot};

fn main() -> Result<(), Box<dyn std::error::Error>> {

// populate cold database with defaults from the files
    
    let metadata_contents = match fs::read_to_string("metadata_database.ts") {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Metadata database missing")),
    };
    
    let datafiles = DataFiles {
        metadata_contents: &metadata_contents,
    };
    
    default_cold(datafiles)?;
    default_hot()?;
    
    Ok(())

}

