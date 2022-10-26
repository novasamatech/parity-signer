package io.parity.signer.screens.logs.items

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Smartphone
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.screens.keysets.KeySetsMenuBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun LogItem() {

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
private fun PreviewOldLog() {
	SignerNewTheme {
		Box() {
			HistoryCardTemplate(
				image = Icons.Default.Smartphone,
				line1 = "2015, 20, 212",
				line2 = "Database initiated",
				line3 = ""
			)
		}
	}
}
