package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Text600
import org.json.JSONObject

@Composable
fun TCBalance(currency: JSONObject) {
	Row {
		Text(currency.getString("amount") + " " + currency.getString("units"), style = MaterialTheme.typography.body1, color = MaterialTheme.colors.Text600)
	}
}
