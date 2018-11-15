use wordlist;
use rustc_serialize::hex::{ToHex, FromHex};
use rustc_serialize::base64::{self, ToBase64};
use blockies::{Blockies, create_icon, ethereum};
use rlp::decode_list;
use string::StringPtr;
use tiny_keccak::Keccak;
use ethkey::{KeyPair, Generator, Brain, Message, Password, sign};
use ethstore::Crypto;

#[no_mangle]
pub unsafe extern fn keccak256(data: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let hex = data.from_hex().unwrap();
  let mut res: [u8; 32] = [0; 32];
  let mut keccak = Keccak::new_keccak256();
  keccak.update(&hex);
  keccak.finalize(&mut res);
  Box::into_raw(Box::new(res.to_hex()))
}

#[no_mangle]
pub unsafe extern fn eth_sign(data: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let hex = data.from_hex().unwrap();
  let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();
  let mut res: [u8; 32] = [0; 32];
  let mut keccak = Keccak::new_keccak256();
  keccak.update(&message);
  keccak.update(&hex);
  keccak.finalize(&mut res);
  Box::into_raw(Box::new(res.to_hex()))
}

// ethkey ffi

#[no_mangle]
pub unsafe extern fn ethkey_keypair_destroy(keypair: *mut KeyPair) {
  let _ = Box::from_raw(keypair);
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let mut generator = Brain::new((*seed).as_str().to_owned());
  Box::into_raw(Box::new(generator.generate().unwrap()))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_secret(keypair: *mut KeyPair) -> *mut String {
  let secret = format!("{:?}", (*keypair).secret());
  Box::into_raw(Box::new(secret))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_address(keypair: *mut KeyPair) -> *mut String {
  let address = format!("{:?}", (*keypair).address());
  Box::into_raw(Box::new(address))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_sign(keypair: *mut KeyPair, message: *mut StringPtr) -> *mut String {
  let secret = (*keypair).secret();
  let message: Message = (*message).as_str().parse().unwrap();
  let signature = format!("{}", sign(secret, &message).unwrap());
  Box::into_raw(Box::new(signature))
}

// data encryption ffi

#[no_mangle]
pub unsafe extern fn encrypt_data(data: *mut StringPtr, password: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let password = Password::from((*password).as_str());
  let crypto = Crypto::with_plain(data.as_bytes(), &password, 10240).unwrap();
  Box::into_raw(Box::new(crypto.into()))
}

#[no_mangle]
pub unsafe extern fn decrypt_data(encrypted_data: *mut StringPtr, password: *mut StringPtr, error: *mut u32) -> *mut String {
  let data = (*encrypted_data).as_str();
  let password = Password::from((*password).as_str());
  let crypto: Crypto = match data.parse() {
    Ok(crypto) => crypto,
    Err(_) => {
      *error = 1;
      return Box::into_raw(Box::new(String::new()))
    }
  };
  match crypto.decrypt(&password) {
    Ok(decrypted) => {
      Box::into_raw(Box::new(String::from_utf8_unchecked(decrypted)))
    },
    Err(_) => {
      *error = 2;
      Box::into_raw(Box::new(String::new()))
    },
  }
}

// RLP

fn safe_rlp_item(rlp: &str, position: u32) -> Result<String, String> {
  let hex = rlp.from_hex().map_err(| e | e.to_string())?;
  let rlp = decode_list::<Vec<u8>>(&hex);
  let data = rlp.get(position as usize).ok_or_else(|| "Invalid RLP position".to_string())?;
  Ok(data.to_hex())
}

#[no_mangle]
pub unsafe extern fn rlp_item(rlp: *mut StringPtr, position: u32, error: *mut u32) -> *mut String {
  match safe_rlp_item((*rlp).as_str(), position) {
    Ok(result) => Box::into_raw(Box::new(result)),
    Err(_err) => {
      *error = 1;
      let s: String = "".into();
      Box::into_raw(Box::new(s))
    }
  }
}

fn blockies_icon_in_base64(seed: Vec<u8>) -> String {
  let mut result = Vec::new();
  let options = ethereum::Options {
    size: 8,
    scale: 16,
    seed: seed,
    color: None,
    background_color: None,
    spot_color: None,
  };

  create_icon(&mut result, Blockies::Ethereum(options)).unwrap();
  result.to_base64(base64::Config {
    char_set: base64::CharacterSet::Standard,
    newline: base64::Newline::LF,
    pad: true,
    line_length: None,
  })
}

// blockies ffi

#[no_mangle]
pub unsafe extern fn blockies_icon(blockies_seed: *mut StringPtr) -> *mut String {
  let blockies_seed = (*blockies_seed).as_str();
  let icon = blockies_icon_in_base64(blockies_seed.into());
  Box::into_raw(Box::new(icon))
}

// random phrase ffi
#[no_mangle]
pub unsafe extern fn random_phrase(words: u32) -> *mut String {
  let words = wordlist::random_phrase(words as usize);
  Box::into_raw(Box::new(words))
}

#[cfg(test)]
mod tests {
  use super::safe_rlp_item;

  #[test]
  fn test_rlp_item() {
    let rlp = "f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804";
    assert_eq!(safe_rlp_item(rlp, 0), Ok("".into()));
    assert_eq!(safe_rlp_item(rlp, 1), Ok("01".into()));
    assert_eq!(safe_rlp_item(rlp, 2), Ok("5208".into()));
    assert_eq!(safe_rlp_item(rlp, 3), Ok("095e7baea6a6c7c4c2dfeb977efac326af552d87".into()));
    assert_eq!(safe_rlp_item(rlp, 4), Ok("0a".into()));
    assert_eq!(safe_rlp_item(rlp, 5), Ok("".into()));
  }
}
