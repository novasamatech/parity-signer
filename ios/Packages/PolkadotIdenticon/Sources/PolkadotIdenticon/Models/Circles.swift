//
//  Circles.swift
//
//
//  Created by Krzysztof Rodak on 24/07/2023.
//

import Foundation
import UIKit

/// `Circle` represents a circle used in generated Identicon.
///
/// It consists of a `CirclePosition` representing its center, a radius, and an RGBA color.
/// `Circle` instances are used in the process of creating visual identifiers or Identicons.
struct Circle: Equatable {
    /// The position of the circle's center.
    let position: CirclePosition

    /// The radius of the circle.
    let radius: Float

    /// The color of the circle in RGBA format.
    let rgba_color: Color
}

/// `CirclePosition` represents the position of a circle's center in a two-dimensional space.
///
/// It is defined by two floating-point numbers representing the x (horizontal) and y (vertical) coordinates of the
/// circle's center.
struct CirclePosition: Equatable {
    /// The x-coordinate of the circle's center.
    let centerX: Float

    /// The y-coordinate of the circle's center.
    let centerY: Float
}
