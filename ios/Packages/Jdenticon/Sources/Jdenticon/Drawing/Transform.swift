//
//  Transform.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

class Transform {
    let x_t: CGFloat
    let y_t: CGFloat
    let size_t: CGFloat
    let rotation_t: CGFloat

    init(x_t: CGFloat, y_t: CGFloat, size_t: CGFloat, rotation_t: CGFloat) {
        self.x_t = x_t
        self.y_t = y_t
        self.size_t = size_t
        self.rotation_t = rotation_t
    }

    func transformPoint(x: CGFloat, y: CGFloat, w: CGFloat? = nil, h: CGFloat? = nil) -> CGPoint {
        let right = x_t + size_t
        let bottom = y_t + size_t
        let height = h ?? 0
        let width = w ?? 0

        if rotation_t == 1 {
            return CGPoint(x: right - y - height, y: y_t + x)
        } else if rotation_t == 2 {
            return CGPoint(x: right - x - width, y: bottom - y - height)
        } else if rotation_t == 3 {
            return CGPoint(x: x_t + y, y: bottom - x - width)
        } else {
            return CGPoint(x: x_t + x, y: y_t + y)
        }
    }

    static func noTransform() -> Transform {
        Transform(x_t: 0, y_t: 0, size_t: 0, rotation_t: 0)
    }
}
