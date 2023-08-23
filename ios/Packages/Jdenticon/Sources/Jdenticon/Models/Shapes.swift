//
//  Shapes.swift
//
//
//  Created by Krzysztof Rodak on 04/08/2023.
//

import CoreGraphics

/// A protocol that represents a geometric shape.
///
/// Types that conform to the `Shape` protocol can be used to draw shapes in a given graphics context.
/// These shapes are expected to fit within a square of the provided size. The index provided can be used
/// to make adjustments or variations in the way the shape is drawn.
protocol Shape {
    /// Draws the conforming shape in a given graphics context.
    ///
    /// This method is required to be implemented by any type conforming to `Shape`. It is called to
    /// draw the shape in the provided `CGContext`.
    ///
    /// - Parameters:
    ///   - context: The graphics context in which to draw the shape.
    ///   - size: The size of the square area in the context in which the shape should fit.
    ///   - index: An index that can be used to adjust or vary how the shape is drawn.
    func draw(in context: CGContext, size: CGFloat, index: Int)
}

/// A shape representing a square with a corner cut off.
struct CutCorner: Shape {
    /// Draws the CutCorner shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let cutSize = size * 0.42
        context.addPolygon(withPoints: [
            CGPoint(x: 0, y: 0),
            CGPoint(x: size, y: 0),
            CGPoint(x: size, y: size - cutSize * 2),
            CGPoint(x: size - cutSize, y: size),
            CGPoint(x: 0, y: size)
        ])
    }
}

/// A shape representing a triangle positioned on the side of a square.
struct SideTriangle: Shape {
    /// Draws the SideTriangle shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let halfWidth = size / 2
        let adjustedHeight = size * 0.8
        context.addTriangle(
            inRect: CGRect(x: size - halfWidth, y: 0, width: halfWidth, height: adjustedHeight),
            fromCorner: 2
        )
    }
}

/// A shape representing a square centered within another square.
struct CenteredSquare: Shape {
    /// Draws the CenteredSquare shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let thirdOfSize = size / 3
        context.addRect(CGRect(x: thirdOfSize, y: thirdOfSize, width: size - thirdOfSize, height: size - thirdOfSize))
    }
}

/// A shape representing a square positioned in the corner of another square.
struct CornerSquare: Shape {
    /// Draws the CornerSquare shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let inner = size * 0.1
        let outer = max(1, size * 0.25)
        context.addRect(CGRect(x: outer, y: outer, width: size - inner - outer, height: size - inner - outer))
    }
}

/// A shape representing a circle that is not centered within a square.
struct OffCenterCircle: Shape {
    /// Draws the OffCenterCircle shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let offset = size * 0.15
        let circleSize = size * 0.5
        context.addCircle(
            withOrigin: CGPoint(x: size - circleSize - offset, y: size - circleSize - offset), radius: circleSize / 2,
            clockwise: true
        )
    }
}

/// A shape representing a triangle inscribed within a square.
struct NegativeTriangle: Shape {
    /// Draws the NegativeTriangle shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let innerSize = size * 0.1
        let outerSize = innerSize * 4

        context.addRect(CGRect(x: 0, y: 0, width: size, height: size))
        context.addPolygon(withPoints: [
            CGPoint(x: outerSize, y: outerSize),
            CGPoint(x: size - innerSize, y: outerSize),
            CGPoint(x: outerSize + (size - outerSize - innerSize) / 2, y: size - innerSize)
        ].reversed())
    }
}

/// A shape representing a square with a corner cut off.
struct CutSquare: Shape {
    /// Draws the CutSquare shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addPolygon(
            withPoints: [
                CGPoint(x: 0, y: 0),
                CGPoint(x: size, y: 0),
                CGPoint(x: size, y: size * 0.7),
                CGPoint(x: size * 0.4, y: size * 0.4),
                CGPoint(x: size * 0.7, y: size),
                CGPoint(x: 0, y: size)
            ]
        )
    }
}

/// A shape representing a triangle within a square in the corner and half of the square filled.
struct CornerPlusTriangle: Shape {
    // Draws the CornerPlusTriangle shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addRect(CGRect(x: 0, y: 0, width: size, height: size / 2))
        context.addRect(CGRect(x: 0, y: size / 2, width: size / 2, height: size / 2))
        context.addTriangle(
            inRect: CGRect(x: size / 2, y: size / 2, width: size / 2, height: size / 2),
            fromCorner: 1
        )
    }
}

/// A shape representing a square with another square cut out from the center.
struct NegativeSquare: Shape {
    /// Draws the NegativeSquare shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let innerSize = size * 0.14
        let outerSize = size * 0.35

