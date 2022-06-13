#![deny(unused_crate_dependencies)]

use anyhow::anyhow;
use bitvec::prelude::{BitVec, Msb0};
use constants::{qr_palette, BORDER, SCALING};
use qrcodegen::{QrCode, QrCodeEcc};

struct QrContent {
    content: Vec<u8>,
    size: u32,
}

fn prepare_qr_png_data(input: &[u8]) -> anyhow::Result<QrContent> {
    if input.len() > 2953 {
        return Err(anyhow!("Data too large to make static qr code."));
    } // 2953 is bytes limit for qr codes having 8-bit binary data
    let qr_code = match QrCode::encode_binary(input, QrCodeEcc::Low) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error making qr code. {}", e)),
    };
    let border_size = BORDER * SCALING;
    let size: u32 = (qr_code.size() as u32) * (SCALING as u32) + 2 * border_size as u32;
    let mut out: Vec<u8> = Vec::new();
    for y in 0..size {
        let mut pixels: BitVec<u8, Msb0> = BitVec::with_capacity(size as usize);
        for x in 0..size {
            pixels
                .push(!qr_code.get_module(x as i32 / SCALING - BORDER, y as i32 / SCALING - BORDER))
        }
        out.extend_from_slice(&pixels.into_vec());
    }
    Ok(QrContent { content: out, size })
}

/// Function to generate static qr code from Vec<u8>
pub fn png_qr(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let qr_content = prepare_qr_png_data(input)?;

    let mut out: Vec<u8> = Vec::new();

    let mut encoder = png::Encoder::new(&mut out, qr_content.size, qr_content.size);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_palette(qr_palette());
    encoder.set_depth(png::BitDepth::One);

    let mut writer = match encoder.write_header() {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("Error writing header. {}", e)),
    };

    match writer.write_image_data(&qr_content.content) {
        Ok(()) => (),
        Err(e) => return Err(anyhow!("Error writing image data. {}", e)),
    };
    drop(writer);
    Ok(out)
}

/// Function to generate static qr code from hex string
pub fn png_qr_from_hex(hex_input: &str) -> anyhow::Result<Vec<u8>> {
    let hex_input = {
        if let Some(a) = hex_input.strip_prefix("0x") {
            a
        } else {
            hex_input
        }
    };
    let input = match hex::decode(&hex_input) {
        Ok(x) => x,
        Err(_) => return Err(anyhow!("String is not in hex format")),
    };
    png_qr(&input)
}

/// Historically was used to generate static qr code from hex string for example in signatures.
/// Used for strings.
pub fn png_qr_from_string(string_input: &str) -> anyhow::Result<Vec<u8>> {
    let input = string_input.as_bytes().to_vec();
    png_qr(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_test_qr_code() {
        let hex_data = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let qr = png_qr_from_hex(hex_data).unwrap();
        std::fs::write("test.png", &qr).unwrap();
    }

    #[test]
    fn make_test_leg_qr_code() {
        let hex_data = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let qr = png_qr_from_string(hex_data).unwrap();
        std::fs::write("test_leg.png", &qr).unwrap();
    }
}
