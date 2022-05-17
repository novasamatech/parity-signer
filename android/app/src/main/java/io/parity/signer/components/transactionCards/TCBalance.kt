package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MscCurrency

@Composable
fun TCBalance(currency: MscCurrency) {
	Row {
		Text(
			currency.amount + " " + currency.units,
			style = MaterialTheme.typography.body1,
			color = MaterialTheme.colors.Text600
		)
	}
}
