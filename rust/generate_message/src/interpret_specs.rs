use definitions::{error::SpecsError, network_specs::NetworkProperties};
use jsonrpsee_types::JsonValue;
use serde_json::map::Map;
use std::convert::TryInto;

use crate::parser::TokenOverride;

/// Function to interpret network properties fetched via rpc call

pub fn interpret_properties(
    x: &Map<String, JsonValue>,
    optional_prefix_from_meta: Option<u16>,
    optional_token_override: Option<TokenOverride>,
) -> Result<NetworkProperties, SpecsError> {
    let mut token_array = None;
    let base58prefix: u16 = match x.get("ss58Format") {
        Some(a) => match a {
            JsonValue::Number(b) => match b.as_u64() {
                Some(c) => match c.try_into() {
                    Ok(d) => match optional_prefix_from_meta {
                        Some(prefix_from_meta) => {
                            if prefix_from_meta == d {
                                d
                            } else {
                                return Err(SpecsError::Base58PrefixMismatch {
                                    specs: d,
                                    meta: prefix_from_meta,
                                });
                            }
                        }
                        None => d,
                    },
                    Err(_) => {
                        return Err(SpecsError::Base58PrefixFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                },
                None => {
                    return Err(SpecsError::Base58PrefixFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            },
            _ => {
                return Err(SpecsError::Base58PrefixFormatNotSupported {
                    value: a.to_string(),
                })
            }
        },
        None => match optional_prefix_from_meta {
            Some(prefix_from_meta) => prefix_from_meta,
            None => return Err(SpecsError::NoBase58Prefix),
        },
    };
    let decimals: Option<u8> = match x.get("tokenDecimals") {
        Some(a) => match a {
            JsonValue::Number(b) => match b.as_u64() {
                Some(c) => match c.try_into() {
                    Ok(d) => Some(d),
                    Err(_) => {
                        return Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                },
                None => {
                    return Err(SpecsError::DecimalsFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            },
            JsonValue::Array(b) => {
                if b.len() == 1 {
                    if let JsonValue::Number(c) = &b[0] {
                        match c.as_u64() {
                            Some(d) => match d.try_into() {
                                Ok(f) => Some(f),
                                Err(_) => {
                                    return Err(SpecsError::DecimalsFormatNotSupported {
                                        value: a.to_string(),
                                    })
                                }
                            },
                            None => {
                                return Err(SpecsError::DecimalsFormatNotSupported {
                                    value: a.to_string(),
                                })
                            }
                        }
                    } else {
                        return Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        });
                    }
                } else {
                    token_array = Some((a.to_string(), b.len()));
                    if let Some(ref token_override) = optional_token_override {
                        Some(token_override.decimals)
                    } else {
                        Some(0)
                    }
                }
            }
            JsonValue::Null => None,
            _ => {
                return Err(SpecsError::DecimalsFormatNotSupported {
                    value: a.to_string(),
                })
            }
        },
        None => None,
    };
    let unit = match x.get("tokenSymbol") {
        Some(a) => match a {
            JsonValue::String(b) => {
                if token_array.is_some() {
                    return Err(SpecsError::DecimalsArrayUnitsNot);
                }
                if optional_token_override.is_some() {
                    return Err(SpecsError::OverrideIgnored);
                }
                Some(b.to_string())
            }
            JsonValue::Array(b) => {
                if b.len() == 1 {
                    if let JsonValue::String(c) = &b[0] {
                        if token_array.is_some() {
                            return Err(SpecsError::DecimalsArrayUnitsNot);
                        }
                        if optional_token_override.is_some() {
                            return Err(SpecsError::OverrideIgnored);
                        }
                        Some(c.to_string())
                    } else {
                        return Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        });
                    }
                } else {
                    match token_array {
                        Some((decimals, decimals_len)) => {
                            if decimals_len != b.len() {
                                return Err(SpecsError::DecimalsUnitsArrayLength {
                                    decimals,
                                    unit: a.to_string(),
                                });
                            } else if let Some(token_override) = optional_token_override {
                                println!("Network supports several tokens. An array of tokenDecimals {} and an array of tokenSymbol {} were fetched. Through override, the decimals value will be set to {} and unit value will be set to {}. To improve this behavior, please file a ticket.", decimals, a, token_override.decimals, token_override.unit);
                                Some(token_override.unit)
                            } else {
                                println!("Network supports several tokens. An array of tokenDecimals {} and an array of tokenSymbol {} were fetched. By default, decimals value will be set to 0, and unit value will be set to UNIT. To override, use -token <value_decimals> <value_unit>. To improve this behavior, please file a ticket.", decimals, a);
                                Some(String::from("UNIT"))
                            }
                        }
                        None => return Err(SpecsError::UnitsArrayDecimalsNot),
                    }
                }
            }
            JsonValue::Null => None,
            _ => {
                return Err(SpecsError::UnitFormatNotSupported {
                    value: a.to_string(),
                })
            }
        },
        None => None,
    };
    let (decimals, unit) = match decimals {
        Some(a) => match unit {
            Some(b) => (a, b),
            None => return Err(SpecsError::DecimalsNoUnit(a)),
        },
        None => match unit {
            Some(b) => return Err(SpecsError::UnitNoDecimals(b)),
            None => {
                println!("Network has no token. By default, decimals value will be set to 0, and unit value will be set to UNIT. To improve this behavior, please file a ticket.");
                (0, String::from("UNIT"))
            }
        },
    };
    Ok(NetworkProperties {
        base58prefix,
        decimals,
        unit,
    })
}
