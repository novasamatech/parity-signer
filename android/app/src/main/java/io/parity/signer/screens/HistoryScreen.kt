package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.HistoryCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.models.toListOfJSONObjects
import uniffi.signer.Action

@Composable
fun HistoryScreen(signerDataModel: SignerDataModel) {
	val history =
		signerDataModel.screenData.value?.optJSONArray("log")?.toListOfJSONObjects()?.sortedBy {
			it.optInt("order")
		}?.reversed() ?: emptyList()

	Column {
		LazyColumn {
			for (recordJSON in history) {
				val order = recordJSON.optInt("order").toString()
				val timestamp = recordJSON.optString("timestamp")
				val record = recordJSON.optJSONArray("events")?.toListOfJSONObjects() ?: emptyList()
				this.items(
					items = record,
					key = { recordJSON.optString("order") + it.toString() }
				) { item ->
					Row(Modifier.clickable { signerDataModel.pushButton(Action.SHOW_LOG_DETAILS, details = order) }) {
						HistoryCard(
							item,
							timestamp
						)
					}
				}
			}
		}
	}
}
