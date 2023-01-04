package io.parity.signer.screens.keyderivation

import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun DeriveKeyBaseScreen() {

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
private fun PreviewDeriveKeyBaseScreen() {
	SignerNewTheme {
		DeriveKeyBaseScreen()
	}
}
