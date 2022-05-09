//! Fit network properties fetched via rpc call into data in expected format
//!
//! Fetched network properties contain data needed to make
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend), used
//! in [`ContentAddSpecs`](definitions::qr_transfers::ContentAddSpecs) payload.
//!
//! Network properties may contain:
//!
//! - base58 prefix for the network (to be fit into `u16`)
//! - decimals to display balance-related values with correct order of magnitude
//! (to be fit into `u8`)
//! - unit to display balance-related values with correct units (to be fit into
//! `String`)
//!
//! There could be base58 prefix information in network metadata. If base58
//! prefix is fetched via `system_properties` rpc call and the metadata contains
//! `SS58Prefix` constant, they **must** be matching for the network to be
//! suitable for use in the Signer. If no base58 prefix is encountered at all,
//! network is not suitable for use in Signer either.
//!
//! Some networks support more than one token, in this case rpc call returns an
//! array of decimals and an array of units, of equal length. No mechanism to
//! automatically choose a token from the fetched set is currently present.
//!
//! Some other networks have no token at all.
//!
//! For networks with token set or no token, decimals default to `0` and units
//! default to `UNIT`. There could be a token override applied by user in
//! command line sequence, setting the decimals and units manually. Token
//! override is not supported for networks that have a single token, or for the
//! networks that are already in the hot database with some other token.
//!
//! Example command line with token override:
//!
//! `$ cargo run add_specs -d -u wss://network.my -sr25519 -token 10 AU`
//!
//! Note that the network could be removed from the hot database, if, for
//! example, the erroneous token was entered as an override, by running:
//!
//! `$ cargo run remove -title <network address book title>`
use definitions::{error_active::SpecsError, network_specs::NetworkProperties};
use serde_json::{map::Map, value::Value};
use std::convert::TryInto;

use crate::parser::TokenOverride;

/// Transfrom the rpc call results into [`NetworkProperties`].
///
/// Function inputs:
///
/// - `&Map<String, Value>` received via `system_properties` rpc call,
/// - optional base58 prefix from the network metadata
/// - optional token override for cases when such override is allowed
///
/// In case of success, function outputs [`NetworkProperties`].
pub fn interpret_properties(
    x: &Map<String, Value>,
    optional_prefix_from_meta: Option<u16>,
    optional_token_override: Option<TokenOverride>,
) -> Result<NetworkProperties, SpecsError> {
    // prepare to encounter token array
    //
    // if an array of decimals or units is encountered, `token_array` becomes
    // `Some(<printed array to display>, <array length>)`,
    let mut token_array = None;

    let base58prefix: u16 = match x.get("ss58Format") {
        // base58 prefix is fetched in `system_properties` rpc call
        Some(a) => match a {
            // base58 prefix value is a number
            Value::Number(b) => match b.as_u64() {
                // number is integer and could be represented as `u64` (the only
                // suitable interpretation available for `Number`)
                Some(c) => match c.try_into() {
                    // this `u64` fits into `u16` that base58 prefix is supposed
                    // to be
                    Ok(d) => match optional_prefix_from_meta {
                        // base58 prefix was found in `SS58Prefix` constant of
                        // the network metadata
                        //
                        // check that the prefixes match
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

                        // no base58 prefix was found in the network metadata
                        None => d,
                    },

                    // `u64` value does not fit into `u16` base58 prefix format,
                    // this is an error
                    Err(_) => {
                        return Err(SpecsError::Base58PrefixFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                },

                // base58 prefix value could not be presented as `u64` number,
                // this is an error
                None => {
                    return Err(SpecsError::Base58PrefixFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            },

            // base58 prefix value is not a number, this is an error
            _ => {
                return Err(SpecsError::Base58PrefixFormatNotSupported {
                    value: a.to_string(),
                })
            }
        },

        // no base58 prefix fetched in `system_properties` rpc call
        None => match optional_prefix_from_meta {
            // base58 prefix was found in `SS58Prefix` constant of the network
            // metadata
            Some(prefix_from_meta) => prefix_from_meta,

            // no base58 prefix at all, this is an error
            None => return Err(SpecsError::NoBase58Prefix),
        },
    };

    let decimals: Option<u8> = match x.get("tokenDecimals") {
        // decimals info is fetched in `system_properties` rpc call
        Some(a) => match a {
            // decimals value is a number
            Value::Number(b) => match b.as_u64() {
                // number is integer and could be represented as `u64` (the only
                // suitable interpretation available for `Number`)
                Some(c) => match c.try_into() {
                    // this `u64` fits into `u8` that decimals is supposed to be
                    Ok(d) => Some(d),

                    // this `u64` does not fit into `u8`, this is an error
                    Err(_) => {
                        return Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                },

                // number could not be represented as `u64`, this is an error
                None => {
                    return Err(SpecsError::DecimalsFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            },

            // fetched decimals is an array
            Value::Array(b) => {
                // array with only one element
                if b.len() == 1 {
                    // this element is a number
                    if let Value::Number(c) = &b[0] {
                        match c.as_u64() {
                            // number is integer and could be represented as
                            // `u64` (the only suitable interpretation available
                            // for `Number`)
                            Some(d) => match d.try_into() {
                                // this `u64` fits into `u8` that decimals is
                                // supposed to be
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
            Value::Null => None,
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
            Value::String(b) => {
                if token_array.is_some() {
                    return Err(SpecsError::DecimalsArrayUnitsNot);
                }
                if optional_token_override.is_some() {
                    return Err(SpecsError::OverrideIgnored);
                }
                Some(b.to_string())
            }
            Value::Array(b) => {
                if b.len() == 1 {
                    if let Value::String(c) = &b[0] {
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
            Value::Null => None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn map1() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!(12u8));
        mock_map.insert("tokenSymbol".to_string(), Value::String("WND".to_string()));
        let properties_maybe = interpret_properties(&mock_map, None, None);
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
    }

    #[test]
    fn map2() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!(12u8));
        mock_map.insert("tokenSymbol".to_string(), Value::String("WND".to_string()));
        let properties_error = interpret_properties(&mock_map, Some(24u16), None).unwrap_err();
        assert!(
            properties_error
                == SpecsError::Base58PrefixMismatch {
                    specs: 42,
                    meta: 24
                },
            "Wrong error in mock specs:\n{:?}",
            properties_error
        );
    }

    #[test]
    fn map3() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!([8u8, 10u8]));
        mock_map.insert(
            "tokenSymbol".to_string(),
            json!(["WND".to_string(), "NWND".to_string()]),
        );
        let properties_maybe = interpret_properties(&mock_map, None, None);
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
    }

    #[test]
    fn map4() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!([8u8]));
        mock_map.insert("tokenSymbol".to_string(), Value::String("WND".to_string()));
        let properties_maybe = interpret_properties(&mock_map, None, None);
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
    }
}
