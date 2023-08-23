//
//  IconGenerator.swift
//
//
//  Created by Krzysztof Rodak on 21/08/2023.
//

import UIKit

/// An object that creates a geometric icon representation based on an input hash.
public final class JdenticonGenerator {
    private let shapesCollection: ShapeCollection = ShapeCollection()
    private let patternGenerator: PatternGenerator

    /// Creates an `IconGenerator` with the specified dimensions and hash.
    ///
    /// - Parameters:
    ///   - iconSize: The size of the icon to be generated.
    ///   - hash: The hash that determines the icon's appearance.
    public init(patternGenerator: PatternGenerator = PatternGenerator()) {
        self.patternGenerator = patternGenerator
    }
}

public extension JdenticonGenerator {
    /// Renders the icon.
    ///
    /// - Parameters:
    ///   - iconSize: The size of the icon to be generated.
    ///   - hash: The hash that determines the icon's appearance.
    /// - Returns: The rendered icon as a UIImage.
    func render(size: CGFloat, hash: Data) -> UIImage {
        let renderer = UIGraphicsImageRenderer(size: CGSize(width: size, height: size))
        let hashDigits = patternGenerator.extractDigits(fromHash: hash, size: size)
        let selectedColors = generateColors(from: hash, with: hashDigits)

        return renderer.image { context in
            let cellSize = size / 4
            let renderingTasks: [RenderShapeParams] = [.init(
                shapes: shapesCollection.outerShapes,
                hashDigits: hashDigits,
                colorIndex: 0,
                digitIndex: 2,
                rotationIndex: 3,
                positions: [
                    .init(x: 1, y: 0),
                    .init(x: 2, y: 0),
                    .init(x: 2, y: 3),
                    .init(x: 1, y: 3),
                    .init(x: 0, y: 1),
                    .init(x: 3, y: 1),
                    .init(x: 3, y: 2),
                    .init(x: 0, y: 2)
                ],
                context: context.cgContext,
                cellSize: cellSize,
                selectedColors: selectedColors
            ), .init(
                shapes: shapesCollection.outerShapes,
                hashDigits: hashDigits,
                colorIndex: 1,
                digitIndex: 4,
                rotationIndex: 5,
                positions: [.init(x: 0, y: 0), .init(x: 3, y: 0), .init(x: 3, y: 3), .init(x: 0, y: 3)],
                context: context.cgContext,
                cellSize: cellSize,
                selectedColors: selectedColors
            ), .init(
                shapes: shapesCollection.centerShapes,
                hashDigits: hashDigits,
                colorIndex: 2,
                digitIndex: 1,
                rotationIndex: nil,
                positions: [.init(x: 1, y: 1), .init(x: 2, y: 1), .init(x: 2, y: 2), .init(x: 1, y: 2)],
                context: context.cgContext,
                cellSize: cellSize,
                selectedColors: selectedColors
            )]
            renderingTasks.forEach { renderShape(with: $0) }
        }
    }
}

private extension JdenticonGenerator {
    private struct RenderShapeParams {
        let shapes: [Shape]
        let hashDigits: [UInt8]
        let colorIndex: Int
        let digitIndex: Int
        let rotationIndex: Int?
        let positions: [CGPoint]
        let context: CGContext
        let cellSize: CGFloat
        let selectedColors: [UIColor]
    }

    private func renderShape(with params: RenderShapeParams) {
        let shape = params.shapes[Int(params.hashDigits[params.digitIndex]) % params.shapes.count]
        params.context.setFillColor(params.selectedColors[params.colorIndex % params.selectedColors.count].cgColor)

        params.positions.enumerated().forEach { index, position in
            params.context.saveGState()
            defer { params.context.restoreGState() }

            let rotation = params.rotationIndex.map { Int(params.hashDigits[$0]) } ?? 0
            let rotationAngle = CGFloat.pi / 2 * CGFloat(rotation + index % 4)

            params.context.translateBy(
                x: (position.x + 0.5) * params.cellSize,
                y: (position.y + 0.5) * params.cellSize
            )
            params.context.rotate(by: rotationAngle)
            params.context.translateBy(x: -0.5 * params.cellSize, y: -0.5 * params.cellSize)

            shape.draw(in: params.context, size: params.cellSize, index: index)
            params.context.fillPath()
        }
    }
}

private extension JdenticonGenerator {
    func generateColors(from hash: Data, with hashDigits: [UInt8]) -> [UIColor] {
        let maxHueValue: UInt32 = 0x0FFF_FFFF
        let hue = hash.withUnsafeBytes { $0.bindMemory(to: UInt32.self).last?.byteSwapped ?? 0 } & maxHueValue
        let colorTheme = ColorTheme(hue: CGFloat(hue) / CGFloat(maxHueValue))

        var selectedColorIndices = [Int]()
        for i in 0 ..< 3 {
            let index = Int(hashDigits[8 + i]) % colorTheme.colors.count
            selectedColorIndices.append(colorTheme.validateIndex(index, selected: selectedColorIndices))
        }
        return selectedColorIndices.map { colorTheme.colors[$0] }
    }
}
