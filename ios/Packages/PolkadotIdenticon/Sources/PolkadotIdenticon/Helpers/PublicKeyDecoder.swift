//
//  PublicKeyDecoder.swift
//
//
//  Created by Krzysztof Rodak on 23/07/2023.
//

import Foundation

/// Represents a public key in various possible formats.
///
/// This enum can hold a public key in three formats: raw `Data`, a `hex` encoded string, or a `base58` encoded string.
public enum PublicKey {
    /// Represents a public key as raw `Data`.
    ///
    /// The associated `Data` value is a byte buffer containing the raw bytes of the public key.
    case data(Data)

    /// Represents a public key as a `hex` encoded string.
    ///
    /// The associated `String` value is a hexadecimal string representation of the public key. Each byte of the key is
    /// represented as a two-digit hexadecimal number, from `00` to `ff`.
    case hex(String)

    /// Represents a public key as a `base58` encoded string.
    ///
    /// The associated `String` value is a Base58 string representation of the public key. Base58 is a binary-to-text
    /// encoding scheme that is designed to be human-readable and that minimizes the risk of mistyping.
    case base58(String)
}

/// `PublicKeyDecoder` converts public keys from various formats to byte arrays.
public final class PublicKeyDecoder {
    private enum Constants {
        static let base58Alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    }

    public init() {}

    /// Convert a public key to a byte array.
    /// - Parameter publicKey: The public key to convert.
    /// - Returns: The public key as a byte array.
    func keyAsData(_ publicKey: PublicKey) -> [UInt8] {
        switch publicKey {
        case let .data(data):
            return [UInt8](data)
        case let .hex(string):
            return hexToBytes(hex: string)
        case let .base58(string):
            return base58ToBytes(base58: string)
        }
    }
}

private extension PublicKeyDecoder {
    /// Convert a hexadecimal string to a byte array.
    /// - Parameter hex: The hexadecimal string to convert.
    /// - Returns: The byte array.
    func hexToBytes(hex: String) -> [UInt8] {
        var startIndex = hex.startIndex
        var bytes = [UInt8]()
        while startIndex < hex.endIndex {
            let endIndex = hex.index(startIndex, offsetBy: 2)
            let substr = hex[startIndex ..< endIndex]
            if let byte = UInt8(substr, radix: 16) {
                bytes.append(byte)
            }
            startIndex = endIndex
        }
        return bytes
    }

    /// Convert a base58 string to a byte array.
    /// - Parameter base58: The base58 string to convert.
    /// - Returns: The byte array.
    func base58ToBytes(base58: String) -> [UInt8] {
        var bytes = [UInt8](repeating: 0, count: base58.count)
        for (i, c) in base58.enumerated() {
            if let charIndex = Constants.base58Alphabet.firstIndex(of: c) {
                var carry = Constants.base58Alphabet.distance(from: Constants.base58Alphabet.startIndex, to: charIndex)
                var j = base58.count - 1 - i
                while j >= 0, carry != 0 {
                    carry += 58 * Int(bytes[j])
                    bytes[j] = UInt8(carry % 256)
                    carry /= 256
                    j -= 1
                }
            }
        }
        let firstNonZero = bytes.firstIndex(where: { $0 != 0 }) ?? bytes.endIndex
        return Array(bytes[firstNonZero...])
    }
}
