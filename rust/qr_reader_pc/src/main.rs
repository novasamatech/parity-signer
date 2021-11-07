use qr_reader_pc::{arg_parser, run_with_camera};
use std::env;


fn main() -> Result<(), String> {

    let arguments = env::args().collect();

    let camera_setings = match arg_parser(arguments) {
        Ok(x) => x,
        Err(e) => return Err(format!("{}", e)),
    };

    match run_with_camera(camera_setings) {
        Ok(line) => println!("Result HEX: {}", line),
        Err(e) => return Err(format!("QR reading error. {}", e)),
    }

    Ok(())
}
