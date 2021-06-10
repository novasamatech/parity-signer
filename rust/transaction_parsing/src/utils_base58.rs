use base58::ToBase58;
use blake2_rfc::blake2b::blake2b;

use super::constants::PREFIX;


/// Function to convert Vec<u8> into base58 address,
/// needs u8 prefix (found in chain specs).
/// Vec<u8> length should be 32 for ed25519 and sr25519 encoding
/// and 33 for ecdsa encoding,
/// this should be checked elsewhere.

pub fn vec_to_base (data: &Vec<u8>, prefix: u8) -> String {
    let mut fin = vec![prefix];
    fin.extend_from_slice(&data);
    let hash = blake2b(64, &[], &[PREFIX, &fin].concat());
    fin.extend_from_slice(&hash.as_bytes()[0..2]);
    fin.to_base58()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bob_vec() {
        let bob_vec = vec![142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
        let bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        assert!(vec_to_base(&bob_vec, 42) == bob, "Bob vec into base58 not converting right.");
    }
        
}
