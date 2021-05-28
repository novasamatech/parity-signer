use base58::{ToBase58, FromBase58};
use blake2_rfc::blake2b::{blake2b};
use hex;
use std::convert::TryInto;

use super::constants::PREFIX;


/// function to convert [u8; 32] array into base58 address,
/// needs u8 prefix (found in chain specs)

pub fn arr_to_base (array: [u8; 32], prefix: u8) -> String {
    let mut fin = vec![prefix];
    fin.extend_from_slice(&array.to_vec());
    let hash = blake2b(64, &[], &[PREFIX, &fin].concat());
    fin.extend_from_slice(&hash.as_bytes()[0..2]);
    fin.to_base58()
}


/// function to convert 64 symbol hex string into base58
/// address, needs u8 prefix (found in chain specs)

pub fn hex_to_base (hex_part: &str, prefix: u8) -> String {
    assert!(hex_part.len()==64, "Wrong hex part length");
    let mut fin = vec![prefix];
    let part_unhex = hex::decode(&hex_part).unwrap();
    fin.extend_from_slice(&part_unhex);
    let hash = blake2b(64, &[], &[PREFIX, &fin].concat());
    fin.extend_from_slice(&hash.as_bytes()[0..2]);
    fin.to_base58()
}


/// function to convert base58 address into [u8; 32] array

pub fn base_to_arr (address: &str) -> [u8; 32] {
    let address_unbase = address.from_base58().unwrap();
// cut off the prefix [0] and the hash [2 last symbols]
    let part = &address_unbase[1..(address_unbase.len()-2)];
    let hash_part = &address_unbase[(address_unbase.len()-2)..];
    let hash = blake2b(64, &[], &[PREFIX, &address_unbase[..(address_unbase.len()-2)]].concat());
// check hash
    assert!(&hash.as_bytes()[0..2] == hash_part, "Hash error?");
    part.try_into().unwrap()
}


/// function to convert a base58 address into 64 symbol hex string

pub fn base_to_hex (address: &str) -> String {
    let address_unbase = address.from_base58().unwrap();
// cut off the prefix [0] and the hash [2 last symbols]
    let part = &address_unbase[1..(address_unbase.len()-2)];
    let hash_part = &address_unbase[(address_unbase.len()-2)..];
    let hash = blake2b(64, &[], &[PREFIX, &address_unbase[..(address_unbase.len()-2)]].concat());
// check hash
    assert!(&hash.as_bytes()[0..2] == hash_part, "Hash error?");
    hex::encode(part)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const BASE58PREFIX: u8 = 42;
    
    #[test]
    fn alice_and_bob_hex() {
        // addresses in base58
        let alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        // 64 symbol pieces extracted from transaction strings in polkadot - difference while changing transfer receiver
        let to_bob = "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";
        let to_alice = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        assert!(base_to_hex(alice) == to_alice, "Base to hex did not work for Alice! {} vs {}", base_to_hex(alice), to_alice);
        assert!(base_to_hex(bob) == to_bob, "Base to hex did not work for Bob! {} vs {}", base_to_hex(bob), to_bob);
        assert!(hex_to_base(to_alice, BASE58PREFIX) == alice, "Hex to base did not work for Alice! {} vs {}", hex_to_base(to_alice, BASE58PREFIX), alice);
        assert!(hex_to_base(to_bob, BASE58PREFIX) == bob, "Hex to base did not work for Bob! {} vs {}", hex_to_base(to_bob, BASE58PREFIX), bob);
    }
    
    #[test]
    fn bob_array() {
        let bob_array = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
        let bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        assert!(arr_to_base(bob_array, BASE58PREFIX) == bob, "Bob array into base58 not converting right.");
        assert!(base_to_arr(bob) == bob_array, "Bob base58 into array not converting right.");
    }
    
}
