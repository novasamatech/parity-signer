use printing_balance::*;

fn main() {
    
    let balance: u128 = 0xffffffffffffffffffffffffffffffff;
    let decimals: u8 = 0;
    let units = "X";
    println!("balance: {}; decimals: {}", balance, decimals);
    println!("{}", print_pretty_test(balance, decimals, units));
}
