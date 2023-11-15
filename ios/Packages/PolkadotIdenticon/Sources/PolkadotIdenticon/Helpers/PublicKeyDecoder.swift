//
//  PublicKeyDecoder.swift
//
//
//  Created by Krzysztof Rodak on 23/07/2023.
//

import BigInt
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
        static let base58Alphabet = [UInt8]("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".utf8)
        static let zero = BigUInt(0)
        static let radix = BigUInt(Constants.base58Alphabet.count)
    }

    public init() {}

    /// Convert a public key to a byte array.
    /// - Parameter publicKey: The public key to convert.
    /// - Returns: The public key as a byte array.
    func keyAsData(_ publicKey: PublicKey) -> [UInt8] {
        switch publicKey {
        case let .data(data):
            [UInt8](data)
        case let .hex(string):
            hexToBytes(hex: string)
        case let .base58(string):
            base58ToBytes(base58: string)
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
        var currentRadixPower = BigUInt(1)
        let base58Bytes: [UInt8] = Array(base58.utf8)

        // Convert base58 characters to their corresponding numeric values
        let base58NumericValue = base58Bytes.reversed()
            .enumerated()
            .reduce(into: Constants.zero) { result, indexedCharacter in
                guard let base58CharacterIndex = Constants.base58Alphabet.firstIndex(of: indexedCharacter.element)
                else { return }
                result += (currentRadixPower * BigUInt(base58CharacterIndex))
                currentRadixPower *= Constants.radix
            }

        let decodedBytes = base58NumericValue.serialize()

        var result = Array(base58Bytes.prefix { $0 == Constants.base58Alphabet[0] }) + decodedBytes
        // Drop 42 substrate prefix and last 2 elements
        result = Array(result.dropFirst().dropLast(2))

        return result
    }
}
