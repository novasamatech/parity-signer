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
import io.parity.signer.uniffi.MNetworkDetails

@Composable
fun NetworkDetails(
	networkDetails: MNetworkDetails,
	button: (Action, String) -> Unit
) {
	Column {
		/* TODO: MNetworkDetails -> MDeriveKey
		NetworkCard(network = content)
		 */
		Row {
			Text("Network name:")
			Text(networkDetails.name)
		}
		Row {
			Text("base58 prefix:")
			Text(networkDetails.base58prefix.toString())
		}
		Row {
			Text("decimals:")
			Text(networkDetails.decimals.toString())
		}
		Row {
			Text("unit:")
			Text(networkDetails.unit)
		}
		Row {
			Text("genesis hash:")
			Text(networkDetails.genesisHash)
		}
		Row {
			Text("Verifier certificate:")
			when (networkDetails.currentVerifier.ttype) {
				"general" -> {
					Text("general")
				}
				"network" -> {
					Column {
						Text("custom")
						Text(
							networkDetails.currentVerifier.details.toString()
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
			items(networkDetails.meta.size) { index ->
				val metadataRecord = networkDetails.meta[index]
				Row(
					Modifier.clickable {
						button(
							Action.MANAGE_METADATA,
							metadataRecord.specsVersion
						)
					}
				) {
					MetadataCard(metadataRecord)
				}
			}
		}
	}
}
