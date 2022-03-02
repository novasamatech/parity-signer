use hex;
use std::convert::TryInto;
use anyhow::anyhow;
use serde_json;

pub mod process_payload;
use process_payload::{process_decoded_payload, Ready, InProgress};

//This will be a temporary fix
pub fn get_payload (line: &str, cleaned: bool) -> anyhow::Result<Vec<u8>> {
    if cleaned {
        match hex::decode(&line) {
            Ok(a) => Ok(a),
            Err(_) => return Err(anyhow!("Not hex")),
        }
    } else {
        if !line.starts_with("4") {return Err(anyhow!("Encountered unexpected qr content start"))}
        let msg_length_info = match line.get(1..5) {
            Some(a) => a,
            None => return Err(anyhow!("Too short")),
        };
        let msg_length_piece: [u8; 2] = match hex::decode(&msg_length_info) {
            Ok(a) => a.try_into().expect("constant slice size, always fits"),
            Err(_) => return Err(anyhow!("Not hex")),
        };
        let msg_length = u16::from_be_bytes(msg_length_piece) as usize;
        match line.get(5..5+msg_length*2) {
            Some(a) => match hex::decode(&a) {
                Ok(b) => Ok(b),
                Err(_) => return Err(anyhow!("Not hex")),
            },
            None => {
                // fast fix for qr codes with version below 10
                // feels abominable, change later
                let msg_length_info = match line.get(1..3) {
                    Some(a) => a,
                    None => return Err(anyhow!("Too short")),
                };
                let msg_length_piece: [u8; 1] = match hex::decode(&msg_length_info) {
                    Ok(a) => a.try_into().expect("constant slice size, always fits"),
                    Err(_) => return Err(anyhow!("Not hex")),
                };
                let msg_length = u8::from_be_bytes(msg_length_piece) as usize;
                match line.get(3..3+msg_length*2) {
                    Some(a) => match hex::decode(&a) {
                        Ok(b) => Ok(b),
                        Err(_) => return Err(anyhow!("Not hex")),
                    },
                    None => return Err(anyhow!("Length error")),
                }
            },
        }
    }
}


pub fn get_length (line: &str, cleaned: bool) -> anyhow::Result<u32> {
    
    let payload = get_payload(line, cleaned)?;
    if payload[0]&0b10000000 > 0 {
//        println!("dealing with element of fountain qr");
// Decoding algorithm is probabilistic, see documentation of the raptorq crate
// Rephrasing from there, the probability to decode message with h 
// additional packets is 1 - 1/256^(h+1).
//
// Thus, if there are no additional packets, probability is ~ 0.99609.
// If one additional packet is added, it is ~ 0.99998.
// It was decided to add one additional packet in the printed estimate, so that 
// the user expectations are lower.
        let length_piece: [u8; 4] = payload[..4].to_vec().try_into().expect("constant vector slice size, always fits");
        let length = u32::from_be_bytes(length_piece)-0x80000000;
        let new_packet = payload[4..].to_vec();
        let number_of_messages = length/(new_packet.len() as u32) + 1;
        Ok(number_of_messages)
    }
    else {
        if payload.starts_with(&[0]) {
//            println!("dealing with element of legacy dynamic multi-element qr");
            let number_of_messages_piece: [u8; 2] = payload[1..3].to_vec().try_into().expect("constant vector slice size, always fits");
            let number_of_messages = u16::from_be_bytes(number_of_messages_piece);
            Ok(number_of_messages as u32)
        }
        else {
//            println!("dealing with static qr");
            Ok(1)
        }
    }
}

pub fn decode_sequence (jsonline: &str, cleaned: bool) -> anyhow::Result<String> {
    let set: Vec<String> = match serde_json::from_str(jsonline) {
        Ok(a) => a,
        Err(_) => return Err(anyhow!("Unable to parse incoming string set")),
    };
    let mut out = Ready::NotYet(InProgress::None);
    let mut final_result: Option<String> = None;
    for x in set.iter() {
        let payload = get_payload(x, cleaned)?;
        if let Ready::NotYet(decoding) = out {
            out = process_decoded_payload (payload, decoding)?;
            if let Ready::Yes(v) = out {
                final_result = Some(hex::encode(&v));
                break;
            }
        }
    }
    match final_result {
        Some(a) => Ok(a),
        None => return Err(anyhow!("Was unable to decode on given dataset")),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn number_of_frames_fountain() {
        let line = "40438800040a400000007184c696d697473022401306576656e745f746f706963730c7533320130737461636b5f6865696768740c753332011c676c6f62616c730c7533320128706172616d65746572730c75333201306d656d6f72795f70616765730c75333201287461626c655f73697a650c753332013462725f7461626c655f73697a650c753332012c7375626a6563745f6c656e0c7533320124636f64655f73697a650c75333254496e737472756374696f6e576569676874733c543e02cc0120693634636f6e73740c753332011c6936346c6f61640c753332012069363473746f72650c753332011873656c6563740c753332010869660c753332010862720c753332011462725f69660c753332012062725f7461626c650c753332014862725f7461626c655f7065725f656e7472790c753332011063616c6c0c753332013463616c6c5f696e6469726563740c753332015c63616c6c5f696e6469726563745f7065725f706172616d0c75333201246c6f63616c5f6765740c75333201246c6f63616c5f7365740c75333201246c6f63616c5f7465650c7533320128676c6f62616c5f6765740c7533320128676c6f62616c5f7365740c75333201386d656d6f72795f63757272656e740c753332012c6d656d6f72795f67726f770c7533320118693634636c7a0c753332011869363463747a0c7533320124693634706f70636e740c753332011869363465717a0c7533320134693634657874656e64736933320c7533320134693634657874656e64756933320c7533320128693332777261706936340c753332011469363465710c75333201146936346e650c75333201186936346c74730c75333201186936346c74750c75333201186936346774730c75333201186936346774750c75333201186936346c65730c75333201186936346c65750c75333201186936346765730c75333201186936346765750c75333201186936346164640c75333201186936347375620c75333201186936346d756c0c753332011c693634646976730c753332011c693634646976750c753332011c69363472656d730c753332011c69363472656d750c7533320118693634616e640c75333201146936346f720c7533320118693634786f720c753332011869363473686c0c753332011c693634736872730c753332011c693634736872750c753332011c693634726f746c0c753332011c693634726f74720c75333240486f7374466e576569676874733c543e02b8011863616c6c657218576569676874011c616464726573731857656967687401206761735f6c65667418576569676874011c62616c616e636518576569676874014476616c75655f7472616e7366657272656418576569676874013c6d696e696d756d5f62616c616e6365185765696768740144746f6d6273746f6e655f6465706f73697418576569676874013872656e745f616c6c6f77616e6365185765696768740130626c6f636b0ec11ec11ec11ec11ec11ec";
        let length = get_length(line, false).unwrap();
        assert!(length == 16, "Expected 16, decoded {}", length);
    }
    
    #[test]
    fn bad_sequence() {
        let jsonline = r#"["400021234","400021456","400021578"]"#;
        let result = decode_sequence(jsonline, false);
        assert!(result.is_ok(), "Expected ok, {:?}", result);
    }
   
    #[test]
    fn legacy_multiframe_one_frame() {
        let jsonline = r#"["400be00000100005301025a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969a40403005a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe9690700e40b5402c5005c00ec07000004000000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafebcd1b489599db4424ed928804ddad3a4689fb8c835151ef34bc250bb845cdc1eb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe0ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec"]"#;
        let result = decode_sequence(jsonline, false);
        assert!(result.is_ok(), "Expected ok, {:?}", result);
    }
}


