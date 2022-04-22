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
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MLog

@Composable
fun HistoryScreen(mLog: MLog, signerDataModel: SignerDataModel) {
	val history = mLog.log.sortedBy {
		it.order
	}?.reversed()

	Column {
		LazyColumn {
			for (record in history) {
				val order = record.order
				val timestamp = record.timestamp

				this.items(
					items = record.events,
					key = { record.order.toString() + it.toString() }
				) { item ->
					Row(Modifier.clickable {
						signerDataModel.pushButton(
							Action.SHOW_LOG_DETAILS,
							details = record.order.toString()
						)
					}) {
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
