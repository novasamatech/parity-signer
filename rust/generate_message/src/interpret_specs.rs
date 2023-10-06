//! Fit network properties fetched via RPC call into data in expected format
//!
//! Fetched network properties contain data needed to make
//! [`NetworkSpecs`](definitions::network_specs::NetworkSpecs), used
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
//! prefix is fetched via `system_properties` RPC call and the metadata contains
//! `SS58Prefix` constant, they must be matching for the network to be
//! suitable for use in the Vault. If no base58 prefix is encountered at all,
//! network is not suitable for use in Vault either.
//!
//! Some networks support more than one token, in this case RPC returns an array
//! of decimals and an array of units, that must have equal length. No mechanism
//! to automatically choose a token from the fetched set is currently present.
//!
//! Some other networks have no token at all.
//!
//! For networks with token set or no token, decimals default to `0` and units
//! default to `UNIT`. For networks with token set there could be a token
//! override applied by user in command line sequence, to set the decimals and
//! units manually. Token override is not supported for networks that have a
//! single token or no token.
//!
//! Command line with token override:
//!
//! `$ cargo run add-specs -d -u <url_address> --encryption sr25519 --token-decimals <decimals> --token-unit <unit>`
use definitions::network_specs::NetworkProperties;
use serde_json::{map::Map, value::Value};
use std::convert::TryInto;

use crate::error::{Result, SpecsError};
use crate::parser::Token;

/// Transform `system_properties` RPC call results into [`NetworkProperties`].
///
/// This function is only used if the network properties are used as is, i.e.
/// without checking with existing database entries.
///
/// Function inputs:
///
/// - `&Map<String, Value>` received via `system_properties` RPC call,
/// - optional base58 prefix from the network metadata
/// - optional token override to be applied if allowed
pub fn interpret_properties(
    x: &Map<String, Value>,
    optional_prefix_from_meta: Option<u16>,
    optional_token_override: Option<Token>,
) -> Result<NetworkProperties> {
    let base58prefix = base58prefix(x, optional_prefix_from_meta)?;

    let (decimals, unit) = match token(x)? {
        TokenFetch::Single(token) => {
            // single unit value and single decimals value, override impossible
            if optional_token_override.is_some() {
                Err(SpecsError::OverrideIgnoredSingle)?;
            }
            (token.decimals, token.unit)
        }
        TokenFetch::Array { decimals, unit } => {
            if let Some(token_override) = optional_token_override {
                // token override possible and invoked by the user
                println!("Network supports several tokens. An array of tokenDecimals {} and an array of tokenSymbol {} were fetched. Through override, the decimals value will be set to {} and unit value will be set to {}. To improve this behavior, please file a ticket.", decimals, unit, token_override.decimals, token_override.unit);
                (token_override.decimals, token_override.unit)
            } else {
                // token override is possible, but not called for by the user
                println!("Network supports several tokens. An array of tokenDecimals {decimals} and an array of tokenSymbol {unit} were fetched. By default, decimals value will be set to 0, and unit value will be set to UNIT. To override, use --token-decimals <value_decimals> --token-unit <value_unit>. To improve this behavior, please file a ticket.");
                (0, String::from("UNIT"))
            }
        }
        TokenFetch::None => {
            // override impossible
            if optional_token_override.is_some() {
                Err(SpecsError::OverrideIgnoredNone)?;
            }
            println!("Network has no token. By default, decimals value will be set to 0, and unit value will be set to UNIT. To improve this behavior, please file a ticket.");
            (0, String::from("UNIT"))
        }
    };
    Ok(NetworkProperties {
        base58prefix,
        decimals,
        unit,
    })
}

