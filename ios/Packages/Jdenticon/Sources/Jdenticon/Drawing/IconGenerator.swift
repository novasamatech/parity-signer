//
//  IconGenerator.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import CoreGraphics
import Foundation

class IconGenerator {
    private var _hash: String
    private let _renderer: Renderer

    private var _padding: CGFloat
    private var _size: CGFloat
    private var graphics: Graphics

    private var cell: CGFloat
    private var _x: CGFloat
    private var _y: CGFloat

    private var hue: CGFloat
    private var availableColors: [String]

    private var selectedColorIndexes: [Int] = []
    private var index: Int = 0

    init(
        renderer: Renderer,
        hash: String,
        x: CGFloat,
        y: CGFloat,
        size: CGFloat,
        padding: CGFloat?,
        config: Config
    ) {
        _hash = hash
        _renderer = renderer
        _x = x
        _y = y
        _padding = floor(size * (padding ?? 0))
        _size = size - _padding * 2.0

        graphics = Graphics(renderer: renderer)

        cell = floor(_size / 4.0)

        _x += floor(_padding + _size / 2.0 - cell * 2.0)
        _y += floor(_padding + _size / 2.0 - cell * 2.0)

        let hueHex = String(hash.suffix(7))
        let hueInt = UInt32(hueHex, radix: 16) ?? 0
        hue = CGFloat(hueInt) / 0xFFFFFFF

        availableColors = colorTheme(hue: hue, config: config)

        for i in 0 ..< 3 {
            let charIndex = hash.index(hash.startIndex, offsetBy: 8 + i)
            let char = hash[charIndex]
            if let intValue = Int(String(char), radix: 16) {
                index = intValue % availableColors.count
                if isDuplicate(values: [0, 4]) || isDuplicate(values: [2, 3]) {
                    index = 1
                }
                selectedColorIndexes.append(index)
            }
        }

        renderShapes()
        _renderer.finish()
    }

    func renderShapes() {
        // Sides
        renderShape(
            colorIndex: 0,
            shapes: Shapes.outer,
            index: 2,
            rotationIndex: 3,
            positions: [
                [1, 0], [2, 0], [2, 3], [1, 3],
                [0, 1], [3, 1], [3, 2], [0, 2]
            ]
        )

        // Corners
        renderShape(
            colorIndex: 1,
            shapes: Shapes.outer,
            index: 4,
            rotationIndex: 5,
            positions: [
                [0, 0], [3, 0], [3, 3], [0, 3]
            ]
        )

        // Center
        renderShape(
            colorIndex: 2,
            shapes: Shapes.center,
            index: 1,
            rotationIndex: nil,
            positions: [
                [1, 1], [2, 1], [2, 2], [1, 2]
            ]
        )
    }

    func renderShape(
        colorIndex: Int,
        shapes: [(Graphics, CGFloat, Int?) -> Void],
        index: Int,
        rotationIndex: Int?,
        positions: [[Int]]
    ) {
        var r: Int = if let rotationIndex {
            Int(String(_hash[_hash.index(_hash.startIndex, offsetBy: rotationIndex)]), radix: 16) ?? 0
        } else {
            0
        }

        let shape =
            shapes[(Int(String(_hash[_hash.index(_hash.startIndex, offsetBy: index)]), radix: 16) ?? 0) % shapes
                .count]

        let color = availableColors[selectedColorIndexes[colorIndex]]
        _renderer.beginShape(color: color)

        for i in 0 ..< positions.count {
            graphics.transform = Transform(
                x_t: _x + CGFloat(positions[i][0]) * cell,
                y_t: _y + CGFloat(positions[i][1]) * cell,
                size_t: cell,
                rotation_t: CGFloat(r % 4)
            )
            shape(graphics, cell, i)
            r += 1
        }

        _renderer.endShape()
    }

    func isDuplicate(values: [Int]) -> Bool {
        if values.firstIndex(of: index) != nil {
            for i in 0 ..< values.count {
                if selectedColorIndexes.contains(values[i]) {
                    return true
                }
            }
        }
        return false
    }
}
