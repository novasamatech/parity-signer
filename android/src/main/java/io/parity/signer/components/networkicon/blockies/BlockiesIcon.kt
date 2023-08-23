package io.parity.signer.components.networkicon.blockies

import android.content.res.Configuration
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.components.networkicon.blockies.svalinn.Blockies
import io.parity.signer.components.networkicon.blockies.svalinn.BlockiesPainter
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.iconsBackground
import kotlin.math.sqrt

@Composable
fun BlockiesIcon(
	seed: String,
	preferedSize: Dp,
	modifier: Modifier = Modifier
) {
	val blockies: Blockies = Blockies.fromSeed(seed)
	Box(
		modifier = modifier
			.size(preferedSize)
			.background(MaterialTheme.colors.iconsBackground, CircleShape),
		contentAlignment = Alignment.Center
	)
	{
		Canvas(
			modifier = Modifier
				.size(preferedSize.div(sqrt(2f)))
		) {
			BlockiesPainter.draw(
				blockies = blockies,
				canvas = drawContext.canvas,
				width = size.width,
				height = size.height
			)
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
private fun PreviewBlockiesIcon() {
	SignerNewTheme {
		Column(horizontalAlignment = Alignment.CenterHorizontally) {
			BlockiesIcon("0xb00adb8980766d75518dfa8efa139fe0d7bb5e4e", 48.dp)
			BlockiesIcon("0x7204ddf9dc5f672b64ca6692da7b8f13b4d408e7", 32.dp)
		}
	}
}
