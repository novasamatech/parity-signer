package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import io.parity.signer.models.abbreviateString
import io.parity.signer.uniffi.Event
import io.parity.signer.uniffi.MEventMaybeDecoded
import io.parity.signer.uniffi.VerifierValue

/**
 * Detailed history event description representation selector
 */
@Composable
fun HistoryCardExtended(
	event: MEventMaybeDecoded,
	timestamp: String
) {
	val decodedTransaction = event.decoded
	val signedBy = event.signedBy
	val verifierDetails = event.verifierDetails
	when (val eventVal = event.event) {
		is Event.DatabaseInitiated -> {
			HistoryCardTemplate(
				image = Icons.Default.Smartphone,
				line1 = timestamp,
				line2 = "Database initiated",
				line3 = ""
			)
		}
		is Event.DeviceWasOnline -> {
			HistoryCardTemplate(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = "Device was connected to network",
				line3 = "",
				danger = true
			)
		}
		is Event.GeneralVerifierSet -> {
			val hex = eventVal.verifier.v.let {
				when (it) {
					is VerifierValue.Standard -> {
						it.m
					}
					else -> listOf()
				}
			}
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "General verifier set",
				line3 = hex.getOrElse(0) { "" }
					.abbreviateString(8) + hex.getOrElse(1) { "" }
			)
		}
		is Event.HistoryCleared -> {
			HistoryCardTemplate(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = ""
			)
		}
		is Event.IdentitiesWiped -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "All keys were wiped",
				line3 = ""
			)
		}
		is Event.IdentityAdded -> {
			eventVal.identityHistory.let {
				HistoryCardTemplate(
					image = Icons.Default.Pattern,
					line1 = timestamp,
					line2 = "Key created",
					line3 = it.seedName + it.path
				)
			}
		}
		is Event.IdentityRemoved -> {
			eventVal.identityHistory.let {
				HistoryCardTemplate(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = "Key removed",
					line3 = it.seedName + it.path
				)
			}
		}
		is Event.MessageSignError -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Message signing error!",
				line3 = "message:" + eventVal.signMessageDisplay.message + " user comment: " + eventVal.signMessageDisplay.userComment,
				danger = true
			)
		}
		is Event.MessageSigned -> {
			HistoryCardTemplate(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = "Generated signature for message",
				line3 = "message:" + eventVal.signMessageDisplay.message + " user comment: " + eventVal.signMessageDisplay.userComment
			)
		}
		is Event.MetadataAdded -> {
			eventVal.metaValuesDisplay.let {
				HistoryCardTemplate(
					image = Icons.Default.QrCodeScanner,
					line1 = timestamp,
					line2 = "Metadata added",
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.MetadataRemoved -> {
			eventVal.metaValuesDisplay.let {
				HistoryCardTemplate(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = "Metadata removed",
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.NetworkSpecsAdded -> {
			HistoryCardTemplate(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "Network added",
				line3 = eventVal.networkSpecsDisplay.specs.title,
			)
		}
		is Event.NetworkSpecsRemoved -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Network removed",
				line3 = eventVal.networkSpecsDisplay.specs.title
			)
		}
		is Event.NetworkVerifierSet -> {
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "Network verifier set",
				line3 = eventVal.networkVerifierDisplay.genesisHash.toString()
			)
		}
		is Event.ResetDangerRecord -> {
			HistoryCardTemplate(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = "",
				danger = true
			)
		}
		is Event.SeedCreated -> {
			HistoryCardTemplate(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = "Seed created",
				line3 = eventVal.seedCreated
			)
		}
		is Event.SeedNameWasShown -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Seed was shown",
				line3 = eventVal.seedNameWasShown
			)
		}
		is Event.NetworkSpecsSigned -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Network specs signed",
				line3 = eventVal.networkSpecsExport.specsToSend.title
			)
		}
		is Event.MetadataSigned -> {
			eventVal.metaValuesExport.let {
				HistoryCardTemplate(
					image = Icons.Default.Verified,
					line1 = timestamp,
					line2 = "Meta signed",
					line3 = it.name + it.version
				)
			}
		}
		is Event.TypesSigned -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Types signed",
				line3 = ""
			)
		}
		is Event.SystemEntry -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "System entry",
				line3 = eventVal.systemEntry
			)
		}
		is Event.TransactionSignError -> {
			HistoryCardTemplate(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = "Signing failure",
				line3 = eventVal.signDisplay.userComment,
				danger = true
			)
		}
		is Event.TransactionSigned -> {
			Column {
				Text("Transaction signed")

				if (decodedTransaction != null) {
					TransactionPreviewField(
						cardSet = decodedTransaction
					)
				}
				Text("Signed by:")
				Row {
					Identicon(
						identicon = signedBy?.identicon ?: listOf()
					)
					Column {
						Text(verifierDetails?.publicKey ?: "")
						Text(
							verifierDetails?.encryption ?: ""
						)
					}
				}
				Text("In network")
				Text(eventVal.signDisplay.networkName)
				Text("Comment:")
				Text(
					eventVal.signDisplay.userComment
				)
			}
		}
		is Event.TypesAdded -> {
			HistoryCardTemplate(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "New types info loaded",
				line3 = ""
			)
		}
		is Event.TypesRemoved -> {
			HistoryCardTemplate(
				image = Icons.Default.Remove,
				line1 = timestamp,
				line2 = "Types info removed",
				line3 = "",
				danger = true
			)
		}
		is Event.UserEntry -> {
			HistoryCardTemplate(
				image = Icons.Default.Note,
				line1 = timestamp,
				line2 = "User entry",
				line3 = eventVal.userEntry
			)
		}
		is Event.Warning -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Warning!",
				line3 = eventVal.warning,
				danger = true
			)
		}
		is Event.WrongPassword -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Wrong password entered",
				line3 = "operation declined",
				danger = true
			)
		}
	}
}
