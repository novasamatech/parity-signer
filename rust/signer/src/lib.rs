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
mod util;

use util::StringPtr;
use eth::KeyPair;

use ethsign::{Protected, keyfile::Crypto};
use rlp::decode_list;
use rustc_hex::{ToHex, FromHex};
use tiny_keccak::Keccak;
use tiny_keccak::keccak256 as keccak;

use std::num::NonZeroU32;

// 10240 is always non-zero, ergo this is safe
const CRYPTO_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10240) };

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

// TODO: REMOVE
#[no_mangle]
pub unsafe extern fn ethkey_keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let keypair = KeyPair::from_auto_phrase(&**seed);
  Box::into_raw(Box::new(keypair))
}

// TODO: REMOVE, use ethkey_brainwallet_address!
#[no_mangle]
pub unsafe extern fn ethkey_keypair_address(keypair: *mut KeyPair) -> *mut String {
  let address: String = (*keypair).address().to_hex();
  Box::into_raw(Box::new(address))
}

// TODO: REMOVE, use ethkey_brainwallet_sign!
#[no_mangle]
pub unsafe extern fn ethkey_keypair_sign(keypair: *mut KeyPair, message: *mut StringPtr) -> *mut String {
  let keypair = &*keypair;
  let message: Vec<u8> = (*message).as_str().from_hex().unwrap();
  let signature = keypair.sign(&message).unwrap().to_hex();
  Box::into_raw(Box::new(signature))
}

export! {
  @Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletAddress
  fn ethkey_brainwallet_address(seed: &str) -> String {
    let keypair = KeyPair::from_auto_phrase(seed);

    keypair.address().to_hex()
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSign
  fn ethkey_brainwallet_sign(seed: &str, message: &str) -> Option<String> {
    let keypair = KeyPair::from_auto_phrase(seed);
    let message: Vec<u8> = message.from_hex().ok()?;
    let signature = keypair.sign(&message).ok()?;

    Some(signature.to_hex())
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyRlpItem
  fn rlp_item(rlp: &str, position: u32) -> Option<String> {
    let hex: Vec<u8> = rlp.from_hex().ok()?;
    let rlp = decode_list::<Vec<u8>>(&hex);

    rlp.get(position as usize).map(|data| data.to_hex())
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyKeccak
  fn keccak256(data: &str) -> Option<String> {
    let data: Vec<u8> = data.from_hex().ok()?;

    Some(keccak(&data).to_hex())
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyBlockiesIcon
  fn blockies_icon(seed: String) -> Option<String> {
    use blockies::{create_icon, Blockies, ethereum};

    let mut result = Vec::new();

    let options = ethereum::Options {
      size: 8,
      scale: 16,
      seed: seed.into(),
      color: None,
      background_color: None,
      spot_color: None,
    };

    create_icon(&mut result, Blockies::Ethereum(options)).ok()?;

    Some(base64::encode(&result))
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyEthSign
  fn eth_sign(data: &str) -> Option<String> {
    let hex: Vec<u8> = data.from_hex().ok()?;
    let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();

    let mut res = [0u8; 32];
    let mut keccak = Keccak::new_keccak256();
    keccak.update(&message);
    keccak.update(&hex);
    keccak.finalize(&mut res);

    Some(res.to_hex())
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyRandomPhrase
  fn random_phrase() -> String {
    use bip39::{Mnemonic, MnemonicType, Language};
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);

    mnemonic.into_phrase()
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyEncryptData
  fn encrypt_data(data: &str, password: String) -> Option<String> {
    let password = Protected::new(password.into_bytes());
    let crypto = Crypto::encrypt(data.as_bytes(), &password, CRYPTO_ITERATIONS).ok()?;

    serde_json::to_string(&crypto).ok()
  }

  @Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData
  fn decrypt_data(data: &str, password: String) -> Option<String> {
    let password = Protected::new(password.into_bytes());

    let crypto: Crypto = serde_json::from_str(data).ok()?;
    let decrypted = crypto.decrypt(&password).ok()?;

    String::from_utf8(decrypted).ok()
  }
}

#[cfg(test)]
mod tests {
  use super::rlp_item;

  #[test]
  fn test_rlp_item() {
    let rlp = "f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804";
    assert_eq!(rlp_item(rlp, 0), Some("".into()));
    assert_eq!(rlp_item(rlp, 1), Some("01".into()));
    assert_eq!(rlp_item(rlp, 2), Some("5208".into()));
    assert_eq!(rlp_item(rlp, 3), Some("095e7baea6a6c7c4c2dfeb977efac326af552d87".into()));
    assert_eq!(rlp_item(rlp, 4), Some("0a".into()));
    assert_eq!(rlp_item(rlp, 5), Some("".into()));
  }
}
