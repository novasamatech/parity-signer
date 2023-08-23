//
//  UIColor+HSL.swift
//
//
//  Created by Krzysztof Rodak on 03/08/2023.
//

import UIKit

extension UIColor {
    /// Initialize a UIColor based on hue, saturation, and lightness values.
    /// - Parameters:
    ///   - hue: The hue component of the color in the range [0, 1].
    ///   - saturation: The saturation component of the color in the range [0, 1].
    ///   - lightness: The lightness component of the color in the range [0, 1].
    convenience init(hue: CGFloat, saturation: CGFloat, lightness: CGFloat) {
        let m2 = lightness <= 0.5 ? lightness * (saturation + 1) : lightness + saturation - lightness * saturation
        let m1 = lightness * 2 - m2
        self.init(
            red: UIColor.convertHueToRGB(m1, m2, hue * 6 + 2),
            green: UIColor.convertHueToRGB(m1, m2, hue * 6),
            blue: UIColor.convertHueToRGB(m1, m2, hue * 6 - 2),
            alpha: 1
        )
    }

    /// Initialize a UIColor with a hue corrected for perceived brightness.
    /// - Parameters:
    ///   - hue: The hue component of the color in the range [0, 1].
    ///   - saturation: The saturation component of the color in the range [0, 1].
    ///   - lightness: The lightness component of the color in the range [0, 1].
    convenience init(correctedHue hue: CGFloat, saturation: CGFloat, lightness: CGFloat) {
        // Correctors specify perceived middle lightness for each hue.
        let correctors: [CGFloat] = [0.55, 0.5, 0.5, 0.46, 0.6, 0.55, 0.55]
        let corrector = correctors[Int(hue * 6 + 0.5)]

        // Adjust input lightness relative to the corrector.
        let adjustedLightness: CGFloat
        if lightness < 0.5 {
            adjustedLightness = lightness * corrector * 2
        } else {
            adjustedLightness = corrector + (lightness - 0.5) * (1 - corrector) * 2
        }

        self.init(hue: hue, saturation: saturation, lightness: adjustedLightness)
    }

    /// Convert hue to RGB.
    /// - Parameters:
    ///   - m1: The first constant used in conversion, based on lightness and saturation.
    ///   - m2: The second constant used in conversion, based on lightness and saturation.
    ///   - h: The hue used in conversion.
    /// - Returns: The RGB component corresponding to the given hue.
    private static func convertHueToRGB(_ m1: CGFloat, _ m2: CGFloat, _ h: CGFloat) -> CGFloat {
        let adjustedHue: CGFloat
        if h < 0 {
            adjustedHue = h + 6
        } else if h > 6 {
            adjustedHue = h - 6
        } else {
            adjustedHue = h
        }

        switch adjustedHue {
        case ..<1:
            return m1 + (m2 - m1) * adjustedHue
        case ..<3:
            return m2
        case ..<4:
            return m1 + (m2 - m1) * (4 - adjustedHue)
        default:
            return m1
        }
    }
}
