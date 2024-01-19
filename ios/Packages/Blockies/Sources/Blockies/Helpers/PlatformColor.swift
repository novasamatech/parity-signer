//
//  PlatformColor.swift
//
//
//  Created by Krzysztof Rodak on 14/07/2023.
//

import Foundation

#if canImport(UIKit)
    import UIKit

    public typealias PlatformColor = UIColor
#elseif canImport(AppKit)
    import AppKit

    public typealias PlatformColor = NSColor
#endif

public extension PlatformColor {
    /// Initializes Color with the given HSL color values.
    ///
    /// - parameter h: The hue value, a number between 0 and 360.
    /// - parameter s: The saturation value, a number between 0 and 1.
    /// - parameter l: The lightness value, a number between 0 and 1.
    convenience init?(hue: Double, saturation: Double, lightness: Double, alpha: CGFloat = 1) {
        guard (0 ... 360).contains(hue), (0 ... 1).contains(saturation), (0 ... 1).contains(lightness) else {
            return nil
        }

        let chroma = (1 - abs(2 * lightness - 1)) * saturation
        let x = chroma * (1 - abs(((hue / 60).truncatingRemainder(dividingBy: 2)) - 1))
        let matchValue = lightness - (chroma / 2)

        let (tempRed, tempGreen, tempBlue): (Double, Double, Double)
        switch hue {
        case 0 ..< 60: (tempRed, tempGreen, tempBlue) = (chroma, x, 0)
        case 60 ..< 120: (tempRed, tempGreen, tempBlue) = (x, chroma, 0)
        case 120 ..< 180: (tempRed, tempGreen, tempBlue) = (0, chroma, x)
        case 180 ..< 240: (tempRed, tempGreen, tempBlue) = (0, x, chroma)
        case 240 ..< 300: (tempRed, tempGreen, tempBlue) = (x, 0, chroma)
        case 300 ..< 360: (tempRed, tempGreen, tempBlue) = (chroma, 0, x)
        default: return nil
        }

        self.init(
            red: CGFloat(tempRed + matchValue),
            green: CGFloat(tempGreen + matchValue),
            blue: CGFloat(tempBlue + matchValue),
            alpha: alpha
        )
    }
}
