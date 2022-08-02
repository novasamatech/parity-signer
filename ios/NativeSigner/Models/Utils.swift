//
//  Utils.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import Foundation

/// Util: convert hex string spit out by rust code into data
/// uft16 is used for string-native performance
extension Data {
    func hexCharValue(character: UInt16) -> UInt8? {
        switch character {
        case 0x30 ... 0x39:
            return UInt8(character - 0x30)
        case 0x61 ... 0x66:
            return UInt8(character - 0x61 + 10)
        default:
            return nil
        }
    }

    init?(fromHexEncodedString string: String) {
        self.init(capacity: string.utf16.count / 2)
        var even = true
        var value: UInt8 = 0
        for character in string.utf16 {
            guard let symbol = hexCharValue(character: character) else { return nil }
            if even {
                value = symbol << 4
            } else {
                value += symbol
                append(value)
            }
            even = !even
        }
        guard even else { return nil }
    }
}

/// Decode markdown object from hex-encoded string passed in JSON
extension AttributedString {
    init?(fromHexDocs string: String) {
        try? self.init(
            markdown: Data(fromHexEncodedString: string) ?? Data(),
            options: AttributedString.MarkdownParsingOptions(
                interpretedSyntax: .inlineOnlyPreservingWhitespace,
                failurePolicy: .returnPartiallyParsedIfPossible
            )
        )
    }
}

extension String {
    /// St...ng
    func truncateMiddle(length: Int) -> String {
        if count > length * 2 {
            return prefix(length) + "..." + suffix(length)
        } else {
            return self
        }
    }
}

extension TransactionCardSet {
    func assemble() -> [TransactionCard] {
        var assembled: [TransactionCard] = []
        assembled.append(contentsOf: author ?? [])
        assembled.append(contentsOf: error ?? [])
        assembled.append(contentsOf: extensions ?? [])
        assembled.append(contentsOf: importingDerivations ?? [])
        assembled.append(contentsOf: message ?? [])
        assembled.append(contentsOf: meta ?? [])
        assembled.append(contentsOf: method ?? [])
        assembled.append(contentsOf: newSpecs ?? [])
        assembled.append(contentsOf: verifier ?? [])
        assembled.append(contentsOf: warning ?? [])
        assembled.append(contentsOf: typesInfo ?? [])
        return assembled.sorted(by: {
            $0.index < $1.index
        })
    }
}

extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        draft.joined(separator: " ")
    }
}

extension Verifier {
    func show() -> String {
        switch v {
        case let .standard(value):
            return value[0]
        case .none:
            return "None"
        }
    }
}

extension VerifierValue {
    func show() -> String {
        switch self {
        case let .standard(value):
            return value[0]
        }
    }
}
