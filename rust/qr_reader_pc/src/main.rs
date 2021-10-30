use std::env;
use qr_reader_pc::{run_with_camera, arg_parser};

fn main() -> Result<(), String> {
   
    let camera_num = match arg_parser(env::args()) {
        Ok(x) => x,
        Err(e) => return Err(format!("{}", e)),
    };
    
    match run_with_camera(camera_num) {
        Ok(line) => println!("Result HEX: {}", line),
        Err(e) => return Err(format!("QR reading error. {}", e)),
    }

    Ok(())

}

