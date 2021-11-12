package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCTip(payload: JSONObject) {
	Row {
		Text("Tip: ")
		Text(payload.getString("amount"))
		Text(" " + payload.getString("units"))
	}
}
