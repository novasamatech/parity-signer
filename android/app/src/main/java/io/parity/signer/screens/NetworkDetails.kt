package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.MetadataCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONObject
import io.parity.signer.uniffi.Action

@Composable
fun NetworkDetails(signerDataModel: SignerDataModel) {
	val content = signerDataModel.screenData.value ?: JSONObject()

	Column {
		NetworkCard(network = content)
		Row {
			Text("Network name:")
			Text(content.optString("name"))
		}
		Row {
			Text("base58 prefix:")
			Text(content.optString("base58prefix"))
		}
		Row {
			Text("decimals:")
			Text(content.optString("decimals"))
		}
		Row {
			Text("unit:")
			Text(content.optString("unit"))
		}
		Row {
			Text("genesis hash:")
			Text(content.optString("genesis_hash"))
		}
		Row {
			Text("Verifier certificate:")
			when (content.optJSONObject("current_verifier")?.optString("type") ?: "") {
				"general" -> {
					Text("general")
				}
				"network" -> {
					Column {
						Text("custom")
						Text(
							content.optJSONObject("current_verifier")?.optString("details")
								?: ""
						)
					}
				}
				"none" -> {
					Text("none")
				}
				else -> {
					Text("unknown!")
				}
			}
		}
		Text("Metadata available:")
		LazyColumn {
			items(content.getJSONArray("meta").length()) { index ->
				val meta = content.getJSONArray("meta").getJSONObject(index)
				Row(
					Modifier.clickable {
						signerDataModel.pushButton(
							Action.MANAGE_METADATA,
							details = meta.optString("spec_version")
						)
					}
				) {
					MetadataCard(meta = meta)
				}
			}
		}
	}
}
