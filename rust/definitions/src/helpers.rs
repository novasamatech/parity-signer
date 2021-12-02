use hex;

use crate::error::ErrorSource;

/// Function to decode hex encoded &str into Vec<u8>,
/// `what` is either of enums (NotHexHot or NotHexSigner) implementing NotHex trait
pub fn unhex<T: ErrorSource>(hex_entry: &str, what: T::NotHex) -> Result<Vec<u8>, T::Error> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => return Err(<T>::hex_to_error(what))
    }
}

