//
//  Utils.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import Foundation

/**
 * Util: convert hex string spit out by rust code into data
 * uft16 is used for string-native performance
 */
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

/**
 * Decode markdown object from hex-encoded string passed in JSON
 */
extension AttributedString {
    init?(fromHexDocs string: String) {
        try? self.init(markdown: Data(fromHexEncodedString: string) ?? Data(), options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace, failurePolicy: .returnPartiallyParsedIfPossible))
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

/**
 * Base64 screening utils
 */
extension String {
    func encode64() -> String {
        return Data(self.utf8).base64EncodedString()
    }
    
    func decode64() -> String {
        return String(decoding: Data(base64Encoded: self) ?? Data(), as: UTF8.self)
    }
}

extension TransactionCardSet {
    func assemble() -> [TransactionCard] {
        var assembled: [TransactionCard] = []
        assembled.append(contentsOf: self.author ?? [])
        assembled.append(contentsOf: self.error ?? [])
        assembled.append(contentsOf: self.extensions ?? [])
        assembled.append(contentsOf: self.importingDerivations ?? [])
        assembled.append(contentsOf: self.message ?? [])
        assembled.append(contentsOf: self.meta ?? [])
        assembled.append(contentsOf: self.method ?? [])
        assembled.append(contentsOf: self.newSpecs ?? [])
        assembled.append(contentsOf: self.verifier ?? [])
        assembled.append(contentsOf: self.warning ?? [])
        assembled.append(contentsOf: self.typesInfo ?? [])
        return assembled.sorted(by: {
            $0.index < $1.index
        })
    }
}

extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        return self.draft.joined(separator: " ")
    }
}

extension Verifier {
    func show() -> String {
        switch(self.v) {
        case .standard(let value):
            return value
        case .none:
            return "None"
        }
    }
}

extension VerifierValue {
    func show() -> String {
        switch(self) {
        case .standard(let value):
            return value
        }
    }
}
