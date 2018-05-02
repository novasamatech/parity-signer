// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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
//  String.swift
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//

import Foundation

extension String {
  static func fromStringPtr(ptr: rust_string_ptr) -> String {
    let data = NSData(bytes: UnsafeRawPointer(ptr.ptr), length: ptr.len)
    return String(data: data as Data, encoding: String.Encoding.utf8)!
  }
  
  func asPtr() -> rust_string_ptr {
    let data = self.data(using: String.Encoding.utf8, allowLossyConversion: false)!
    return rust_string_ptr(ptr: (data as NSData).bytes.bindMemory(to: UInt8.self, capacity: data.count), len: data.count)
  }
}
