package io.parity.signer.screens.logs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.domain.DateUtils
import io.parity.signer.domain.abbreviateString
import io.parity.signer.domain.toLogDateString
import io.parity.signer.domain.toLogTimeString
import io.parity.signer.screens.logs.items.LogsListEntryModel
import io.parity.signer.uniffi.*


data class LogsScreenModel(val logs: List<LogsListEntryModel>)

@Composable
fun MLog.toLogsScreenModel(): LogsScreenModel {
	val result = mutableListOf<LogsListEntryModel>()

	var lastShownDay: String? = null
	log.forEach { (order, rustTimestamp, listOfEvents) ->
		val date = DateUtils.parseLogTime(rustTimestamp)
		val dayString = date?.toLogDateString()
		val timeString = date?.toLogTimeString()

		listOfEvents.forEach { event ->
			if (lastShownDay != dayString && dayString != null) {
				result.add(LogsListEntryModel.TimeSeparatorModel(dayString))
				lastShownDay = dayString
			}
			result.add(
				LogsListEntryModel.LogEntryModel(
					logGroupId = order,
					title = event.getViewTitle(),
					message = event.getViewMessage() ?: "",
					timeStr = timeString ?: "",
					isDanger = event.isDanger(),
				)
			)
		}
	}
	return LogsScreenModel(result.toList())
}


@Composable
fun Event.getViewTitle(): String {
	return when (this) {
		Event.DatabaseInitiated -> stringResource(R.string.log_title_database_initiated)
		Event.DeviceWasOnline -> stringResource(R.string.log_title_device_was_online)
		is Event.GeneralVerifierSet -> stringResource(R.string.log_title_general_virifier_set)
		Event.HistoryCleared -> stringResource(R.string.log_title_history_cleared)
		Event.IdentitiesWiped -> stringResource(R.string.log_title_identities_wiped)
		is Event.IdentityAdded -> stringResource(R.string.log_title_identity_added)
		is Event.IdentityRemoved -> stringResource(R.string.log_title_identity_removed)
		is Event.MessageSignError -> stringResource(R.string.log_title_messages_error)
		is Event.MessageSigned -> stringResource(R.string.log_title_message_signed)
		is Event.MetadataAdded -> stringResource(R.string.log_title_metadata_added)
		is Event.MetadataRemoved -> stringResource(R.string.log_title_metadata_removed)
		is Event.MetadataSigned -> stringResource(R.string.log_title_metadata_signed)
		is Event.NetworkSpecsAdded -> stringResource(R.string.log_title_network_added)
		is Event.NetworkSpecsRemoved -> stringResource(R.string.log_title_network_removed)
		is Event.NetworkSpecsSigned -> stringResource(R.string.log_title_network_specs_signed)
		is Event.NetworkVerifierSet -> stringResource(R.string.log_title_network_verifier_set)
		Event.ResetDangerRecord -> stringResource(R.string.log_title_reset_danger_record)
		is Event.SecretWasExported -> stringResource(R.string.log_title_secret_was_exported)
		is Event.SeedCreated -> stringResource(R.string.log_title_seed_created)
		is Event.SeedNameWasShown -> stringResource(R.string.log_title_seed_name_was_shown)
		is Event.SeedRemoved -> stringResource(R.string.log_title_seed_removed)
		is Event.SystemEntry -> stringResource(R.string.log_title_system_entry)
		is Event.TransactionSignError -> stringResource(R.string.log_title_transaction_sign_error)
		is Event.TransactionSigned -> stringResource(R.string.log_title_transaction_signed)
		is Event.TypesAdded -> stringResource(R.string.log_title_types_added)
		is Event.TypesRemoved -> stringResource(R.string.log_title_types_removed)
		is Event.TypesSigned -> stringResource(R.string.log_title_types_signed)
		is Event.UserEntry -> stringResource(R.string.log_title_user_entry)
		is Event.Warning -> stringResource(R.string.log_title_warning)
		Event.WrongPassword -> stringResource(R.string.log_title_wrong_password)
	}
}

/**
 * Null when item was not created in legacy History screen
 */
@Composable
fun Event.getViewMessage(): String? {
	return when (this) {
		Event.DatabaseInitiated -> ""
		Event.DeviceWasOnline -> ""
		is Event.GeneralVerifierSet -> {
			this.verifier.v.let {
				if (it is VerifierValue.Standard) {
					it.m.getOrElse(0) { "" }
						.abbreviateString(8) + it.m.getOrElse(1) { "" }
				} else null
			}
		}
		Event.HistoryCleared -> ""
		Event.IdentitiesWiped -> ""
		is Event.IdentityAdded -> this.identityHistory.seedName + this.identityHistory.path
		is Event.IdentityRemoved -> this.identityHistory.seedName + this.identityHistory.path
		is Event.MessageSignError -> this.signMessageDisplay.userComment
		is Event.MessageSigned -> this.signMessageDisplay.userComment
		is Event.MetadataAdded -> stringResource(R.string.log_message_metadata, this.metaValuesDisplay.name, this.metaValuesDisplay.version)
		is Event.MetadataRemoved -> stringResource(R.string.log_message_metadata, this.metaValuesDisplay.name, this.metaValuesDisplay.version)
		is Event.MetadataSigned -> this.metaValuesExport.name + this.metaValuesExport.version
		is Event.NetworkSpecsAdded -> this.networkSpecsDisplay.network.specs.title
		is Event.NetworkSpecsRemoved -> this.networkSpecsDisplay.network.specs.title
		is Event.NetworkSpecsSigned -> this.networkSpecsExport.specsToSend.title
		is Event.NetworkVerifierSet -> {
			val verifier = when (val ver = this.networkVerifierDisplay.validCurrentVerifier) {
				is ValidCurrentVerifier.Custom -> {
					when (ver.v.v) {
						is VerifierValue.Standard -> stringResource(id = R.string.log_message_network_custom)
						null -> ""
					}
				}
				ValidCurrentVerifier.General -> {
					when (this.networkVerifierDisplay.generalVerifier.v) {
						is VerifierValue.Standard ->  stringResource(id = R.string.log_message_network_general)
						null -> ""
					}
				}
			}
			stringResource(R.string.log_message_network_verifier, verifier, this.networkVerifierDisplay.genesisHash.toUByteArray())
		}
		Event.ResetDangerRecord -> ""
		is Event.SecretWasExported -> this.identityHistory.seedName + this.identityHistory.path
		is Event.SeedCreated -> this.seedCreated
		is Event.SeedNameWasShown -> this.seedNameWasShown
		is Event.SeedRemoved -> this.seedName
		is Event.SystemEntry -> this.systemEntry
		is Event.TransactionSignError -> this.signDisplay.userComment
		is Event.TransactionSigned -> this.signDisplay.userComment
		is Event.TypesAdded -> ""
		is Event.TypesRemoved -> ""
		is Event.TypesSigned -> ""
		is Event.UserEntry -> this.userEntry
		is Event.Warning -> this.warning
		Event.WrongPassword -> stringResource(R.string.log_message_wrong_passowrd)
	}
}

fun Event.isDanger(): Boolean {
	return when (this) {
		Event.DeviceWasOnline -> true
		is Event.MessageSignError -> true
		Event.ResetDangerRecord -> true
		is Event.TransactionSignError -> true
		is Event.TypesRemoved -> true
		is Event.Warning -> true
		Event.WrongPassword -> true
		else -> false
	}
}
