package io.parity.signer.screens.logs

import io.parity.signer.uniffi.Event
import io.parity.signer.uniffi.History
import io.parity.signer.uniffi.MLog


data class LogsScreenModel(val logs: List<LogsListEntryModel>)

fun MLog.toLogListEntries(): List<LogsListEntryModel> {
	val logs: Sequence<LogsListEntryModel.LogEntryModel> =
		log.asSequence().flatMap { it.toLogEntryModels() }
	val result = mutableListOf<LogsListEntryModel>()

	logs.forEach {
		//todo dmitry add dates
		result.add(it)
	}

	return result.toList()
}


sealed class LogsListEntryModel {
	data class LogEntryModel(
		val timestamp: String,
		//id of this group of events, not unique per event
		val logGroupId: UInt,
		val event: Event,
	) : LogsListEntryModel()

	data class TimeSeparatorModel(
		val month: String,
		val dayOfWeek: Byte,
		val year: Int,
	) : LogsListEntryModel()
}

fun History.toLogEntryModels(): List<LogsListEntryModel.LogEntryModel> =
	events.map { LogsListEntryModel.LogEntryModel(timestamp, order, it) }
