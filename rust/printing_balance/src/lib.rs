//! This crate is part of transcation parsing in the
//! [Signer](https://github.com/paritytech/parity-signer).
//!
//! This crate is used to represent numbers as balance values in given network.
//! Every network introduced to Signer has characteristic network specs. Among
//! other parameters, network specs contain decimals and unit, that are used
//! here to represent integer numbers from transactions as an actual balance
//! values in the network units.
//!
//! Decimals indicate the order of magnitude, by which the token `unit`
//! exceeds the integer representing unit. All symbols from the input number
//! except leading zeroes must end up in the representation.
//!
//! <code>balance = integer_input &times; 10<sup>(-decimals)</sup></code>
//!
//! To make displayed numbers more readable, prefixes, such as milli-, micro-,
//! nano- etc, are used.
//!
//! <table>
//!     <tr>
//!         <th>decoded integer</th>
//!         <th>decimals</th>
//!         <th>unit</th>
//!         <th>correct displaying</th>
//!     </tr>
//!     <tr><td>1</td><td>12</td><td>WND</td><td>1 pWND</td></tr>
//!     <tr><td>10</td><td>12</td><td>WND</td><td>10 pWND</td></tr>
//!     <tr><td>100</td><td>12</td><td>WND</td><td>100 pWND</td></tr>
//!     <tr><td>1000</td><td>12</td><td>WND</td><td>1.000 nWND</td></tr>
//!     <tr><td>1000000</td><td>12</td><td>WND</td><td>1.000000 uWND</td></tr>
//!     <tr><td>1000000000</td><td>12</td><td>WND</td><td>1.000000000 mWND</td></tr>
//! </table>
//!
//! Balance-representing integers could have different types as determined by
//! the network metadata. The trait [`AsBalance`] generalizes balance
//! formatting, and is implemented here for `u8`, `u16`, `u32`, `u64` and
//! `u128`.
//!
//! ## Examples
//! ```
//! use printing_balance::AsBalance;
//!
//! let balance = <u128>::convert_balance_pretty(1, 12, "WND");
//! assert!(balance.number == "1");
//! assert!(balance.units == "pWND");
//!
//! let balance = <u32>::convert_balance_pretty(1000000, 12, "WND");
//! assert!(balance.number == "1.000000");
//! assert!(balance.units == "uWND");
//!
//! let balance = <u64>::convert_balance_pretty(0, 12, "WND");
//! assert!(balance.number == "0");
//! assert!(balance.units == "pWND");
//!
//! let balance = <u128>::convert_balance_pretty(123456000123, 12, "WND");
//! assert!(balance.number == "123.456000123");
//! assert!(balance.units == "mWND");
//!
//! let balance = <u64>::convert_balance_pretty(0, 14, "SMTH");
//! assert!(balance.number == "0.00");
//! assert!(balance.units == "pSMTH");
//! ```
//!
//! This crate **only formats** the data for output as text, it is not expected
//! that any operations will be performed on the values except displaying them.
#![deny(unused_crate_dependencies)]
/// Trait for correct displaying of balance-related values.
pub trait AsBalance {
    /// Represent numerical value as a balance.
    fn convert_balance_pretty(value: Self, decimals: u8, unit: &str) -> PrettyOutput;
}

/// Implement [`AsBalance`] for all reasonable input types.
macro_rules! impl_balance {
    ($($uint_type: ty), *) => {
        $(
            impl AsBalance for $uint_type {
                fn convert_balance_pretty(value: $uint_type, decimals: u8, unit: &str) -> PrettyOutput {
                    convert_balance_string(&value.to_string(), decimals, unit)
                }
            }
        )*
    }
}

impl_balance!(u8, u16, u32, u64, u128);

/// String-represented input cut in parts.
struct CutNumber {
    /// Integer part
    before_point: String,

    /// Fractional part
    after_point: Option<String>,

