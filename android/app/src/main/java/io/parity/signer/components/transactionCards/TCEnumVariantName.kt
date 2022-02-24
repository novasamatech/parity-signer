package io.parity.signer.components.transactionCards

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCEnumVariantName(payload: JSONObject) {
	//TODO: add docs
	Text(payload.optString("name"))
}
