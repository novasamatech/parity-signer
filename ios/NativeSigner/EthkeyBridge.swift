//
//  EthkeyBridge.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2019 Facebook. All rights reserved.
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

  @objc func blake2s(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
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
}
