//
//  ShapeCollection.swift
//
//
//  Created by Krzysztof Rodak on 04/08/2023.
//

import Foundation

/// `ShapeCollection` class represents a collection of center and outer shapes.
/// Center shapes are drawn in the center of the canvas and outer shapes are drawn covering the entire canvas.
final class ShapeCollection {
    /// An array of `Shape` instances representing different shapes that can be drawn in the center of the canvas.
    let centerShapes: [Shape] = [
        CutCorner(),
        SideTriangle(),
        MiddleSquare(),
        CornerSquare(),
        OffCenterCircle(),
        NegativeTriangle(),
        CutSquare(),
        HalfTriangle(),
        CornerPlusTriangle(),
        CutSquare(),
        NegativeCircle(),
        HalfTriangle(),
        NegativeRhombus(),
        ConditionalCircle()
    ]

    /// An array of `Shape` instances representing different shapes that can be drawn covering the entire canvas.
    let outerShapes: [Shape] = [
        Triangle(),
        BottomHalfTriangle(),
        Rhombus(),
        Circle()
    ]
}
