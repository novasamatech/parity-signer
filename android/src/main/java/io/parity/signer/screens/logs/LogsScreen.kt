package io.parity.signer.screens.logs

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.HistoryCard
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.Navigator
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.Event
import io.parity.signer.uniffi.History
import io.parity.signer.uniffi.MLog


@Composable
fun LogsScreen(
	model: LogsScreenModel,
	navigator: Navigator,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
//	ScreenHeader(stringId = , backEnabled = , menuEnabled = , navigator = )
		Box() {
			LazyColumn {
				items(
					items = model.logs,
					key = { it.hashCode() }
				) { item ->
					Row(
						Modifier.clickable {
							navigator.navigate(
								Action.SHOW_LOG_DETAILS,
								item.logGroupId.toString()
							)
						}
					) {
//							HistoryCard( //todo dmitry item create
//								item,
//								timestamp
//							)
					}
				}
			}
		}
		BottomBar2(navigator = navigator, state = BottomBar2State.LOGS)
	}
}

@Composable
fun LogItem(
	model: History
) {

}

data class LogsScreenModel(val logs: List<LogsListEntry>)

fun MLog.toLogListEntries(): List<LogsListEntry> {
	val logs: Sequence<LogsListEntry.LogEntryModel> = log.asSequence().flatMap { it.toLogEntryModels() }
	val added = log.flatMapC
}


sealed class LogsListEntry {
	data class LogEntryModel(
		val timestamp: String,
		//id of this group of events, not unique per event
		val logGroupId: UInt,
		val event: Event,
	) : LogsListEntry()

	data class TimeSeparatorModel(
		val timestamp: String,
	) : LogsListEntry()
}

fun History.toLogEntryModels(): List<LogEntryModel> =
	events.map { LogEntryModel(timestamp, order, it) }

