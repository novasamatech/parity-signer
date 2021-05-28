struct CutNumber {
    before_point: String,
    after_point: Option<String>,
    mag: i8,
}

pub struct PrettyOutput {
    pub number: String,
    pub units: String,
}

const MINUS_MIN: u8 = 6;
const PLUS_MAX: u8 = 4;

fn assist (a: String, decimals: u8, order: u8) -> (String, Option<String>, i8) {
    let t = decimals-order-MINUS_MIN*3-1;
    let mut out = String::new();
    for _i in 0..t {out.push_str("0");}
    out.push_str(&a);
    (String::from("0"), Some(out), MINUS_MIN as i8)
}

pub fn convert_balance_pretty (balance: u128, decimals: u8, units: &str) -> Result<PrettyOutput, &'static str> {
    
    let a = balance.to_string();
    let order = (a.len() as u8) -1;
    
    let transformed_number = match order {
        0 => {
            if balance == 0 {
                let (before_point, after_point, mag) = {
                    if decimals <= MINUS_MIN*3 {
                        match decimals%3 {
                            0 => (a, None, (decimals/3) as i8),
                            1 => (a, Some(String::from("0")), (decimals/3) as i8),
                            2 => (a, Some(String::from("00")), (decimals/3) as i8),
                            _ => return Err("Should not be here"),
                        }
                    }
                    else {assist(a, decimals, order)}
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            }
            else {
                let (before_point, after_point, mag) = {
                    if decimals <= MINUS_MIN*3 {
                        match decimals%3 {
                            0 => (a, None, (decimals/3) as i8),
                            1 => (format!("{}00",a), None, (decimals/3) as i8 + 1),
                            2 => (format!("{}0",a), None, (decimals/3) as i8 + 1),
                            _ => return Err("Should not be here"),
                        }
                    }
                    else {assist(a, decimals, order)}
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            }
        },
        1 => {
            let (before_point, after_point, mag) = {
                if order <= decimals {
                    if (decimals-order) <= MINUS_MIN*3 {
                        match (decimals+2)%3 {
                            0 => (a[..1].to_string(), Some(a[1..].to_string()), (decimals/3) as i8),
                            1 => (format!("{}0",a), None, (decimals/3) as i8 + 1),
                            2 => (a, None, (decimals/3) as i8),
                            _ => return Err("Should not be here"),
                        }
                    }
                    else {assist(a, decimals, order)}
                }
                else {
                    (a, None, 0)
                }
            };
            CutNumber {
                before_point,
                after_point,
                mag,
            }
        },
        2 => {
            let (before_point, after_point, mag) = {
                if order <= decimals {
                    if (decimals-order) <= MINUS_MIN*3 {
                        match (decimals+1)%3 {
                            0 => (a[..1].to_string(), Some(a[1..].to_string()), (decimals/3) as i8),
                            1 => (a, None, (decimals/3) as i8),
                            2 => (a[..2].to_string(), Some(a[2..].to_string()), (decimals/3) as i8),
                            _ => return Err("Should not be here"),
                        }
                    }
                    else {assist(a, decimals, order)}
                }
                else {
                    if decimals == 0 {(a, None, 0)}
                    else {(a[..2].to_string(), Some(a[2..].to_string()), 0)}
                }
            };
            CutNumber {
                before_point,
                after_point,
                mag,
            }
        },
        _ => {
            if order <= decimals {
                let length = {
                    if (decimals-order) <= MINUS_MIN*3 {
                        match (decimals - order)%3 {
                            0 => order as usize,
                            1 => (order-2) as usize,
                            _ => (order-1) as usize,
                        }
                    }
                    else {(decimals-MINUS_MIN*3) as usize}
                };
                let before_point = a[..a.len()-length].to_string();
                let after_point = Some(a[a.len()-length..].to_string());
                let mag = {
                    if (decimals-order) <= MINUS_MIN*3 {
                        match (decimals-order)%3 {
                            0 => ((decimals-order)/3) as i8,
                            _ => ((decimals-order)/3) as i8 + 1,
                        }
                    }
                    else {MINUS_MIN as i8}
                };
                CutNumber {
                    before_point,
                    after_point,
                    mag,
                }
            }
            else {
                let num = (order - decimals)%3;
                let length = {
                    if (order-decimals) <= (PLUS_MAX*3) {(order-num) as usize}
                    else {(PLUS_MAX*3-decimals) as usize}
                };
                let before_point = a[..a.len()-length].to_string();
                let after_point = Some(a[a.len()-length..].to_string());
                let mag = {
                    if (order-decimals) <= (PLUS_MAX*3) {-(((order-decimals) as i8)/3)}
                    else {-(PLUS_MAX as i8)}
                };
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
        _ => return Err("Error in prefixes"),
    };
    
    let number = match transformed_number.after_point {
        Some(x) => format!("{}.{}", transformed_number.before_point, x),
        None => format!("{}", transformed_number.before_point),
    };
    
    Ok(PrettyOutput {
        number,
        units: format!("{}{}", unit_prefix, units),
    })
}

pub fn print_pretty_test (balance: u128, decimals: u8, units: &str) -> String {
    let out = convert_balance_pretty (balance, decimals, units).unwrap();
    format!("{} {}", out.number, out.units)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test01() {
        let try_me = print_pretty_test (0, 0, "X");
        assert_eq!(try_me, "0 X");
    }
    
    #[test]
    fn test02() {
        let try_me = print_pretty_test (0, 1, "X");
        assert_eq!(try_me, "0.0 X");
    }
    
    #[test]
    fn test03() {
        let try_me = print_pretty_test (0, 2, "X");
        assert_eq!(try_me, "0.00 X");
    }
    
    #[test]
    fn test04() {
        let try_me = print_pretty_test (0xffffffffffffffffffffffffffffffff, 0, "X");
        assert_eq!(try_me, "340282366920938463463374607.431768211455 TX");
    }
    
    #[test]
    fn test05() {
        let try_me = print_pretty_test (0, 20, "X");
        assert_eq!(try_me, "0.00 aX");
    }
    
    #[test]
    fn test06() {
        let try_me = print_pretty_test (0, 24, "X");
        assert_eq!(try_me, "0.000000 aX");
    }
    
    #[test]
    fn test07() {
        let try_me = print_pretty_test (0, 3, "X");
        assert_eq!(try_me, "0 mX");
    }
    
    #[test]
    fn test08() {
        let try_me = print_pretty_test (0, 4, "X");
        assert_eq!(try_me, "0.0 mX");
    }
    
    #[test]
    fn test09() {
        let try_me = print_pretty_test (1, 0, "X");
        assert_eq!(try_me, "1 X");
    }
    
    #[test]
    fn test10() {
        let try_me = print_pretty_test (12, 0, "X");
        assert_eq!(try_me, "12 X");
    }
    
    #[test]
    fn test11() {
        let try_me = print_pretty_test (123, 0, "X");
        assert_eq!(try_me, "123 X");
    }
    
    #[test]
    fn test12() {
        let try_me = print_pretty_test (123, 1, "X");
        assert_eq!(try_me, "12.3 X");
    }
    
    #[test]
    fn test13() {
        let try_me = print_pretty_test (123, 2, "X");
        assert_eq!(try_me, "1.23 X");
    }
    
    #[test]
    fn test14() {
        let try_me = print_pretty_test (1, 1, "X");
        assert_eq!(try_me, "100 mX");
    }
    
    #[test]
    fn test15() {
        let try_me = print_pretty_test (1, 2, "X");
        assert_eq!(try_me, "10 mX");
    }
    
    #[test]
    fn test16() {
        let try_me = print_pretty_test (1, 3, "X");
        assert_eq!(try_me, "1 mX");
    }
    
    #[test]
    fn test17() {
        let try_me = print_pretty_test (1, 4, "X");
        assert_eq!(try_me, "100 uX");
    }
    
    #[test]
    fn test18() {
        let try_me = print_pretty_test (12, 1, "X");
        assert_eq!(try_me, "1.2 X");
    }
    
    #[test]
    fn test19() {
        let try_me = print_pretty_test (12, 2, "X");
        assert_eq!(try_me, "120 mX");
    }
    
    #[test]
    fn test20() {
        let try_me = print_pretty_test (12, 3, "X");
        assert_eq!(try_me, "12 mX");
    }
    
    #[test]
    fn test21() {
        let try_me = print_pretty_test (12, 4, "X");
        assert_eq!(try_me, "1.2 mX");
    }
    
    #[test]
    fn test22() {
        let try_me = print_pretty_test (123, 1, "X");
        assert_eq!(try_me, "12.3 X");
    }
    
    #[test]
    fn test23() {
        let try_me = print_pretty_test (123, 2, "X");
        assert_eq!(try_me, "1.23 X");
    }
    
    #[test]
    fn test24() {
        let try_me = print_pretty_test (123, 3, "X");
        assert_eq!(try_me, "123 mX");
    }
    
    #[test]
    fn test25() {
        let try_me = print_pretty_test (123, 4, "X");
        assert_eq!(try_me, "12.3 mX");
    }
    
    #[test]
    fn test26() {
        let try_me = print_pretty_test (1, 40, "X");
        assert_eq!(try_me, "0.0000000000000000000001 aX");
    }
    
    #[test]
    fn test27() {
        let try_me = print_pretty_test (12345, 21, "X");
        assert_eq!(try_me, "12.345 aX");
    }
    
    #[test]
    fn test28() {
        let try_me = print_pretty_test (12345, 18, "X");
        assert_eq!(try_me, "12.345 fX");
    }
    
    #[test]
    fn test29() {
        let try_me = print_pretty_test (12345, 15, "X");
        assert_eq!(try_me, "12.345 pX");
    }
    
    #[test]
    fn test30() {
        let try_me = print_pretty_test (12345, 12, "X");
        assert_eq!(try_me, "12.345 nX");
    }
    
    #[test]
    fn test31() {
        let try_me = print_pretty_test (12345, 9, "X");
        assert_eq!(try_me, "12.345 uX");
    }
    
    #[test]
    fn test32() {
        let try_me = print_pretty_test (12345, 6, "X");
        assert_eq!(try_me, "12.345 mX");
    }
    
    #[test]
    fn test33() {
        let try_me = print_pretty_test (12345, 10, "X");
        assert_eq!(try_me, "1.2345 uX");
    }
    
    #[test]
    fn test34() {
        let try_me = print_pretty_test (12345, 3, "X");
        assert_eq!(try_me, "12.345 X");
    }
    
    #[test]
    fn test35() {
        let try_me = print_pretty_test (12345, 0, "X");
        assert_eq!(try_me, "12.345 kX");
    }
    
    #[test]
    fn test36() {
        let try_me = print_pretty_test (123450000, 0, "X");
        assert_eq!(try_me, "123.450000 MX");
    }
    
    #[test]
    fn test37() {
        let try_me = print_pretty_test (1234500000, 0, "X");
        assert_eq!(try_me, "1.234500000 GX");
    }
    
    #[test]
    fn test38() {
        let try_me = print_pretty_test (1234500000000, 0, "X");
        assert_eq!(try_me, "1.234500000000 TX");
    }
    
    #[test]
    fn test39() {
        let try_me = print_pretty_test (10000000000000001, 0, "X");
        assert_eq!(try_me, "10000.000000000001 TX");
    }
}

