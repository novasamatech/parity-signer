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
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary


@Composable
fun RootExposedScreen() {
	val iconBackground = Color(0x1FAC7D1F)
	val iconTint = Color(0xFFFD4935)

	Column(horizontalAlignment = Alignment.CenterHorizontally) {
		Spacer(modifier = Modifier.weight(1f))
		Box(
			contentAlignment = Alignment.Center,
			modifier = Modifier
				.size(156.dp)
				.background(iconBackground, CircleShape)
		) {
			Image(
				imageVector = Icons.Rounded.Warning,
				contentDescription = null,
				colorFilter = ColorFilter.tint(iconTint),
				modifier = Modifier
					.size(80.dp)
			)
		}
		Spacer(modifier = Modifier.padding(top = 12.dp))
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 32.dp, vertical = 12.dp),
			text = stringResource(R.string.root_exposed_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 40.dp),
			text = stringResource(R.string.root_exposed_description),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.weight(1f))
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
	Box(modifier = Modifier.fillMaxSize(1f)) {
		SignerNewTheme() {
			RootExposedScreen()
		}
	}
}
