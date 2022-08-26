use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::error::ErrorKind;
use nom::sequence::tuple;
use nom::IResult;

/// QR code prefix always starts with 0x4 symbol indicating "raw" encoding.
fn qr_prefix(i: &str) -> IResult<&str, &str> {
    tag("4")(i)
}

/// Subsequent N bytes encode content length. Normally N=2 but can it be 1 for old QRs.
fn length_prefixed(prefix_bytes: u8) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let (i, prefix) = take(prefix_bytes * 2)(i)?; // *2 because each byte is represented as two hex digits.
        let len_bytes = u64::from_str_radix(prefix, 16).map_err(|_| {
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
pub(crate) fn parse_payload(i: &str) -> anyhow::Result<&str> {
    let (_, (_, payload)) = tuple((qr_prefix, alt((length_prefixed(2), length_prefixed(1)))))(i)
        .map_err(|e| e.map(|e| anyhow!("Unexpected qr content: {}", e.input.to_string())))?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_payload_wrong_prefix() {
        let res = parse_payload("30001ab");
        assert!(res.is_err(), "Expected err, {:?}", res);
    }

    #[test]
    fn get_valid_payload() {
        let res = parse_payload("40001ab");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }

    #[test]
    fn get_old_version_payload() {
        let res = parse_payload("401ab");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }

    #[test]
    fn ignore_remaining() {
        let res = parse_payload("40001abf");
        assert!(res.is_ok(), "Expected ok, {:?}", res);
        assert_eq!(res.unwrap(), "ab");
    }
}
