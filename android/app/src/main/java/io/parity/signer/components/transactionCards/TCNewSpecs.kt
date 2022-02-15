package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCNewSpecs(payload: JSONObject) {
	Column {
		Text("NEW NETWORK")
		TCNameValueTemplate(name = "Network name:", value = payload.optString("title"))
		TCNameValueTemplate(name = "base58 prefix:", value = payload.optString("base58prefix"))
		TCNameValueTemplate(name = "decimals:", value = payload.optString("decimals"))
		TCNameValueTemplate(name = "unit:", value = payload.optString("unit"))
		TCNameValueTemplate(name = "genesis hash:", value = payload.optString("genesis_hash"))
		TCNameValueTemplate(name = "crypto:", value = payload.optString("encryption"))
		TCNameValueTemplate(name = "spec name:", value = payload.optString("name"))
		TCNameValueTemplate(name = "logo:", value = payload.optString("logo"))
		TCNameValueTemplate(name = "default path", value = payload.optString("path_id"))
	}
}
