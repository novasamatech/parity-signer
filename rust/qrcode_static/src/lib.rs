#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use anyhow::anyhow;
use bitvec::prelude::{BitVec, Msb0};
use constants::{qr_palette, qr_palette_danger, BORDER, SCALING};
use qrcodegen::{QrCode, QrCodeEcc};

struct QrContent {
    content: Vec<u8>,
    size: u32,
}

/// Transform data slice `&[u8]` into qr data with indexed colors
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

/// Generate static qr code from slice `&[u8]`
pub fn png_qr(input: &[u8], data_type: DataType) -> anyhow::Result<Vec<u8>> {
    let qr_content = prepare_qr_png_data(input)?;

    let mut out: Vec<u8> = Vec::new();

    let mut encoder = png::Encoder::new(&mut out, qr_content.size, qr_content.size);
    encoder.set_color(png::ColorType::Indexed);

    let qr_palette = match data_type {
        DataType::Regular => qr_palette(),
        DataType::Sensitive => qr_palette_danger(),
    };

    encoder.set_palette(qr_palette);
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

/// Generate static qr code from a string.
///
/// For keys export and for signature generation.
pub fn png_qr_from_string(string_input: &str, data_type: DataType) -> anyhow::Result<Vec<u8>> {
    png_qr(string_input.as_bytes(), data_type)
}

/// What kind of data goes into QR, to additionally distinguish sensitive QRs
/// with color
pub enum DataType {
    /// signatures for transactions and public keys export
    Regular,

    /// secret keys export
    Sensitive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_data_1() {
        let data = hex::decode("530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .unwrap();
        assert!(png_qr(&data, DataType::Regular).is_ok());
    }

    #[test]
    fn qr_data_2() {
        let data = "secret:0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a:e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        assert!(png_qr_from_string(data, DataType::Sensitive).is_ok());
    }
}
