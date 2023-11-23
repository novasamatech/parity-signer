package io.parity.signer.screens.scan.camera

import android.content.res.Configuration
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.BlendMode
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.elements.CameraCloseIcon
import io.parity.signer.screens.scan.elements.CameraLightIcon
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.forcedFill40
import io.parity.signer.ui.theme.pink500


object ScanConstants{
	val CLIP_SIDE_PADDING = 40.dp
}

@Composable
internal fun TransparentCutoutLayout(
	modifier: Modifier = Modifier,
) {

	val sidePaddingInPX: Float

	with(LocalDensity.current) {
		sidePaddingInPX = ScanConstants.CLIP_SIDE_PADDING.toPx()
	}

	val background = MaterialTheme.colors.forcedFill40
	val frameColor = MaterialTheme.colors.pink500
	val roundClip = remember { 56.dp }

	Canvas(modifier = modifier.fillMaxSize()) {

		val offsetInPx = (size.height/2) - (size.width/2)
		val canvasWidth = size.width
		val smallestSide = minOf(size.height, size.width)
		val sideInPx = smallestSide - 2 * sidePaddingInPX

		with(drawContext.canvas.nativeCanvas) {
			//full screen blur
			val checkPoint = saveLayer(null, null)
			// Destination
			drawRect(background)

			// Source
			val topLeftClipX = (canvasWidth - sideInPx) / 2
			drawRoundRect(
				topLeft = Offset(
					x = topLeftClipX,
					y = offsetInPx
				),
				size = Size(sideInPx, sideInPx),
				cornerRadius = CornerRadius(roundClip.toPx(), roundClip.toPx()),
				color = Color.Transparent,
				blendMode = BlendMode.Clear
			)
			restoreToCount(checkPoint)

			//draw frame
			val frameThikness = 8.dp.toPx()
			val checkPointFrame = saveLayer(null, null)
			drawRoundRect(
				topLeft = Offset(
					x = topLeftClipX,
					y = offsetInPx,
				),
				size = Size(sideInPx, sideInPx),
				cornerRadius = CornerRadius(roundClip.toPx(), roundClip.toPx()),
				color = frameColor,
				style = Stroke(width = frameThikness)
			)
			//cutout horizontal
			drawRect(
				topLeft = Offset(
					x = topLeftClipX - frameThikness, //to overcover full width
					y = offsetInPx + sideInPx / 3
				),
				size = Size(
					width = sideInPx + frameThikness * 2,
					height = sideInPx / 3
				),
				color = Color.Transparent,
				blendMode = BlendMode.Clear
			)
			//cutout vertical
			drawRect(
				topLeft = Offset(
					x = topLeftClipX + sideInPx / 3,
					y = offsetInPx - frameThikness
				),
				size = Size(sideInPx / 3, sideInPx + frameThikness * 2),
				color = Color.Transparent,
				blendMode = BlendMode.Clear
			)
			restoreToCount(checkPointFrame)

		}
	}
}

@Composable
internal fun ScanHeader(
	modifier: Modifier = Modifier,
	onClose: Callback,
	viewModel: CameraViewModel,
) {
	val torchEnabled by viewModel.isTorchEnabled.collectAsStateWithLifecycle()
	Row(
		modifier
			.fillMaxWidth(1f)
			.padding(horizontal = 16.dp)
	) {
		CameraCloseIcon(
			onCloseClicked = onClose
		)
		Spacer(modifier = Modifier.weight(1f))
		CameraLightIcon(isEnabled = torchEnabled,
			onClick = {
				viewModel.isTorchEnabled.value = !torchEnabled
			})
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
		TransparentCutoutLayout()
	}
}
