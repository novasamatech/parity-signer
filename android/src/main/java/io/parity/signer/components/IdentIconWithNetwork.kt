package io.parity.signer.components

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.*
import androidx.compose.ui.graphics.*
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun IdentIconWithNetwork(
	identicon: ImageContent,
	network: NetworkModel,
	size: Dp = 28.dp,
	modifier: Modifier = Modifier,
) {
	Image(
		identicon.toBytes().intoImageBitmap(),
		stringResource(R.string.description_identicon),
		modifier = modifier
			.size(size)
			.clip(CircleShape)
			.clip(SubIconCutShape)
	)
}

/**
 * This is a shape with cuts out a rectangle in the center
 */
object SubIconCutShape : Shape {
	override fun createOutline(
		size: Size,
		layoutDirection: LayoutDirection,
		density: Density
	): Outline {
		val outlinePath = Path()
		outlinePath.addRect(Rect(Offset(0f, 0f), size))

		val cutoutHeight = size.height * 0.3f
		val cutoutWidth = size.width * 0.75f
		val center = Offset(size.width / 2f, size.height / 2f)

		val cutoutPath = Path()
		cutoutPath.addRoundRect(
			RoundRect(
				Rect(
					topLeft = center - Offset(
						cutoutWidth / 2f,
						cutoutHeight / 2f
					),
					bottomRight = center + Offset(
						cutoutWidth / 2f,
						cutoutHeight / 2f
					)
				),
				cornerRadius = CornerRadius(16f, 16f)
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
		val network = NetworkModel.createStub()
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
				icon, network,
				size = 24.dp
			)
			IdentIconWithNetwork(
				icon, network2,
				size = 24.dp
			)
		}
	}
}
