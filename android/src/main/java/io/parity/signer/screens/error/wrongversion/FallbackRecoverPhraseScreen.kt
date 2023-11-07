package io.parity.signer.screens.error.wrongversion

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface

@Composable
fun FallbackRecoverPhraseScreen(
	seedList: List<String>,
	onSelected: (clickedKeyName: String) -> Unit,
	onBack: Callback,
) {
	Column {

		ScreenHeader(
			title = "Select Key Set", //todo dmitry export
			onBack = onBack
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f),
			text = "Select a key set you want to Expose a Secret Recovery Phrase for",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
		)
		Column {

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
private fun FallbackRecoverPhraseScreenPreview() {
	SignerNewTheme() {
		FallbackRecoverPhraseScreen(
			seedList = listOf(
				"Omni Wallet", "One very very very very very very long lina name text",
				"Stacking", "Nova Wallet", "Crowdloans"
			),
			onSelected = {},
			onBack = {},
		)
	}
}
