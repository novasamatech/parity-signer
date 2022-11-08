package io.parity.signer.screens.logs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.models.DateUtils
import io.parity.signer.uniffi.Event
import io.parity.signer.uniffi.History
import io.parity.signer.uniffi.MLog
import java.util.Calendar


data class LogsScreenModel(val logs: List<LogsListEntryModel>)

@Composable
fun MLog.toLogsScreenModel(): LogsScreenModel {
	val result = mutableListOf<LogsListEntryModel>()

	var date: Calendar? = null
	var lastShownDay: String? = null

	log.forEach { (order, rustTimestamp, listOfEvents) ->
		date = DateUtils.parseLogTime(rustTimestamp)
		val dayString = "some"//date.some todo dmitry
		val timeString = "10:42" // DateUtils todo dmitry
		listOfEvents.forEach { event ->
			if (lastShownDay != dayString) {
				result.add(LogsListEntryModel.TimeSeparatorModel(dayString))
				lastShownDay = dayString
			}
			result.add(
				LogsListEntryModel.LogEntryModel(
					logGroupId = order,
					title = event.getViewTitle(),
					message = event.getViewMessage(),
					timeStr = timeString,
					isDanger = event.isDanger(),
				)
			)
		}
	}
	return LogsScreenModel(result.toList())
}


sealed class LogsListEntryModel {
	data class LogEntryModel(
		//id of this group of events, not unique per event
		val logGroupId: UInt,
		val title: String,
		val message: String,
		val timeStr: String,
		val isDanger: Boolean,
	) : LogsListEntryModel()

	data class TimeSeparatorModel(
		val dateStr: String,
	) : LogsListEntryModel()
}


@Composable
fun Event.getViewTitle(): String {
	return when (this) {
		Event.DatabaseInitiated -> stringResource(R.string.log_title_database_initiated)
		Event.DeviceWasOnline -> TODO()
		is Event.GeneralVerifierSet -> TODO()
		Event.HistoryCleared -> TODO()
		Event.IdentitiesWiped -> TODO()
		is Event.IdentityAdded -> TODO()
		is Event.IdentityRemoved -> TODO()
		is Event.MessageSignError -> TODO()
		is Event.MessageSigned -> TODO()
		is Event.MetadataAdded -> TODO()
		is Event.MetadataRemoved -> TODO()
		is Event.MetadataSigned -> TODO()
		is Event.NetworkSpecsAdded -> TODO()
		is Event.NetworkSpecsRemoved -> TODO()
		is Event.NetworkSpecsSigned -> TODO()
		is Event.NetworkVerifierSet -> TODO()
		Event.ResetDangerRecord -> TODO()
		is Event.SecretWasExported -> TODO()
		is Event.SeedCreated -> TODO()
		is Event.SeedNameWasShown -> TODO()
		is Event.SeedRemoved -> TODO()
		is Event.SystemEntry -> TODO()
		is Event.TransactionSignError -> TODO()
		is Event.TransactionSigned -> TODO()
		is Event.TypesAdded -> TODO()
		is Event.TypesRemoved -> TODO()
		is Event.TypesSigned -> TODO()
		is Event.UserEntry -> TODO()
		is Event.Warning -> TODO()
		Event.WrongPassword -> TODO()
	}
}

@Composable
fun Event.getViewMessage(): String {
	return when (this) {
		Event.DatabaseInitiated -> TODO()
		Event.DeviceWasOnline -> TODO()
		is Event.GeneralVerifierSet -> TODO()
		Event.HistoryCleared -> TODO()
		Event.IdentitiesWiped -> TODO()
		is Event.IdentityAdded -> TODO()
		is Event.IdentityRemoved -> TODO()
		is Event.MessageSignError -> TODO()
		is Event.MessageSigned -> TODO()
		is Event.MetadataAdded -> TODO()
		is Event.MetadataRemoved -> TODO()
		is Event.MetadataSigned -> TODO()
		is Event.NetworkSpecsAdded -> TODO()
		is Event.NetworkSpecsRemoved -> TODO()
		is Event.NetworkSpecsSigned -> TODO()
		is Event.NetworkVerifierSet -> TODO()
		Event.ResetDangerRecord -> TODO()
		is Event.SecretWasExported -> TODO()
		is Event.SeedCreated -> TODO()
		is Event.SeedNameWasShown -> TODO()
		is Event.SeedRemoved -> TODO()
		is Event.SystemEntry -> TODO()
		is Event.TransactionSignError -> TODO()
		is Event.TransactionSigned -> TODO()
		is Event.TypesAdded -> TODO()
		is Event.TypesRemoved -> TODO()
		is Event.TypesSigned -> TODO()
		is Event.UserEntry -> TODO()
		is Event.Warning -> TODO()
		Event.WrongPassword -> TODO()
	}
}

fun Event.isDanger(): Boolean {
	return false //todo dmitry
}
