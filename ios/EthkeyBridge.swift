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
	@objc func brainWalletAddress(_ seed: String, callback: RCTResponseSenderBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let address_rust_str = ethkey_keypair_address(keypair_ptr)
		let address_rust_str_ptr = rust_string_ptr(address_rust_str)
		let address = String.fromStringPtr(ptr: address_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(address_rust_str_ptr)
		rust_string_destroy(address_rust_str)
		ethkey_keypair_destroy(keypair_ptr)
		callback([address])
	}
	
	@objc func brainWalletSecret(_ seed: String, callback: RCTResponseSenderBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let secret_rust_str = ethkey_keypair_secret(keypair_ptr)
		let secret_rust_str_ptr = rust_string_ptr(secret_rust_str)
		let secret = String.fromStringPtr(ptr: secret_rust_str_ptr!.pointee)
		rust_string_ptr_destroy(secret_rust_str_ptr)
		rust_string_destroy(secret_rust_str)
		ethkey_keypair_destroy(keypair_ptr)
		callback([secret])
	}
}
