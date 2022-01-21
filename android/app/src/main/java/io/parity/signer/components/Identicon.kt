package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.intoImageBitmap

/**
 * Just draw a standard identicon used everywhere, with standard size
 */
@Composable
fun Identicon(identicon: String) {
	Image(
			identicon.intoImageBitmap(), "identicon", modifier = Modifier.size(28.dp)
		)
}
