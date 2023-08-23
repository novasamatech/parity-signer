//
//  ColorTheme.swift
//
//
//  Created by Krzysztof Rodak on 03/08/2023.
//

import UIKit

/// Represents a color theme.
public final class ColorTheme {
    public let hue: CGFloat
    public let saturation: CGFloat
    public let colors: [UIColor]

    /// Initialize a new color theme with the given hue and saturation.
    /// - Parameters:
    ///   - hue: The base hue for the color theme.
    ///   - saturation: The saturation for the color theme, defaulting to 0.5.
    public init(hue: CGFloat, saturation: CGFloat = 0.5) {
        self.hue = hue
        self.saturation = saturation
        colors = [
            // Dark gray
            UIColor(hue: 0, saturation: 0, lightness: ColorUtility.grayscaleLightness(0)),
            // Mid color
            UIColor(correctedHue: hue, saturation: saturation, lightness: ColorUtility.colorLightness(0.5)),
            // Light gray
            UIColor(hue: 0, saturation: 0, lightness: ColorUtility.grayscaleLightness(1)),
            // Light color
            UIColor(correctedHue: hue, saturation: saturation, lightness: ColorUtility.colorLightness(1)),
            // Dark color
            UIColor(correctedHue: hue, saturation: saturation, lightness: ColorUtility.colorLightness(0))
        ]
    }

    /// Given a suggested index, returns another index that combines well with the other selected indices.
    /// - Parameters:
    ///   - index: The index to validate.
    ///   - selected: An array of already selected indices.
    /// - Returns: A validated index.
    public func validateIndex(_ index: Int, selected: [Int]) -> Int {
        ColorUtility.isDuplicate(index: index, values: [0, 4], selected: selected) ||
            ColorUtility.isDuplicate(index: index, values: [2, 3], selected: selected)
            ? 1 : index
    }
}
