package io.parity.signer.components2.base

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.pink500

@Composable
fun CtaButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	background: Color = MaterialTheme.colors.pink500,
	onClicked: () -> Unit,
) {
	Column(
		modifier = modifier
			.clickable(onClick = onClicked)
			.background(MaterialTheme.colors.pink500, RoundedCornerShape(40.dp))
			.fillMaxWidth()
			.padding(vertical = 16.dp),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Text(
			text = label,
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun SecondaryButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	onClicked: () -> Unit,
) {
	Column(
		modifier = modifier
			.clickable(onClick = onClicked)
			.padding(vertical = 16.dp)
			.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Text(
			text = label,
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.TitleS,
			maxLines = 1,
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewCtaButtons() {
	SignerNewTheme {
		Box(modifier = Modifier.size(300.dp),
			contentAlignment = Alignment.BottomStart) {
			CtaButtonBottomSheet("button") {}
		}
	}
}
