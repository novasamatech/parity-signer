
use db_handling::{default_cold, default_hot};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    default_cold()?;
    default_hot()?;
    
    Ok(())

}

