#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! # QR reader crate for PC
//!
//! `qr_reader_pc` is a utility to capture (via webcam) QR codes from Vault
//! and extracting data from it.

use anyhow::anyhow;
use image::{GrayImage, ImageBuffer, Luma};
use indicatif::ProgressBar;
use qr_reader_phone::process_payload::{process_decoded_payload, InProgress, Ready};

use opencv::{
    core::AlgorithmHint,
    highgui,
    imgproc::{cvt_color, COLOR_BGR2GRAY},
    prelude::*,
    videoio,
    videoio::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH},
    Result,
};

// Default camera settings
const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;
const MAX_CAMERA_INDEX: i32 = 6;
const SKIPPED_FRAMES_QTY: u32 = 10;

/// Structure for storing camera settings.
#[derive(Debug)]
pub struct CameraSettings {
    /// Camera index
    pub index: Option<i32>,
}

/// Main cycle of video capture.
/// Returns a string with decoded QR message in HEX format or error.
///
/// # Arguments
///
/// * `camera_settings` - `CameraSettings` struct that holds the camera parameters
pub fn run_with_camera(camera_settings: CameraSettings) -> anyhow::Result<String> {
    let camera_index = match camera_settings.index {
        Some(index) => index,
        None => return Err(anyhow!("There is no camera index.")),
    };

    let window = "video capture";
    highgui::named_window(window, 1)?;

    let mut camera = create_camera(camera_index, DEFAULT_WIDTH, DEFAULT_HEIGHT)?;
    skip_frames(&mut camera); // clearing old frames if they are in the camera buffer

    let mut out = Ready::NotYet(InProgress::None);
    let mut line = String::new();

    let pb = ProgressBar::new(1);
    loop {
        match out {
            Ready::NotYet(decoding) => {
                if let InProgress::Fountain(f) = &decoding {
                    pb.set_length(f.total as u64);
                    pb.set_position(f.collected() as u64)
                }
                out = match camera_capture(&mut camera, window) {
                    Ok(img) => process_qr_image(&img, decoding)?,
                    Err(_) => Ready::NotYet(decoding),
                };
            }
            Ready::Yes(a) => {
                line.push_str(&hex::encode(a));
                break;
            }
            _ => todo!(),
        }

        if highgui::wait_key(10)? > 0 {
            println!("Exit");
            break;
        };
    }
    highgui::destroy_window(window)?;
    Ok(line)
}

fn create_camera(
    camera_index: i32,
    width: u32,
    height: u32,
) -> anyhow::Result<videoio::VideoCapture> {
    #[cfg(ocvrs_opencv_branch_32)]
    let mut camera = videoio::VideoCapture::new_default(camera_index)?;
    #[cfg(not(ocvrs_opencv_branch_32))]
    let mut camera = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)?;

    match videoio::VideoCapture::is_opened(&camera) {
        Ok(opened) if opened => {
            camera.set(CAP_PROP_FRAME_WIDTH, width.into())?;
            camera.set(CAP_PROP_FRAME_HEIGHT, height.into())?;
        }
        Ok(_) => return Err(anyhow!("Camera already opened.")),
        Err(e) => return Err(anyhow!("Can`t open camera. {}", e)),
    };

    let mut frame = Mat::default();

    match camera.read(&mut frame) {
        Ok(_) if frame.size()?.width > 0 => Ok(camera),
        Ok(_) => Err(anyhow!("Zero frame size.")),
        Err(e) => Err(anyhow!("Can`t read camera. {}", e)),
    }
}

fn camera_capture(camera: &mut videoio::VideoCapture, window: &str) -> Result<GrayImage> {
    let mut frame = Mat::default();
    camera.read(&mut frame)?;

    if frame.size()?.width > 0 {
        highgui::imshow(window, &frame)?;
    };

    let mut image: GrayImage = ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let mut ocv_gray_image = Mat::default();

    cvt_color(
        &frame,
        &mut ocv_gray_image,
        COLOR_BGR2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    for y in 0..ocv_gray_image.rows() {
        for x in 0..ocv_gray_image.cols() {
            let pixel: Luma<u8> = Luma([*ocv_gray_image.at_2d(y, x)?]);
            image.put_pixel(x as u32, y as u32, pixel);
        }
    }

    Ok(image)
}

/// Function for decoding QR grayscale image.
/// Returns a string with decoded QR message in HEX format or error.
///
/// # Arguments
///
/// * `image` - Grayscale image containing QR and background
/// * `decoding` - Stores accumulated payload data for animated QR.
pub fn process_qr_image(image: &GrayImage, decoding: InProgress) -> anyhow::Result<Ready> {
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(image.width() as usize, image.height() as usize, image);

    match codes.last() {
        Some(Ok(code)) => match code.decode() {
            Ok(decoded) => process_decoded_payload(decoded.payload, &None, decoding)
                .map_err(|e| anyhow!(e.to_string())),
            Err(_) => Ok(Ready::NotYet(decoding)),
        },
        Some(_) => Ok(Ready::NotYet(decoding)),
        None => Ok(Ready::NotYet(decoding)),
    }
}

fn print_list_of_cameras() {
    let mut indexes: Vec<i32> = vec![];
    for dev_port in 0..=MAX_CAMERA_INDEX {
        if create_camera(dev_port, DEFAULT_WIDTH, DEFAULT_HEIGHT).is_ok() {
            indexes.push(dev_port);
        };
    }
    println!("\nList of available devices:");
    for index in indexes {
        println!("Camera index: {index}");
    }
}

fn skip_frames(camera: &mut videoio::VideoCapture) {
    for _x in 0..SKIPPED_FRAMES_QTY {
        if let Ok(false) | Err(_) = camera.grab() {
            break;
        }
    }
}

/// The program's argument parser.
/// The parser initializes the `CameraSettings` structure with program's arguments
/// (described in the `readme.md` file).
pub fn arg_parser(arguments: Vec<String>) -> anyhow::Result<CameraSettings> {
    let mut args = arguments.into_iter();
    args.next(); // skip program name

    let mut settings = CameraSettings { index: None };

    while let Some(arg) = args.next() {
        let par = args.next().unwrap_or_default();

        match &arg[..] {
            "d" | "-d" | "--device" => match par.trim().parse() {
                Ok(index) => settings.index = Some(index),
                Err(e) => return Err(anyhow!("Camera index parsing error: {}", e)),
            },
            "h" | "-h" | "--help" => println!("Please read readme.md file."),
            "l" | "-l" | "--list" => print_list_of_cameras(),
            _ => return Err(anyhow!("Argument parsing error.")),
        };
    }

    match settings.index {
        Some(_) => Ok(settings),
        None => Err(anyhow!(
            "Need to provide camera index. Please read readme.md file."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_camera_index() {
        let arguments: Vec<String> = vec![
            String::from("program_name"),
            String::from("d"),
            String::from("0"),
        ];
        let result = arg_parser(arguments).unwrap();
        assert_eq!(result.index, Some(0));
    }
}
