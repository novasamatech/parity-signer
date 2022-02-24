package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import org.json.JSONObject

@Composable
fun TCTypesInfo(payload: JSONObject) {
	Row {
		Identicon(identicon = payload.optString("types_id_pic"))
		TCNameValueTemplate(name = "Types hash:", value = payload.optString("types_hash"))
	}
}
