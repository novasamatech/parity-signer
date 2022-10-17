use crate::{Error, LegacyFrame, RaptorqFrame, Result};
use constants::CHUNK_SIZE;
use raptorq::{self, EncodingPacket};
use std::{collections::HashSet, convert::TryFrom};

#[derive(PartialEq, Eq)]
pub struct Fountain {
    decoder: raptorq::Decoder,
    length: u32,
    pub total: u32,
    collected: HashSet<usize>,
}

impl Fountain {
    /// Return the number of packets collected.
    pub fn collected(&self) -> usize {
        self.collected.len()
    }

    /// Called to inform that the packet with the id has been collected.
    pub fn collect(&mut self, frame_index: usize) {
        self.collected.insert(frame_index);
    }
}

#[derive(PartialEq, Eq)]
pub struct LegacyMulti {
    length: u16,
    elements: Vec<Element>,
}

#[derive(PartialEq, Eq)]
pub struct Element {
    number: u16,
    contents: Vec<u8>,
}

#[derive(PartialEq, Eq)]
pub enum InProgress {
    None,
    Fountain(Fountain),
    LegacyMulti(LegacyMulti),
}

#[derive(PartialEq, Eq)]
pub enum Ready {
    NotYet(InProgress),
    Yes(Vec<u8>),
}

pub fn process_decoded_payload(payload: Vec<u8>, mut decoding: InProgress) -> Result<Ready> {
    if let Ok(frame) = RaptorqFrame::try_from(payload.as_ref()) {
        let length = frame.size;
        let total = frame.total();
        let new_packet = frame.payload;
        let decoded_packet = EncodingPacket::deserialize(&new_packet);
        let block_number = decoded_packet.payload_id().encoding_symbol_id() as usize;
        match decoding {
            InProgress::None => {
                let config = raptorq::ObjectTransmissionInformation::with_defaults(
                    length as u64,
                    CHUNK_SIZE,
                );
                let mut decoder = raptorq::Decoder::new(config);
                match try_fountain(decoded_packet, &mut decoder) {
                    Some(v) => Ok(Ready::Yes(v)),
                    None => {
                        let mut in_progress = Fountain {
                            decoder,
                            length,
                            total,
                            collected: HashSet::new(),
                        };
                        in_progress.collect(block_number);
                        decoding = InProgress::Fountain(in_progress);
                        Ok(Ready::NotYet(decoding))
                    }
                }
            }
            InProgress::Fountain(mut in_progress) => {
                if in_progress.length != length {
                    return Err(Error::ConflictingPayloads(in_progress.length, length));
                }

                in_progress.collect(block_number);
                match try_fountain(decoded_packet, &mut in_progress.decoder) {
                    Some(v) => Ok(Ready::Yes(v)),
                    None => Ok(Ready::NotYet(InProgress::Fountain(in_progress))),
                }
            }
            InProgress::LegacyMulti(_) => Err(Error::LegacyInterruptedByFountain),
        }
    } else if let Ok(frame) = LegacyFrame::try_from(payload.as_ref()) {
        let length = frame.total;
        let number = frame.index;
        if number >= length {
            return Err(Error::LengthExceeded);
        }
        let contents = frame.data;
        let new_element = Element { number, contents };
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
            }
            InProgress::Fountain(_) => Err(Error::FountainInterruptedByLegacy),
            InProgress::LegacyMulti(mut collected) => {
                if collected.length != length {
                    return Err(Error::ConflictingLegacyLengths(collected.length, length));
                }
                if !collected.elements.contains(&new_element) {
                    for x in collected.elements.iter() {
                        if x.number == number {
                            return Err(Error::SameNumber);
                        }
                    }
                    collected.elements.push(new_element);
                    match try_legacy(&mut collected) {
                        Some(v) => Ok(Ready::Yes(v)),
                        None => Ok(Ready::NotYet(InProgress::LegacyMulti(collected))),
                    }
                } else {
                    Ok(Ready::NotYet(InProgress::LegacyMulti(collected)))
                }
            }
        }
    } else if let InProgress::None = decoding {
        Ok(Ready::Yes(payload))
    } else {
        Err(Error::DynamicInterruptedByStatic)
    }
}

fn try_fountain(packet: EncodingPacket, decoder: &mut raptorq::Decoder) -> Option<Vec<u8>> {
    decoder.add_new_packet(packet);
    decoder.get_result()
}

fn try_legacy(collected: &mut LegacyMulti) -> Option<Vec<u8>> {
    if collected.length < collected.elements.len() as u16 {
        None
    } else {
        collected.elements.sort_by_key(|element| element.number);
        let mut out: Vec<u8> = Vec::new();
        for x in collected.elements.iter() {
            out.extend_from_slice(&x.contents);
        }
        Some(out)
    }
}