    /// Order of magnitude modifier, for optimal prefix index.
    ///
    /// <code>balance [unit with prefix] = balance \[unit\] &times; 10<sup>(-3 &times; mag)</sup></code>
    ///
    /// <table>
    ///     <tr><th>mag</th><th colspan="2">corresponding unit prefix</th>
    ///     <tr><td>-4</td><td>tera-</td><td>T</td></tr>
    ///     <tr><td>-3</td><td>giga-</td><td>G</td></tr>
    ///     <tr><td>-2</td><td>mega-</td><td>M</td></tr>
    ///     <tr><td>-1</td><td>kilo-</td><td>k</td></tr>
    ///     <tr><td>0</td><td></td><td></td></tr>
    ///     <tr><td>+1</td><td>milli-</td><td>m</td></tr>
    ///     <tr><td>+2</td><td>micro-</td><td>u</td></tr>
    ///     <tr><td>+3</td><td>nano-</td><td>n</td></tr>
    ///     <tr><td>+4</td><td>pico-</td><td>p</td></tr>
    ///     <tr><td>+5</td><td>femto-</td><td>f</td></tr>
    ///     <tr><td>+6</td><td>atto-</td><td>a</td></tr>
    /// </table>
    mag: i8,
}

#[derive(Debug, PartialEq)]
/// Formatted balance.
pub struct PrettyOutput {
    /// Balance value with correctly placed point, to match the modified units
    pub number: String,

    /// Modified units, with optimal unit prefix (milli-, micro-, nano-, etc.)
    pub units: String,
}

/// Hisgest positive magnitude modifier, for low values (`mag = 6`, atto-).
const MAG_HIGHEST_POS: u8 = 6;

/// Lowest negative magnitude modifier, for high values (`mag = -4`, tera-).
const MAG_LOWEST_NEG: u8 = 4;

/// Format low balance high decimals values.
///
/// For cases when decimals exceeding balance string length by at least
/// 3&times;[`MAG_HIGHEST_POS`].
///
/// `zeroes_after_point_vefore_value` is always calculated as
/// `decimals - balance.len() - MAG_HIGHEST_POS * 3`
///
/// Outputs components for [`CutNumber`].
fn assist(balance: &str, zeroes_after_point_before_value: u8) -> (String, Option<String>, i8) {
    let out = format!(
        "{}{}",
        "0".repeat(zeroes_after_point_before_value as usize),
        balance
    );
    (String::from("0"), Some(out), MAG_HIGHEST_POS as i8)
}

