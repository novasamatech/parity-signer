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
fun TCEra(payload: JSONObject) {
	Row(
		horizontalArrangement = Arrangement.SpaceEvenly,
		modifier = Modifier.fillMaxWidth()
	) {
		when(payload.getString("era")) {
			"Mortal" -> {
				Text("Mortality: ")
				Column {
					Text("phase")
					Text(payload.getString("phase"))
				}
				Column {
					Text("period")
					Text(payload.getString("period"))
				}
			}
			"Immortal" -> {
				Text("Immortal transaction")
			}
			else -> {
				Text("Era invalid, file a bug report")
			}
		}
	}
}
