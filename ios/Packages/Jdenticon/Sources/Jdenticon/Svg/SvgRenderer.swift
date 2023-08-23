//
//  SvgRenderer.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

protocol Renderer {
    func setBackground(fillColor: String)
    func beginShape(color: String)
    func endShape()
    func addPolygon(points: [CGPoint])
    func addCircle(point: CGPoint, diameter: CGFloat, counterClockwise: Bool)
    func finish()
}

class SvgRenderer: Renderer {
    private var _pathsByColor = [String: SvgPath]()
    private var _target: SvgWriter
    private var size: Int {
        _target.size
    }

    private var _path: SvgPath?

    init(target: SvgWriter) {
        _target = target
    }

    func setBackground(fillColor: String) {
        let re = try! NSRegularExpression(pattern: "^(#......)(..)?")
        if let match = re.firstMatch(in: fillColor, range: NSRange(fillColor.startIndex..., in: fillColor)),
           let colorMatchRange = Range(match.range(at: 1), in: fillColor),
           let opacityMatchRange = Range(match.range(at: 2), in: fillColor) {
            let colorMatch = String(fillColor[colorMatchRange])
            let opacityMatch = String(fillColor[opacityMatchRange])
            let opacity = CGFloat(UInt32(opacityMatch, radix: 16) ?? 0) / 255.0
            _target.setBackground(fillColor: colorMatch, opacity: opacity)
        }
    }

    func beginShape(color: String) {
        if _pathsByColor[color] == nil {
            _pathsByColor[color] = SvgPath()
        }
        _path = _pathsByColor[color]
    }

    func endShape() {
        _path = nil
    }

    func addPolygon(points: [CGPoint]) {
        _path?.addPolygon(points: points)
    }

    func addCircle(point: CGPoint, diameter: CGFloat, counterClockwise: Bool) {
        _path?.addCircle(point: point, diameter: diameter, counterClockwise: counterClockwise)
    }

    func finish() {
        _pathsByColor.keys.sorted().forEach { color in
            if let dataString = _pathsByColor[color]?.dataString {
                _target.append(color: color, dataString: dataString)
            }
        }
    }
}
