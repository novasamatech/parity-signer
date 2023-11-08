package io.parity.signer.components.networkicon

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.components.networkicon.blockies.BlockiesIcon
import io.parity.signer.components.networkicon.dot.DotIcon
import io.parity.signer.components.networkicon.jdenticon.Jdenticon
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Identicon

/**
 * Just draw a standard IdentIcon used everywhere, with standard size
 */
@Composable
fun IdentIconImage(
	identicon: Identicon,
	modifier: Modifier = Modifier,
	size: Dp = 28.dp
) {
	when (identicon) {
		is Identicon.Blockies -> {
			BlockiesIcon(
				seed = identicon.identity,
				preferedSize = size,
				modifier = modifier,
			)
		}

		is Identicon.Dots -> {
			DotIcon(
				seed = identicon.identity,
				size = size,
				modifier = modifier,
			)
		}

		is Identicon.Jdenticon -> {
			Jdenticon(seed = identicon.identity, size = size, modifier = modifier)
		}
	}
}

/**
 * As svg parsed async and in preview Dispatchers.IO is not working
 * run preview on device to see it.
 * Consider creating custom coil.ImageLoader with main ui fetch and parse dispatchers. Maybe for preview only.
 */
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
		val iconDot = PreviewData.Identicon.dotIcon
		val iconBlockies = PreviewData.Identicon.blockiesIcon
		val iconJdenticon = PreviewData.Identicon.jdenticonIcon

		Column(
			modifier = Modifier.padding(horizontal = 24.dp),
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			IdentIconImage(iconDot)
			IdentIconImage(iconBlockies)
			//jdenticon preview works if you run it, not in default preview and svg showing async
			IdentIconImage(iconJdenticon)
			IdentIconImage(iconDot, size = 18.dp)
			IdentIconImage(iconBlockies, size = 18.dp)
			IdentIconImage(iconJdenticon, size = 18.dp)
			IdentIconImage(iconDot, size = 56.dp)
			IdentIconImage(iconBlockies, size = 56.dp)
			IdentIconImage(iconJdenticon, size = 56.dp)

			IdentIconImage(
				iconDot, modifier = Modifier.padding(
					top = 16.dp, bottom = 16.dp, start = 16.dp, end = 12.dp
				), size = 32.dp
			)
			IdentIconImage(
				iconBlockies, modifier = Modifier.padding(
					top = 16.dp, bottom = 16.dp, start = 16.dp, end = 12.dp
				), size = 32.dp
			)
			IdentIconImage(
				iconJdenticon, modifier = Modifier.padding(
					top = 16.dp, bottom = 16.dp, start = 16.dp, end = 12.dp
				), size = 32.dp
			)
		}
	}
}
