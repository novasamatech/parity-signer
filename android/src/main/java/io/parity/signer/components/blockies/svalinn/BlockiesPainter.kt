package io.parity.signer.components.blockies.svalinn


import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.graphics.PaintingStyle
import androidx.compose.ui.graphics.Path

/**
 * Based on svalinn-kotlin project.
 * Reimplemented for compose
 */
internal object BlockiesPainter {

	fun draw(blockies: Blockies, canvas: Canvas, width: Float, height: Float) {
		val canvasPaint = Paint().apply { style = PaintingStyle.Fill }
		val dimen = Math.min(width, height)
		val offsetX = width - dimen
		val offsetY = height - dimen

		canvas.save()
		canvasPaint.color = Color(blockies.backgroundColor)
		canvas.drawRect(
			offsetX, offsetY, offsetX + dimen, offsetY + dimen,
			canvasPaint
		)

		val scale = dimen / Blockies.SIZE
		val main = Color(blockies.primaryColor)
		val sColor = Color(blockies.spotColor)

		for (i in blockies.data.indices) {
			val col = i % Blockies.SIZE
			val row = i / Blockies.SIZE

			canvasPaint.color = if (blockies.data[i] == 1.0) main else sColor

			if (blockies.data[i] > 0.0) {
				canvas.drawRect(
					offsetX + (col * scale),
					offsetY + (row * scale),
					offsetX + (col * scale + scale),
					offsetY + (row * scale + scale),
					canvasPaint
				)
			}
		}
		canvas.restore()
	}

}