        context.addRectangle(CGRect(x: 0, y: 0, width: size, height: size))
        context.addRectangle(
            CGRect(
                x: outerSize,
                y: outerSize,
                width: size - outerSize - innerSize,
                height: size - outerSize - innerSize
            ),
            inverted: true
        )
    }
}

/// A shape representing a circle cut out from a square.
struct NegativeCircle: Shape {
    /// Draws the NegativeCircle shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let innerSize = size * 0.12
        let outerSize = innerSize * 3

        context.addRectangle(CGRect(x: 0, y: 0, width: size, height: size), inverted: true)
        context.addCircle(
            withOrigin: CGPoint(x: outerSize, y: outerSize),
            radius: (size - innerSize - outerSize) / 2,
            clockwise: false
        )
    }
}

/// A shape representing a rhombus cut out from a square.
struct NegativeRhombus: Shape {
    /// Draws the NegativeRhombus shape within the given context.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let marginSize = size * 0.25
        context.addRectangle(CGRect(x: 0, y: 0, width: size, height: size))
        context.addRhombus(
            inRect: CGRect(x: marginSize, y: marginSize, width: size - marginSize, height: size - marginSize),
            inverted: true
        )
    }
}

/// A `ConditionalCircle` structure represents a shape that conditionally renders a circle.
/// This shape will only be rendered if the `index` provided to the `draw` function equals 0.
struct ConditionalCircle: Shape {
    /// This method draws a `ConditionalCircle` shape on the given `CGContext`.
    /// The circle will only be drawn if `index` equals 0.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index: Int) {
        let margin = size * 0.4
        let circleSize = size * 1.2
        if index == 0 {
            context.addCircle(withOrigin: CGPoint(x: margin, y: margin), radius: circleSize / 2, clockwise: true)
        }
    }
}

/// A `HalfTriangle` structure represents a triangle shape that covers half of the drawing area.
struct HalfTriangle: Shape {
    /// This method draws a `HalfTriangle` shape on the given `CGContext`.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addTriangle(inRect: CGRect(x: size / 2, y: size / 2, width: size / 2, height: size / 2), fromCorner: 3)
    }
}

/// A `Triangle` structure represents a triangle shape which covers the entire drawing area.
/// The direction of the triangle depends on the `corner` property.
struct Triangle: Shape {
    let corner: Int

    /// Creates a new `Triangle` with a specified `corner`.
    ///
    /// - Parameter corner: The corner of the rectangle in which the triangle will be placed.
    ///
    /// - Returns: A new `Triangle` instance.
    init(corner: Int = 0) {
        self.corner = corner
    }

    /// This method draws a `Triangle` shape on the given `CGContext`.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addTriangle(inRect: CGRect(x: 0, y: 0, width: size, height: size), fromCorner: corner)
    }
}

/// A `BottomHalfTriangle` structure represents a triangle shape which covers the bottom half of the drawing area.
struct BottomHalfTriangle: Shape {
    /// This method draws a `BottomHalfTriangle` shape on the given `CGContext`.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addTriangle(inRect: CGRect(x: 0, y: size / 2, width: size, height: size / 2), fromCorner: 0)
    }
}

/// `Rhombus` structure represents a rhombus shape that covers the entire drawing area.
struct Rhombus: Shape {
    /// This method draws a `Rhombus` shape on the given `CGContext`.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        context.addRhombus(inRect: CGRect(x: 0, y: 0, width: size, height: size))
    }
}

/// `Circle` structure represents a circle that is positioned with a margin from the top-left corner of the drawing
/// area.
struct Circle: Shape {
    /// This method draws a `Circle` shape on the given `CGContext`.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` to draw the shape on.
    ///     - size: The size of the shape to draw.
    ///     - index: A number representing the current index of the shape.
    ///
    /// - Returns: None
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let margin = size / 6
        context.addCircle(withOrigin: CGPoint(x: margin, y: margin), radius: size / 2 - margin, clockwise: true)
    }
}

/// `MiddleSquare` is a `Shape` that draws a square in the middle of the given context.
struct MiddleSquare: Shape {
    /// Draw the shape in the given context.
    ///
    /// This function creates a square in the center of the given context. The size of the square
    /// is determined by the `size` parameter, where the square's size is a third of the context's size.
    ///
    /// - Parameters:
    ///     - context: The `CGContext` in which the shape is to be drawn.
    ///     - size: The size of the context in which the shape is to be drawn.
    ///     - index: The index of the shape. Not used in this shape.
    func draw(in context: CGContext, size: CGFloat, index _: Int) {
        let squareSize = size / 3
        let origin = CGPoint(x: squareSize, y: squareSize)
        let squareRect = CGRect(
            origin: origin,
            size: CGSize(width: size - 2 * squareSize, height: size - 2 * squareSize)
        )
        context.addRect(squareRect)
    }
}
