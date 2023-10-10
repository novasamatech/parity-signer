//
//  HashGenerator.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import CommonCrypto
import Foundation

final class HashGenerator {
    private enum Constants {
        static let minimumHexStringRegex = try! NSRegularExpression(pattern: "^[0-9a-fA-F]{11,}$", options: [])
    }

    func keepOrCreateHash(_ hashOrValue: String) -> String {
        if isValidHash(hashOrValue) {
            hashOrValue
        } else {
            computeHash(for: hashOrValue)
        }
    }
}

private extension HashGenerator {
    func isValidHash(_ hashCandidate: String) -> Bool {
        Constants.minimumHexStringRegex.firstMatch(
            in: hashCandidate,
            options: [],
            range: NSRange(hashCandidate.startIndex..., in: hashCandidate)
        ) != nil
    }

    func computeHash(for value: String) -> String {
        let data = Data(value.utf8)
        var digest = [UInt8](repeating: 0, count: Int(CC_SHA1_DIGEST_LENGTH))
        data.withUnsafeBytes {
            _ = CC_SHA1($0.baseAddress, CC_LONG(data.count), &digest)
        }
        let hashString = digest.map { String(format: "%02hhx", $0) }.joined()
        return hashString
    }
}
