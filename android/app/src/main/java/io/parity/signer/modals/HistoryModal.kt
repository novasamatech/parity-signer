package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.components.HistoryCard
import io.parity.signer.models.SignerDataModel

@Composable
fun HistoryModal(signerDataModel: SignerDataModel) {
	val history = signerDataModel.history.observeAsState()

	Column{
		Text("History")
		LazyColumn {
			items(history.value!!.length()) { item ->
				HistoryCard(history.value!!.getJSONObject(item))
			}
		}
	}
}
