package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCBitVec(payload: JSONObject) {
	TCNameValueTemplate(name = payload.optString("BitVec"), value = payload.optString("content"))
}
