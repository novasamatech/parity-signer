use qr_reader_pc::{run_with_camera, arg_check};
use nokhwa::{query_devices, CaptureAPIBackend};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (camera_num, camera_request) = arg_check(args);
    
    if camera_request {
        println!("{:?}", query_devices(CaptureAPIBackend::Video4Linux).unwrap());
    }
    else {
        match run_with_camera(camera_num) {
            Ok(line) => println!("Success! {}", line),
            Err(e) => println!("Error. {}", e),
        }
    }
}

