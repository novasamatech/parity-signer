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

//
//  EthkeyBridge.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//

import Foundation

@objc(EthkeyBridge)
class EthkeyBridge: NSObject {

  public static func requiresMainQueueSetup() -> Bool {
    return true;
  }

  @objc func brainWalletAddress(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    let address_rust_str = ethkey_brainwallet_address(&error, &seed_ptr)
    let address_rust_str_ptr = rust_string_ptr(address_rust_str)
    let address = String.fromStringPtr(ptr: address_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(address_rust_str_ptr)
    rust_string_destroy(address_rust_str)
    resolve(address)
  }

  @objc func brainWalletSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    var message_ptr = message.asPtr()
    let signature_rust_str = ethkey_brainwallet_sign(&error, &seed_ptr, &message_ptr)
    let signature_rust_str_ptr = rust_string_ptr(signature_rust_str)
    let signature = String.fromStringPtr(ptr: signature_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(signature_rust_str_ptr)
    rust_string_destroy(signature_rust_str)
    resolve(signature)
  }

  @objc func rlpItem(_ rlp: String, position: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var rlp_ptr = rlp.asPtr()
    let item_rust_str = rlp_item(&error, &rlp_ptr, position)
    let item_rust_str_ptr = rust_string_ptr(item_rust_str)
    let item = String.fromStringPtr(ptr: item_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(item_rust_str_ptr)
    rust_string_destroy(item_rust_str)
    if (error == 0) {
      resolve(item)
    } else {
      reject("invalid rlp", nil, nil)
    }
  }

  @objc func keccak(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    let hash_rust_str = keccak256(&error, &data_ptr)
    let hash_rust_str_ptr = rust_string_ptr(hash_rust_str)
    let hash = String.fromStringPtr(ptr: hash_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(hash_rust_str_ptr)
    rust_string_destroy(hash_rust_str)
    if (error == 0) {
      resolve(hash)
    } else {
      reject("invalid data, expected hex-encoded string", nil, nil)
    }
  }

  @objc func blake2b(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    let hash_rust_str = blake(&error, &data_ptr)
    let hash_rust_str_ptr = rust_string_ptr(hash_rust_str)
    let hash = String.fromStringPtr(ptr: hash_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(hash_rust_str_ptr)
    rust_string_destroy(hash_rust_str)
    if (error == 0) {
      resolve(hash)
    } else {
      reject("invalid data, expected hex-encoded string", nil, nil)
    }
  }

  @objc func ethSign(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    let hash_rust_str = eth_sign(&error, &data_ptr)
    let hash_rust_str_ptr = rust_string_ptr(hash_rust_str)
    let hash = String.fromStringPtr(ptr: hash_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(hash_rust_str_ptr)
    rust_string_destroy(hash_rust_str)
    resolve(hash)
  }

  @objc func blockiesIcon(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    let icon_rust_str = blockies_icon(&error, &seed_ptr)
    let icon_rust_str_ptr = rust_string_ptr(icon_rust_str)
    let icon = String.fromStringPtr(ptr: icon_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(icon_rust_str_ptr)
    rust_string_destroy(icon_rust_str)
    if error == 0 {
      resolve(icon)
    } else {
      reject("Failed to generate blockies", nil, nil)
    }
  }

  @objc func randomPhrase(_ resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let words_rust_str = random_phrase(&error)
    let words_rust_str_ptr = rust_string_ptr(words_rust_str)
    let words = String.fromStringPtr(ptr: words_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(words_rust_str_ptr)
    rust_string_destroy(words_rust_str)
    resolve(words)
  }

  @objc func encryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    var password_ptr = password.asPtr()
    let encrypted_data_rust_str = encrypt_data(&error, &data_ptr, &password_ptr)
    let encrypted_data_rust_str_ptr = rust_string_ptr(encrypted_data_rust_str)
    let encrypted_data = String.fromStringPtr(ptr: encrypted_data_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(encrypted_data_rust_str_ptr)
    rust_string_destroy(encrypted_data_rust_str)
    resolve(encrypted_data)
  }

  @objc func decryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    var password_ptr = password.asPtr()
    let decrypted_data_rust_str = decrypt_data(&error, &data_ptr, &password_ptr)
    let decrypted_data_rust_str_ptr = rust_string_ptr(decrypted_data_rust_str)
    let decrypted_data = String.fromStringPtr(ptr: decrypted_data_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(decrypted_data_rust_str_ptr)
    rust_string_destroy(decrypted_data_rust_str)
    if error == 0 {
      resolve(decrypted_data)
    } else {
      reject("invalid password", nil, nil)
    }
  }

  @objc func qrCode(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    let icon_rust_str = qrcode(&error, &data_ptr)
    let icon_rust_str_ptr = rust_string_ptr(icon_rust_str)
    let icon = String.fromStringPtr(ptr: icon_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(icon_rust_str_ptr)
    rust_string_destroy(icon_rust_str)
    if error == 0 {
      resolve(icon)
    } else {
      reject("Failed to generate blockies", nil, nil)
    }
  }

  @objc func qrCodeHex(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var data_ptr = data.asPtr()
    let icon_rust_str = qrcode_hex(&error, &data_ptr)
    let icon_rust_str_ptr = rust_string_ptr(icon_rust_str)
    let icon = String.fromStringPtr(ptr: icon_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(icon_rust_str_ptr)
    rust_string_destroy(icon_rust_str)
    if error == 0 {
      resolve(icon)
    } else {
      reject("Failed to generate blockies", nil, nil)
    }
  }

  @objc func substrateAddress(_ seed: String, version: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    let address_rust_str = substrate_brainwallet_address(&error, &seed_ptr, version)
    let address_rust_str_ptr = rust_string_ptr(address_rust_str)
    let address = String.fromStringPtr(ptr: address_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(address_rust_str_ptr)
    rust_string_destroy(address_rust_str)
    resolve(address)
  }

  @objc func substrateSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    var message_ptr = message.asPtr()
    let signature_rust_str = substrate_brainwallet_sign(&error, &seed_ptr, &message_ptr)
    let signature_rust_str_ptr = rust_string_ptr(signature_rust_str)
    let signature = String.fromStringPtr(ptr: signature_rust_str_ptr!.pointee)
    rust_string_ptr_destroy(signature_rust_str_ptr)
    rust_string_destroy(signature_rust_str)
    resolve(signature)
  }

  @objc func schnorrkelVerify(_ seed: String, message: String, signature: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    var seed_ptr = seed.asPtr()
    var message_ptr = message.asPtr()
    var signature_ptr = signature.asPtr()
    let is_valid = schnorrkel_verify(&error, &seed_ptr, &message_ptr, &signature_ptr)
    if error == 0 {
      resolve(is_valid)
    } else {
      reject("Failed to verify signature.", nil, nil)
    }
  }
  
/* secure native */

  @objc func securePut(_ app: String, key: String, seed: String, withBiometry: CInt, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_put(&error, app, key, seed, withBiometry)
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_void(res)
    if error == 0 {
      resolve(nil)
    } else {
      reject("put", error_msg, nil)
    }
  }
  
  @objc func secureGet(_ app: String, key: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_get(&error, app, key)
    let value = String(cString: res!.pointee.value!)
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_string(res)
    if error == 0 {
      resolve(value)
    } else {
      reject("get", error_msg, nil)
    }
  }

  @objc func secureContains(_ app: String, key: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_contains(&error, app, key)
    let value = res!.pointee.value.pointee
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_bool(res)
    if error == 0 {
      resolve(value)
    } else {
      reject("contains", error_msg, nil)
    }
  }

  @objc func secureDelete(_ app: String, key: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_delete(&error, app, key)
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_void(res)
    if error == 0 {
      resolve(nil)
    } else {
      reject("delete", error_msg, nil)
    }
  }

  @objc func secureEthkeySign(_ app: String, key: String, message: String, encrypted: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_ethkey_brainwallet_sign(&error, app, key, message, encrypted)
    let value = String(cString: res!.pointee.value!)
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_string(res)
    if error == 0 {
      resolve(value)
    } else {
      reject("ethkey_sign", error_msg, nil)
    }
  }

  @objc func secureSubstrateSign(_ app: String, key: String, message: String, encrypted: String, legacy: CInt, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    var error: UInt32 = 0
    let res = sn_substrate_brainwallet_sign(&error, app, key, message, encrypted, legacy)
    let value = String(cString: res!.pointee.value!)
    let error_msg = String(cString: res!.pointee.error_msg!)
    destroy_cresult_string(res)
    if error == 0 {
      resolve(value)
    } else {
      reject("substrate_sign", error_msg, nil)
    }
  }
}
