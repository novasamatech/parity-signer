package io.parity.signer.components.networkicon.jdenticon

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import coil.compose.rememberAsyncImagePainter
import coil.decode.SvgDecoder
import coil.request.ImageRequest
import coil.size.Size
import io.parity.signer.R
import io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin.Jdenticon
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.appliedStroke
import kotlin.math.sqrt


@Composable
fun Jdenticon(
	seed: String,
	size: Dp,
	modifier: Modifier = Modifier
) {
	val svg = Jdenticon.toSvg(seed, size.value.toInt())

	val context = LocalContext.current
	val painter = rememberAsyncImagePainter(
		model = ImageRequest.Builder(context)
			.decoderFactory(SvgDecoder.Factory())
			.data(svg.toByteArray())
			.size(Size.ORIGINAL) // Set the target size to load the image at.
			.build(),
	)
	Box(
		modifier = modifier
			.size(size)
			.background(Color.White, CircleShape)
			.border(2.dp, MaterialTheme.colors.appliedStroke, CircleShape),
		contentAlignment = Alignment.Center
	)
	{
		Image(
			painter = painter,
			contentDescription = stringResource(R.string.description_identicon),
			modifier = Modifier
				.size(size.div(sqrt(2f)))
		)
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
private fun PreviewJdenticon() {
	SignerNewTheme {
		val seed_name = "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5"
		Column(horizontalAlignment = Alignment.CenterHorizontally) {
			Jdenticon(seed_name, 48.dp)
			Jdenticon(seed_name, 32.dp,)
			Jdenticon(
				seed_name, 32.dp,
				modifier = Modifier.padding(
					top = 16.dp, bottom = 16.dp, start = 16.dp, end = 12.dp
				),
			)
		}
	}
}
