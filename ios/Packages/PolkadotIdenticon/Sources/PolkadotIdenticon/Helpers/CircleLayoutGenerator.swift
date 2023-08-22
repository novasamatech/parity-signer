//
//  CircleLayoutGenerator.swift
//
//
//  Created by Krzysztof Rodak on 24/07/2023.
//

import Foundation

public final class CircleLayoutGenerator {
    public init() {}
    /// Returns an array of circles with assigned positions, radii and colors.
    /// The circles are positioned in a radial layout and each is assigned a color from the provided array.
    /// - Parameters:
    ///   - distanceBetweenCenters: The distance between the centers of adjacent circles.
    ///   - circleRadius: The radius of each circle.
    ///   - colors: An array of colors to be assigned to the circles. The colors are assigned in the order they appear
    /// in the array.
    /// - Returns: An array of circles with assigned positions, radii and colors.
    func generateCircles(distanceBetweenCenters: Float, circleRadius: Float, colors: [Color]) -> [Circle] {
        calculateCirclePositions(distanceBetweenCenters: distanceBetweenCenters)
            .enumerated()
            .map { index, position in Circle(
                position: .init(centerX: position.centerX, centerY: position.centerY),
                radius: circleRadius,
                rgba_color: colors[index]
            ) }
    }
}

private extension CircleLayoutGenerator {
    /// Calculates the positions for the centers of the circles in a radial layout.
    ///
    /// The layout is as follows:
    ///
    ///                             0
    ///
    ///                    2               17
    ///
    ///           3                 1              15
    ///
    ///                    4               16
    ///
    ///           5                18              14
    ///
    ///                    7               13
    ///
    ///           6                10              12
    ///
    ///                    8               11
    ///
    ///                             9
    ///
    /// - Parameter distanceBetweenCenters: The distance between the centers of adjacent circles.
    /// - Returns: An array of positions for the circle centers.
    func calculateCirclePositions(distanceBetweenCenters: Float) -> [CirclePosition] {
        let a = distanceBetweenCenters
        let b = distanceBetweenCenters * sqrt(3) / 2
        return [
            CirclePosition(centerX: 0, centerY: -2 * a),
            CirclePosition(centerX: 0, centerY: -a),
            CirclePosition(centerX: -b, centerY: -3 * a / 2),
            CirclePosition(centerX: -2 * b, centerY: -a),
            CirclePosition(centerX: -b, centerY: -a / 2),
            CirclePosition(centerX: -2 * b, centerY: 0),
            CirclePosition(centerX: -2 * b, centerY: a),
            CirclePosition(centerX: -b, centerY: a / 2),
            CirclePosition(centerX: -b, centerY: 3 * a / 2),
            CirclePosition(centerX: 0, centerY: 2 * a),
            CirclePosition(centerX: 0, centerY: a),
            CirclePosition(centerX: b, centerY: 3 * a / 2),
            CirclePosition(centerX: 2 * b, centerY: a),
            CirclePosition(centerX: b, centerY: a / 2),
            CirclePosition(centerX: 2 * b, centerY: 0),
            CirclePosition(centerX: 2 * b, centerY: -a),
            CirclePosition(centerX: b, centerY: -a / 2),
            CirclePosition(centerX: b, centerY: -3 * a / 2),
            CirclePosition(centerX: 0, centerY: 0)
        ]
    }
}
