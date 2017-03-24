//
//  String.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

import Foundation

extension String {
	static func fromStringPtr(ptr: rust_string_ptr) -> String {
		let data = NSData(bytes: UnsafeRawPointer(ptr.ptr), length: ptr.len)
		return String(data: data as Data, encoding: String.Encoding.utf8)!
	}
	
	static func fromBinaryStringPtr(ptr: rust_string_ptr) -> String {
		let data = NSData(bytes: UnsafeRawPointer(ptr.ptr), length: ptr.len)
		return data.base64EncodedString(options: NSData.Base64EncodingOptions.init(rawValue: 0))
	}
	
	func asPtr() -> rust_string_ptr {
		let data = self.data(using: String.Encoding.utf8, allowLossyConversion: false)!
		return rust_string_ptr(ptr: (data as NSData).bytes.bindMemory(to: UInt8.self, capacity: data.count), len: data.count)
	}
}
