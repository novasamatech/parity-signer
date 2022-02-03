package io.parity.signer.modals

import androidx.compose.foundation.layout.*
import androidx.compose.runtime.Composable
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.KeyCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.TransactionPreviewField
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.TransactionType
import io.parity.signer.models.getSeed
import io.parity.signer.models.parseTransaction

@Composable
fun TransactionPreview(
	button: (button: ButtonID, details: String, seedPhrase: String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val transaction =
		signerDataModel.screenData.value!!.getJSONObject("content")
			.parseTransaction()
	val action =
		TransactionType.valueOf(signerDataModel.screenData.value!!.getString("type"))

	Column {
		TransactionPreviewField(transaction = transaction)
		signerDataModel.screenData.value!!.getJSONObject("author_info").let {
			KeyCard(identity = it)
		}
		signerDataModel.screenData.value!!.getJSONObject("network_info").let {
			NetworkCard(network = it)
		}
		when (action) {
			TransactionType.sign -> {
				BigButton(
					text = "Unlock key and sign",
					action = {
						signerDataModel.authentication.authenticate(signerDataModel.activity) {
							val seedPhrase = signerDataModel.getSeed(
								signerDataModel.screenData.value?.optJSONObject("author_info")
									?.optString("seed") ?: ""
							)
							if (seedPhrase.isNotBlank()) {
								button(ButtonID.GoForward, "", seedPhrase)
							}
						}
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(ButtonID.GoBack, "", "")
					}
				)
			}
			TransactionType.done ->
				BigButton(
					text = "Done",
					action = {
						button(ButtonID.GoBack, "", "")
					}
				)
			TransactionType.stub -> {
				BigButton(
					text = "Approve",
					action = {
						button(ButtonID.GoForward, "", "")
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(ButtonID.GoBack, "", "")
					}
				)
			}
			TransactionType.read ->
				BigButton(
					text = "Back",
					action = {
						button(ButtonID.GoBack, "", "")
					}
				)
			TransactionType.import_derivations -> {
				BigButton(
					text = "Select seed",
					action = {
						button(ButtonID.GoForward, "", "")
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(ButtonID.GoBack, "", "")
					}
				)
			}
		}
	}
}

