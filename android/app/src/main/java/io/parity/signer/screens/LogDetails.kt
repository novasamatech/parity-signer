package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.HistoryCardExtended
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun LogDetails(screenData: JSONObject) {
	Column {
		Text(screenData.optString("timestamp"))
		LazyColumn {
			items(screenData.optJSONArray("events")?.length() ?: 0) { index ->
				HistoryCardExtended(card = screenData.getJSONArray("events").getJSONObject(index))
			}
		}
	}
}
