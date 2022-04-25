package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.HistoryCardExtended
import io.parity.signer.models.SignerDataModel
import io.parity.signer.uniffi.MLogDetails
import org.json.JSONObject

@Composable
fun LogDetails(logDetails: MLogDetails) {
	Column {
		Text(logDetails.timestamp)
		LazyColumn {
			items(logDetails.events.size) { index ->
				HistoryCardExtended(logDetails.events[index], logDetails.timestamp)
			}
		}
	}
}
