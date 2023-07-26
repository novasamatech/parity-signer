package io.parity.signer.components.blockies


import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.graphics.PaintingStyle
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.clipPath
import androidx.compose.ui.graphics.painter.Painter
import io.parity.signer.components.blockies.svalinn.Blockies

/**
 * Based on svalinn-kotlin project which is MIT licensed.
 * Reimplemented for compose
 */
class BlockiesPainter() {

	private val canvasPaint = Paint().apply { style = PaintingStyle.Fill }
	private var dimen = 0.0f
	private var offsetX = 0.0f
	private var offsetY = 0.0f
	private val path = Path()

	fun setDimensions(width: Float, height: Float) {
		dimen = Math.min(width, height)
		offsetX = width - dimen
		offsetY = height - dimen
		path.reset()
		path.addOval(
			Rect(
				Offset(offsetX + (dimen / 2), offsetY + (dimen / 2)),
				dimen / 2,
			)
		)
		path.close()
	}

	fun draw(blockies: Blockies, canvas: Canvas) {
		canvas.save()
		canvas.clipPath(path)
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
