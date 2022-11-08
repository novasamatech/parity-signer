package io.parity.signer.screens.logs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.screens.logs.items.HistoryCardSelectorOld
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MLog

/**
 * todo dmitry remove it use LogsScreen
 */
@Composable
fun HistoryScreen(
	mLog: MLog,
	button: (action: Action, details: String) -> Unit
) {
	val history = mLog.log

	Column {
		LazyColumn {
			for (record in history) {
				val timestamp = record.timestamp

				this.items(
					items = record.events,
					key = { record.order.toString() + it.toString() }
				) { item ->
					Row(
						Modifier.clickable {
							button(
								Action.SHOW_LOG_DETAILS,
								record.order.toString()
							)
						}
					) {
						HistoryCardSelectorOld(
							item,
							timestamp
						)
					}
				}
			}
		}
	}
}
