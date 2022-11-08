package io.parity.signer.screens.logs.items

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.outlined.Delete
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.encodeHex
import io.parity.signer.uniffi.Event
import io.parity.signer.uniffi.ValidCurrentVerifier
import io.parity.signer.uniffi.VerifierValue

/**
 * Selector for rendering history cards in general list;
 */
@Composable
fun HistoryCardSelectorOld(card: Event, timestamp: String) {
	when (card) {
		is Event.DatabaseInitiated -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Smartphone,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_database_initiated),
				line3 = ""
			)
		}
		is Event.DeviceWasOnline -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_device_was_online),
				line3 = "",
				danger = true
			)
		}
		is Event.GeneralVerifierSet -> {
			card.verifier.v.let {
				if (it is VerifierValue.Standard) {
					HistoryCardTemplateOld(
						image = Icons.Default.Shield,
						line1 = timestamp,
						line2 = stringResource(R.string.log_title_general_virifier_set),
						line3 = it.m.getOrElse(0) { "" }
							.abbreviateString(8) + it.m.getOrElse(1) { "" }
					)
				}
			}
		}
		is Event.HistoryCleared -> {
			HistoryCardTemplateOld(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_history_cleared),
				line3 = ""
			)
		}
		is Event.IdentitiesWiped -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_identities_wiped),
				line3 = ""
			)
		}
		is Event.IdentityAdded -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_identity_added),
				line3 = card.identityHistory.seedName + card.identityHistory.path
			)
		}
		is Event.IdentityRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_identity_removed),
				line3 = card.identityHistory.seedName + card.identityHistory.path
			)
		}
		is Event.SecretWasExported -> {
			HistoryCardTemplateOld(
				image = Icons.Default.WbSunny,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_secret_was_exported),
				line3 = card.identityHistory.seedName + card.identityHistory.path
			)
		}
		is Event.MessageSignError -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_messages_error),
				line3 = card.signMessageDisplay.userComment,
				danger = true
			)
		}
		is Event.MessageSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_message_signed),
				line3 = card.signMessageDisplay.userComment
			)
		}
		is Event.MetadataAdded -> {
			card.metaValuesDisplay.let {
				HistoryCardTemplateOld(
					image = Icons.Default.QrCodeScanner,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_metadata_added),
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.MetadataRemoved -> {
			card.metaValuesDisplay.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_metadata_removed),
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.NetworkSpecsAdded -> {
			card.networkSpecsDisplay.network.let {
				HistoryCardTemplateOld(
					image = Icons.Default.QrCodeScanner,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_network_added),
					line3 = it.specs.title
				)
			}
		}
		is Event.NetworkSpecsRemoved -> {
			card.networkSpecsDisplay.network.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_network_removed),
					line3 = it.specs.title
				)
			}
		}
		is Event.NetworkVerifierSet -> {
			var line3 =
				when (val ver = card.networkVerifierDisplay.validCurrentVerifier) {
					is ValidCurrentVerifier.Custom -> {
						when (val v = ver.v.v) {
							is VerifierValue.Standard -> "custom"
							null -> ""
						}
					}
					ValidCurrentVerifier.General -> {
						when (val v = card.networkVerifierDisplay.generalVerifier.v) {
							is VerifierValue.Standard -> "general"
							null -> ""
						}
					}
				}

			line3 += " for network with genesis hash " + card.networkVerifierDisplay.genesisHash.toUByteArray()
				.toByteArray().encodeHex()
			card.networkVerifierDisplay.genesisHash.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Shield,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_network_verifier_set),
					line3 = line3
				)
			}
		}
		is Event.ResetDangerRecord -> {
			HistoryCardTemplateOld(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_reset_danger_record),
				line3 = "",
				danger = true
			)
		}
		is Event.SeedCreated -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_seed_created),
				line3 = card.seedCreated
			)
		}
		is Event.SeedRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Outlined.Delete,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_seed_removed),
				line3 = card.seedName
			)
		}
		is Event.SeedNameWasShown -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_seed_name_was_shown),
				line3 = card.seedNameWasShown
			)
		}
		is Event.NetworkSpecsSigned -> {
			card.networkSpecsExport.specsToSend.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Verified,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_network_specs_signed),
					line3 = it.title
				)
			}
		}
		is Event.MetadataSigned -> {
			card.metaValuesExport.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Verified,
					line1 = timestamp,
					line2 = stringResource(R.string.log_title_metadata_signed),
					line3 = it.name + it.version
				)
			}
		}
		is Event.TypesSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_types_signed),
				line3 = ""
			)
		}
		is Event.SystemEntry -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_system_entry),
				line3 = card.systemEntry
			)
		}
		is Event.TransactionSignError -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_transaction_sign_error),
				line3 = card.signDisplay.userComment,
				danger = true
			)
		}
		is Event.TransactionSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_transaction_signed),
				line3 = card.signDisplay.userComment
			)
		}
		is Event.TypesAdded -> {
			HistoryCardTemplateOld(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_types_added),
				line3 = ""
			)
		}
		is Event.TypesRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Remove,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_types_removed),
				line3 = "",
				danger = true
			)
		}
		is Event.UserEntry -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Note,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_user_entry),
				line3 = card.userEntry
			)
		}
		is Event.Warning -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_warning),
				line3 = card.warning,
				danger = true
			)
		}
		is Event.WrongPassword -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = stringResource(R.string.log_title_wrong_password),
				line3 = stringResource(R.string.log_message_wrong_passowrd),
				danger = true
			)
		}
	}
}
