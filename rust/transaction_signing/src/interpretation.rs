use regex::Regex;
use lazy_static::lazy_static;


// Making lazy statics for regex interpreting input action string

lazy_static! {
    static ref REG_CHECKSUM: Regex = Regex::new(r#"(?i)"checksum":( )*"(?P<checksum>[0-9]*)""#).unwrap();
}

pub fn get_checksum (action_line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let checksum: u32 = match REG_CHECKSUM.captures(&action_line) {
        Some(caps) => {
            match caps.name("checksum") {
                Some(c) => c.as_str().parse()?,
                None => {return Err(Box::from("Checksum missing."))}
            }
        },
        None => {return Err(Box::from("Checksum missing."))},
    };
    Ok(checksum)
}


