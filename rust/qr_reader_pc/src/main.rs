use qr_reader_pc::run_with_camera;

fn main() {
    match run_with_camera() {
        Ok(line) => println!("Success! {}", line),
        Err(e) => println!("Error. {}", e),
    }
}

