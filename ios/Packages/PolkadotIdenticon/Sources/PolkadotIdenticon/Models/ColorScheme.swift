//
//  ColorScheme.swift
//
//
//  Created by Krzysztof Rodak on 24/07/2023.
//

import Foundation

/// `ColorScheme` represents a color configuration for the identicon generation.
///
/// Each `ColorScheme` consists of a frequency and a list of colorPaletteIndices,
/// which correspond to predefined colors used in the identicon generation.
/// A collection of default color schemes is also provided.
struct ColorScheme: Equatable {
    /// The frequency with which this color scheme is used in identicon generation.
    let frequency: Int

    /// An array of indices referring to specific colors in a predefined color palette.
    let colorPaletteIndices: [Int]

    /// A collection of predefined color schemes used in identicon generation.
    ///
    /// Each `ColorScheme` in this array represents a unique color configuration with a specific frequency and color
    /// index array.
    static let defaultColorSchemes: [ColorScheme] =
        [
            ColorScheme(
                frequency: 1,
                colorPaletteIndices: [0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 1]
            ),
            ColorScheme(frequency: 20, colorPaletteIndices: [0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 5]),
            ColorScheme(frequency: 16, colorPaletteIndices: [1, 2, 3, 1, 2, 4, 5, 5, 4, 1, 2, 3, 1, 2, 4, 5, 5, 4, 0]),
            ColorScheme(frequency: 32, colorPaletteIndices: [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 3]),
            ColorScheme(frequency: 32, colorPaletteIndices: [0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6]),
            ColorScheme(
                frequency: 128,
                colorPaletteIndices: [0, 1, 2, 3, 4, 5, 3, 4, 2, 0, 1, 6, 7, 8, 9, 7, 8, 6, 10]
            ),
            ColorScheme(
                frequency: 128,
                colorPaletteIndices: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 6, 7, 5, 3, 4, 2, 11]
            )
        ]
}
