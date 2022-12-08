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
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.models.toBytes
import io.parity.signer.uniffi.SignerImage

/**
 * Just draw a standard identicon used everywhere, with standard size
 */
@Composable
fun IdentIcon(identicon: SignerImage, size: Dp = 28.dp,
							modifier: Modifier = Modifier) {
	Image(
		identicon.toBytes().intoImageBitmap(),
		stringResource(R.string.description_identicon),
		modifier = modifier
			.size(size)
			.clip(CircleShape)
	)
}
