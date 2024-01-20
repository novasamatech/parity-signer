//
//  Graphics.swift
//
//
//  Created by Krzysztof Rodak on 23/08/2023.
//

import CoreGraphics
import Foundation

final class Graphics {
    private let renderer: Renderer
    var transform: Transform = .noTransform()

    init(renderer: Renderer) {
        self.renderer = renderer
    }

    func addPolygon(_ points: [CGPoint], _ inverted: Bool = false) {
        let currentTransform = transform
        var transformedPoints = [CGPoint]()

        let indices: [Int] =
            if inverted {
                points.indices.reversed()
            } else {
                Array(points.indices)
            }

        indices.forEach { index in
            let transformedPoint = currentTransform.transformPoint(x: points[index].x, y: points[index].y)
            transformedPoints.append(transformedPoint)
        }

        renderer.addPolygon(points: transformedPoints)
    }

    func addCircle(x: CGFloat, y: CGFloat, size: CGFloat, invert: Bool = false) {
        let transformedPoint = transform.transformPoint(x: x, y: y, w: size, h: size)
        renderer.addCircle(point: transformedPoint, diameter: size, counterClockwise: invert)
    }

    func addRectangle(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat, invert: Bool = false) {
        let rectanglePoints = [
            CGPoint(x: x, y: y),
            CGPoint(x: x + w, y: y),
            CGPoint(x: x + w, y: y + h),
            CGPoint(x: x, y: y + h)
        ]
        addPolygon(rectanglePoints, invert)
    }

    func addTriangle(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat, r: CGFloat, invert: Bool = false) {
        var points = [
            CGPoint(x: x + w, y: y),
            CGPoint(x: x + w, y: y + h),
            CGPoint(x: x, y: y + h),
            CGPoint(x: x, y: y)
        ]
        points.remove(at: Int(floor(r)) % 4)
        addPolygon(points, invert)
    }

    func addRhombus(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat, invert: Bool = false) {
        let rhombusPoints = [
            CGPoint(x: x + w / 2, y: y),
            CGPoint(x: x + w, y: y + h / 2),
            CGPoint(x: x + w / 2, y: y + h),
            CGPoint(x: x, y: y + h / 2)
        ]
        addPolygon(rhombusPoints, invert)
    }
}
