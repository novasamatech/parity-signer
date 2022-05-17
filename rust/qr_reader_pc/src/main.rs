use qr_reader_pc::{arg_parser, run_with_camera};
use std::env;

fn main() -> Result<(), String> {
    let arguments = env::args().collect();

    let camera_settings = match arg_parser(arguments) {
        Ok(x) => x,
        Err(e) => return Err(format!("{}", e)),
    };

    let line = match run_with_camera(camera_settings) {
        Ok(line) => line,
        Err(e) => return Err(format!("QR reading error. {}", e)),
    };

    println!("Result HEX: {}", line);
    match std::fs::write("decoded_output.txt", &line) {
        Ok(_) => (),
        Err(e) => println!("Unable to write decoded information in the file. {}", e),
    };

    Ok(())
}
