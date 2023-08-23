//
//  SvgPath.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

final class SvgPath {
    var dataString = ""

    func addPolygon(points: [CGPoint]) {
        var dataString = "M\(svgValue(points[0].x)) \(svgValue(points[0].y))"

        for i in 1 ..< points.count {
            dataString += "L\(svgValue(points[i].x)) \(svgValue(points[i].y))"
        }
        self.dataString += dataString + "Z"
    }

    func addCircle(point: CGPoint, diameter: CGFloat, counterClockwise: Bool) {
        let sweepFlag = counterClockwise ? 0 : 1
        let svgRadius = svgValue(diameter / 2)
        let svgDiameter = svgValue(diameter)

        dataString +=
            "M\(svgValue(point.x)) \(svgValue(point.y + diameter / 2))" +
            "a\(svgRadius),\(svgRadius) 0 1,\(sweepFlag) \(svgDiameter),0" +
            "a\(svgRadius),\(svgRadius) 0 1,\(sweepFlag) \(-svgDiameter),0"
    }

    private func svgValue(_ value: CGFloat) -> Int {
        Int(floor(value))
    }
}
