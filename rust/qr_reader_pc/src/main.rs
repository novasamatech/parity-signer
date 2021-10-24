use qr_reader_pc::run_with_camera;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let camera_num = match args[1].parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    match run_with_camera(camera_num) {
        Ok(line) => println!("Success! {}", line),
        Err(e) => println!("Error. {}", e),
    }
}

