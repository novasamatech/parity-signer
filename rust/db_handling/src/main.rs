use definitions::{constants::{COLD_DB_NAME, HOT_DB_NAME}};
use db_handling::{default_cold, default_hot, metadata::transfer_metadata, network_details::get_network_details_by_hex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
//    default_cold()?;
//    default_hot()?;
    transfer_metadata(HOT_DB_NAME, COLD_DB_NAME)
    
//    let line = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
//    let out = get_network_details_by_hex(line, COLD_DB_NAME)?;
//    Ok(println!("{}", out))

}

