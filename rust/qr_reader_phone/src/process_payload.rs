use std::convert::TryInto;
use anyhow::anyhow;
use raptorq;
use definitions::constants::CHUNK_SIZE;


#[derive(PartialEq)]
pub struct Fountain {
    decoder: raptorq::Decoder,
    collected_ser_packets: Vec<Vec<u8>>,
    length: u32,
}

#[derive(PartialEq)]
pub struct LegacyMulti {
    length: u16,
    elements: Vec<Element>,
}

#[derive(PartialEq)]
pub struct Element {
    number: u16,
    contents: Vec<u8>,
}

#[derive(PartialEq)]
pub enum InProgress {
    None,
    Fountain(Fountain),
    LegacyMulti(LegacyMulti),
}

#[derive(PartialEq)]
pub enum Ready {
    NotYet(InProgress),
    Yes(Vec<u8>),
}

pub fn process_decoded_payload (payload: Vec<u8>, mut decoding: InProgress) -> anyhow::Result<Ready> {
    if payload[0]&0b10000000 > 0 {
//        println!("dealing with part of dynamic fountain qr code");
        let length_piece: [u8; 4] = payload[..4].to_vec().try_into().expect("constant vector slice size, always fits");
        let length = u32::from_be_bytes(length_piece)-0x80000000;
        let new_packet = payload[4..].to_vec();
        let number_of_messages = length/(new_packet.len() as u32) + 2;
        match decoding {
            InProgress::None => {
                let collected_ser_packets = vec![new_packet];
                println!("collected {} packets, out of approx. {} expected", collected_ser_packets.len(), number_of_messages);
                let config = raptorq::ObjectTransmissionInformation::with_defaults(length as u64, CHUNK_SIZE);
                let mut decoder = raptorq::Decoder::new(config);
                match try_fountain (&collected_ser_packets, &mut decoder) {
                    Some(v) => Ok(Ready::Yes(v)),
                    None => {
                        let in_progress = Fountain {
                            decoder,
                            collected_ser_packets,
                            length,
                        };
                        decoding = InProgress::Fountain(in_progress);
                        Ok(Ready::NotYet(decoding))
                    },
                }
            },
            InProgress::Fountain(mut in_progress) => {
                if in_progress.length != length {return Err(anyhow!("Was decoding fountain qr code with message length {}, got interrupted by fountain qr code with message length {}", in_progress.length, length))}
                if !in_progress.collected_ser_packets.contains(&new_packet) {
                    in_progress.collected_ser_packets.push(new_packet);
                    println!("collected {} packets, out of approx. {} expected", in_progress.collected_ser_packets.len(), number_of_messages);
                    match try_fountain (&in_progress.collected_ser_packets, &mut in_progress.decoder) {
                        Some(v) => Ok(Ready::Yes(v)),
                        None => Ok(Ready::NotYet(InProgress::Fountain(in_progress))),
                    }
                }
                else {
                    Ok(Ready::NotYet(InProgress::Fountain(in_progress)))
                }
            },
            InProgress::LegacyMulti(_) => return Err(anyhow!("Was decoding legacy multi-element qr, and got interrupted by a fountain one.")),
        }
    }
    else {
        if payload.starts_with(&[0]) {
//            println!("dealing with part of legacy dynamic multi-element qr code");
            let length_piece: [u8; 2] = payload[1..3].to_vec().try_into().expect("constant vector slice size, always fits");
            let length = u16::from_be_bytes(length_piece);
            let element_number_piece: [u8; 2] = payload[3..5].to_vec().try_into().expect("constant vector slice size, always fits");
            let number = u16::from_be_bytes(element_number_piece);
            if number >= length {return Err(anyhow!("Number of element in legacy multi-element qr sequence exceeds expected sequence length."))}
            let contents = payload[5..].to_vec();
            let new_element = Element {
                number,
                contents,
            };
            match decoding {
                InProgress::None => {
                    let mut collected = LegacyMulti {
                        length,
                        elements: vec![new_element],
                    };
                    match try_legacy(&mut collected) {
                        Some(v) => Ok(Ready::Yes(v)),
                        None => Ok(Ready::NotYet(InProgress::LegacyMulti(collected))),
                    }
                },
                InProgress::Fountain(_) => {
                    return Err(anyhow!("Was decoding fountain qr code, and got interrupted by a legacy multi-element one."))
                },
                InProgress::LegacyMulti(mut collected) => {
                    if collected.length != length {return Err(anyhow!("Was decoding legacy multi-element qr code with {} elements, got interrupted by legacy multi-element qr code with {} elements", collected.length, length))}
                    if !collected.elements.contains(&new_element) {
                        for x in collected.elements.iter() {
                            if x.number == number {return Err(anyhow!("Encountered two legacy multi-element qr code fragments with same number."))}
                        }
                        collected.elements.push(new_element);
                        match try_legacy(&mut collected) {
                            Some(v) => Ok(Ready::Yes(v)),
                            None => Ok(Ready::NotYet(InProgress::LegacyMulti(collected))),
                        }
                    }
                    else {
                        Ok(Ready::NotYet(InProgress::LegacyMulti(collected)))
                    }
                },
            }
        }
        else {
            if let InProgress::None = decoding {Ok(Ready::Yes(payload))}
            else {return Err(anyhow!("Was reading dynamic qr, and got interrupted by a static one."))}
        }
    }
}

fn try_fountain (collected_ser_packets: &Vec<Vec<u8>>, current_decoder: &mut raptorq::Decoder) -> Option<Vec<u8>> {
    let mut result = None;
    for x in collected_ser_packets.iter() {
        result = current_decoder.decode(raptorq::EncodingPacket::deserialize(x));
    }
    result
}

fn try_legacy (collected: &mut LegacyMulti) -> Option<Vec<u8>> {
    if collected.length < collected.elements.len() as u16 {None}
    else {
        collected.elements.sort_by_key(|element| element.number);
        let mut out: Vec<u8> = Vec::new();
        for x in collected.elements.iter() {
            out.extend_from_slice(&x.contents);
        }
        Some(out)
    }
}
