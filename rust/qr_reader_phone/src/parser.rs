use nom::bits::complete::take as bit_take;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use std::convert::TryFrom;

use nom::combinator::{rest, verify};
use nom::complete::tag as bit_tag;
use nom::error::ErrorKind;
use nom::number::complete::be_u16;
use nom::sequence::{preceded, tuple};
use nom::{bits, IResult};

use crate::{Error, Result};

pub(crate) struct RaptorqFrame {
    // size of the original message in bytes
    pub(crate) size: u32,

    // current frame payload
    pub(crate) payload: Vec<u8>,
}

impl RaptorqFrame {
    // https://github.com/cberner/raptorq/blob/afe2e83206efdc90c3da614e381bf2884994d844/src/base.rs#L77
    const HEADER_SIZE: u32 = 4;

    // total number of frames needed to reconstruct the original message
    pub(crate) fn total(&self) -> u32 {
        // Decoding algorithm is probabilistic, see documentation of the `raptorq` crate
        // Rephrasing from there, the probability to decode message with h
        // additional packets is 1 - 1/256^(h+1).
        //
        // Thus, if there are no additional packets, probability is ~ 0.99609.
        // If one additional packet is added, it is ~ 0.99998.
        // It was decided to add one additional packet in the printed estimate, so that
        // the user expectations are lower.
        self.size
            .checked_div(self.payload.len() as u32 - Self::HEADER_SIZE)
            .unwrap_or_default()
            + 1
    }
}

impl TryFrom<&[u8]> for RaptorqFrame {
    type Error = Error;

    fn try_from(i: &[u8]) -> Result<Self> {
        let (_, (size, payload)) =
            parse_raptor_frame(i).map_err(|e| Error::RaptorqFrame(e.to_string()))?;
        Ok(Self {
            size,
            payload: payload.to_vec(),
        })
    }
}

pub(crate) struct LegacyFrame {
    pub(crate) total: u16,
    pub(crate) index: u16,
    pub(crate) data: Vec<u8>,
}

impl TryFrom<&[u8]> for LegacyFrame {
    type Error = Error;

    fn try_from(i: &[u8]) -> Result<Self> {
        let (_, (total, index, data)) =
            parse_legacy_frame(i).map_err(|e| Error::LegacyFrame(e.to_string()))?;
        Ok(Self {
            total,
            index,
            data: data.to_vec(),
        })
    }
}

/// QR code prefix always starts with `0x4` symbol indicating "raw" encoding.
fn qr_prefix(i: &str) -> IResult<&str, &str> {
    tag("4")(i)
}

/// Subsequent N bytes encode content length. Normally N=2 but can it be 1 for old QRs.
fn length_prefixed(prefix_bytes: u8) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let (i, prefix) = take(prefix_bytes * 2)(i)?; // *2 because each byte is represented as two hex digits.
        let len_bytes = usize::from_str_radix(prefix, 16).map_err(|_| {
            nom::Err::Error(nom::error::Error {
                input: "Not hex",
                code: ErrorKind::HexDigit,
            })
        })?;
        let (_, payload) = take(len_bytes * 2)(i)?;
        Ok((i, payload))
    }
}

/// Parse QR code envelope and return payload.
pub(crate) fn parse_qr_payload(i: &str) -> Result<&str> {
    let (_, payload) = preceded(qr_prefix, alt((length_prefixed(2), length_prefixed(1))))(i)
        .map_err(|e| Error::UnexpectedData(e.to_string()))?;
    Ok(payload)
}

fn raptorq_tag(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    bit_tag(0x1, 1usize)(i)
}

fn raptor_payload_size(i: (&[u8], usize)) -> IResult<(&[u8], usize), u32> {
    bit_take(31usize)(i)
}

fn parse_raptor_frame(i: &[u8]) -> IResult<&[u8], (u32, &[u8])> {
    tuple((
        bits(preceded(raptorq_tag, raptor_payload_size)),
        verify(rest, |a: &[u8]| !a.is_empty()),
    ))(i)
}

fn legacy_tag(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(&[0])(i)
}

fn parse_legacy_frame(i: &[u8]) -> IResult<&[u8], (u16, u16, &[u8])> {
    tuple((preceded(legacy_tag, be_u16), be_u16, rest))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_payload_wrong_prefix() {
        let res = parse_qr_payload("30001ab");
        assert!(res.is_err(), "Expected err, {:?}", res);
    }

    #[test]
    fn get_valid_payload() {
        let res = parse_qr_payload("40001ab");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }

    #[test]
    fn get_old_version_payload() {
        let res = parse_qr_payload("401ab");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }

    #[test]
    fn ignore_remaining() {
        let res = parse_qr_payload("40001abf");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }
}
