use serde_json::map::Map;
use jsonrpsee_types::JsonValue;
use std::convert::TryInto;
use definitions::{error::SpecsError, network_specs::NetworkProperties};

use crate::parser::TokenOverride;

/// Function to interpret network properties fetched via rpc call

pub fn interpret_properties (x: &Map<String, JsonValue>, optional_prefix_from_meta: Option<u16>, optional_token_override: Option<TokenOverride>) -> Result<NetworkProperties, SpecsError> {
    let mut token_array = None;
    let base58prefix: u16 = match x.get("ss58Format") {
        Some(a) => {
            match a {
                JsonValue::Number(b) => {
                    match b.as_u64() {
                        Some(c) => {
                            match c.try_into() {
                                Ok(d) => {
                                    match optional_prefix_from_meta {
                                        Some(prefix_from_meta) => {
                                            if prefix_from_meta == d {d}
                                            else {return Err(SpecsError::Base58PrefixMismatch{specs: d, meta: prefix_from_meta})}
                                        },
                                        None => d,
                                    }
                                },
                                Err(_) => return Err(SpecsError::Base58PrefixFormatNotSupported{value: a.to_string()}),
                            }
                        },
                        None => return Err(SpecsError::Base58PrefixFormatNotSupported{value: a.to_string()}),
                    }
                },
                _ => return Err(SpecsError::Base58PrefixFormatNotSupported{value: a.to_string()}),
            }
        },
        None => {
            match optional_prefix_from_meta {
                Some(prefix_from_meta) => prefix_from_meta,
                None => return Err(SpecsError::NoBase58Prefix),
            }
        },
    };
    let decimals: u8 = match x.get("tokenDecimals") {
        Some(a) => {
            match a {
                JsonValue::Number(b) => {
                    match b.as_u64() {
                        Some(c) => {
                            match c.try_into() {
                                Ok(d) => d,
                                Err(_) => return Err(SpecsError::DecimalsFormatNotSupported{value: a.to_string()}),
                            }
                        },
                        None => return Err(SpecsError::DecimalsFormatNotSupported{value: a.to_string()}),
                    }
                },
                JsonValue::Array(b) => {
                    token_array = Some((a, b.len()));
                    if let Some(ref token_override) = optional_token_override {
                        token_override.decimals
                    }
                    else {0}
                },
                _ => return Err(SpecsError::DecimalsFormatNotSupported{value: a.to_string()}),
            }
        },
        None => return Err(SpecsError::NoDecimals),
    };
    let unit = match x.get("tokenSymbol") {
        Some(a) => {
            match a {
                JsonValue::String(b) => {
                    if let Some(_) = token_array {return Err(SpecsError::DecimalsArrayUnitsNot)}
                    if let Some(_) = optional_token_override {return Err(SpecsError::OverrideIgnored)}
                    b.to_string()
                },
                JsonValue::Array(b) => {
                    match token_array {
                        Some((decimals, decimals_len)) => {
                            if decimals_len != b.len() {return Err(SpecsError::DecimalsUnitsArrayLength{decimals_len, unit_len: b.len()})}
                            else {
                                if let Some(token_override) = optional_token_override {
                                    println!("Network supports several tokens. An array of tokenDecimals {} and an array of tokenSymbol {} were fetched. Through override, the decimals value will be set to {} and unit value will be set to {}. To improve this behavior, please file a ticket.", decimals.to_string(), a.to_string(), token_override.decimals, token_override.unit);
                                    token_override.unit
                                }
                                else {
                                    println!("Network supports several tokens. An array of tokenDecimals {} and an array of tokenSymbol {} were fetched. By default, decimals value will be set to 0, and unit value will be set to UNIT. To override, use -token <value_decimals> <value_unit>. To improve this behavior, please file a ticket.", decimals.to_string(), a.to_string());
                                    String::from("UNIT")
                                }
                            }
                        },
                        None => {return Err(SpecsError::UnitsArrayDecimalsNot)},
                    }
                },
                _ => return Err(SpecsError::UnitFormatNotSupported{value: a.to_string()}),
            }
        },
        None => return Err(SpecsError::NoUnit),
    };
    Ok(NetworkProperties {
        base58prefix,
        decimals,
        unit,
    })
}

