//
//  Utils.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import Foundation

//Util: convert hex string spit out by rust code into data
//uft16 is used for string-native performance
extension Data {
    
    func hexCharValue(u: UInt16) -> UInt8? {
        switch (u) {
        case 0x30 ... 0x39:
            return UInt8(u - 0x30)
        case 0x61 ... 0x66:
            return UInt8(u-0x61 + 10)
        default:
            return nil
        }
    }
    
    init?(fromHexEncodedString string: String) {
        self.init(capacity: string.utf16.count/2)
        var even = true
        var value: UInt8 = 0
        for i in string.utf16 {
            guard let symbol = hexCharValue(u: i) else { return nil }
            if even {
                value = symbol << 4
            } else {
                value += symbol
                self.append(value)
            }
            even = !even
        }
        guard even else { return nil }
    }
}

extension String {
    init?(fromHexDocs string: String) {
        self.init(decoding: (Data(fromHexEncodedString: string) ?? Data()), as: UTF8.self)
    }
}

extension String {
    /**
     * St...ng
     */
    func truncateMiddle(length: Int) -> String {
        if (self.count) > length*2 {
            return self.prefix(length) + "..." + self.suffix(length)
        } else {
            return self
        }
    }
}

//Getting font:
//Text("kusama").font(Font.custom("Web3-Regular", size: 24))
