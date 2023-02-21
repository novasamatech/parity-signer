package io.parity.signer.screens.onboarding.eachstartchecks.rootcheck

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Warning
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun RootExposedScreen() {
	val iconBackground = Color(0x1FAC7D1F)
	val iconTint = Color(0xFFFD4935)
	Column() {
		Box(
			contentAlignment = Alignment.Center,
			modifier = Modifier
				.size(156.dp)
				.background(iconBackground, CircleShape),
		) {
			Image(
				imageVector = Icons.Rounded.Warning,
				contentDescription = null,
				colorFilter = ColorFilter.tint(iconTint),
				modifier = Modifier
					.size(80.dp)
			)
		}

		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 8.dp),
			text = "Please try again with not rooted device",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 8.dp),
			text = "We’ve detected that this Device Has Been Rooted and Isn’t Safe to Use. Please try again with another device",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
	}
}


@Preview(
	name = "light", group = "themes", uiMode = UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewRootExposedScreen() {
	SignerNewTheme() {
		RootExposedScreen()
	}
}
