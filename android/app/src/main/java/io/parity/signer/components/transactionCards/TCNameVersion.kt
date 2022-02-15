package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCNameVersion(payload: JSONObject) {
	TCNameValueTemplate(name = payload.optString("name"), value = payload.optString("version"))
}
