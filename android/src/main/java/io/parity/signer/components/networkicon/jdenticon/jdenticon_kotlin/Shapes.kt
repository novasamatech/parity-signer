package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin.Graphics
import io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin.Point
import kotlin.math.floor

internal class Shapes {
    companion object {
        val center = listOf(
                fun(g: Graphics, cell: Float, index: Int?) {
                    val k = cell * 0.42f
                    g.addPolygon(
                            listOf(
                                    Point(0f, 0f),
                                    Point(cell, 0f),
                                    Point(cell, cell - k * 2f),
                                    Point(cell - k, cell),
                                    Point(0f, cell)
                            )
                    )
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    val w = floor(cell * 0.5f)
                    val h = floor(cell * 0.8f)
                    g.addTriangle(cell - w, 0f, w, h, 2f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var s = floor(cell / 3f)
                    g.addRectangle(s, s, cell - s, cell - s)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var inner = cell * 0.1f
                    inner = if (inner > 1f) floor(inner)    // large icon => truncate decimals
                    else if (inner > 0.5) 1f                // medium size icon => fixed width
                    else inner                              // small icon => anti-aliased border

                    // Use fixed outer border widths in small icons to ensure the border is drawn
                    var outer = if (cell < 6f) 1f
                    else if (cell < 8f) 2f
                    else floor(cell * 0.25f)

                    g.addRectangle(outer, outer, cell - inner - outer, cell - inner - outer)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var m = floor(cell * 0.15f)
                    var s = floor(cell * 0.5f)
                    g.addCircle(cell - s - m, cell - s - m, s)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var inner = cell * 0.1f
                    var outer = inner * 4f

                    g.addRectangle(0f, 0f, cell, cell)
                    g.addPolygon(
                            listOf(
                                    Point(outer, floor(outer)),
                                    Point(cell - inner, floor(outer)),
                                    Point(outer + (cell - outer - inner) / 2f, cell - inner)
                            ),
                            true)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addPolygon(
                            listOf(
                                    Point(0f, 0f),
                                    Point(cell, 0f),
                                    Point(cell, cell * 0.7f),
                                    Point(cell * 0.4f, cell * 0.4f),
                                    Point(cell * 0.7f, cell),
                                    Point(0f, cell)
                            )
                    )
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addTriangle(cell / 2f, cell / 2f, cell / 2f, cell / 2f, 3f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addRectangle(0f, 0f, cell, cell / 2f)
                    g.addRectangle(0f, cell / 2f, cell / 2f, cell / 2f)
                    g.addTriangle(cell / 2f, cell / 2f, cell / 2f, cell / 2f, 1f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var inner = cell * 0.14f
                    inner = if (cell < 8f) inner    // small icon => anti-aliased border
                    else floor(inner)               // large icon => truncate decimals

                    // Use fixed outer border widths in small icons to ensure the border is drawn
                    var outer = if (cell < 4f) 1f
                    else if (cell < 6f) 2f
                    else floor(cell * 0.35f)

                    g.addRectangle(0f, 0f, cell, cell)
                    g.addRectangle(outer, outer, cell - outer - inner, cell - outer - inner, true)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    val inner = cell * 0.12f
                    val outer = inner * 3f

                    g.addRectangle(0f, 0f, cell, cell)
                    g.addCircle(outer, outer, cell - inner - outer, true)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addTriangle(cell / 2f, cell / 2f, cell / 2f, cell / 2f, 3f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var m = cell * 0.25f
                    g.addRectangle(0f, 0f, cell, cell)
                    g.addRhombus(m, m, cell - m, cell - m, true)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    val m = cell * 0.4f
                    val s = cell * 1.2f
                    if (index == null || index == 0) {
                        g.addCircle(m, m, s)
                    }
                }
        )

        val outer = listOf(
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addTriangle(0f, 0f, cell, cell, 0f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addTriangle(0f, cell / 2f, cell, cell / 2f, 0f)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    g.addRhombus(0f, 0f, cell, cell)
                },
                fun(g: Graphics, cell: Float, index: Int?) {
                    var m = cell / 6f
                    g.addCircle(m, m, cell - 2 * m)
                }
        )
    }
}
