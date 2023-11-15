package io.parity.signer.screens.settings.logs.logdetails

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.outlined.Delete
import androidx.compose.runtime.Composable
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.abbreviateString
import io.parity.signer.domain.encodeHex
import io.parity.signer.screens.scan.transaction.components.TransactionPreviewField
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
	when (val eventVal = event.event) {
		is Event.DatabaseInitiated -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Smartphone,
				line1 = timestamp,
				line2 = "Database initiated",
				line3 = ""
			)
		}
		is Event.DeviceWasOnline -> {
			HistoryCardTemplateOld(
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
			HistoryCardTemplateOld(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "General verifier set",
				line3 = hex.getOrElse(0) { "" }
					.abbreviateString(BASE58_STYLE_ABBREVIATE) + hex.getOrElse(1) { "" }
			)
		}
		is Event.HistoryCleared -> {
			HistoryCardTemplateOld(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = ""
			)
		}
		is Event.IdentitiesWiped -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "All keys were wiped",
				line3 = ""
			)
		}
		is Event.IdentityAdded -> {
			eventVal.identityHistory.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Pattern,
					line1 = timestamp,
					line2 = "Key created",
					line3 = it.seedName + it.path
				)
			}
		}
		is Event.IdentityRemoved -> {
			eventVal.identityHistory.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = "Key removed",
					line3 = it.seedName + it.path
				)
			}
		}
		is Event.SecretWasExported -> {
			eventVal.identityHistory.let {
				HistoryCardTemplateOld(
					image = Icons.Default.WbSunny,
					line1 = timestamp,
					line2 = "Secret was exported",
					line3 = it.seedName + it.path
				)
			}
		}
		is Event.MessageSignError -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Message signing error!",
				line3 = "message:" + eventVal.signMessageDisplay.message + " user comment: " + eventVal.signMessageDisplay.userComment,
				danger = true
			)
		}
		is Event.MessageSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = "Generated signature for message",
				line3 = "message:" + eventVal.signMessageDisplay.message + " user comment: " + eventVal.signMessageDisplay.userComment
			)
		}
		is Event.MetadataAdded -> {
			eventVal.metaValuesDisplay.let {
				HistoryCardTemplateOld(
					image = Icons.Default.QrCodeScanner,
					line1 = timestamp,
					line2 = "Metadata added",
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.MetadataRemoved -> {
			eventVal.metaValuesDisplay.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Delete,
					line1 = timestamp,
					line2 = "Metadata removed",
					line3 = it.name + " version " + it.version
				)
			}
		}
		is Event.NetworkSpecsAdded -> {
			HistoryCardTemplateOld(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "Network added",
				line3 = eventVal.networkSpecsDisplay.network.specs.title,
			)
		}
		is Event.NetworkSpecsRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Network removed",
				line3 = eventVal.networkSpecsDisplay.network.specs.title
			)
		}
		is Event.NetworkVerifierSet -> {
			var line3 =
				when (val ver = eventVal.networkVerifierDisplay.validCurrentVerifier) {
					is ValidCurrentVerifier.Custom -> {
						when (val v = ver.v.v) {
							is VerifierValue.Standard -> v.m.getOrElse(0) { "" } + " with encryption " + v.m.getOrElse(1) {""}
							null -> ""
						}
					}
					ValidCurrentVerifier.General -> {
						when (val v = eventVal.networkVerifierDisplay.generalVerifier.v) {
							is VerifierValue.Standard -> "general"
							null -> ""
						}
					}
				}

			line3 += " for network with genesis hash " + eventVal.networkVerifierDisplay.genesisHash.toUByteArray()
				.toByteArray().encodeHex()
			HistoryCardTemplateOld(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "Network verifier set",
				line3 = line3
			)
		}
		is Event.ResetDangerRecord -> {
			HistoryCardTemplateOld(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = "",
				danger = true
			)
		}
		is Event.SeedCreated -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = "Seed created",
				line3 = eventVal.seedCreated
			)
		}
		is Event.SeedRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Outlined.Delete,
				line1 = timestamp,
				line2 = "Seed removed",
				line3 = eventVal.seedName
			)
		}
		is Event.SeedNameWasShown -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Seed was shown",
				line3 = eventVal.seedNameWasShown
			)
		}
		is Event.NetworkSpecsSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Network specs signed",
				line3 = eventVal.networkSpecsExport.specsToSend.title
			)
		}
		is Event.MetadataSigned -> {
			eventVal.metaValuesExport.let {
				HistoryCardTemplateOld(
					image = Icons.Default.Verified,
					line1 = timestamp,
					line2 = "Meta signed",
					line3 = it.name + it.version
				)
			}
		}
		is Event.TypesSigned -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Types signed",
				line3 = ""
			)
		}
		is Event.SystemEntry -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "System entry",
				line3 = eventVal.systemEntry
			)
		}
		is Event.TransactionSignError -> {
			HistoryCardTemplateOld(
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
					signedBy?.address?.identicon?.let { identicon ->
						IdentIconImage(identicon = identicon)
					}
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
			HistoryCardTemplateOld(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "New types info loaded",
				line3 = ""
			)
		}
		is Event.TypesRemoved -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Remove,
				line1 = timestamp,
				line2 = "Types info removed",
				line3 = "",
				danger = true
			)
		}
		is Event.UserEntry -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Note,
				line1 = timestamp,
				line2 = "User entry",
				line3 = eventVal.userEntry
			)
		}
		is Event.Warning -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Warning!",
				line3 = eventVal.warning,
				danger = true
			)
		}
		is Event.WrongPassword -> {
			HistoryCardTemplateOld(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Wrong password entered",
				line3 = "operation declined",
				danger = true
			)
		}
	}
}
