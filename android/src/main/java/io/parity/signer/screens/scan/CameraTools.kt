package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.BlendMode
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill30

@Composable
fun TransparentClipLayout(
	modifier: Modifier = Modifier,
) {
	val width = 280.dp
	val height = 280.dp
	val offsetY = 150.dp

	val offsetInPx: Float
	val widthInPx: Float
	val heightInPx: Float

	with(LocalDensity.current) {
		offsetInPx = offsetY.toPx()
		widthInPx = width.toPx()
		heightInPx = height.toPx()
	}

	val background = MaterialTheme.colors.fill30
	val roundClip = remember { 56.dp }

	Canvas(modifier = modifier.fillMaxSize()) {

		val canvasWidth = size.width

		with(drawContext.canvas.nativeCanvas) {
			val checkPoint = saveLayer(null, null)

			// Destination
			drawRect(background)

			// Source
			drawRoundRect(
				topLeft = Offset(
					x = (canvasWidth - widthInPx) / 2,
					y = offsetInPx
				),
				size = Size(widthInPx, heightInPx),
				cornerRadius = CornerRadius(roundClip.toPx(),roundClip.toPx()),
				color = Color.Transparent,
				blendMode = BlendMode.Clear
			)
			restoreToCount(checkPoint)
		}
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xB0FFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTransparentClipLayout() {

	SignerNewTheme {
		TransparentClipLayout()
	}
}
