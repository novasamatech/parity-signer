use rscam::{Camera, Config};
use std::io::Write;
use image::{GenericImageView, Pixel, Luma, ImageBuffer, GrayImage};
use quircs;
use hex;
use qr_reader_phone::process_payload::{process_decoded_payload, Ready, InProgress};
use anyhow::anyhow;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub fn run_with_camera() -> anyhow::Result<String> {

    let mut camera = match Camera::new("/dev/video0") {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error opening camera. {}", e)),
    };

    match camera.start(&Config {
        interval: (1, 30),      // 30 fps.
        resolution: (WIDTH, HEIGHT),
        format: b"MJPG",
        ..Default::default()
    }) {
        Ok(_) => (),
        Err(e) => return Err(anyhow!("Error starting camera. {}", e)),
    };
    
    let mut out = Ready::NotYet(InProgress::None);
    let mut line = String::new();
    
    loop {
        match out {
            Ready::NotYet(decoding) => {
                out = match camera_capture(&camera) {
                    Ok(img) => process_qr_image (&img, decoding)?,
                    Err(_) => Ready::NotYet(decoding),
                };
            },
            Ready::Yes(a) => {
                line.push_str(&hex::encode(&a));
                match std::fs::write("decoded_output.txt", &line) {
                    Ok(_) => (),
                    Err(e) => println!("Unable to write decoded information in the file. {}", e)
                };
                break;
            },
        }
    }
    Ok(line)
}


fn camera_capture(camera: &Camera) -> anyhow::Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let frame = match camera.capture() {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error with camera capture. {}", e)),
    };
    let mut captured_data: Vec<u8> = Vec::new();
    match captured_data.write_all(&frame[..]) {
        Ok(_) => (),
        Err(e) => return Err(anyhow!("Error writing data from camera into buffer. {}", e)),
    };
    match image::load_from_memory(&captured_data[..]) {
        Ok(a) => {
            let mut gray_img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let new_pixel = a.get_pixel(x, y).to_luma();
                    gray_img.put_pixel(x, y, new_pixel);
                }
            }
//        println!("got gray img");
            Ok(gray_img)
        },
        Err(e) => return Err(anyhow!("Error loading data from buffer. {}", e)),
    }
}

fn process_qr_image (img: &ImageBuffer<Luma<u8>, Vec<u8>>, decoding: InProgress) -> anyhow::Result<Ready> {
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(img.width() as usize, img.height() as usize, img);
    match codes.last() {
        Some(x) => {
            if let Ok(code) = x {
                match code.decode() {
                    Ok(decoded) => {
                        process_decoded_payload(decoded.payload, decoding)
                    },
                    Err(_) => {
//                        println!("Error with this scan: {}", e);
                        Ok(Ready::NotYet(decoding))
                    }
                }
            }
        else {Ok(Ready::NotYet(decoding))}
        },
        None => {
//            println!("no qr in this scan");
            Ok(Ready::NotYet(decoding))
        },
    }
}

