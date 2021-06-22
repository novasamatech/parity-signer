use std::fs;
use raptorq;
use qrcodegen::{QrCode, QrCodeEcc};
use apng_encoder;

mod constants;
    use constants::{CHUNK_SIZE, BORDER, SCALING, FPS_NOM, FPS_DEN, MAIN_COLOR, BACK_COLOR};

/// function to take data as Vec<u8>, apply raptorq to get Vec<EncodingPacket>
/// and serialize it to get Vec<u8> output

pub fn make_data_packs (input: &Vec<u8>) -> Result<Vec<Vec<u8>>, &'static str> {

// checking that data is not too long, set limit for now at 2^31 bit
    if input.len() >= 0x80000000 { 
        return Err("Input data is too long, processing not possible");
    }
// added at the beginning to each vector before transforming into qr code: contains input length info, also has first bit always 1 indicating it is new fountain qr - possibly need to change this later
    let data_size_info = (input.len() as u32 + 0x80000000).to_be_bytes();

// number of additional packets; currently roughly equal to number of core packets
    let repair_packets_per_block: u32 = {
        if input.len() as u32 <= CHUNK_SIZE as u32 {0}
        else {input.len() as u32/CHUNK_SIZE as u32}
    };
// making raptorq Encoder, with defaults
    let raptor_encoder = raptorq::Encoder::with_defaults(input, CHUNK_SIZE);
// making EncodingPacket and deserializing each into Vec<u8>
    let out: Vec<Vec<u8>> = raptor_encoder.get_encoded_packets(repair_packets_per_block)
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
            return Err("Encoded chunks too large to be turned into QR codes");
    }
    Ok(out)
}

/// function to take data as Vec<Vec<u8>> with all stuff added and make Vec<QrCode>

pub fn make_qr_codes (data: Vec<Vec<u8>>) -> Result<Vec<QrCode>, Box<dyn std::error::Error>> {
    let mut out: Vec<QrCode> = Vec::new();
    for x in data.iter() {
        let new = QrCode::encode_binary(&x, QrCodeEcc::Low)?;
        out.push(new);
    }
    Ok(out)
}

pub fn make_apng (data: Vec<QrCode>, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_file = fs::File::create(output_name)?;
    let frames_count: u32 = data.len() as u32;
    let border_size = BORDER*SCALING;
    let size: u32 = (data[0].size() as u32) * (SCALING as u32) + 2*border_size as u32; // size is always positive and small
    let apng_meta = apng_encoder::Meta {
        width: size,
        height: size,
        color: apng_encoder::Color::Grayscale(8), 
        frames: frames_count,
        plays: None,
    };
    let apng_frame = apng_encoder::Frame {
        delay: Some(apng_encoder::Delay::new(FPS_NOM, FPS_DEN)),
        ..Default::default()
    };
    let mut apng_encoder = match apng_encoder::Encoder::create(&mut output_file, apng_meta) {
        Ok(a) => a,
        Err(e) => {
            let err_text = format!("Apng encoder error. {}", e);
            let err: Box<dyn std::error::Error> = From::from(err_text);
            return Err(err)
        },
    };

// making actual apng
// qr.get_module(x,y) = false corresponds to back color (white by default)
// qr.get_module(x,y) = true corresponds to main color (black by default)

    for qr in data.iter() {
        let mut buffer: Vec<u8> = Vec::new();
        for x in 0..size {
            for y in 0..size {
                if qr.get_module(x as i32/SCALING - BORDER, y as i32/SCALING - BORDER) {
                    buffer.push(MAIN_COLOR);
                }
                else {
                    buffer.push(BACK_COLOR);
                }
            }
        }
        match apng_encoder.write_frame(&buffer, Some(&apng_frame), None, None) {
            Ok(a) => a,
            Err(e) => {
                let err_text = format!("Apng encoder error. {}", e);
                let err: Box<dyn std::error::Error> = From::from(err_text);
                return Err(err)
            },
        }
    }
    match apng_encoder.finish() {
        Ok(a) => a,
        Err(e) => {
            let err_text = format!("Apng encoder error. {}", e);
            let err: Box<dyn std::error::Error> = From::from(err_text);
            return Err(err)
        },
    }
    Ok(())
}

/// Function to transform input Vec<u8> into fountain qr-code

pub fn transform_into_qr_apng (input: &Vec<u8>, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data_packs = make_data_packs(input)?;
    make_apng(make_qr_codes(data_packs)?, output_name)?;
    Ok(())
}
