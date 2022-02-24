package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.ui.theme.Crypto400
import org.json.JSONObject

@Composable
fun TCVerifier(payload: JSONObject) {
	Column {
		Text("VERIFIER CERTIFICATE")
		Row {
			Identicon(identicon = payload.optString("identicon"))
			Column {
				Row {
					Text("key:")
					Text(payload.optString("public_key"), color = MaterialTheme.colors.Crypto400)
				}
				Row {
					Text("crypto:")
					Text(payload.optString("encryption"), color = MaterialTheme.colors.Crypto400)
				}
			}
		}
	}
}
