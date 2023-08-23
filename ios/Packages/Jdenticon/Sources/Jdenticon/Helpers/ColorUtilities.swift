//
//  ColorUtilities.swift
//
//
//  Created by Krzysztof Rodak on 04/08/2023.
//

import Foundation

/// Helper struct for color-related calculations.
struct ColorUtility {
    /// Computes lightness for a color based on the given value.
    /// - Parameter value: The value for which to compute the lightness.
    /// - Returns: The computed lightness.
    static func colorLightness(_ value: CGFloat) -> CGFloat {
        lightness(value, min: 0.4, max: 0.8)
    }

    /// Computes lightness for a grayscale color based on the given value.
    /// - Parameter value: The value for which to compute the lightness.
    /// - Returns: The computed lightness.
    static func grayscaleLightness(_ value: CGFloat) -> CGFloat {
        lightness(value, min: 0.3, max: 0.9)
    }

    /// Computes lightness for a color based on the given value, minimum, and maximum.
    /// - Parameters:
    ///   - value: The value for which to compute the lightness.
    ///   - min: The minimum lightness.
    ///   - max: The maximum lightness.
    /// - Returns: The computed lightness.
    static func lightness(_ value: CGFloat, min: CGFloat, max: CGFloat) -> CGFloat {
        let lightness = min + value * (max - min)
        return Swift.min(1, Swift.max(0, lightness))
    }

    /// Checks if a given index is a duplicate within a certain set of values and selected values.
    /// - Parameters:
    ///   - index: The index to check.
    ///   - values: The set of values to compare against.
    ///   - selected: The set of selected values.
    /// - Returns: `true` if the index is a duplicate; `false` otherwise.
    static func isDuplicate(index: Int, values: [Int], selected: [Int]) -> Bool {
        guard values.contains(index) else {
            return false
        }
        return values.contains(where: { selected.contains($0) })
    }
}