/// Convert printed to string number into formatted balance.
fn convert_balance_string(balance: &str, decimals: u8, unit: &str) -> PrettyOutput {
    // at least one symbol always is there;
    //
    // length of input number without 1 symbol
    let order = (balance.len() as u8) - 1;

    let transformed_number = match order {
        // single digit input, special case
        0 => {
            // zero input
            if balance == "0" {
                let (before_point, after_point, mag) = {
                    // decimals sufficiently low to use custom prefix index
                    if decimals <= MAG_HIGHEST_POS * 3 {
                        match decimals % 3 {
                            0 => (balance.to_string(), None, (decimals / 3) as i8),
                            1 => (
                                balance.to_string(),
                                Some(String::from("0")),
                                (decimals / 3) as i8,
                            ),
                            2 => (
                                balance.to_string(),
                                Some(String::from("00")),
                                (decimals / 3) as i8,
                            ),
                            _ => unreachable!(),
                        }
                    } else {
                        // decimals too high, smallest prefix (`atto-`) is used
                        assist(balance, decimals - MAG_HIGHEST_POS * 3 - 1)
                    }
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            } else {
                // non-zero single-digit input
                let (before_point, after_point, mag) = {
                    // decimals sufficiently low to use custom prefix index
                    if decimals <= MAG_HIGHEST_POS * 3 {
                        match decimals % 3 {
                            0 => (balance.to_string(), None, (decimals / 3) as i8),
                            1 => (format!("{}00", balance), None, (decimals / 3) as i8 + 1),
                            2 => (format!("{}0", balance), None, (decimals / 3) as i8 + 1),
                            _ => unreachable!(),
                        }
                    } else {
                        // decimals too high, prefix `atto-` is used
                        assist(balance, decimals - MAG_HIGHEST_POS * 3 - 1)
                    }
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            }
        }
        // two-digit input, special case; could be `None` after point.
        1 => {
            let (before_point, after_point, mag) = {
                // no prefix, or milli-, micro-, nano- etc
                if order <= decimals {
                    // `(decimals-order)` sufficiently low to use custom prefix
                    // index
                    if (decimals - order) <= MAG_HIGHEST_POS * 3 {
                        match (decimals + 2) % 3 {
                            0 => (
                                balance[..1].to_string(),
                                Some(balance[1..].to_string()),
                                (decimals / 3) as i8,
                            ),
                            1 => (format!("{}0", balance), None, (decimals / 3) as i8 + 1),
                            2 => (balance.to_string(), None, (decimals / 3) as i8),
                            _ => unreachable!(),
                        }
                    } else {
                        // `(decimals-order)` too high, prefix `atto-` is used
                        assist(balance, decimals - order - MAG_HIGHEST_POS * 3 - 1)
                    }
                } else {
                    // goes here only if `decimals = 0`, leaving balance as is
                    (balance.to_string(), None, 0)
                }
            };
            CutNumber {
                before_point,
                after_point,
                mag,
            }
        }
        // tree-digit input, special case; could be `None` after point.
        2 => {
            let (before_point, after_point, mag) = {
                // no prefix, or milli-, micro-, nano- etc
                if order <= decimals {
                    // `(decimals-order)` sufficiently low to use custom prefix
                    // index
                    if (decimals - order) <= MAG_HIGHEST_POS * 3 {
                        match (decimals + 1) % 3 {
                            0 => (
                                balance[..1].to_string(),
                                Some(balance[1..].to_string()),
                                (decimals / 3) as i8,
                            ),
                            1 => (balance.to_string(), None, (decimals / 3) as i8),
                            2 => (
                                balance[..2].to_string(),
                                Some(balance[2..].to_string()),
                                (decimals / 3) as i8,
                            ),
                            _ => unreachable!(),
                        }
                    } else {
                        // `(decimals-order)` too high, prefix `atto-` is used
                        assist(balance, decimals - order - MAG_HIGHEST_POS * 3 - 1)
                    }
                } else if decimals == 0 {
                    // leave balance as is
                    (balance.to_string(), None, 0)
                } else {
                    // get here only if `decimals = 1`, last balance digit goes
                    // after the point
                    (balance[..2].to_string(), Some(balance[2..].to_string()), 0)
                }
            };
            CutNumber {
                before_point,
                after_point,
                mag,
            }
        }
        // everyting else; no `None` possible after point, all treated similarly
        _ => {
            // no prefix, or milli-, micro-, nano- etc
            if order <= decimals {
                let (before_point, after_point, mag) = {
                    // `(decimals-order)` sufficiently low to use custom prefix
                    // index
                    if (decimals - order) <= MAG_HIGHEST_POS * 3 {
                        // length after point and magnitude modifier
                        let (length, mag) = match (decimals - order) % 3 {
                            0 => (order as usize, ((decimals - order) / 3) as i8),
                            1 => ((order - 2) as usize, ((decimals - order) / 3) as i8 + 1),
                            2 => ((order - 1) as usize, ((decimals - order) / 3) as i8 + 1),
                            _ => unreachable!(),
                        };
                        let before_point = balance[..balance.len() - length].to_string();
                        let after_point = Some(balance[balance.len() - length..].to_string());
                        (before_point, after_point, mag)
                    } else {
                        // `(decimals-order)` too high, prefix `atto-` is used
                        assist(balance, decimals - order - MAG_HIGHEST_POS * 3 - 1)
                    }
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            } else {
                // prefixes kilo-, mega-, giga- etc
                //
                // length after point and magnitude modifier
                let (length, mag) = {
                    // (order-decimals) sufficiently low to use custom prefix
                    if (order - decimals) <= (MAG_LOWEST_NEG * 3) {
                        (
                            (order - (order - decimals) % 3) as usize,
                            -(((order - decimals) as i8) / 3),
                        )
                    } else {
                        // (decimals + MAG_LOWEST_NEG * 3) jointly go after the
                        // point, prefix `tera-` is always used.
                        (
                            (MAG_LOWEST_NEG * 3 + decimals) as usize,
                            -(MAG_LOWEST_NEG as i8),
                        )
                    }
                };
                let before_point = balance[..balance.len() - length].to_string();
                let after_point = Some(balance[balance.len() - length..].to_string());
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            }
        }
    };
    let unit_prefix = match transformed_number.mag {
        -4 => "T",
        -3 => "G",
        -2 => "M",
        -1 => "k",
        0 => "",
        1 => "m",
        2 => "u",
        3 => "n",
        4 => "p",
        5 => "f",
        6 => "a",
        _ => unreachable!(),
    };

    let number = match transformed_number.after_point {
        Some(x) => format!("{}.{}", transformed_number.before_point, x),
        None => transformed_number.before_point.to_string(),
    };

    PrettyOutput {
        number,
        units: format!("{}{}", unit_prefix, unit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let try_me = <u128>::convert_balance_pretty(0, 0, "X");
        assert_eq!(try_me.number, "0");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test02() {
        let try_me = <u128>::convert_balance_pretty(0, 1, "X");
        assert_eq!(try_me.number, "0.0");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test03() {
        let try_me = <u128>::convert_balance_pretty(0, 2, "X");
        assert_eq!(try_me.number, "0.00");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test04() {
        let try_me = <u128>::convert_balance_pretty(0, 3, "X");
        assert_eq!(try_me.number, "0");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test05() {
        let try_me = <u128>::convert_balance_pretty(0, 4, "X");
        assert_eq!(try_me.number, "0.0");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test06() {
        let try_me = <u128>::convert_balance_pretty(0, 20, "X");
        assert_eq!(try_me.number, "0.00");
        assert_eq!(try_me.units, "aX");
    }

    #[test]
    fn test07() {
        let try_me = <u128>::convert_balance_pretty(0, 24, "X");
        assert_eq!(try_me.number, "0.000000");
        assert_eq!(try_me.units, "aX");
    }

    #[test]
    fn test08() {
        let try_me = <u128>::convert_balance_pretty(0xffffffffffffffffffffffffffffffff, 0, "X");
        assert_eq!(try_me.number, "340282366920938463463374607.431768211455");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test09() {
        let try_me = <u128>::convert_balance_pretty(1, 0, "X");
        assert_eq!(try_me.number, "1");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test10() {
        let try_me = <u128>::convert_balance_pretty(1, 1, "X");
        assert_eq!(try_me.number, "100");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test11() {
        let try_me = <u128>::convert_balance_pretty(1, 2, "X");
        assert_eq!(try_me.number, "10");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test12() {
        let try_me = <u128>::convert_balance_pretty(1, 3, "X");
        assert_eq!(try_me.number, "1");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test13() {
        let try_me = <u128>::convert_balance_pretty(1, 4, "X");
        assert_eq!(try_me.number, "100");
        assert_eq!(try_me.units, "uX");
    }

    #[test]
    fn test14() {
        let try_me = <u128>::convert_balance_pretty(12, 0, "X");
        assert_eq!(try_me.number, "12");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test15() {
        let try_me = <u128>::convert_balance_pretty(12, 1, "X");
        assert_eq!(try_me.number, "1.2");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test16() {
        let try_me = <u128>::convert_balance_pretty(12, 2, "X");
        assert_eq!(try_me.number, "120");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test17() {
        let try_me = <u128>::convert_balance_pretty(12, 3, "X");
        assert_eq!(try_me.number, "12");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test18() {
        let try_me = <u128>::convert_balance_pretty(12, 4, "X");
        assert_eq!(try_me.number, "1.2");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test19() {
        let try_me = <u128>::convert_balance_pretty(123, 0, "X");
        assert_eq!(try_me.number, "123");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test20() {
        let try_me = <u128>::convert_balance_pretty(123, 1, "X");
        assert_eq!(try_me.number, "12.3");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test21() {
        let try_me = <u128>::convert_balance_pretty(123, 2, "X");
        assert_eq!(try_me.number, "1.23");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test22() {
        let try_me = <u128>::convert_balance_pretty(123, 3, "X");
        assert_eq!(try_me.number, "123");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test23() {
        let try_me = <u128>::convert_balance_pretty(123, 4, "X");
        assert_eq!(try_me.number, "12.3");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test24() {
        let try_me = <u128>::convert_balance_pretty(1, 40, "X");
        assert_eq!(try_me.number, "0.0000000000000000000001");
        assert_eq!(try_me.units, "aX");
    }

    #[test]
    fn test25() {
        let try_me = <u128>::convert_balance_pretty(12345, 21, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "aX");
    }

    #[test]
    fn test26() {
        let try_me = <u128>::convert_balance_pretty(12345, 18, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "fX");
    }

    #[test]
    fn test27() {
        let try_me = <u128>::convert_balance_pretty(12345, 15, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "pX");
    }

    #[test]
    fn test28() {
        let try_me = <u128>::convert_balance_pretty(12345, 12, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "nX");
    }

    #[test]
    fn test29() {
        let try_me = <u128>::convert_balance_pretty(12345, 10, "X");
        assert_eq!(try_me.number, "1.2345");
        assert_eq!(try_me.units, "uX");
    }

    #[test]
    fn test30() {
        let try_me = <u128>::convert_balance_pretty(12345, 9, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "uX");
    }

    #[test]
    fn test31() {
        let try_me = <u128>::convert_balance_pretty(12345, 6, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "mX");
    }

    #[test]
    fn test32() {
        let try_me = <u128>::convert_balance_pretty(12345, 3, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "X");
    }

    #[test]
    fn test33() {
        let try_me = <u128>::convert_balance_pretty(12345, 0, "X");
        assert_eq!(try_me.number, "12.345");
        assert_eq!(try_me.units, "kX");
    }

    #[test]
    fn test34() {
        let try_me = <u128>::convert_balance_pretty(123450000, 0, "X");
        assert_eq!(try_me.number, "123.450000");
        assert_eq!(try_me.units, "MX");
    }

    #[test]
    fn test35() {
        let try_me = <u128>::convert_balance_pretty(1234500000, 0, "X");
        assert_eq!(try_me.number, "1.234500000");
        assert_eq!(try_me.units, "GX");
        assert_eq!(try_me.number == "1.234500000", try_me.units == "GX");
    }

    #[test]
    fn test36() {
        let try_me = <u128>::convert_balance_pretty(1234500000000, 0, "X");
        assert_eq!(try_me.number, "1.234500000000");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test37() {
        let try_me = <u128>::convert_balance_pretty(10000000000000001, 0, "X");
        assert_eq!(try_me.number, "10000.000000000001");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test38() {
        let try_me = <u128>::convert_balance_pretty(1234, 24, "X");
        assert_eq!(try_me.number, "0.001234");
        assert_eq!(try_me.units, "aX");
    }

    #[test]
    fn test39() {
        let try_me = <u128>::convert_balance_pretty(0xffffffffffffffffffffffffffffffff, 15, "X");
        assert_eq!(try_me.number, "340282366920.938463463374607431768211455");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test40() {
        let try_me = <u128>::convert_balance_pretty(0xffffffffffffffffffffffffffffffff, 18, "X");
        assert_eq!(try_me.number, "340282366.920938463463374607431768211455");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test41() {
        let try_me = <u128>::convert_balance_pretty(0xffffffffffffffffffffffffffffffff, 9, "X");
        assert_eq!(try_me.number, "340282366920938463.463374607431768211455");
        assert_eq!(try_me.units, "TX");
    }

    #[test]
    fn test42() {
        let try_me = <u128>::convert_balance_pretty(0xffffffffffffffffffffffffffffffff, 27, "X");
        assert_eq!(try_me.number, "340.282366920938463463374607431768211455");
        assert_eq!(try_me.units, "GX");
    }
}
