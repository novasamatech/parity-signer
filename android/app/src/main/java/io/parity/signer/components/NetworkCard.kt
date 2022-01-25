package io.parity.signer.components

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowCircleDown
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Web3Typography
import org.json.JSONObject

@Composable
fun NetworkCard(network: JSONObject) {
	Row {
		Text(network.optString("logo", network.optString("network_logo")), style = Web3Typography.h4)
		Spacer(Modifier.width(15.dp))
		Text(network.optString("title", network.optString("network_title")), style = MaterialTheme.typography.h3)
	}
}
