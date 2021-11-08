use qr_reader_pc::{process_qr_image};
use image::{open};
use qr_reader_phone::process_payload::{InProgress, Ready};

#[test]
fn check_single_qr_hex() -> Result<(), String> {
    let correct_result = String::from("01d43593c715fdd31c61141abd0\
    4a99fd6822c8558854ccde39a5684e7a56da27d82750682cdb4208cd7c\
    13bf399b097dad0a8064c45e79a8bc50978f6a8a5db0775bcb4c335897\
    8ca625496e056f2e7ddf724cf0040e5ff106d06f54efbd95389");
    
    let gray_img = match open("./tests/test_qr_1.jpg") {
        Ok(x) => x.into_luma8(),
        Err(_) => return Err(String::from("File reading error.")),
    };

    let mut result = String::new();

    match process_qr_image(&gray_img, InProgress::None) {
        Ok(x) => match x {
            Ready::Yes(a) => result.push_str(&hex::encode(&a)),
            Ready::NotYet(_) => return Err(String::from("Waiting animated QR.")),
        },
        Err(_) => return Err(String::from("QR image processing error.")),
    };

    if result != correct_result {
        println!("Correct result: {}", correct_result);
        println!("Decoding result: {}", result);
        Err(String::from("Incorrect result"))
    } else {
        Ok(())
    }
}
