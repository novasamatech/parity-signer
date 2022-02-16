package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCEra(payload: JSONObject) {
	Column {
		TCNameValueTemplate(name = "phase", value = payload.optString("phase"))
		TCNameValueTemplate(name = "period", value = payload.optString("period"))
	}
}
