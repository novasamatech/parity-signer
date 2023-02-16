package io.parity.signer.screens.onboarding.eachstartchecks.airgap

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.Text600


@Composable
fun EnableAirgapScreen() {
	Box(
		contentAlignment = Alignment.Center,
		modifier = Modifier
			.padding(12.dp)
			.fillMaxSize(1f),
	) {
		Text(
			text = stringResource(R.string.enable_airplane_mode_error),
			color = MaterialTheme.colors.Text600
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
private fun PreviewEnableAirgapScreen() {
	EnableAirgapScreen()
}
