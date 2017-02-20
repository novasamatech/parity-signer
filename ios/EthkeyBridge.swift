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
	@objc func show(_ name: String) {
		NSLog("%@", name);
	}
	
	@objc func brainWalletAddress(_ seed: String, callback: RCTResponseSenderBlock) -> Void {
		var seed_ptr = seed.asPtr()
		let keypair_ptr = ethkey_keypair_brainwallet(&seed_ptr)
		let address_ptr = ethkey_keypair_address(keypair_ptr)
		let address = String.fromStringPtr(ptr: address_ptr!.pointee)
		ethkey_keypair_destroy(keypair_ptr)
		ethkey_string_destroy(address_ptr)
		callback([address])
	}
}