/// Process base58 prefix
///
/// Function inputs:
///
/// - `&Map<String, Value>` received via `system_properties` RPC call,
/// - optional base58 prefix from the network metadata
fn base58prefix(x: &Map<String, Value>, optional_prefix_from_meta: Option<u16>) -> Result<u16> {
    let base58prefix: u16 = match x.get("ss58Format") {
        // base58 prefix is fetched in `system_properties` RPC call
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
                                Err(SpecsError::Base58PrefixMismatch {
                                    specs: d,
                                    meta: prefix_from_meta,
                                })?
                            }
                        }

                        // no base58 prefix was found in the network metadata
                        None => d,
                    },

                    // `u64` value does not fit into `u16` base58 prefix format,
                    // this is an error
                    Err(_) => Err(SpecsError::Base58PrefixFormatNotSupported {
                        value: a.to_string(),
                    })?,
                },

                // base58 prefix value could not be presented as `u64` number,
                // this is an error
                None => Err(SpecsError::Base58PrefixFormatNotSupported {
                    value: a.to_string(),
                })?,
            },

            // base58 prefix value is not a number, this is an error
            _ => Err(SpecsError::Base58PrefixFormatNotSupported {
                value: a.to_string(),
            })?,
        },

        // no base58 prefix fetched in `system_properties` RPC call
        None => match optional_prefix_from_meta {
            // base58 prefix was found in `SS58Prefix` constant of the network
            // metadata
            Some(prefix_from_meta) => prefix_from_meta,

            // no base58 prefix at all, this is an error
            None => return Err(SpecsError::NoBase58Prefix)?,
        },
    };
    Ok(base58prefix)
}

/// Accepted token from `system_properties` RPC call
pub enum TokenFetch {
    /// Single decimals parameter and single unit.
    Single(Token),

    /// Equal size arrays for decimals and units.
    ///
    /// Token override could be applied.
    Array { decimals: String, unit: String },

    /// No decimals and no unit.
    None,
}

/// Interpreted decimals
enum DecimalsFetch {
    /// Single decimals parameter.
    Single(u8),

    /// Decimals array.
    ///
    /// Associated data is printed array to display to user and array size.
    Array(String, usize),

    /// No decimals.
    None,
}

/// Interpreted unit
enum UnitFetch {
    /// Single unit parameter.
    Single(String),

    /// Units array.
    ///
    /// Associated data is printed array to display to user and array size.
    Array(String, usize),

    /// No unit.
    None,
}

/// Process decimals
///
/// Function inputs only `&Map<String, Value>` received via `system_properties`
/// RPC call.
fn decimals(x: &Map<String, Value>) -> Result<DecimalsFetch> {
    match x.get("tokenDecimals") {
        // decimals info is fetched in `system_properties` RPC call
        Some(a) => match a {
            // fetched decimals value is a number
            Value::Number(b) => match b.as_u64() {
                // number is integer and could be represented as `u64` (the only
                // suitable interpretation available for `Number`)
                Some(c) => match c.try_into() {
                    // this `u64` fits into `u8` that decimals is supposed to be
                    Ok(d) => Ok(DecimalsFetch::Single(d)),

                    // this `u64` does not fit into `u8`, this is an error
                    Err(_) => Err(SpecsError::DecimalsFormatNotSupported {
                        value: a.to_string(),
                    })?,
                },

                // number could not be represented as `u64`, this is an error
                None => Err(SpecsError::DecimalsFormatNotSupported {
                    value: a.to_string(),
                })?,
            },

            // fetched decimals is an array
            Value::Array(b) => {
                // array with only one element
                if b.len() == 1 {
                    // this element is a number, process same as
                    // `Value::Number(_)`
                    if let Value::Number(c) = &b[0] {
                        match c.as_u64() {
                            // number is integer and could be represented as
                            // `u64` (the only suitable interpretation available
                            // for `Number`)
                            Some(d) => match d.try_into() {
                                // this `u64` fits into `u8` that decimals is
                                // supposed to be
                                Ok(f) => Ok(DecimalsFetch::Single(f)),

                                // this `u64` does not fit into `u8`, this is an
                                // error
                                Err(_) => Err(SpecsError::DecimalsFormatNotSupported {
                                    value: a.to_string(),
                                })?,
                            },

                            // number could not be represented as `u64`, this is
                            // an error
                            None => Err(SpecsError::DecimalsFormatNotSupported {
                                value: a.to_string(),
                            })?,
                        }
                    } else {
                        // element is not a number, this is an error
                        Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        })?
                    }
                } else {
                    // decimals are an array with more than one element
                    Ok(DecimalsFetch::Array(a.to_string(), b.len()))
                }
            }

            // same as missing decimals
            Value::Null => Ok(DecimalsFetch::None),

            // unexpected decimals format
            _ => Err(SpecsError::DecimalsFormatNotSupported {
                value: a.to_string(),
            })?,
        },

        // decimals are missing
        None => Ok(DecimalsFetch::None),
    }
}

