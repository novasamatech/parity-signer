#![deny(rustdoc::broken_intra_doc_links)]

use bitvec::prelude::{BitVec, Msb0};
use constants::{qr_palette, BORDER, CHUNK_SIZE, FPS_DEN, FPS_NOM, SCALING};
use qrcode_static::{png_qr, DataType};
use qrcodegen::{QrCode, QrCodeEcc};
use std::fs;
use std::path::Path;

/// function to take data as `Vec<u8>`, apply `raptorq` to get `Vec<EncodingPacket>`
/// and serialize it to get `Vec<u8>` output
fn make_data_packs(input: &[u8]) -> Result<Vec<Vec<u8>>, &'static str> {
    // checking that data is not too long, set limit for now at 2^31 bit
    if input.len() >= 0x80000000 {
        return Err("Input data is too long, processing not possible");
    }
    // added at the beginning to each vector before transforming into qr code: contains input length info, also has first bit always 1 indicating it is new fountain qr - possibly need to change this later
    let data_size_info = (input.len() as u32 + 0x80000000).to_be_bytes();

    // number of additional packets; currently roughly equal to number of core packets
    let repair_packets_per_block: u32 = {
        if input.len() as u32 <= CHUNK_SIZE as u32 {
            0
        } else {
            input.len() as u32 / CHUNK_SIZE as u32
        }
    };
    // making `raptorq` Encoder, with defaults
    let raptor_encoder = raptorq::Encoder::with_defaults(input, CHUNK_SIZE);
    // making EncodingPacket and deserializing each into `Vec<u8>`
    let out: Vec<Vec<u8>> = raptor_encoder
        .get_encoded_packets(repair_packets_per_block)
        .iter()
        .map(|x| [data_size_info.to_vec(), x.serialize()].concat())
        .collect();
    let len_check = out[0].len();
    for x in out.iter() {
        if x.len() != len_check {
            return Err("Encoded chunks have different length");
        }
    }
    if len_check > 2953 {
        // 2953 is bytes limit for qr codes having 8-bit binary data
        return Err("Encoded chunks too large to be turned into QR codes");
    }
    Ok(out)
}

/// function to take data as `Vec<Vec<u8>>` with all stuff added and make `Vec<QrCode>`
fn make_qr_codes(data: Vec<Vec<u8>>) -> Result<Vec<QrCode>, Box<dyn std::error::Error>> {
    let mut out: Vec<QrCode> = Vec::new();
    for x in data.iter() {
        let new = QrCode::encode_binary(x, QrCodeEcc::Low)?;
        out.push(new);
    }
    Ok(out)
}

fn make_apng<P>(data: Vec<QrCode>, output_name: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let mut output_file = fs::File::create(output_name)?;
    let frames_count: u32 = data.len() as u32;
    let border_size = BORDER * SCALING;

    // size is always positive and small
    let size: u32 = (data[0].size() as u32) * (SCALING as u32) + 2 * border_size as u32;
    let mut encoder = png::Encoder::new(&mut output_file, size, size);

    encoder.set_color(png::ColorType::Indexed);
    encoder.set_palette(qr_palette());
    encoder.set_animated(frames_count, 0)?;
    encoder.set_frame_delay(FPS_NOM, FPS_DEN)?;
    encoder.set_depth(png::BitDepth::One);

    let mut writer = encoder.write_header()?;
    // making actual apng
    // qr.get_module(x,y) = false corresponds to back color (white by default)
    // qr.get_module(x,y) = true corresponds to main color (black by default)
    for qr in data.iter() {
        let mut buffer: Vec<u8> = Vec::new();
        for y in 0..size {
            let mut pixels: BitVec<u8, Msb0> = BitVec::with_capacity(size as usize);
            for x in 0..size {
                pixels
                    .push(!qr.get_module(x as i32 / SCALING - BORDER, y as i32 / SCALING - BORDER))
            }
            buffer.extend_from_slice(&pixels.into_vec());
        }
        writer.write_image_data(&buffer)?;
    }
    writer.finish()?;

    Ok(())
}

/// Function to transform input `Vec<u8>` into fountain qr-code
fn transform_into_qr_apng<P>(input: &[u8], output_name: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let data_packs = make_data_packs(input)?;
    make_apng(make_qr_codes(data_packs)?, output_name)?;
    Ok(())
}

/// Function to make appropriately sized qr code, apng or static
pub fn make_pretty_qr<P>(input: &[u8], output_name: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    if input.len() <= 2953 {
        let qr = png_qr(input, DataType::Regular)?;
        match std::fs::write(output_name, &qr) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::from(format!("Output error {}", e))),
        }
    } else {
        transform_into_qr_apng(input, output_name)
    }
}
