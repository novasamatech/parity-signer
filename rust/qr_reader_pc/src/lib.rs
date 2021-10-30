#![deny(missing_docs)]

//! # QR reader crate for PC
//!
//! `qr_reader_pc` is a utility to scan (via webcam) QR codes from Signer
//! and extracting data from it.

extern crate minifb;
use minifb::{Window, WindowOptions};

use nokhwa::{Camera, CameraFormat, FrameFormat};
use nokhwa::{query_devices, CaptureAPIBackend};

use image::{Pixel, Luma, ImageBuffer, GrayImage};
use quircs;
use hex;
use qr_reader_phone::process_payload::{process_decoded_payload, Ready, InProgress};
use anyhow::anyhow;
use std::env;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;
const DEFAULT_FRAME_FORMAT: FrameFormat = FrameFormat::YUYV;
const DEFAULT_FPS: u32 = 30;

/// Main cycle of video input reading.
/// 
pub fn run_with_camera(camera_settings : CameraSettings) -> anyhow::Result<String> {

    let mut camera = match Camera::new(camera_settings.index, 
        Some(CameraFormat::new_from(DEFAULT_WIDTH, DEFAULT_HEIGHT, camera_settings.frame_format, camera_settings.fps)),) {

        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error opening camera. {}", e)),
    };   

    let mut window = match Window::new(
        "Camera capture",
        DEFAULT_WIDTH as usize,
        DEFAULT_HEIGHT as usize,
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
    let mut gray_img: GrayImage = ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    for y in 0..DEFAULT_HEIGHT {
        for x in 0..DEFAULT_WIDTH {
            let new_pixel = frame.get_pixel(x, y).to_luma();
            let buf_pix = frame.get_pixel(x, y).0;
            let buf_col = u32::from_be_bytes([0,buf_pix[0], buf_pix[1], buf_pix[2]]);
            gray_img.put_pixel(x, y, new_pixel);
            out_buf.push(buf_col);
        }            
    }

    match window.update_with_buffer(&out_buf[..], DEFAULT_WIDTH as usize, DEFAULT_HEIGHT as usize) {
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

/// Structure for camera settings.
///
pub struct CameraSettings {
    index: usize,
    frame_format: FrameFormat,
    fps: u32,
}

/// Program's argument parser.
///
pub fn arg_parser(mut args: env::Args) -> anyhow::Result<CameraSettings>
{
    args.next(); // skip program name

    match (args.next(), args.next()) {

        (Some(argument), Some(camera_index)) if argument == "d" => { 

            let index = match camera_index.trim().parse() {
                Ok(index) => index,
                Err(e) => return Err(anyhow!("Index parsing error: {}", e)),
            };  

            let frame_format = match args.next() {
                Some(argument) if argument == "YUYV" => FrameFormat::YUYV,
                Some(argument) if argument == "MJPEG" => FrameFormat::MJPEG,
                _ => {println!("Frame format parsing error. Set default frame format"); DEFAULT_FRAME_FORMAT},
            };  
            
            let fps = match args.next() {
                Some(fps) => match fps.trim().parse() {
                        Ok(fps) => fps,
                        Err(_) => {println!("FPS parsing error. Set default framerate"); DEFAULT_FPS},
                    },
                None => {println!("FPS parsing error. Set default framerate"); DEFAULT_FPS},
            };                    

            Ok(CameraSettings {index, frame_format, fps})
        }
        (Some(_), ..) => Err(anyhow!("Can`t recognize arguments.")),
        (None, ..) => {
            println!("\nNot enough arguments. Use 'd' argument to set index of camera, frame format (YUYV or MJPEG) and fps.\
            \nExample: cargo run d 0 MJPEG 30\
            \nYou can only provide index. Default frame format: YUYV, default fps: 30, hardcoded framesize: 640x480\
            \nExample: cargo run d 0");

            println!("List of available devices:");
            if let Ok(list) = query_devices(CaptureAPIBackend::Video4Linux) {
                for device in list {
                    println!("{:?}", device);
                };
                println!();
            };
            Err(anyhow!("Error parsing arguments."))},
    }
}