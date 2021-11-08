use qr_reader_pc::{arg_parser, run_with_camera};
use std::env;

fn main() -> Result<(), String> {

    let arguments = env::args().collect();

    let camera_settings = match arg_parser(arguments) {
        Ok(x) => x,
        Err(e) => return Err(format!("{}", e)),
    };

    match run_with_camera(camera_settings) {
        Ok(line) => println!("Result HEX: {}", line),
        Err(e) => return Err(format!("QR reading error. {}", e)),
    }

    Ok(())
}
