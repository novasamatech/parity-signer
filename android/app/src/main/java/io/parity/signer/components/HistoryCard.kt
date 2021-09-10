package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import org.json.JSONObject

@Composable
fun HistoryCard(card: JSONObject) {
	Column {
		Text(card.getString("timestamp"))
		Column {
			for (i in 0 until card.getJSONArray("events").length()) {
				Text(card.getJSONArray("events").getJSONObject(i).getString("event"), modifier = Modifier.padding(start = 40.dp))
			}
		}
	}
}
