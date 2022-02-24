package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import io.parity.signer.ButtonID
import io.parity.signer.components.*
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Text400

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
  val comment = remember{ mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		Modifier.verticalScroll(rememberScrollState())
	) {
		TransactionPreviewField(transaction = transaction)
		signerDataModel.screenData.value!!.optJSONObject("author_info")?.let {
			KeyCard(identity = it)
		}
		signerDataModel.screenData.value!!.optJSONObject("network_info")?.let {
			NetworkCard(network = it)
		}
		when (action) {
			TransactionType.sign -> {
				Text("LOG NOTE", style = MaterialTheme.typography.overline, color = MaterialTheme.colors.Text400)

				SingleTextInput(
					content = comment,
					update = {comment.value = it},
					onDone = { },
					focusManager = focusManager,
					focusRequester = focusRequester
				)

				Text("visible only on this device", style = MaterialTheme.typography.subtitle1, color = MaterialTheme.colors.Text400)

				BigButton(
					text = "Unlock key and sign",
					action = {
						signerDataModel.authentication.authenticate(signerDataModel.activity) {
							val seedPhrase = signerDataModel.getSeed(
								signerDataModel.screenData.value?.optJSONObject("author_info")
									?.optString("seed") ?: ""
							)
							if (seedPhrase.isNotBlank()) {
								button(ButtonID.GoForward, comment.value.encode64(), seedPhrase)
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