/// Process unit
///
/// Function inputs only `&Map<String, Value>` received via `system_properties`
/// RPC call.
fn unit(x: &Map<String, Value>) -> Result<UnitFetch> {
    match x.get("tokenSymbol") {
        // unit info is fetched in `system_properties` RPC call
        Some(a) => match a {
            // fetched unit value is a `String`
            Value::String(b) => {
                // definitive unit found
                Ok(UnitFetch::Single(b.to_string()))
            }

            // fetched an array of units
            Value::Array(b) => {
                // array with a single element
                if b.len() == 1 {
                    // single `String` element array, process same as `String`
                    if let Value::String(c) = &b[0] {
                        // definitive unit found
                        Ok(UnitFetch::Single(c.to_string()))
                    } else {
                        // element is not a `String`, this is an error
                        Err(SpecsError::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        })?
                    }
                } else {
                    // units are an array with more than one element
                    Ok(UnitFetch::Array(a.to_string(), b.len()))
                }
            }

            // same as missing unit
            Value::Null => Ok(UnitFetch::None),

            // unexpected unit format
            _ => Err(SpecsError::UnitFormatNotSupported {
                value: a.to_string(),
            })?,
        },

        // unit missing
        None => Ok(UnitFetch::None),
    }
}

/// Combine decimals and unit information
fn token(x: &Map<String, Value>) -> Result<TokenFetch> {
    let decimals_fetch = decimals(x)?;
    let unit_fetch = unit(x)?;

    match decimals_fetch {
        DecimalsFetch::Single(decimals) => match unit_fetch {
            UnitFetch::Single(unit) => Ok(TokenFetch::Single(Token { decimals, unit })),
            UnitFetch::Array(..) => Err(SpecsError::UnitsArrayDecimalsNot)?,
            UnitFetch::None => Err(SpecsError::DecimalsNoUnit(decimals.to_string()))?,
        },
        DecimalsFetch::Array(decimals, decimals_array_size) => match unit_fetch {
            UnitFetch::Single(_) => Err(SpecsError::DecimalsArrayUnitsNot)?,
            UnitFetch::Array(unit, unit_array_size) => {
                if decimals_array_size != unit_array_size {
                    Err(SpecsError::DecimalsUnitsArrayLength { decimals, unit })?
                } else {
                    Ok(TokenFetch::Array { decimals, unit })
                }
            }
            UnitFetch::None => Err(SpecsError::DecimalsNoUnit(decimals))?,
        },
        DecimalsFetch::None => match unit_fetch {
            UnitFetch::Single(unit) => Err(SpecsError::UnitNoDecimals(unit))?,
            UnitFetch::Array(unit, _) => Err(SpecsError::UnitNoDecimals(unit))?,
            UnitFetch::None => Ok(TokenFetch::None),
        },
    }
}

