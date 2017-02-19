//
//  String.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

import Foundation

extension String {
	static func fromStringPtr(ptr: string_ptr) -> String {
		let data = NSData(bytes: UnsafeRawPointer(ptr.ptr), length: ptr.len)
		return String(data: data as Data, encoding: String.Encoding.utf8)!
	}
	
	func asPtr() -> string_ptr {
		let data = self.data(using: String.Encoding.utf8, allowLossyConversion: false)!
		return string_ptr(ptr: (data as NSData).bytes.bindMemory(to: UInt8.self, capacity: data.count), len: data.count)
	}
}
