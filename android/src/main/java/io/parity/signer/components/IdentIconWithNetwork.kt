package io.parity.signer.components

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.*
import androidx.compose.ui.graphics.Outline
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.PathOperation
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.*
import io.parity.signer.R
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun IdentIconWithNetwork(
	identicon: ImageContent,
	networkLogoName: String,
	size: Dp = 28.dp,
	modifier: Modifier = Modifier,
) {
	Box(modifier = modifier, contentAlignment = Alignment.BottomEnd) {
		val cutoutSize = size/2
		Image(
			identicon.toBytes().intoImageBitmap(),
			stringResource(R.string.description_identicon),
			modifier = Modifier
				.size(size)
				.clip(CircleShape)
				.clip(SubIconCutShape(cutoutSize))
		)
		//todo dmitry think about right to left
		NetworkIcon(networkLogoName = networkLogoName, size = cutoutSize)
	}
}

/**
 * This is a shape with cuts out a rectangle in the center
 */
class SubIconCutShape(val innerIconSize: Dp) : Shape {
	private val cutoutBorderRadius = 2.dp
	override fun createOutline(
		size: Size,
		layoutDirection: LayoutDirection,
		density: Density
	): Outline {
		val outlinePath = Path()
		outlinePath.addRect(Rect(Offset(0f, 0f), size))

		val cutoutRadius:Float = kotlin.math.min(size.height, kotlin.math.min(size.width,
			(innerIconSize + cutoutBorderRadius * 2).value * density.density))

		val cutoutPath = Path()
		val borderWidth = cutoutBorderRadius.value * density.density
		cutoutPath.addRoundRect(
			RoundRect(
				Rect(
					topLeft = Offset(
						size.width - cutoutRadius + borderWidth,
						size.height - cutoutRadius + borderWidth,
					),
					bottomRight = Offset(
						size.width + borderWidth,
						size.height + borderWidth)
				),
				cornerRadius = CornerRadius(cutoutRadius, cutoutRadius)
			)
		)

		val finalPath = Path.combine(
			PathOperation.Difference,
			outlinePath,
			cutoutPath
		)

		return Outline.Generic(finalPath)
	}
}

@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewNetworkIconSizes() {
	SignerNewTheme {
		val icon = PreviewData.exampleIdenticonPng
		val network = NetworkModel.createStub().logo
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			IdentIconWithNetwork(icon, network)
			IdentIconWithNetwork(icon, network)
			IdentIconWithNetwork(icon, network, size = 18.dp)
			IdentIconWithNetwork(icon, network, size = 18.dp)
			IdentIconWithNetwork(icon, network, size = 56.dp)
			IdentIconWithNetwork(icon, network, size = 56.dp)
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewNetworkIconUnknownIcons() {
	val icon = PreviewData.exampleIdenticonPng
	val network = NetworkModel.createStub()
	val network2 = NetworkModel.createStub("Some")

	SignerNewTheme {
		Column {
			IdentIconWithNetwork(
				icon, network.logo,
				size = 24.dp
			)
			IdentIconWithNetwork(
				icon, network2.logo,
				size = 24.dp
			)
		}
	}
}
