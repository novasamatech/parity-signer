#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::convert::TryFrom;

use banana_recovery::Share;
use constants::ENABLE_DYNAMIC_DERIVATIONS;

use definitions::navigation::{BananaSplitRecoveryResult, DecodeSequenceResult};

mod error;
mod parser;
pub mod process_payload;

use crate::parser::{parse_qr_payload, LegacyFrame, RaptorqFrame};
pub use error::{Error, Result};
use process_payload::{process_decoded_payload, InProgress, Ready};
use transaction_parsing::decode_payload;

pub fn get_payload(line: &str, cleaned: bool) -> Result<Vec<u8>> {
    let payload = match cleaned {
        true => line,
        false => parse_qr_payload(line)?,
    };
    Ok(hex::decode(payload)?)
}

pub fn get_length(line: &str, cleaned: bool) -> Result<u32> {
    let payload = get_payload(line, cleaned)?;
    if let Ok(frame) = RaptorqFrame::try_from(payload.as_ref()) {
        Ok(frame.total())
    } else if let Ok(frame) = LegacyFrame::try_from(payload.as_ref()) {
        Ok(frame.total as u32)
    } else if let Ok(banana_spilt_qr) = Share::new(payload) {
        Ok(banana_spilt_qr.required_shards() as u32)
    } else {
        Ok(1)
    }
}

pub fn decode_sequence(
    set: &[String],
    password: &Option<String>,
    cleaned: bool,
) -> Result<DecodeSequenceResult> {
    let mut out = Ready::NotYet(InProgress::None);
    let mut final_result: Option<String> = None;
    for x in set {
        let payload = get_payload(x, cleaned)?;
        if let Ready::NotYet(decoding) = out {
            out = process_decoded_payload(payload, password, decoding)?;
            match out {
                Ready::Yes(v) => {
                    final_result = Some(hex::encode(v));
                    break;
                }
                Ready::BananaSplitPasswordRequest => {
                    return Ok(DecodeSequenceResult::BBananaSplitRecoveryResult {
                        b: BananaSplitRecoveryResult::RequestPassword,
                    });
                }
                Ready::BananaSplitReady(s) => {
                    return Ok(DecodeSequenceResult::BBananaSplitRecoveryResult {
                        b: BananaSplitRecoveryResult::RecoveredSeed { s },
                    })
                }
                _ => (),
            }
        }
    }
    match final_result {
        Some(s) => Ok(decode_payload(&s, ENABLE_DYNAMIC_DERIVATIONS)?),
        None => Err(Error::UnableToDecode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_frames_fountain() {
        let line = "40438800040a400000007184c696d697473022401306576656e745f746f706963730c7533320130737461636b5f6865696768740c753332011c676c6f62616c730c7533320128706172616d65746572730c75333201306d656d6f72795f70616765730c75333201287461626c655f73697a650c753332013462725f7461626c655f73697a650c753332012c7375626a6563745f6c656e0c7533320124636f64655f73697a650c75333254496e737472756374696f6e576569676874733c543e02cc0120693634636f6e73740c753332011c6936346c6f61640c753332012069363473746f72650c753332011873656c6563740c753332010869660c753332010862720c753332011462725f69660c753332012062725f7461626c650c753332014862725f7461626c655f7065725f656e7472790c753332011063616c6c0c753332013463616c6c5f696e6469726563740c753332015c63616c6c5f696e6469726563745f7065725f706172616d0c75333201246c6f63616c5f6765740c75333201246c6f63616c5f7365740c75333201246c6f63616c5f7465650c7533320128676c6f62616c5f6765740c7533320128676c6f62616c5f7365740c75333201386d656d6f72795f63757272656e740c753332012c6d656d6f72795f67726f770c7533320118693634636c7a0c753332011869363463747a0c7533320124693634706f70636e740c753332011869363465717a0c7533320134693634657874656e64736933320c7533320134693634657874656e64756933320c7533320128693332777261706936340c753332011469363465710c75333201146936346e650c75333201186936346c74730c75333201186936346c74750c75333201186936346774730c75333201186936346774750c75333201186936346c65730c75333201186936346c65750c75333201186936346765730c75333201186936346765750c75333201186936346164640c75333201186936347375620c75333201186936346d756c0c753332011c693634646976730c753332011c693634646976750c753332011c69363472656d730c753332011c69363472656d750c7533320118693634616e640c75333201146936346f720c7533320118693634786f720c753332011869363473686c0c753332011c693634736872730c753332011c693634736872750c753332011c693634726f746c0c753332011c693634726f74720c75333240486f7374466e576569676874733c543e02b8011863616c6c657218576569676874011c616464726573731857656967687401206761735f6c65667418576569676874011c62616c616e636518576569676874014476616c75655f7472616e7366657272656418576569676874013c6d696e696d756d5f62616c616e6365185765696768740144746f6d6273746f6e655f6465706f73697418576569676874013872656e745f616c6c6f77616e6365185765696768740130626c6f636b0ec11ec11ec11ec11ec11ec";
        let length = get_length(line, false).unwrap();
        assert_eq!(length, 16, "Expected 16, decoded {length}");
    }

    #[test]
    fn get_length_raptorq() {
        let line = "40088800000820000000190c30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec11ec11ec11ec11ec11ec11ec11ec";
        let res = get_length(line, false);
        assert!(res.is_ok(), "{}", "Expected ok, {res:?}");
        assert_eq!(res.unwrap(), 2); // 1 + 1 extra packet
    }

    #[test]
    fn get_length_legacy_format() {
        let line = format!("40005{}{}{}{}", "00", "0009", "ff", "ff");
        let res = get_length(&line, false);
        assert!(res.is_ok(), "{}", "Expected ok, {res:?}");
        assert_eq!(res.unwrap(), 9);
    }

    #[test]
    fn get_length_static_qr() {
        let line = format!("40001{}", "01");
        let res = get_length(&line, false);
        assert!(res.is_ok(), "{}", "Expected ok, {res:?}");
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn bad_sequence() {
        let jsonline = vec![
            "40003533412".to_string(),
            "400021456".to_string(),
            "400021578".to_string(),
        ];
        let result = decode_sequence(&jsonline, &None, false);
        assert!(result.is_ok(), "{}", "Expected ok, {result:?}");
    }

    #[test]
    fn legacy_multiframe_one_frame() {
        let jsonline = vec!["400be00000100005301025a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969a40403005a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe9690700e40b5402c5005c00ec07000004000000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafebcd1b489599db4424ed928804ddad3a4689fb8c835151ef34bc250bb845cdc1eb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe0ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec".to_string()];
        let result = decode_sequence(&jsonline, &None, false);
        assert!(result.is_ok(), "{}", "Expected ok, {result:?}");
    }

    #[test]
    fn get_cleaned_payload() {
        let res = get_payload("ab", true);
        assert!(res.is_ok(), "{}", "Expected ok, {res:?}");
        assert_eq!(res.unwrap(), vec![171]);
    }
}
