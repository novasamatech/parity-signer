//
//  Circles.swift
//
//
//  Created by Krzysztof Rodak on 20/07/2023.
//

import Foundation
import UIKit

// Information about the circle
struct Circle {
    var x_center: Float
    var y_center: Float
    var radius: Float
    var rgba_color: Color
}

// Function to determine if the point (x, y) is within the circle
func inCircle(x: Int, y: Int, circle: Circle) -> Bool {
    pow(Float(x) - circle.x_center, 2) + pow(Float(y) - circle.y_center, 2) < pow(circle.radius, 2)
}

// Information about circle center position
struct CirclePosition {
    var x_center: Float
    var y_center: Float
}

// Set default positions of small circles in 19-circles icon
func positionCircleSet(center_to_center: Float) -> [CirclePosition] {
    let a = center_to_center
    let b = center_to_center * sqrt(3) / 2
    return [
        CirclePosition(x_center: 0, y_center: -2 * a),
        CirclePosition(x_center: 0, y_center: -a),
        CirclePosition(x_center: -b, y_center: -3 * a / 2),
        CirclePosition(x_center: -2 * b, y_center: -a),
        CirclePosition(x_center: -b, y_center: -a / 2),
        CirclePosition(x_center: -2 * b, y_center: 0),
        CirclePosition(x_center: -2 * b, y_center: a),
        CirclePosition(x_center: -b, y_center: a / 2),
        CirclePosition(x_center: -b, y_center: 3 * a / 2),
        CirclePosition(x_center: 0, y_center: 2 * a),
        CirclePosition(x_center: 0, y_center: a),
        CirclePosition(x_center: b, y_center: 3 * a / 2),
        CirclePosition(x_center: 2 * b, y_center: a),
        CirclePosition(x_center: b, y_center: a / 2),
        CirclePosition(x_center: 2 * b, y_center: 0),
        CirclePosition(x_center: 2 * b, y_center: -a),
        CirclePosition(x_center: b, y_center: -a / 2),
        CirclePosition(x_center: b, y_center: -3 * a / 2),
        CirclePosition(x_center: 0, y_center: 0)
    ]
}

// Function to finalize 19 circles with properly corresponding colors and radius
func getColoredCircles(center_to_center: Float, small_radius: Float, colors: [Color]) -> [Circle] {
    let positions = positionCircleSet(center_to_center: center_to_center)
    var out: [Circle] = []
    for (i, position) in positions.enumerated() {
        let new = Circle(
            x_center: position.x_center,
            y_center: position.y_center,
            radius: small_radius,
            rgba_color: colors[i]
        )
        out.append(new)
    }
    return out
}
