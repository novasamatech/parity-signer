package io.parity.signer.screens.scan

import androidx.compose.foundation.Canvas
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.BlendMode
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp

@Composable
fun TransparentClipLayout(
	modifier: Modifier = Modifier,
) {
	val width = 300.dp
	val height = 200.dp
	val offsetY = 150.dp


	val offsetInPx: Float
	val widthInPx: Float
	val heightInPx: Float

	with(LocalDensity.current) {
		offsetInPx = offsetY.toPx()
		widthInPx = width.toPx()
		heightInPx = height.toPx()
	}

	Canvas(modifier = modifier) {

		val canvasWidth = size.width

		with(drawContext.canvas.nativeCanvas) {
			val checkPoint = saveLayer(null, null)

			// Destination
			drawRect(Color(0x77000000))

			// Source
			drawRoundRect(
				topLeft = Offset(
					x = (canvasWidth - widthInPx) / 2,
					y = offsetInPx
				),
				size = Size(widthInPx, heightInPx),
				cornerRadius = CornerRadius(30f,30f),
				color = Color.Transparent,
				blendMode = BlendMode.Clear
			)
			restoreToCount(checkPoint)
		}
	}
}
