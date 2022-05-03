package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.decode64
import io.parity.signer.uniffi.*

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
	val event = event.event
	when (event) {
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
			val hex = event.verifier.v.let {
				when (it) {
					is VerifierValue.Standard -> {
						it.m
					}
					else -> ""
				}
			}
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "General verifier set",
				line3 = hex.abbreviateString(8)
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
			event.identityHistory.let {
				HistoryCardTemplate(
					image = Icons.Default.Pattern,
					line1 = timestamp,
					line2 = "Key created",
					line3 = it.seedName.decode64() + it.path
				)
			}
		}
		is Event.IdentityRemoved -> {
			event.identityHistory.let {
				HistoryCardTemplate(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = "Key removed",
					line3 = it.seedName.decode64() + it.path
				)
			}
		}
		is Event.MessageSignError -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Message signing error!",
				line3 = "", // TODO: Error,
				danger = true
			)
		}
		is Event.MessageSigned -> {
			HistoryCardTemplate(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = "Generated signature for message",
				line3 = event.signMessageDisplay.userComment.decode64()
			)
		}
		is Event.MetadataAdded -> {
			event.metaValuesDisplay.let {
				HistoryCardTemplate(
					image = Icons.Default.QrCodeScanner,
					line1 = timestamp,
					line2 = "Metadata added",
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.MetadataRemoved -> {
			event.metaValuesDisplay.let {
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
				line3 = event.networkSpecsDisplay.specs.title,
			)
		}
		is Event.NetworkSpecsRemoved -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Network removed",
				line3 = event.networkSpecsDisplay.specs.title
			)
		}
		is Event.NetworkVerifierSet -> {
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "Network verifier set",
				line3 = event.networkVerifierDisplay.genesisHash.toString()
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
				line3 = event.seedCreated.decode64()
			)
		}
		is Event.SeedNameWasShown -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Seed was shown",
				line3 = event.seedNameWasShown.decode64()
			)
		}
		is Event.NetworkSpecsSigned -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Network specs signed",
				line3 = event.networkSpecsExport.specsToSend.title
			)
		}
		is Event.MetadataSigned -> {
			event.metaValuesExport.let {
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
				line3 = event.systemEntry
			)
		}
		is Event.TransactionSignError -> {
			HistoryCardTemplate(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = "Signing failure",
				line3 = event.signDisplay.userComment.decode64(),
				danger = true
			)
		}
		is Event.TransactionSigned -> {
			Column {
				Text("Transaction signed")

				if (decodedTransaction != null) {
					TransactionPreviewField(
						cardSet = decodedTransaction,
						authorInfo = signedBy
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
				Text(event.signDisplay.networkName)
				Text("Comment:")
				Text(
					event.signDisplay.userComment.decode64()
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
				line3 = event.userEntry
			)
		}
		is Event.Warning -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Warning!",
				line3 = event.warning,
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
