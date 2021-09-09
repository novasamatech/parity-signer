package io.parity.signer.components.TransactionCards

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import org.json.JSONObject

@Composable
fun TCEraImmortalNonce(payload: JSONObject) {
	Row(
		horizontalArrangement = Arrangement.SpaceEvenly,
		modifier = Modifier.fillMaxWidth()
	) {
		Column {
			Text("Immortal")
			Text("era")
		}
		Column {
			Text("nonce")
			Text(payload.getString("nonce"))
		}
	}
}
