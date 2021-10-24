extern crate minifb;
use minifb::{Window, WindowOptions};

use nokhwa::{Camera, CameraFormat, FrameFormat};
use image::{Pixel, Luma, ImageBuffer, GrayImage};
use quircs;
use hex;
use qr_reader_phone::process_payload::{process_decoded_payload, Ready, InProgress};
use anyhow::anyhow;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub fn run_with_camera(camera_num : usize) -> anyhow::Result<String> {

    let mut camera = match Camera::new(camera_num, Some(CameraFormat::new_from(WIDTH, HEIGHT, FrameFormat::MJPEG, 30)),) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error opening camera. {}", e)),
    };   

    let mut window = match Window::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    ) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error creating new output window. {}", e)),
    };
    
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    match camera.open_stream() {
        Ok(_) => (),
        Err(e) => return Err(anyhow!("Error starting camera. {}", e)),
    };

    let mut out = Ready::NotYet(InProgress::None);
    let mut line = String::new();
    
    loop {
        match out {
            Ready::NotYet(decoding) => {
                out = match camera_capture(&mut camera, &mut window) {
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

fn camera_capture(camera: &mut Camera, window: &mut Window) -> anyhow::Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
    
    let frame = match camera.frame() {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error with camera capture. {}", e)),
    };

    let mut out_buf: Vec<u32> = Vec::new();
    let mut gray_img: GrayImage = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let new_pixel = frame.get_pixel(x, y).to_luma();
            let buf_pix = frame.get_pixel(x, y).0;
            let buf_col = u32::from_be_bytes([0,buf_pix[0], buf_pix[1], buf_pix[2]]);
            gray_img.put_pixel(x, y, new_pixel);
            out_buf.push(buf_col);
        }            
    }

    match window.update_with_buffer(&out_buf[..], 640, 480) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error writing videobuffer. {}", e)),
    };

    Ok(gray_img)
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

