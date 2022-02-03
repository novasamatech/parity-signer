package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addKey
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject

@Composable
fun EnterPassword(signerDataModel: SignerDataModel) {
	val password by remember {
		mutableStateOf("")
	}
	val content = signerDataModel.screenData.value ?: JSONObject()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.large,
		modifier = Modifier.fillMaxSize(1f)
	) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			HeaderBar(line1 = "SECRET PATH", line2 = "///password")
			KeyCard(
				identity = content.optJSONObject("author_info") ?: JSONObject(),
				signerDataModel = signerDataModel
			)
			if (content.optInt("counter") > 0) {
				Text("Attempt " + content.optInt("counter").toString() + " of 3")
			}
			Row {
				Text("///")
				TextField(
					value = password,
					onValueChange = { },
					label = { Text("SECRET PATH") },
					singleLine = true,
					keyboardOptions = KeyboardOptions(
						autoCorrect = false,
						capitalization = KeyboardCapitalization.None,
						keyboardType = KeyboardType.Password,
						imeAction = ImeAction.Done
					),
					keyboardActions = KeyboardActions(
						onDone = {
							focusManager.clearFocus()
							if (password.isNotBlank()) {
								signerDataModel.pushButton(
									ButtonID.GoForward,
									details = password
								)
							}
						}
					),
					modifier = Modifier.focusRequester(focusRequester = focusRequester)
				)
			}
			BigButton(
				text = "Next",
				isCrypto = true,
				action = {
					signerDataModel.pushButton(ButtonID.GoForward, details = password)
				},
				isDisabled = password.isBlank()
			)
		}
	}
}
