// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! This is a reference fountain QR message generator for Signer;
//!
//! !!! Do NOT compile this into native lib for Signer itself !!!
//!
//! Usage: echo filename | cargo run

use raptorq;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use qrcodegen::{QrCode, QrCodeEcc};
use apng_encoder;
use hex;
//use num_integer::Roots;

//Chunk size is chosen to fit nicely in easy-to-recognize QR frame
const CHUNK_SIZE: u16 = 1072;

//const SIZE: u16 = 113;

// apng specs
const WHITE_COLOR: u8 = 0xFF;
const SCALING: i32 = 4;
const FPS_NOM: u16 = 1;
const FPS_DEN: u16 = 4;
const BORDER: i32 = 4;

fn main() {

    // Get data from env::args()
    let chunk_size = CHUNK_SIZE; //placeholder for args parser
    println!("Reading input...");
    let mut source_data_string = String::new();
    io::stdin().read_line(&mut source_data_string).expect("Failed to read input");
    let source_data = source_data_string.trim();


    let filename_out = "out.png";

    // Compactify data
    println!("Compressing...");
    //TODO: compress more!!!
    let compressed_data = hex::decode(source_data).expect("Not a hex string");
    let data_size = compressed_data.len() as u64;
    assert!(data_size < 0x80000000);   //upstream limited to u40, we will limit to u31
    let data_size_vec = ((data_size + 0x80000000) as u32).to_be_bytes();
    let repair_packets_per_block = (data_size/(chunk_size as u64)) as u32;
    println!("appended data size: {:?}", data_size_vec);
    println!("repair packets count: {}", repair_packets_per_block);

    // Generate raptorq frames
    println!("Generating fountain frames...");
    let mut output_file = File::create(filename_out).unwrap();
    let mut qr_frames_nervous_counter = 0;

    let raptor_encoder = raptorq::Encoder::with_defaults(&compressed_data, chunk_size);
    let frames: Vec<QrCode> = raptor_encoder.get_encoded_packets(repair_packets_per_block)
        .iter()
        .map(|packet| packet.serialize())   //TODO: these packets have useless fileds that could be derived
        .map(|serpacket| [data_size_vec.to_vec(), serpacket].concat())
        .map(|qrpacket| {
            qr_frames_nervous_counter += 1;
            println!("Generating fountain codes: {}", qr_frames_nervous_counter);
            //println!("{:?}", qrpacket);
            QrCode::encode_binary(&qrpacket, QrCodeEcc::Low).unwrap()
        })
        .collect();

    // Generate video frames
    let frames_count = frames.len().try_into().unwrap();
    println!("Generating {} frames", frames_count);
    let border_size = BORDER*SCALING;
    let size: u32 = (frames[0].size() as u32) * (SCALING as u32) + 2*border_size as u32; // size is always positive and small

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

    let mut apng_encoder = apng_encoder::Encoder::create(&mut output_file, apng_meta).unwrap();
    let mut nervous_counter = 0;

    frames.iter().for_each(|qr| {
            nervous_counter += 1;
            println!("Generating frame {} of {}", nervous_counter, frames_count);
            let mut buffer: Vec<u8> = Vec::new();
            for x in 0..size {
                for y in 0..size {
                    buffer.push((qr.get_module(x as i32 / SCALING - BORDER, y as i32 / SCALING - BORDER) as u8) * WHITE_COLOR);
            }}
            apng_encoder.write_frame(&buffer, Some(&apng_frame), None, None).unwrap();
        });
    apng_encoder.finish().unwrap();

    println!("Done!");
}
