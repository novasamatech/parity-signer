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
import org.json.JSONObject

@Composable
fun TransactionPreview(
	screenData: JSONObject,
	button: (button: ButtonID, details: String, seedPhrase: String) -> Unit,
  sign: (String) -> Unit
) {
	val transaction =
		screenData.getJSONObject("content")
			.parseTransaction()
	val action =
		TransactionType.valueOf(screenData.getString("type"))
  val comment = remember{ mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		Modifier.verticalScroll(rememberScrollState())
	) {
		TransactionPreviewField(transaction = transaction)
		screenData.optJSONObject("author_info")?.let {
			KeyCard(identity = it)
		}
		screenData.optJSONObject("network_info")?.let {
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
						sign(comment.value)
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

