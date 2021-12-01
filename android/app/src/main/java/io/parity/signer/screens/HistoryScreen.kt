package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.components.HistoryCard
import io.parity.signer.models.SignerDataModel

@Composable
fun HistoryScreen(signerDataModel: SignerDataModel) {
	val history = signerDataModel.screenInfo.getJSONArray("log")

	Column{
		LazyColumn {
			for(i in 0 until history.length()) {
				items(history.getJSONObject(i).getJSONArray("events").length()) { item ->
					HistoryCard(history.getJSONObject(i).getJSONArray("events").getJSONObject(item), history.getJSONObject(i).getString("timestamp"))
				}
			}
		}
	}
}
