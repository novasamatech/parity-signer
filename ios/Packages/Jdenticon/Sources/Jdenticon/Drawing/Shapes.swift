//
//  Shapes.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import CoreGraphics
import Foundation

enum Shapes {
    static let center: [(Graphics, CGFloat, Int?) -> Void] = [
        { (g: Graphics, cell: CGFloat, _: Int?) in
            let k = cell * 0.42
            g.addPolygon([
                CGPoint(x: 0, y: 0),
                CGPoint(x: cell, y: 0),
                CGPoint(x: cell, y: cell - k * 2),
                CGPoint(x: cell - k, y: cell),
                CGPoint(x: 0, y: cell)
            ])
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            let w = floor(cell * 0.5)
            let h = floor(cell * 0.8)
            g.addTriangle(x: cell - w, y: 0, w: w, h: h, r: 2)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            let s = floor(cell / 3)
            g.addRectangle(x: s, y: s, w: cell - s, h: cell - s)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            var inner = cell * 0.1
            if inner > 1 {
                inner = floor(inner) // Large icon => truncate decimals
            } else if inner > 0.5 {
                inner = 1 // Medium size icon => fixed width
            }
            // Small icon => anti-aliased border
            var outer: CGFloat = cell < 6 ? 1 : (cell < 8 ? 2 : floor(cell * 0.25))
            g.addRectangle(x: outer, y: outer, w: cell - inner - outer, h: cell - inner - outer)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            var m = floor(cell * 0.15)
            var s = floor(cell * 0.5)
            g.addCircle(x: cell - s - m, y: cell - s - m, size: s)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            var inner = cell * 0.1
            var outer = inner * 4
            g.addRectangle(x: 0, y: 0, w: cell, h: cell)
            g.addPolygon([
                CGPoint(x: outer, y: floor(outer)),
                CGPoint(x: cell - inner, y: floor(outer)),
                CGPoint(x: outer + (cell - outer - inner) / 2, y: cell - inner)
            ], true)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addPolygon([
                CGPoint(x: 0, y: 0),
                CGPoint(x: cell, y: 0),
                CGPoint(x: cell, y: cell * 0.7),
                CGPoint(x: cell * 0.4, y: cell * 0.4),
                CGPoint(x: cell * 0.7, y: cell),
                CGPoint(x: 0, y: cell)
            ])
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addTriangle(x: cell / 2, y: cell / 2, w: cell / 2, h: cell / 2, r: 3)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addRectangle(x: 0, y: 0, w: cell, h: cell / 2)
            g.addRectangle(x: 0, y: cell / 2, w: cell / 2, h: cell / 2)
            g.addTriangle(x: cell / 2, y: cell / 2, w: cell / 2, h: cell / 2, r: 1)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            var inner = cell * 0.14
            inner = cell < 8 ? inner : floor(inner)
            var outer: CGFloat = if cell < 4 {
                1
            } else if cell < 6 {
                2
            } else {
                floor(cell * 0.35)
            }

            g.addRectangle(x: 0, y: 0, w: cell, h: cell)
            g.addRectangle(x: outer, y: outer, w: cell - outer - inner, h: cell - outer - inner, invert: true)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            let inner = cell * 0.12
            let outer = inner * 3

            g.addRectangle(x: 0, y: 0, w: cell, h: cell)
            g.addCircle(x: outer, y: outer, size: cell - inner - outer, invert: true)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addTriangle(x: cell / 2, y: cell / 2, w: cell / 2, h: cell / 2, r: 3)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            var m = cell * 0.25
            g.addRectangle(x: 0, y: 0, w: cell, h: cell)
            g.addRhombus(x: m, y: m, w: cell - m, h: cell - m, invert: true)
        },
        { (g: Graphics, cell: CGFloat, index: Int?) in
            let m = cell * 0.4
            let s = cell * 1.2
            if index == nil || index == 0 {
                g.addCircle(x: m, y: m, size: s)
            }
        }
    ]

    static let outer: [(Graphics, CGFloat, Int?) -> Void] = [
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addTriangle(x: 0, y: 0, w: cell, h: cell, r: 0)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addTriangle(x: 0, y: cell / 2, w: cell, h: cell / 2, r: 0)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            g.addRhombus(x: 0, y: 0, w: cell, h: cell)
        },
        { (g: Graphics, cell: CGFloat, _: Int?) in
            let m = cell / 6
            g.addCircle(x: m, y: m, size: cell - 2 * m)
        }
    ]
}
