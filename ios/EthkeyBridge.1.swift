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
		var error: UInt32 = 0
		let address = String.fromRust(ethkey_brainwallet_address(&error, &seed_ptr))
		if error == 0 {
			resolve(address)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func brainWalletSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		print(seed, " + ", message)
		var seed_ptr = seed.asPtr()
		var message_ptr = message.asPtr()
		var error: UInt32 = 0
		let signature = String.fromRust(ethkey_brainwallet_sign(&error, &seed_ptr, &message_ptr))
		if error == 0 {
			resolve(signature)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func rlpItem(_ rlp: String, position: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var rlp_ptr = rlp.asPtr()
		var error: UInt32 = 0
		let item = String.fromRust(rlp_item(&error, &rlp_ptr, position))
		if error == 0 {
			resolve(item)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func keccak(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var error: UInt32 = 0
		let hash = String.fromRust(keccak256(&error, &data_ptr))
		if error == 0 {
			resolve(hash)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func ethSign(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var error: UInt32 = 0
		let hash = String.fromRust(eth_sign(&error, &data_ptr))
		if error == 0 {
			resolve(hash)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func blockiesIcon(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var seed_ptr = seed.asPtr()
		var error: UInt32 = 0
		let icon = String.fromRust(blockies_icon(&error, &seed_ptr))
		if error == 0 {
			resolve(icon)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func randomPhrase(resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var error: UInt32 = 0
		let words = String.fromRust(random_phrase(&error))
		if error == 0 {
			resolve(words)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func encryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var password_ptr = password.asPtr()
		var error: UInt32 = 0
		let encrypted_data = String.fromRust(encrypt_data(&error, &data_ptr, &password_ptr))
		if error == 0 {
			resolve(encrypted_data)
		} else {
			reject("There was an error", nil, nil)
		}
	}

	@objc func decryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
		var data_ptr = data.asPtr()
		var password_ptr = password.asPtr()
		var error: UInt32 = 0
		let decrypted_data = String.fromRust(decrypt_data(&error, &data_ptr, &password_ptr))
		if error == 0 {
			resolve(decrypted_data)
		} else {
			reject("There was an error", nil, nil)
		}
	}
}
