package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import kotlin.math.floor

internal class IconGenerator(renderer: SvgRenderer, hash: String,
														 x: Float, y: Float, size: Float,
														 padding: Float?, config: Config,
	) {

    var _hash = hash
    val _renderer = renderer

    // Calculate padding
    var _padding = floor(size * (padding ?: 0f))
    var _size = size - _padding * 2f

    var graphics = Graphics(renderer)

    // Calculate cell size and ensure it is an integer
    var cell = floor(_size / 4f)

    // Since the cell size is integer based, the actual icon will be slightly smaller than specified => center icon
    var _x = x + floor(_padding + _size / 2f - cell * 2f);
    var _y = y + floor(_padding + _size / 2f - cell * 2f);

    // AVAILABLE COLORS
    var hue = hash.substring(hash.length-7).toInt(16).toFloat() / 0xfffffff

    // Available colors for this icon
    var availableColors = colorTheme(hue.toFloat(), config)

    // The index of the selected colors
    var selectedColorIndexes = ArrayList<Int>()
    var index = 0

    fun renderShape(
			colorIndex: Int,
			shapes: List<(Graphics, Float, Int?) -> Unit>,
			index: Int,
			rotationIndex: Int?,
			positions: Array<Array<Int>>
    ) {
        var r = if(rotationIndex != null)_hash.elementAt(rotationIndex).toString().toInt(16) else 0
        val shape = shapes[_hash.elementAt(index).toString().toInt(16) % shapes.size]

        _renderer.beginShape(availableColors[selectedColorIndexes[colorIndex]])
        for (i in 0 until positions.size) {
            graphics._transform = Transform(
                    _x + positions[i][0] * cell,
                    _y + positions[i][1] * cell,
                    cell,
                    ((r++ % 4).toFloat())
            )
            shape(graphics, cell, i)
        }

        _renderer.endShape()
    }

    fun isDuplicate(values: List<Int>) : Boolean {
        if (values.indexOf(index) >= 0) {
            for (i in 0 until values.size) {
                if (selectedColorIndexes.indexOf(values[i]) >= 0) {
                    return true
                }
            }
        }
        return false
    }

    init {
        for (i in 0 until 3) {
            index = (hash.elementAt(8 + i).toString().toInt(16) % availableColors.size)
            if (isDuplicate(listOf(0, 4)) || // Disallow dark gray and dark color combo
                    isDuplicate(listOf(2, 3))) { // Disallow light gray and light color combo
                index = 1;
            }
            selectedColorIndexes.add(index);
        }
        // ACTUAL RENDERING
        // Sides
        renderShape(0,
					Shapes.outer, 2, 3, arrayOf(arrayOf(1, 0), arrayOf(2, 0), arrayOf(2, 3), arrayOf(1, 3), arrayOf(0, 1), arrayOf(3, 1), arrayOf(3, 2), arrayOf(0, 2)))
        // Corners
        renderShape(1,
					Shapes.outer, 4, 5, arrayOf(arrayOf(0, 0), arrayOf(3, 0), arrayOf(3, 3), arrayOf(0, 3)))
        // Center
        renderShape(2,
					Shapes.center, 1, null, arrayOf(arrayOf(1, 1), arrayOf(2, 1), arrayOf(2, 2), arrayOf(1, 2)))

        _renderer.finish()
    }
};
