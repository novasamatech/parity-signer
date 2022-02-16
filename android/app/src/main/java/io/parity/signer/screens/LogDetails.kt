package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.HistoryCardExtended
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun LogDetails(signerDataModel: SignerDataModel) {
	val content = signerDataModel.screenData.value?: JSONObject()
	Column {
		Text(content.optString("timestamp"))
		LazyColumn {
			items(content.optJSONArray("events")?.length() ?: 0) { index ->
				HistoryCardExtended(card = content.getJSONArray("events").getJSONObject(index))
			}
		}
	}
}
