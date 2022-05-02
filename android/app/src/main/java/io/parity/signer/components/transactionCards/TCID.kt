package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import org.json.JSONObject

@Composable
fun TCID(payload: JSONObject) {
	/*
	Row {
		Identicon(identicon = payload.optString("identicon"))
		Column {
			Text(payload.optString("base58"))
		}
	}
	 */
}
