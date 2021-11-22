package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Done
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import org.json.JSONObject

@Composable
fun HistoryCard(card: JSONObject, timestamp: String) {
	when(card.getString("event")) {
		"database_initiated" -> {
			HistoryCardTemplate(image = Icons.Default.CheckCircle, line1 = timestamp, line2 = "Database initiated", line3 = "")
		}
		"device_was_online" -> {
			HistoryCardTemplate(image = Icons.Default.CheckCircle, line1 = timestamp, line2 = "Device was online", line3 = "", danger = true)
		}
		"error" -> {
			HistoryCardTemplate(image = Icons.Default.CheckCircle, line1 = timestamp, line2 = "Error!", line3 = card.getJSONObject("payload").toString(), danger = true)
		}
		"general_verifier_added" -> {
			HistoryCardTemplate(image = Icons.Default.Lock, line1 = timestamp, line2 = "General verifier set", line3 = card.getJSONObject("payload").getString("hex"))
		}
		"identity_added" -> {
			HistoryCardTemplate(image = Icons.Default.Lock, line1 = timestamp, line2 = "Key created", line3 = card.getJSONObject("payload").getString("seed_name") + card.getJSONObject("payload").getString("path"))
		}
		"transaction_signed" -> {
			HistoryCardTemplate(image = Icons.Default.Done, line1 = timestamp, line2 = "Transaction signed", line3 = card.getJSONObject("payload").getString("user_comment"))
		}
		else -> {
			HistoryCardTemplate(image = Icons.Default.CheckCircle, line1 = timestamp, line2 = card.getString("event"), line3 = "")
		}
	}
}
