package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import kotlin.math.floor

internal class SvgPath {
    var dataString = ""

    fun addPolygon(points: List<Point>) {
        var dataString = "M" + svgValue(points[0].x) + " " + svgValue(points[0].y)

        for (i in 1 until points.size) {
            dataString += "L" + svgValue(points[i].x) + " " + svgValue(points[i].y)
        }
        this.dataString += dataString + "Z"
    }

    fun addCircle(point: Point, diameter: Float, counterClockwise: Boolean) {
        var sweepFlag = if (counterClockwise != null) 0 else 1
        var svgRadius = svgValue(diameter / 2f)
        var svgDiameter = svgValue(diameter)

        this.dataString +=
                "M" + svgValue(point.x) + " " + svgValue(point.y + diameter / 2f) +
                        "a" + svgRadius + "," + svgRadius + " 0 1," + sweepFlag + " " + svgDiameter + ",0" +
                        "a" + svgRadius + "," + svgRadius + " 0 1," + sweepFlag + " " + (-svgDiameter) + ",0"
    }
}

fun svgValue(value: Float) : Int {
    return floor(value).toInt()
}
