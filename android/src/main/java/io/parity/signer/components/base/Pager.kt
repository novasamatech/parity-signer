package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill30
import io.parity.signer.ui.theme.pink300


@Composable
fun PageIndicatorDots(
	totalDots: Int,
	selectedIndex: Int,
	selectedColor: Color = MaterialTheme.colors.pink300,
	unSelectedColor: Color = MaterialTheme.colors.fill30,
) {
	LazyRow(
		modifier = Modifier
			.wrapContentWidth()
			.wrapContentHeight()
	) {
		items(totalDots) { index ->
			if (index + 1 == selectedIndex) {
				Box(
					modifier = Modifier
						.size(5.dp)
						.clip(CircleShape)
						.background(selectedColor)
				)
			} else {
				Box(
					modifier = Modifier
						.size(5.dp)
						.clip(CircleShape)
						.background(unSelectedColor)
				)
			}

			if (index + 1 != totalDots) {
				Spacer(modifier = Modifier.padding(horizontal = 2.dp))
			}
		}
	}
}


@Composable
fun PageIndicatorLine(
	totalDots: Int,
	selectedIndex: Int,
	selectedColor: Color = MaterialTheme.colors.primary,
	unSelectedColor: Color = MaterialTheme.colors.fill30,
	modifier: Modifier = Modifier,
) {
	Row(
		modifier = modifier
	) {
		repeat(totalDots) { index ->
			Box(
				modifier = Modifier
					.weight(1f)
					.height(4.dp)
					.background(
						color = if (index + 1 <= selectedIndex) selectedColor else unSelectedColor,
						shape = RoundedCornerShape(4.dp)
					)
			)
			if (index + 1 != totalDots) {
				Spacer(modifier = Modifier.padding(horizontal = 4.dp))
			}
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
private fun PreviewPageIndicator() {
	SignerNewTheme {
		Column(Modifier.width(80.dp)) {
			Spacer(modifier = Modifier.padding(top = 2.dp))
			PageIndicatorDots(4, 2)
			SignerDivider(modifier = Modifier.padding(vertical = 2.dp))
			PageIndicatorLine(4, 2)
			Spacer(modifier = Modifier.padding(top = 2.dp))
		}
	}
}

