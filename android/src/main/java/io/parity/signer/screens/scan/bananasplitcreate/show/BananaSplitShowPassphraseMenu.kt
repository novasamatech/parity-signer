package io.parity.signer.screens.scan.bananasplitcreate.show

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun BananaSplitShowPassphraseMenu(
	password: String,
	onClose: Callback,
) {
	Column() {
		BottomSheetHeader(title = "Passphrase", onClose = onClose)
		Text(
			text = password,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			maxLines = 1,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 16.dp)
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
private fun PreviewBananaSplitShowPassphraseMenu() {
	SignerNewTheme {
		BananaSplitShowPassphraseMenu(
			"delirium-claim-clad-down", {},
		)
	}
}
