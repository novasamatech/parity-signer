package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import org.json.JSONObject

@Composable
fun TCAuthorPlain(payload: JSONObject) {
	Row {
		Identicon(payload.optString("identicon"))
		Column {
			Text("From: ", style = MaterialTheme.typography.body1, color = MaterialTheme.colors.Text400)
			Text(payload.getString("base58"), style = MaterialTheme.typography.body1, color = MaterialTheme.colors.Text600)
		}
	}
}
