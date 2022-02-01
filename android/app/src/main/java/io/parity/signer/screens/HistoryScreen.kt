package io.parity.signer.modals

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.HistoryCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONArray

@Composable
fun HistoryScreen(signerDataModel: SignerDataModel) {
	val history =
		signerDataModel.screenData.value?.optJSONArray("log") ?: JSONArray()

	Column {
		LazyColumn {
			for (i in 0 until history.length()) {
				items(
					history.getJSONObject(i).getJSONArray("events").length()
				) { item ->
					Row(Modifier.clickable { signerDataModel.pushButton(ButtonID.ShowLogDetails, details = i.toString()) }) {
						HistoryCard(
							history.getJSONObject(i).getJSONArray("events")
								.getJSONObject(item),
							history.getJSONObject(i).getString("timestamp")
						)
					}
				}
			}
		}
	}
}
