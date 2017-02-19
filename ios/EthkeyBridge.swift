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
	
	@objc func brainWallet(_ seed: NSString, callback: RCTResponseSenderBlock) -> Void {
		let s = tmp_string()
		let tmp_s = String.fromStringPtr(ptr: s!.pointee)
		callback([tmp_s as NSString])
	}
}
