package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.intoImageBitmap

/**
 * Just draw a standard identicon used everywhere, with standard size
 */
@Composable
fun IdentIcon(identicon: List<UByte>, size: Dp = 28.dp, modifier: Modifier = Modifier) {
	Image(
		identicon.intoImageBitmap(), stringResource(R.string.description_identicon), modifier = modifier.size(size)
	)
}
