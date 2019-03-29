// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

mod eth;
mod strate;
mod util;

use util::StringPtr;
use eth::KeyPair;

use blockies::{Blockies, create_icon, ethereum};
use ethsign::{Protected, keyfile::Crypto};
use rlp::decode_list;
use rustc_hex::{ToHex, FromHex};
use tiny_keccak::Keccak;
use tiny_keccak::keccak256 as keccak;

use std::num::NonZeroU32;

// 10240 is always non-zero, ergo this is safe
const CRYPTO_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10240) };

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

  base64::encode(&result)
}

// string ffi

#[no_mangle]
pub unsafe extern fn rust_string_ptr(s: *mut String) -> *mut StringPtr {
  Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}

// ethkey ffi

#[no_mangle]
pub unsafe extern fn ethkey_keypair_destroy(keypair: *mut KeyPair) {
  let _ = Box::from_raw(keypair);
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let keypair = KeyPair::from_auto_phrase(&**seed);
  Box::into_raw(Box::new(keypair))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_address(keypair: *mut KeyPair) -> *mut String {
  let address: String = (*keypair).address().to_hex();
  Box::into_raw(Box::new(address))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_sign(keypair: *mut KeyPair, message: *mut StringPtr) -> *mut String {
  let keypair = &*keypair;
  let message: Vec<u8> = (*message).as_str().from_hex().unwrap();
  let signature = keypair.sign(&message).unwrap().to_hex();
  Box::into_raw(Box::new(signature))
}

fn safe_rlp_item(rlp: &str, position: u32) -> Result<String, String> {
  let hex: Vec<u8> = rlp.from_hex().map_err(| e | e.to_string())?;
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

#[no_mangle]
pub unsafe extern fn keccak256(data: *mut StringPtr) -> *mut String {
  let data: Vec<u8> = (*data).as_str().from_hex().unwrap();
  let res = keccak(&data);
  Box::into_raw(Box::new(res.to_hex()))
}

#[no_mangle]
pub unsafe extern fn eth_sign(data: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let hex: Vec<u8> = data.from_hex().unwrap();
  let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();
  let mut res: [u8; 32] = [0; 32];
  let mut keccak = Keccak::new_keccak256();
  keccak.update(&message);
  keccak.update(&hex);
  keccak.finalize(&mut res);
  Box::into_raw(Box::new(res.to_hex()))
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
pub unsafe extern fn random_phrase() -> *mut String {
  use bip39::{Mnemonic, MnemonicType, Language};
  let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
  Box::into_raw(Box::new(mnemonic.into_phrase()))
}

// data encryption ffi

#[no_mangle]
pub unsafe extern fn encrypt_data(data: *mut StringPtr, password: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let password = Protected::new((*password).as_str().as_bytes().to_vec());
  let crypto = Crypto::encrypt(data.as_bytes(), &password, CRYPTO_ITERATIONS).unwrap();
  Box::into_raw(Box::new(serde_json::to_string(&crypto).unwrap()))
}

#[no_mangle]
pub unsafe extern fn decrypt_data(encrypted_data: *mut StringPtr, password: *mut StringPtr, error: *mut u32) -> *mut String {
  let data = (*encrypted_data).as_str();
  let password = Protected::new((*password).as_str().as_bytes().to_vec());

  let crypto: Crypto = match serde_json::from_str(&data) {
    Ok(crypto) => crypto,
    Err(_) => {
      *error = 1;
      return Box::into_raw(Box::new(String::new()))
    },
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

#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
  use super::*;
  use jni::JNIEnv;
  use jni::objects::{JClass, JString, JThrowable};
  use jni::sys::{jint, jstring};

  fn new_exception<'a>(env: &'a JNIEnv<'a>) -> JThrowable<'a> {
    let exception = env.find_class("java/lang/Exception").unwrap();
    // javap -s java.lang.Exception
    env.new_object(exception, "()V", &[]).unwrap().into()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletAddress(env: JNIEnv, _: JClass, seed: JString) -> jstring {
    let seed: String = env.get_string(seed).expect("Invalid seed").into();
    let keypair = KeyPair::from_auto_phrase(&seed);
    let java_address = env.new_string(keypair.address().to_hex::<String>()).expect("Could not create java string");
    java_address.into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSign(env: JNIEnv, _: JClass, seed: JString, message: JString) -> jstring {
    let seed: String = env.get_string(seed).expect("Invalid seed").into();
    let message: String = env.get_string(message).expect("Invalid message").into();
    let keypair = KeyPair::from_auto_phrase(&seed);
    let message: Vec<u8> = message.from_hex().unwrap();
    let signature = keypair.sign(&message).unwrap();
    let java_signature = env.new_string(signature.to_hex::<String>()).expect("Could not create java string");
    java_signature.into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyRlpItem(env: JNIEnv, _: JClass, data: JString, position: jint) -> jstring {
    let data: String = env.get_string(data).expect("Invalid seed").into();
    match safe_rlp_item(&data, position as u32) {
      Ok(result) => env.new_string(result).expect("Could not create java string").into_inner(),
      Err(_) => {
        let res = env.new_string("").expect("").into_inner();
        env.throw(new_exception(&env)).unwrap();
        res
      },
    }
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyKeccak(env: JNIEnv, _: JClass, data: JString) -> jstring {
    let data: String = env.get_string(data).expect("Invalid seed").into();
    let data: Vec<u8> = data.from_hex().unwrap();
    let res = keccak(&data);
    env.new_string(res.to_hex::<String>()).expect("Could not create java string").into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyEthSign(env: JNIEnv, _: JClass, data: JString) -> jstring {
    let data: String = env.get_string(data).expect("Invalid seed").into();
    let hex: Vec<u8> = data.from_hex().unwrap();
    let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();
    let mut res: [u8; 32] = [0; 32];
    let mut keccak = Keccak::new_keccak256();
    keccak.update(&message);
    keccak.update(&hex);
    keccak.finalize(&mut res);
    env.new_string(res.to_hex::<String>()).expect("Could not create java string").into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyBlockiesIcon(env: JNIEnv, _: JClass, seed: JString) -> jstring {
    let seed: String = env.get_string(seed).expect("Invalid seed").into();
    let icon = blockies_icon_in_base64(seed.into());
    env.new_string(icon).expect("Could not create java string").into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyRandomPhrase(env: JNIEnv, _: JClass) -> jstring {
    use bip39::{Mnemonic, MnemonicType, Language};
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    env.new_string(mnemonic.into_phrase()).expect("Could not create java string").into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyEncryptData(env: JNIEnv, _: JClass, data: JString, password: JString) -> jstring {
    let data: String = env.get_string(data).expect("Invalid data").into();
    let password: String = env.get_string(password).expect("Invalid password").into();
    let password = Protected::new(password.into_bytes());
    let crypto = Crypto::encrypt(data.as_bytes(), &password, CRYPTO_ITERATIONS).unwrap();
    env.new_string(serde_json::to_string(&crypto).unwrap()).expect("Could not create java string").into_inner()
  }

  #[no_mangle]
  pub unsafe extern fn Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData(env: JNIEnv, _: JClass, data: JString, password: JString) -> jstring {
    let data: String = env.get_string(data).expect("Invalid data").into();
    let password: String = env.get_string(password).expect("Invalid password").into();
    let password = Protected::new(password.into_bytes());

    let crypto: Crypto = match serde_json::from_str(&data) {
      Ok(crypto) => crypto,
      Err(_) => {
        let result = env.new_string("").expect("Could not create java string").into_inner();
        env.throw(new_exception(&env)).expect("first throw failed");
        return result
      },
    };

    match crypto.decrypt(&password) {
      Ok(decrypted) => {
        env.new_string(String::from_utf8_unchecked(decrypted)).expect("Could not create java string").into_inner()
      },
      Err(_) => {
        let result = env.new_string("").expect("Could not create java string").into_inner();
        env.throw(new_exception(&env)).expect("second throw failed");
        result
      },
    }
  }
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

