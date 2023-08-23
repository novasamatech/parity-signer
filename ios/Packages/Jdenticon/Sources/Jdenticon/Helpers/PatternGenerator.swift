//
//  PatternGenerator.swift
//
//
//  Created by Krzysztof Rodak on 05/08/2023.
//

import Foundation

/// `PatternGenerator` is a class responsible for extracting pseudorandom digits from a provided hash data.
/// These digits can be used to generate a unique and reproducible pattern.
public class PatternGenerator {
    public init() {}
    /// Converts a hash to a byte array where each byte represents a digit of the hash.
    ///
    /// The method iterates through each byte in the hash, splitting it into two four-bit digits
    /// and storing them in the array. The high-order bits are stored before the low-order bits.
    ///
    /// If the size of the resulting array is less than 12, it will be filled up to 12 with zeros. If the
    /// size is greater than the hash length * 2, only the first `size` digits are used.
    ///
    /// - Parameters:
    ///   - hash: The hash to extract the digits from. This hash should be a `Data` object containing bytes.
    ///   - size: The desired size of the resulting array. It should be a `CGFloat` which will be converted to `Int` for
    /// array count.
    /// - Returns: An array of `UInt8` digits extracted from the hash. Each digit is a value from 0 to 15.
    func extractDigits(fromHash hash: Data, size: CGFloat) -> [UInt8] {
        var digits = [UInt8](repeating: 0, count: max(12, Int(size * 2)))
        var index = 0
        for byte in hash {
            if index >= digits.count { break }
            digits[index] = (byte & 0xF0) >> 4
            digits[index + 1] = byte & 0x0F
            index += 2
        }
        return digits
    }
}
