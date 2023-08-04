//
//  IdenticonImageRenderer.swift
//
//
//  Created by Krzysztof Rodak on 01/08/2023.
//

import CoreGraphics
import UIKit

/// `IdenticonImageRenderer` is responsible for generating an image for a given set of colors.
/// It utilizes `CircleLayoutGenerator` to generate a layout of circles and colors them accordingly.
public final class IdenticonImageRenderer {
    private enum Constants {
        static let fullImageRadiusFactor: Float = 0.5
        static let individualCircleRadiusFactor: Float = 0.15625 // 5 / 32
        static let distanceBetweenCentersFactor: Float = 0.375 // 3 / 8
    }

    private let circleLayoutGenerator: CircleLayoutGenerator

    /// Initializes a new instance of `IdenticonImageRenderer` with a given `CircleLayoutGenerator`.
    ///
    /// - Parameter circleLayoutGenerator: A `CircleLayoutGenerator` used to generate the layout of circles.
    public init(circleLayoutGenerator: CircleLayoutGenerator = CircleLayoutGenerator()) {
        self.circleLayoutGenerator = circleLayoutGenerator
    }

    /// Generates an image of the specified size and colors the circles according to the supplied colors array.
    ///
    /// - Parameters:
    ///   - size: The size (width and height) of the image.
    ///   - colors: An array of colors to be used to color the circles.
    /// - Returns: A `UIImage` representing the generated Identicon image.
    func generateImage(size: CGFloat, colors: [Color]) -> UIImage {
        let fullImageRadius = Float(size) * Constants.fullImageRadiusFactor
        let individualCircleRadius = fullImageRadius * Constants.individualCircleRadiusFactor
        let distanceBetweenCenters = fullImageRadius * Constants.distanceBetweenCentersFactor
        let largeBackgroundCircle = Circle(
            position: .init(centerX: 0, centerY: 0),
            radius: fullImageRadius,
            rgba_color: Color.foregroundColor
        )
        let smallerCirclesSet = circleLayoutGenerator.generateCircles(
            distanceBetweenCenters: distanceBetweenCenters,
            circleRadius: individualCircleRadius,
            colors: colors
        )

        return createImage(
            size: size,
            largeBackgroundCircle: largeBackgroundCircle,
            smallerCirclesSet: smallerCirclesSet,
            fullImageRadius: fullImageRadius
        )
    }
}

private extension IdenticonImageRenderer {
    func createImage(
        size: CGFloat,
        largeBackgroundCircle: Circle,
        smallerCirclesSet: [Circle],
        fullImageRadius: Float
    ) -> UIImage {
        UIGraphicsImageRenderer(size: CGSize(width: size, height: size)).image { ctx in
            drawLargeBackgroundCircle(ctx: ctx, largeBackgroundCircle: largeBackgroundCircle, size: size)
            smallerCirclesSet.forEach {
                drawCircle(ctx: ctx, circle: $0, fullImageRadius: fullImageRadius, size: size)
            }
        }
    }

    func drawLargeBackgroundCircle(
        ctx: UIGraphicsImageRendererContext,
        largeBackgroundCircle: Circle,
        size: CGFloat
    ) {
        let fullImageRect = CGRect(x: 0, y: 0, width: size, height: size)
        ctx.cgContext.setFillColor(largeBackgroundCircle.rgba_color.toCGColor())
        ctx.cgContext.fillEllipse(in: fullImageRect)
    }

    func drawCircle(
        ctx: UIGraphicsImageRendererContext,
        circle: Circle,
        fullImageRadius: Float,
        size: CGFloat
    ) {
        let circleRectangle = calculateCircleRectangle(circle: circle, fullImageRadius: fullImageRadius, size: size)
        ctx.cgContext.setFillColor(circle.rgba_color.toCGColor())
        ctx.cgContext.fillEllipse(in: circleRectangle)
    }

    /// Calculate the rectangle that will enclose a circle in the final image.
    ///
    /// - Parameters:
    ///   - circle: The circle for which to calculate the rectangle.
    ///   - fullImageRadius: The radius of the full image.
    ///   - size: The size of the image.
    /// - Returns: A `CGRect` that will enclose the circle.
    func calculateCircleRectangle(
        circle: Circle,
        fullImageRadius: Float,
        size: CGFloat
    ) -> CGRect {
        let x = circle.position.centerX * Float(size) / (2 * fullImageRadius) + Float(size) / 2
        let y = circle.position.centerY * Float(size) / (2 * fullImageRadius) + Float(size) / 2
        let circleRectX = CGFloat(x - circle.radius)
        let circleRectY = CGFloat(y - circle.radius)
        let circleSize = CGFloat(2 * circle.radius)
        return CGRect(x: circleRectX, y: circleRectY, width: circleSize, height: circleSize)
    }
}
