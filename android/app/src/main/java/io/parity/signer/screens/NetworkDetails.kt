package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.MetadataCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONObject

@Composable
fun NetworkDetails(
	screenData: JSONObject,
	button: (ButtonID, String) -> Unit
) {
	Column {
		NetworkCard(network = screenData)
		Row {
			Text("Network name:")
			Text(screenData.optString("name"))
		}
		Row {
			Text("base58 prefix:")
			Text(screenData.optString("base58prefix"))
		}
		Row {
			Text("decimals:")
			Text(screenData.optString("decimals"))
		}
		Row {
			Text("unit:")
			Text(screenData.optString("unit"))
		}
		Row {
			Text("genesis hash:")
			Text(screenData.optString("genesis_hash"))
		}
		Row {
			Text("Verifier certificate:")
			when (screenData.optJSONObject("current_verifier")?.optString("type")
				?: "") {
				"general" -> {
					Text("general")
				}
				"network" -> {
					Column {
						Text("custom")
						Text(
							screenData.optJSONObject("current_verifier")?.optString("details")
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
			items(screenData.getJSONArray("meta").length()) { index ->
				val meta = screenData.getJSONArray("meta").getJSONObject(index)
				Row(
					Modifier.clickable {
						button(
							ButtonID.ManageMetadata,
							meta.optString("spec_version")
						)
					}
				) {
					MetadataCard(meta = meta)
				}
			}
		}
	}
}
