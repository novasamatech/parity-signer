package io.parity.signer.components.blockies

import android.content.res.Configuration
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.components.blockies.svalinn.Blockies
import io.parity.signer.components.blockies.svalinn.BlockiesPainter
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun BlockiesIcon(
	seed: String,
	preferedSize: Dp,
	modifier: Modifier = Modifier
) {
	val blockies: Blockies = Blockies.fromSeed(seed)
// Layout
	Canvas(
		modifier = modifier
            .size(preferedSize)
            .clip(CircleShape)
	) {
		BlockiesPainter.draw(
			blockies = blockies,
			canvas = drawContext.canvas,
			width = size.width,
			height = size.height
		)
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
			BlockiesIcon("0xc0ffee254729296a45a3885639AC7E10F9d54979", 64.dp)
			BlockiesIcon("0x999999cf1046e68e36E1aA2E0E07105eDDD1f08E", 32.dp)
			BlockiesIcon("0xD2AAD5732c980AaddDe38CEAD950dBa91Cd2C726", 18.dp)
			BlockiesIcon("0x1524d026FCAa9F1ceeE3540dEeeE3359BAD6bfF9", 64.dp)
		}
	}
}
