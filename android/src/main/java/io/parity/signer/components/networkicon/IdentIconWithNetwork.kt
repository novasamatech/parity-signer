package io.parity.signer.components.networkicon

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.RoundRect
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Outline
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.PathOperation
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.platform.LocalLayoutDirection
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Identicon


@Composable
fun IdentIconWithNetwork(
	identicon: Identicon,
	networkLogoName: String,
	size: Dp = 28.dp,
	modifier: Modifier = Modifier,
) {
	//cutout always on right side, so force icon end be on right as well
	CompositionLocalProvider(LocalLayoutDirection provides LayoutDirection.Ltr) {
		Box(modifier = modifier, contentAlignment = Alignment.BottomEnd) {
			val cutoutSize = size / 2
			IdentIconImage(
				identicon = identicon,
				modifier = Modifier.clip(SubIconCutShape(cutoutSize)),
				size = size
			)

			NetworkIcon(networkLogoName = networkLogoName, size = cutoutSize)
		}
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

		val cutoutRadius: Float = kotlin.math.min(
			size.height, kotlin.math.min(
				size.width,
				(innerIconSize + cutoutBorderRadius * 2).value * density.density
			)
		)

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
						size.height + borderWidth
					)
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
		val iconPng = PreviewData.Identicon.dotIcon
		val network = NetworkModel.createStub().logo
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			IdentIconWithNetwork(iconPng, network)
			IdentIconWithNetwork(iconPng, network)
			IdentIconWithNetwork(iconPng, network, size = 18.dp)
			IdentIconWithNetwork(iconPng, network, size = 18.dp)
			IdentIconWithNetwork(iconPng, network, size = 56.dp)
			IdentIconWithNetwork(iconPng, network, size = 56.dp)
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
	val icon = PreviewData.Identicon.dotIcon
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
