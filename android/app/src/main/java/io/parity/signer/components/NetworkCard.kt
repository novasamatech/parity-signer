package io.parity.signer.components

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun NetworkCard(network: JSONObject) {
	Text(network.optString("title"))
}
