package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import org.json.JSONObject

@Composable
fun TCETXSpec(payload: JSONObject) {
	Row(
		horizontalArrangement = Arrangement.SpaceEvenly,
		modifier = Modifier.fillMaxWidth()
	) {
		Column {
			Text("network")
			Text(payload.getString("network"))
		}
		Column {
			Text("spec version")
			Text(payload.getString("version"))
		}
		Column {
			Text("tx version")
			Text(payload.getString("tx_version"))
		}
	}
}
