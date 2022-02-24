package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCTXSpecPlain(payload: JSONObject) {
	Column {
		Text("Unknown network")
		TCNameValueTemplate(name = "Genesis hash", value = payload.optString("network_genesis_hash"))
		TCNameValueTemplate(name = "Version", value = payload.optString("version"))
		TCNameValueTemplate(name = "Tx version", value = payload.optString("tx_version"))
	}
}
