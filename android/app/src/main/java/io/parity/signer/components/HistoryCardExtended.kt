package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.parseTransaction
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun HistoryCardExtended(card: JSONObject) {
	val timestamp = ""
	val payload = card.optJSONObject("payload")
	when (card.getString("event")) {

		"database_initiated" -> {
			HistoryCardTemplate(
				image = Icons.Default.Smartphone,
				line1 = timestamp,
				line2 = "Database initiated",
				line3 = ""
			)
		}
		"device_online" -> {
			HistoryCardTemplate(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = "Device was connected to network",
				line3 = "",
				danger = true
			)
		}
		"general_verifier_added" -> {
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "General verifier set",
				line3 = payload?.optString("hex")
					?.abbreviateString(8) + payload
					?.optString("encryption")
			)
		}
		"history_cleared" -> {
			HistoryCardTemplate(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = ""
			)
		}
		"identities_wiped" -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "All keys were wiped",
				line3 = ""
			)
		}
		"identity_added" -> {
			HistoryCardTemplate(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = "Key created",
				line3 = payload
					?.optString("seed_name") + payload
					?.optString("path")
			)
		}
		"identity_removed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Key removed",
				line3 = payload
					?.optString("seed_name") + payload
					?.optString("path")
			)
		}
		"message_sign_error" -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Message signing error!",
				line3 = payload?.optString("Error") ?: "",
				danger = true
			)
		}
		"message_signed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Done,
				line1 = timestamp,
				line2 = "Generated signature for message",
				line3 = payload?.optString("user_comment") ?: ""
			)
		}
		"metadata_added" -> {
			HistoryCardTemplate(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "Metadata added",
				line3 = payload?.optString("specname") + " version " + payload?.optString(
					"spec_version"
				)
			)
		}
		"metadata_removed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Metadata removed",
				line3 = payload?.optString("specname") + " version " + payload?.optString(
					"spec_version"
				)
			)
		}
		"network_specs_added" -> {
			HistoryCardTemplate(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "Network added",
				line3 = payload?.optString("title") ?: ""
			)
		}
		"network_removed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Delete,
				line1 = timestamp,
				line2 = "Network removed",
				line3 = payload?.optString("title") ?: ""
			)
		}
		"network_verifier_set" -> {
			HistoryCardTemplate(
				image = Icons.Default.Shield,
				line1 = timestamp,
				line2 = "Network verifier set",
				line3 = payload?.optString("genesis_hash") ?: ""
			)
		}
		"reset_danger_record" -> {
			HistoryCardTemplate(
				image = Icons.Default.DeleteForever,
				line1 = timestamp,
				line2 = "History cleared",
				line3 = "",
				danger = true
			)
		}
		"seed_created" -> {
			HistoryCardTemplate(
				image = Icons.Default.Pattern,
				line1 = timestamp,
				line2 = "Seed created",
				line3 = card.optString("payload")
			)
		}
		"seed_name_shown" -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Seed was shown",
				line3 = card.optString("payload")
			)
		}
		"add_specs_message_signed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Network specs signed",
				line3 = payload?.optString("title") ?: ""
			)
		}
		"load_metadata_message_signed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Meta signed",
				line3 = payload?.optString("specname") + payload?.optString("spec_version")
			)
		}
		"load_types_message_signed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Verified,
				line1 = timestamp,
				line2 = "Types signed",
				line3 = ""
			)
		}
		"system_entered_event" -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "System entry",
				line3 = card.optString("payload")
			)
		}
		"transaction_sign_error" -> {
			HistoryCardTemplate(
				image = Icons.Default.Dangerous,
				line1 = timestamp,
				line2 = "Signing failure",
				line3 = card.getJSONObject("payload").getString("user_comment"),
				danger = true
			)
		}
		"transaction_signed" -> {
			val content = card.optJSONObject("payload")?: JSONObject()
			Column {
				Text("Transaction signed")
				TransactionPreviewField(
					transaction = content.optJSONObject("transaction")?.parseTransaction() ?: JSONArray()
				)
				Text("Signed by:")
				Row {
					Identicon(identicon = content.optJSONObject("signed_by")?.optString("identicon") ?: "")
					Column {
						Text(content.optJSONObject("signed_by")?.optString("hex") ?: "")
						Text(content.optJSONObject("signed_by")?.optString("encryption") ?: "")
					}
				}
				Text("In network")
				Text(content.optString("network_name"))
				Text("Comment:")
				Text("placeholder")
			}
		}
		"types_info_updated" -> {
			HistoryCardTemplate(
				image = Icons.Default.QrCodeScanner,
				line1 = timestamp,
				line2 = "New types info loaded",
				line3 = ""
			)
		}
		"types_removed" -> {
			HistoryCardTemplate(
				image = Icons.Default.Remove,
				line1 = timestamp,
				line2 = "Types info removed",
				line3 = "",
				danger = true
			)
		}
		"user_entered_event" -> {
			HistoryCardTemplate(
				image = Icons.Default.Note,
				line1 = timestamp,
				line2 = "User entry",
				line3 = card.optString("payload")
			)
		}
		"warning" -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Warning!",
				line3 = card.optString("payload"),
				danger = true
			)
		}
		"wrong_password_entered" -> {
			HistoryCardTemplate(
				image = Icons.Default.Warning,
				line1 = timestamp,
				line2 = "Wrong password entered",
				line3 = "operation declined",
				danger = true
			)
		}
		else -> {
			HistoryCardTemplate(
				image = Icons.Default.Error,
				line1 = timestamp,
				line2 = "Record corrupted",
				line3 = card.getString("event"),
				danger = true
			)
		}
	}
}