/// Get from `system_properties` RPC call results the network data to be
/// compared with already known data.
///
/// This function is used if the fetch results are used to check already
/// existing database entry.
///
/// Function inputs:
///
/// - `&Map<String, Value>` received via `system_properties` RPC call,
/// - optional base58 prefix from the network metadata
///
/// Function outputs:
///
/// - network base58 prefix
/// - [`TokenFetch`]
pub fn check_specs(
    x: &Map<String, Value>,
    optional_prefix_from_meta: Option<u16>,
) -> Result<(u16, TokenFetch)> {
    let base58prefix = base58prefix(x, optional_prefix_from_meta)?;
    let token_fetch = token(x)?;
    Ok((base58prefix, token_fetch))
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::*;
    use serde_json::json;
    #[test]
    /// Good network properties
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
    /// Mismatch in base58 prefix
    fn map2() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!(12u8));
        mock_map.insert("tokenSymbol".to_string(), Value::String("WND".to_string()));
        let properties_error = interpret_properties(&mock_map, Some(24u16), None).unwrap_err();
        if let Error::Specs(SpecsError::Base58PrefixMismatch {
            specs: 42,
            meta: 24,
        }) = properties_error
        {
        } else {
            panic!("Wrong error in mock specs:\n{:?}", properties_error);
        }
    }

    #[test]
    /// Can not override token for case when single token is fetched
    fn map3() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!(12u8));
        mock_map.insert("tokenSymbol".to_string(), Value::String("WND".to_string()));
        let properties_error = interpret_properties(
            &mock_map,
            None,
            Some(Token {
                decimals: 12,
                unit: "WND".to_string(),
            }),
        )
        .unwrap_err();
        if let Error::Specs(SpecsError::OverrideIgnoredSingle) = properties_error {
        } else {
            panic!("Wrong error in mock specs:\n{:?}", properties_error);
        }
    }

    #[test]
    /// Token array, equal length
    fn map4() {
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
        let properties = properties_maybe.unwrap();
        assert!(
            properties.decimals == 0,
            "Wrong decimals, got: {}",
            properties.decimals
        );
        assert!(
            properties.unit == "UNIT",
            "Wrong unit, got: {}",
            properties.unit
        );
    }

    #[test]
    /// Can override token for case when array of equal length is fetched
    ///
    /// Override not necessarily matches whatever was offered initially.
    fn map5() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!([8u8, 10u8]));
        mock_map.insert(
            "tokenSymbol".to_string(),
            json!(["WND".to_string(), "NWND".to_string()]),
        );
        let properties_maybe = interpret_properties(
            &mock_map,
            None,
            Some(Token {
                decimals: 12,
                unit: "WND".to_string(),
            }),
        );
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
        let properties = properties_maybe.unwrap();
        assert!(
            properties.decimals == 12,
            "Wrong decimals, got: {}",
            properties.decimals
        );
        assert!(
            properties.unit == "WND",
            "Wrong unit, got: {}",
            properties.unit
        );
    }

    #[test]
    /// Decimals value is array with single entry, unit is normal entry
    fn map6() {
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

    #[test]
    /// Decimals value is normal entry, unit is array with single entry
    fn map7() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!(8u8));
        mock_map.insert("tokenSymbol".to_string(), json!(["WND".to_string()]));
        let properties_maybe = interpret_properties(&mock_map, None, None);
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
    }

    #[test]
    /// Fetched decimals array and units array, with different length
    fn map8() {
        let mut mock_map = Map::with_capacity(3);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        mock_map.insert("tokenDecimals".to_string(), json!([8u8, 8u8]));
        mock_map.insert(
            "tokenSymbol".to_string(),
            json!(["Unknown".to_string(), "WND".to_string(), "NWND".to_string()]),
        );
        let properties_error = interpret_properties(&mock_map, None, None).unwrap_err();
        if let Error::Specs(SpecsError::DecimalsUnitsArrayLength { decimals, unit }) =
            properties_error
        {
            assert_eq!(decimals, "[8,8]");
            assert_eq!(unit, "[\"Unknown\",\"WND\",\"NWND\"]");
        } else {
            panic!("Wrong error in mock specs:\n{:?}", properties_error);
        }
    }

    #[test]
    /// No decimals and no units
    fn map9() {
        let mut mock_map = Map::with_capacity(1);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        let properties_maybe = interpret_properties(&mock_map, None, None);
        assert!(
            properties_maybe.is_ok(),
            "Error getting mock specs:\n{:?}",
            properties_maybe.unwrap_err()
        );
        let properties = properties_maybe.unwrap();
        assert!(
            properties.decimals == 0,
            "Wrong decimals, got: {}",
            properties.decimals
        );
        assert!(
            properties.unit == "UNIT",
            "Wrong unit, got: {}",
            properties.unit
        );
    }

    #[test]
    /// No decimals and no units, try override
    fn map10() {
        let mut mock_map = Map::with_capacity(1);
        mock_map.insert("ss58Format".to_string(), json!(42u16));
        let properties_error = interpret_properties(
            &mock_map,
            None,
            Some(Token {
                decimals: 12,
                unit: "WND".to_string(),
            }),
        )
        .unwrap_err();
        if let Error::Specs(SpecsError::OverrideIgnoredNone) = properties_error {
        } else {
            panic!("Wrong error in mock specs:\n{:?}", properties_error);
        }
    }
}
