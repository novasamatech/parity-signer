package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.*
import io.parity.signer.models.encodeHex
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MNetworkDetails

@Composable
fun NetworkDetails(
	networkDetails: MNetworkDetails,
	button: (Action, String) -> Unit
) {
	Column {
		NetworkCard(
			network = NetworkCardModel(
				networkTitle = networkDetails.title,
				networkLogo = networkDetails.logo
			)
		)

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
			Text(
				networkDetails.genesisHash.toUByteArray()
					.toByteArray().encodeHex()
			)
		}
		Row {
			Text("Verifier certificate:")
			when (networkDetails.currentVerifier.ttype) {
				"general" -> {
					Text("general")
				}
				"custom" -> {
					Row {
						IdentIcon(identicon = networkDetails.currentVerifier.details.identicon.toImageContent())
						Column {
							Text("custom")
							Text(
								networkDetails.currentVerifier.details.publicKey
							)
							Text("encryption: " + networkDetails.currentVerifier.details.encryption)
						}
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
