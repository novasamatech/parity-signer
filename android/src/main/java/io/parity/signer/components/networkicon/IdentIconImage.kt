package io.parity.signer.components.networkicon

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.components.networkicon.blockies.BlockiesIcon
import io.parity.signer.components.networkicon.dot.DotIcon
<<<<<<< HEAD
import io.parity.signer.components.networkicon.jdenticon.Jdenticon
=======
>>>>>>> master
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Identicon

/**
 * Just draw a standard IdentIcon used everywhere, with standard size
 */
@Composable
fun IdentIconImage(
	identIcon: Identicon,
	modifier: Modifier = Modifier,
	size: Dp = 28.dp
) {
	when (identIcon) {
		is Identicon.Blockies -> {
			BlockiesIcon(
				seed = identIcon.identity,
				preferedSize = size,
				modifier = modifier,
			)
		}

		is Identicon.Dots -> {
<<<<<<< HEAD
			DotIcon(
				seed = identIcon.identity,
				size = size,
				modifier = modifier,
			)
		}
		is Identicon.Jdenticon -> {
			Jdenticon(seed = identIcon.identity, size = size, modifier = modifier)
		}
=======
			DotIcon(seed = identIcon.identity,
				size = size,
				modifier = modifier,
				)
		}
//		is ImageContent.Svg -> {//will be used for another type of icons
//			val context = LocalContext.current
//			val painter = rememberAsyncImagePainter(
//				model = ImageRequest.Builder(context)
//					.decoderFactory(SvgDecoder.Factory())
//					.data(identIcon.toByteArray())
//					.size(Size.ORIGINAL) // Set the target size to load the image at.
//					.build(),
//			)
//			Image(
//				painter = painter,
//				contentDescription = stringResource(R.string.description_identicon),
//				modifier = modifier
//					.size(size)
//					.clip(CircleShape)
//			)
//		}
>>>>>>> master
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
<<<<<<< HEAD
		val iconJdenticon = PreviewData.Identicon.jdenticonIcon
=======
>>>>>>> master

		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
		) {
			IdentIconImage(iconDot)
			IdentIconImage(iconBlockies)
<<<<<<< HEAD
			//jdenticon preview works if you run it, not in default preview and svg showing async
			IdentIconImage(iconJdenticon)
			IdentIconImage(iconDot, size = 18.dp)
			IdentIconImage(iconBlockies, size = 18.dp)
			IdentIconImage(iconJdenticon, size = 18.dp)
			IdentIconImage(iconDot, size = 56.dp)
			IdentIconImage(iconBlockies, size = 56.dp)
			IdentIconImage(iconJdenticon, size = 56.dp)
=======
			IdentIconImage(iconDot, size = 18.dp)
			IdentIconImage(iconBlockies, size = 18.dp)
			IdentIconImage(iconDot, size = 56.dp)
			IdentIconImage(iconBlockies, size = 56.dp)
>>>>>>> master
		}
	}
}
