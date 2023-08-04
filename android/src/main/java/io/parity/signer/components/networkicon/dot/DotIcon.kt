package io.parity.signer.components.networkicon.dot

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun DotIcon(
	seed: String,
	size: Dp,
	modifier: Modifier = Modifier
) {
	val colors = DotIconColors.getColors(seed = seed)

	val foreground = DotIconColors.DotIconColorRgb.foreground.toCompose()
	val circleSize = size / 32 * 5

	val positions = DotIconPositions.calculatePositionsCircleSet(size)
	Box(
		modifier = modifier
			.size(size)
			.background(foreground, CircleShape),
		contentAlignment = Alignment.Center,
	) {
		repeat(19) { i ->
			Circle(
				offset = positions[i],
				color = colors[i].toCompose(),
				size = circleSize,
			)
		}
	}
}

@Composable
private fun Circle(offset: DotIconCircleOffset, color: Color, size: Dp) {
	Box(
		modifier = Modifier
			.size(size)
			.offset(x = offset.x, y = offset.y)
			.background(color, CircleShape)
	)
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
private fun PreviewDotIcon() {
	SignerNewTheme {
		Column(horizontalAlignment = Alignment.CenterHorizontally) {
			DotIcon("0xb00adb8980766d75518dfa8efa139fe0d7bb5e4e", 48.dp)
//			DotIcon("0x7204ddf9dc5f672b64ca6692da7b8f13b4d408e7", 32.dp)
		}
	}
}
