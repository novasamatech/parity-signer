package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.uniffi.SignerImage

/**
 * Just draw a standard identicon used everywhere, with standard size
 */
@OptIn(ExperimentalUnsignedTypes::class)
@Composable
fun IdentIcon(identicon: ImageContent, size: Dp = 28.dp,
							modifier: Modifier = Modifier) {
	Image(
		identicon.toBytes().intoImageBitmap(),
		stringResource(R.string.description_identicon),
		modifier = modifier
			.size(size)
			.clip(CircleShape)
	)
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

fun ImageContent.toBytes(): List<UByte> {
	val image = when (this) {
		is ImageContent.Png -> this.image
		is ImageContent.Svg -> listOf() //todo implementSvg
	}
	return image
}
