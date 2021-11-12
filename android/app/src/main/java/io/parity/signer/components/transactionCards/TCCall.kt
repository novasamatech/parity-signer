package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCCall(payload: JSONObject) {
	Row {
		Text(payload.getString("method"))
		Text(" from ")
		Text(payload.getString("pallet"))
	}
}
