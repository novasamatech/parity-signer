package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.ui.theme.*

@Composable
fun TCDerivations(payload: List<String>) {
	Column {
		Text(
			stringResource(R.string.transaction_field_import_derivations),
			style = SignerTypeface.BodyL,
			color = MaterialTheme.colors.textTertiary
		)
		for (record in payload) {
			Text(
				record,
				style = SignerTypeface.BodyM,
				color = MaterialTheme.colors.pink300
			)
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
private fun PreviewTCDerivations() {
	SignerNewTheme {
		Column {
			TCDerivations(payload = listOf("Derivation 1", "Derivation 2"))
//			SignerDivider()
		}
	}
}
