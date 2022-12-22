package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.SeedKeysPreview

@Composable
fun TCDerivations(payload: List<SeedKeysPreview>) {
	Column {
		Text(
			"Importing derivations:",
			style = MaterialTheme.typography.h1,
			color = MaterialTheme.colors.Text600
		)
		for (record in payload) {
			Text(
				record.name,
				style = MaterialTheme.typography.body2,
				color = MaterialTheme.colors.Crypto400
			)
		}
	}
}
