package io.parity.signer.screens.onboarding

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R

/**
 * Screen that might be shown when user should wait for something
 */
@Composable
fun WaitingScreen() {
	Column {
		Text("Please wait...")
		CircularProgressIndicator()
		Image(
			painter = painterResource(id = R.drawable.app_logo),
			contentDescription = "Icon"
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
private fun PreviewWaitingScreen() {
	WaitingScreen()
}
