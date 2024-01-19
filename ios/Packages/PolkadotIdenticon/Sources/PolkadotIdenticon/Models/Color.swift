//
//  Color.swift
//
//
//  Created by Krzysztof Rodak on 24/07/2023.
//

import UIKit

/// `Color` represents a color using RGBA (Red, Green, Blue, Alpha) components.
///
/// Each component is represented by a `UInt8` (0-255). The `Color` structure provides
/// pre-defined static colors for foreground and background, and supports the conversion
/// to `CGColor` for use in drawing operations.
struct Color: Equatable {
    /// The red component of the color.
    let red: UInt8

    /// The green component of the color.
    let green: UInt8

    /// The blue component of the color.
    let blue: UInt8

    /// The alpha component of the color.
    let alpha: UInt8

    /// A predefined color white with an alpha of 0, typically used as the background color.
    ///
    /// - Returns: A `Color` instance representing the background color.
    static let backgroundColor: Color = .init(red: 255, green: 255, blue: 255, alpha: 0)

    /// A predefined gray color with an alpha of 255, typically used as the foreground color.
    ///
    /// - Returns: A `Color` instance representing the foreground color.
    static let foregroundColor: Color = .init(red: 238, green: 238, blue: 238, alpha: 255)
}

extension Color {
    /// Converts the `Color` instance into a `CGColor` which can be used in UIKit's drawing APIs.
    ///
    /// - Returns: A `CGColor` representation of the color.
    func toCGColor() -> CGColor {
        UIColor(
            red: CGFloat(red) / 255.0,
            green: CGFloat(green) / 255.0,
            blue: CGFloat(blue) / 255.0,
            alpha: CGFloat(alpha) / 255.0
        ).cgColor
    }
}
