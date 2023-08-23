//
//  CGContext+Drawing.swift
//
//
//  Created by Krzysztof Rodak on 04/08/2023.
//

import CoreGraphics

extension CGContext {
    /// Adds a polygon defined by an array of points to the path.
    /// - Parameters:
    ///   - points: An array of `CGPoint` elements defining the vertices of the polygon.
    ///   - invert: A boolean value determining whether the polygon's vertices should be defined in reverse order.
    /// Defaults to `false`.
    func addPolygon(withPoints points: [CGPoint], inverted: Bool = false) {
        let polygonPoints = inverted ? points.reversed() : points
        guard let firstPoint = polygonPoints.first else { return }

        move(to: firstPoint)
        for point in polygonPoints.dropFirst() {
            addLine(to: point)
        }
        closePath()
    }

    /// Adds a circle to the path.
    /// - Parameters:
    ///   - origin: The center of the circle as a `CGPoint`.
    ///   - radius: The radius of the circle as a `CGFloat`.
    ///   - clockwise: A boolean value determining whether the circle is drawn in a clockwise direction. Defaults to
    /// `false`.
    func addCircle(withOrigin origin: CGPoint, radius: CGFloat, clockwise: Bool) {
        let circleCenter = CGPoint(x: origin.x + radius, y: origin.y + radius)
        addArc(center: circleCenter, radius: radius, startAngle: 0, endAngle: 2 * .pi, clockwise: clockwise)
    }

    /// Adds a rectangle to the path.
    /// - Parameters:
    ///   - rect: The rectangle as a `CGRect`.
    ///   - inverted: A boolean value determining whether the rectangle's vertices should be defined in reverse order.
    /// Defaults to `false`.
    func addRectangle(_ rect: CGRect, inverted: Bool = false) {
        let rectanglePoints = [
            CGPoint(x: rect.minX, y: rect.minY),
            CGPoint(x: rect.maxX, y: rect.minY),
            CGPoint(x: rect.maxX, y: rect.maxY),
            CGPoint(x: rect.minX, y: rect.maxY)
        ]
        addPolygon(withPoints: rectanglePoints, inverted: inverted)
    }

    /// Adds a triangle to the path.
    /// - Parameters:
    ///   - rect: The bounding rectangle as a `CGRect`.
    ///   - corner: An integer specifying the corner from which the triangle is drawn.
    func addTriangle(inRect rect: CGRect, fromCorner corner: Int) {
        let trianglePoints: [CGPoint]
        switch corner % 4 {
        case 0:
            trianglePoints = [
                CGPoint(x: rect.maxX, y: rect.maxY),
                CGPoint(x: rect.minX, y: rect.maxY),
                CGPoint(x: rect.minX, y: rect.minY)
            ]
        case 1:
            trianglePoints = [
                CGPoint(x: rect.maxX, y: rect.minY),
                CGPoint(x: rect.minX, y: rect.minY),
                CGPoint(x: rect.minX, y: rect.maxY)
            ]
        case 2:
            trianglePoints = [
                CGPoint(x: rect.minX, y: rect.minY),
                CGPoint(x: rect.maxX, y: rect.minY),
                CGPoint(x: rect.maxX, y: rect.maxY)
            ]
        case 3:
            trianglePoints = [
                CGPoint(x: rect.maxX, y: rect.minY),
                CGPoint(x: rect.maxX, y: rect.maxY),
                CGPoint(x: rect.minX, y: rect.maxY)
            ]
        default:
            return
        }
        addPolygon(withPoints: trianglePoints)
    }

    /// Adds a rhombus to the path.
    /// - Parameters:
    ///   - rect: The bounding rectangle as a `CGRect`.
    ///   - inverted: A boolean value determining whether the rhombus's vertices should be defined in reverse order.
    /// Defaults to `false`.
    func addRhombus(inRect rect: CGRect, inverted: Bool = false) {
        let rhombusPoints = [
            CGPoint(x: rect.midX, y: rect.minY),
            CGPoint(x: rect.maxX, y: rect.midY),
            CGPoint(x: rect.midX, y: rect.maxY),
            CGPoint(x: rect.minX, y: rect.midY)
        ]
        addPolygon(withPoints: rhombusPoints, inverted: inverted)
    }
}
