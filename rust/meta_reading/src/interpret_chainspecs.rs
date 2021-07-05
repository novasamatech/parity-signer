use serde_json::map::Map;
use jsonrpsee_types::JsonValue;
use std::convert::TryInto;
use parity_scale_codec::Encode;
use parity_scale_codec_derive;

#[derive(Debug, parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)]
pub struct ChainSpecsShort {
    base58prefix: u8,
    decimals: u8,
    genesis_hash: [u8; 32],
    name: String,
    unit: String,
}

pub fn interpret_chainspecs <'a> (x: &'a Map<String, JsonValue>, genesis_hash: [u8; 32], name: &'a str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let base58prefix: u8 = match x.get("ss58Format") {
        Some(a) => {
            match a {
                JsonValue::Number(b) => {
                    match b.as_u64() {
                        Some(c) => {
                            c.try_into()?
                        },
                        None => return Err(Box::from("Expected base58prefix to convert into u64.")),
                    }
                },
                _ => return Err(Box::from("Unsupported base58prefix format.")),
            }
        },
        None => return Err(Box::from("No base58prefix fetched.")),
    };
    let decimals: u8 = match x.get("tokenDecimals") {
        Some(a) => {
            match a {
                JsonValue::Number(b) => {
                    match b.as_u64() {
                        Some(c) => {
                            c.try_into()?
                        },
                        None => return Err(Box::from("Expected decimals to convert into u64.")),
                    }
                },
                _ => return Err(Box::from("Unsupported decimals format.")),
            }
        },
        None => return Err(Box::from("No decimals fetched.")),
    };
    let unit = match x.get("tokenSymbol") {
        Some(a) => {
            match a {
                JsonValue::String(b) => b,
                _ => return Err(Box::from("Unsupported unit format.")),
            }
        },
        None => return Err(Box::from("No unit fetched.")),
    };

    Ok(ChainSpecsShort{
        base58prefix,
        decimals,
        genesis_hash,
        name: name.to_string(),
        unit: unit.to_string(),
    }.encode())

}
