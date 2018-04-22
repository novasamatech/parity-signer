//
//  EthkeyBridge.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

import Foundation

@objc(EthkeyBridge)
class EthkeyBridge: NSObject {
	@objc func brainWalletAddress(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let address_rust_str = ethkey_keypair_address(keypair_ptr)
		let address_rust_str_ptr = rust_string_ptr(address_rust_str)
		let address = String.fromStringPtr(ptr: address_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(address_rust_str_ptr)
		rust_string_destroy(address_rust_str)
		ethkey_keypair_destroy(keypair_ptr)
		resolve(address)
	}
	
	@objc func brainWalletSecret(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let secret_rust_str = ethkey_keypair_secret(keypair_ptr)
		let secret_rust_str_ptr = rust_string_ptr(secret_rust_str)
		let secret = String.fromStringPtr(ptr: secret_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(secret_rust_str_ptr)
		rust_string_destroy(secret_rust_str)
		ethkey_keypair_destroy(keypair_ptr)
		resolve(secret)
	}
	
	@objc func brainWalletSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		print(seed, " + ", message)
		var seed_ptr = seed.asPtr()
		var message_ptr = message.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let signature_rust_str = ethkey_keypair_sign(keypair_ptr, &message_ptr)
		let signature_rust_str_ptr = rust_string_ptr(signature_rust_str)
		let signature = String.fromStringPtr(ptr: signature_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(signature_rust_str_ptr)
		rust_string_destroy(signature_rust_str)
		ethkey_keypair_destroy(keypair_ptr)
		resolve(signature)
	}
	
	@objc func rlpItem(_ rlp: String, position: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var rlp_ptr = rlp.asPtr()
		var error: UInt32 = 0
		let item_rust_str = rlp_item(&rlp_ptr, position, &error)
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
		var data_ptr = data.asPtr()
		let hash_rust_str = keccak256(&data_ptr)
		let hash_rust_str_ptr = rust_string_ptr(hash_rust_str)
		let hash = String.fromStringPtr(ptr: hash_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(hash_rust_str_ptr)
		rust_string_destroy(hash_rust_str)
		resolve(hash)
	}
	
	@objc func ethSign(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		let hash_rust_str = eth_sign(&data_ptr)
		let hash_rust_str_ptr = rust_string_ptr(hash_rust_str)
		let hash = String.fromStringPtr(ptr: hash_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(hash_rust_str_ptr)
		rust_string_destroy(hash_rust_str)
		resolve(hash)
	}
	
	@objc func blockiesIcon(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let icon_rust_str = blockies_icon(&seed_ptr)
		let icon_rust_str_ptr = rust_string_ptr(icon_rust_str)
		let icon = String.fromStringPtr(ptr: icon_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(icon_rust_str_ptr)
		rust_string_destroy(icon_rust_str)
		resolve(icon)
	}
	
	@objc func randomPhrase(_ words: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		let words_rust_str = random_phrase(words)
		let words_rust_str_ptr = rust_string_ptr(words_rust_str)
		let words = String.fromStringPtr(ptr: words_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(words_rust_str_ptr)
		rust_string_destroy(words_rust_str)
		resolve(words)
	}
	
	@objc func encryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var password_ptr = password.asPtr()
		let encrypted_data_rust_str = encrypt_data(&data_ptr, &password_ptr)
		let encrypted_data_rust_str_ptr = rust_string_ptr(encrypted_data_rust_str)
		let encrypted_data = String.fromStringPtr(ptr: encrypted_data_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(encrypted_data_rust_str_ptr)
		rust_string_destroy(encrypted_data_rust_str)
		resolve(encrypted_data)
	}
	
	@objc func decryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var password_ptr = password.asPtr()
		var error: UInt32 = 0
		let decrypted_data_rust_str = decrypt_data(&data_ptr, &password_ptr, &error)
		let decrypted_data_rust_str_ptr = rust_string_ptr(decrypted_data_rust_str)
		let decrypted_data = String.fromStringPtr(ptr: decrypted_data_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(decrypted_data_rust_str_ptr)
		rust_string_destroy(decrypted_data_rust_str)
		if error == 0 {
			resolve(decrypted_data)
		} else if error == 1{
			reject("invalid data", nil, nil)
		} else {
			reject("invalid password", nil, nil)
		}
	}
}
