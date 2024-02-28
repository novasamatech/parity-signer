use crate::{Error, LegacyFrame, RaptorqFrame, Result};
use banana_recovery::{NextAction, Share, ShareSet};
use raptorq::{self, EncodingPacket};
use std::{collections::HashSet, convert::TryFrom};
use db_handling::helpers::validate_mnemonic;

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

pub struct BananaRecovery {
    share_set: ShareSet,
}

#[derive(PartialEq, Eq)]
pub struct Element {
    number: u16,
    contents: Vec<u8>,
}

pub enum InProgress {
    None,
    Fountain(Fountain),
    LegacyMulti(LegacyMulti),
    BananaRecovery(BananaRecovery),
}

pub enum Ready {
    NotYet(InProgress),
    Yes(Vec<u8>),
    BananaSplitPasswordRequest,
    BananaSplitReady(String),
}

pub fn process_decoded_payload(
    payload: Vec<u8>,
    password: &Option<String>,
    mut decoding: InProgress,
) -> Result<Ready> {
    if let Ok(share) = Share::new(payload.clone()) {
        match decoding {
            InProgress::None => {
                let share_set = ShareSet::init(share);
                decoding = InProgress::BananaRecovery(BananaRecovery { share_set });
                Ok(Ready::NotYet(decoding))
            }
            InProgress::BananaRecovery(ref mut recovery) => {
                recovery.share_set.try_add_share(share)?;
                let next = recovery.share_set.next_action();
                match next {
                    NextAction::MoreShares { .. } => Ok(Ready::NotYet(decoding)),
                    NextAction::AskUserForPassword => {
                        if let Some(password) = password {
                            let result = match recovery.share_set.recover_with_passphrase(password)
                            {
                                Ok(seed) => if validate_mnemonic(&seed) {
                                    seed
                                } else {
                                    return Err(Error::InvalidMnemonic);
                                },
                                Err(banana_recovery::Error::DecodingFailed) => {
                                    return Err(Error::BananaSplitWrongPassword);
                                }
                                Err(e) => return Err(e.into()),
                            };
                            Ok(Ready::BananaSplitReady(result))
                        } else {
                            Ok(Ready::BananaSplitPasswordRequest)
                        }
                    }
                }
            }
            _ => Err(Error::DynamicInterruptedByStatic),
        }
    } else if let Ok(frame) = RaptorqFrame::try_from(payload.as_ref()) {
        let length = frame.size;
        let total = frame.total();
        let new_packet = frame.payload;
        let decoded_packet = EncodingPacket::deserialize(&new_packet);
        let block_number = decoded_packet.payload_id().encoding_symbol_id() as usize;
        match decoding {
            InProgress::None => {
                let config = raptorq::ObjectTransmissionInformation::with_defaults(
                    length as u64,
                    decoded_packet.data().len() as u16,
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
            InProgress::BananaRecovery(_) => Err(Error::LegacyInterruptedByBanana),
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
            InProgress::BananaRecovery(_) => Err(Error::LegacyInterruptedByFountain),
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
    if collected.length > collected.elements.len() as u16 {
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
