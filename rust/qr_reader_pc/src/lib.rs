#![deny(missing_docs)]

//! # QR reader crate for PC
//!
//! `qr_reader_pc` is a utility to scan (via webcam) QR codes from Signer
//! and extracting data from it.

use minifb::{Window, WindowOptions};
use nokhwa::{query_devices, CaptureAPIBackend};
use nokhwa::{Camera, CameraFormat, FrameFormat};
use anyhow::anyhow;
use image::{GrayImage, ImageBuffer, Luma, Pixel};
use qr_reader_phone::process_payload::{process_decoded_payload, InProgress, Ready};
use std::env;
// use quircs;
// use hex;

// Default camera settings
const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;
const DEFAULT_FRAME_FORMAT: FrameFormat = FrameFormat::YUYV;
const DEFAULT_FPS: u32 = 30;
const DEFAULT_BACKEND: CaptureAPIBackend = CaptureAPIBackend::Video4Linux;

/// Main cycle of video capture.
/// Returns a string with decoded QR message in HEX format or error
///
/// # Arguments
///
/// * `camera_settings` - A CameraSettings struct that holds the camera parameters
pub fn run_with_camera(camera_settings: CameraSettings) -> anyhow::Result<String> {
    let mut camera = match Camera::new(
        camera_settings.index.unwrap(),
        Some(CameraFormat::new_from(
            DEFAULT_WIDTH,
            DEFAULT_HEIGHT,
            camera_settings.frame_format,
            camera_settings.fps,
        )),
    ) {
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
                    Ok(img) => process_qr_image(&img, decoding)?,
                    Err(_) => Ready::NotYet(decoding),
                };
            }
            Ready::Yes(a) => {
                line.push_str(&hex::encode(&a));
                match std::fs::write("decoded_output.txt", &line) {
                    Ok(_) => (),
                    Err(e) => println!("Unable to write decoded information in the file. {}", e),
                };
                break;
            }
        }
    }
    Ok(line)
}

fn camera_capture(camera: &mut Camera, window: &mut Window,) -> anyhow::Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
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
            let buf_col = u32::from_be_bytes([0, buf_pix[0], buf_pix[1], buf_pix[2]]);
            gray_img.put_pixel(x, y, new_pixel);
            out_buf.push(buf_col);
        }
    };

    match window.update_with_buffer(
        &out_buf[..],
        DEFAULT_WIDTH as usize,
        DEFAULT_HEIGHT as usize,
    ) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error writing videobuffer. {}", e)),
    };

    Ok(gray_img)
}

fn process_qr_image(img: &ImageBuffer<Luma<u8>, Vec<u8>>, decoding: InProgress,) -> anyhow::Result<Ready> {    
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(img.width() as usize, img.height() as usize, img);
    match codes.last() {
        Some(Ok(code)) => {
            match code.decode() {
                Ok(decoded) => process_decoded_payload(decoded.payload, decoding),
                Err(_) => {
                    Ok(Ready::NotYet(decoding))
                }
            }
        },
        None | Some(Err(_)) => {
            Ok(Ready::NotYet(decoding))
        },
    }
}

/// Structure for storing camera settings.
pub struct CameraSettings {
    index: Option<usize>,
    frame_format: FrameFormat,
    fps: u32,
}

fn print_list_of_cameras() {
    println!("List of available devices:");
    match query_devices(DEFAULT_BACKEND) {
        Ok(list) => {
            for device in list {
                println!("{:?}", device);
            }
        }
        Err(_) => println!("Can`t capture list of cameras."),
    };
}

/// Program's argument parser.
/// Parser initialize CameraSettings struct with default values or program arguments.
/// The program arguments are described in the readme.md file.
pub fn arg_parser(mut args: env::Args) -> anyhow::Result<CameraSettings> {
    args.next(); // skip program name

    let mut settings = CameraSettings {
        index: None,
        frame_format: DEFAULT_FRAME_FORMAT,
        fps: DEFAULT_FPS,
    };

    while let Some(arg) = args.next() {
        let par = match args.next() {
            Some(x) => x,
            None => String::from(""),
        };

        match &arg[..] {
            "d" | "-d" | "--device" => match par.trim().parse() {
                Ok(index) => settings.index = Some(index),
                Err(e) => return Err(anyhow!("Camera index parsing error: {}", e)),
            },

            "ff" | "-ff" => match &par[..] {
                "YUYV" => settings.frame_format = FrameFormat::YUYV,
                "MJPEG" => settings.frame_format = FrameFormat::MJPEG,
                _ => println!("Frame format parsing error. Set default frame format."),
            },

            "fps" | "-fps" => match par.trim().parse() {
                Ok(fps) => settings.fps = fps,
                Err(_) => println!("FPS parsing error. Set default framerate."),
            },

            "h" | "-h" | "--help" => {
                // TODO. The reference to the readme.md implemented now.
            }

            _ => return Err(anyhow!("Argument parsing error.")),
        };
    }

    match settings.index {
        Some(_) => Ok(settings),
        None => {
            print_list_of_cameras();
            Err(anyhow!(
                "Need to provide camera index. Please read readme.md file."
            ))
        }
    }
}
