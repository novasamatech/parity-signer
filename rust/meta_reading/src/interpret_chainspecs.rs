use serde_json::map::Map;
use jsonrpsee_types::JsonValue;
use std::convert::TryInto;
use definitions::network_specs::ChainProperties;

/// Function to interpret network properties fetched via rpc call

pub fn interpret_properties (x: &Map<String, JsonValue>) -> Result<ChainProperties, Box<dyn std::error::Error>> {
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
    Ok(ChainProperties {
        base58prefix,
        decimals,
        unit: unit.to_string(),
    })
}

