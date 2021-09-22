use std::fs;
use std::process;

use qrcode_rtx::transform_into_qr_apng;

fn main() {
    let filename = "networkMetadata.txt";
    let data = fs::read_to_string(&filename).unwrap();
    
    let input = hex::decode(&data.trim()).unwrap();
    
    let out_name = format!("{}-qrcoded", filename);
    
    if let Err(e) = transform_into_qr_apng(&input, &out_name) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
