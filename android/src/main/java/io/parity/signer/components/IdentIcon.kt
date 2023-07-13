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
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import coil.compose.rememberAsyncImagePainter
import coil.decode.SvgDecoder
import coil.request.ImageRequest
import coil.size.Size
import io.parity.signer.R
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.SignerImage

/**
 * Just draw a standard identicon used everywhere, with standard size
 */
@OptIn(ExperimentalUnsignedTypes::class)
@Composable
fun IdentIcon(identicon: ImageContent,
							size: Dp = 28.dp,
							modifier: Modifier = Modifier) {
	when (identicon) {
		is ImageContent.Png -> {
			Image(
				bitmap = identicon.image.intoImageBitmap(),
				contentDescription = stringResource(R.string.description_identicon),
				modifier = modifier
					.size(size)
					.clip(CircleShape)
			)
		}
		is ImageContent.Svg -> {
			//todo dmitry svg
			//todo #1457 implementSvg
			val ctx = LocalContext.current
			val painter = rememberAsyncImagePainter(
				model = ImageRequest.Builder(ctx)
					.decoderFactory(SvgDecoder.Factory())
					.data(identicon.image)
					.size(Size.ORIGINAL) // Set the target size to load the image at.
					.build()
			)
			Image(
				painter = painter,
				contentDescription = stringResource(R.string.description_identicon),
				modifier = modifier
					.size(size)
					.clip(CircleShape)
			)
		}
	}

}

/**
 * Local copy of shared [SignerImage] class
 */
sealed class ImageContent {
	data class Svg(
		val image: List<UByte>
	) : ImageContent()
	data class Png(
		val image: List<UByte>
	) : ImageContent()
}
fun SignerImage.toImageContent(): ImageContent {
	return when (this) {
		is SignerImage.Png -> ImageContent.Png(this.image)
		is SignerImage.Svg -> ImageContent.Svg(this.image)
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
private fun PreviewIdentIcon() {
	SignerNewTheme {
		val iconPng = PreviewData.exampleIdenticonPng
		val iconSvg = PreviewData.exampleIdenticonPng //todo dmitry sample

		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			IdentIcon(iconPng)
			IdentIcon(iconPng)
			IdentIcon(iconPng, size = 18.dp)
			IdentIcon(iconPng, size = 18.dp)
			IdentIcon(iconPng, size = 56.dp)
			IdentIcon(iconPng, size = 56.dp)
		}
	}
}
