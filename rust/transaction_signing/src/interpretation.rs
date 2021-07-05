use regex::Regex;
use lazy_static::lazy_static;


// Making lazy statics for regex interpreting input action string

lazy_static! {
    static ref REG_CHECKSUM: Regex = Regex::new(r#"(?i)"checksum":( )*"(?P<checksum>[0-9]*)""#).expect("constructed from checked static value");
    static ref REG_ACTION: Regex = Regex::new(r#"(?i)"type":( )*"(?P<action_type>.*?)""#).expect("constructed from checked static value");
}


/// Function for integrity check of action line returned from RN.
/// In case of success produces u32 checksum for database.

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


pub enum ActionType {
    SignTransaction,
    LoadMetadata,
    AddMetadataVerifier,
    LoadTypes,
    AddTypesVerifier,
}

/// Function to determine the action type of incoming action line
/// returned from RN.
/// In case of success produces ActionType enum value.

pub fn get_action_type (action_line: &str) -> Result<ActionType, &'static str> {
    let action_type = match REG_ACTION.captures(&action_line) {
        Some(caps) => {
            match caps.name("action_type") {
                Some(c) => {
                    match c.as_str() {
                        "sign_transaction" => ActionType::SignTransaction,
                        "load_metadata" => ActionType::LoadMetadata,
                        "add_metadata_verifier" => ActionType::AddMetadataVerifier,
                        "load_types" => ActionType::LoadTypes,
                        "add_types_verifier" => ActionType::AddTypesVerifier,
                        _ => return Err("Action type not supported."),
                    }
                },
                None => return Err("Action type missing."),
            }
        },
        None => return Err("Action type missing."),
    };
    Ok(action_type)
}
