use anyhow;
use regex::Regex;
use lazy_static::lazy_static;

use transaction_parsing::cards::Action;
use super::error::{Error, Interpretation};

// Making lazy statics for regex interpreting input action string

lazy_static! {
    static ref REG_READ: Regex = Regex::new(r#"(?i)"type":( )*"(?P<action_type>.*?)","checksum":( )*"(?P<checksum>[0-9]*)""#).expect("constructed from checked static value");
}


/// Function to determine the action type and the corresponding checksum
/// for integrity check of action line returned from RN.
/// In case of success produces Action enum.

pub fn interpret_action (action_line: &str) -> anyhow::Result<Action> {
    match REG_READ.captures(&action_line) {
        Some(caps) => {
            let checksum: u32 = match caps.name("checksum") {
                Some(c) => match c.as_str().parse() {
                    Ok(a) => a,
                    Err(_) => return Err(Error::Interpretation(Interpretation::ChecksumNotU32).show()),
                },
                None => return Err(Error::Interpretation(Interpretation::ChecksumMissing).show()),
            };
            match caps.name("action_type") {
                Some(c) => {
                    match c.as_str() {
                        "sign_transaction" => Ok(Action::SignTransaction(checksum)),
                        "load_metadata" => Ok(Action::LoadMetadata(checksum)),
                        "add_metadata_verifier" => Ok(Action::AddMetadataVerifier(checksum)),
                        "load_types" => Ok(Action::LoadTypes(checksum)),
                        "add_general_verifier" => Ok(Action::AddGeneralVerifier(checksum)),
                        "add_two_verifiers" => Ok(Action::AddTwoVerifiers(checksum)),
                        "load_metadata_and_add_general_verifier" => Ok(Action::LoadMetadataAndAddGeneralVerifier(checksum)),
                        "add_network" => Ok(Action::AddNetwork(checksum)),
                        "add_network_and_add_general_verifier" => Ok(Action::AddNetworkAndAddGeneralVerifier(checksum)),
                        _ => return Err(Error::Interpretation(Interpretation::UnsupportedAction).show()),
                    }
                },
                None => return Err(Error::Interpretation(Interpretation::ActionMissing).show()),
            }
        },
        None => return Err(Error::Interpretation(Interpretation::BadActionLine).show()),
    }
}

