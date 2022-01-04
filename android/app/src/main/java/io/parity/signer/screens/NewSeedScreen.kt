package io.parity.signer.modals

import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.ImeOptions
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addSeed
import io.parity.signer.models.pushButton

@Composable
fun NewSeedScreen(button: (button: ButtonID, details: String) -> Unit, signerDataModel: SignerDataModel) {
	var seedName by remember { mutableStateOf("") }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier.fillMaxSize()
	) {
			Text("DISPLAY NAME", style = MaterialTheme.typography.overline)
			Text(lastError.value.toString())
			TextField(
				value = seedName,
				onValueChange = {
					seedName = it
					signerDataModel.clearError()
				},
				label = { Text("Seed name") },
				singleLine = true,
				keyboardOptions = KeyboardOptions(
					autoCorrect = false,
					capitalization = KeyboardCapitalization.Words,
					keyboardType = KeyboardType.Text,
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(
					onDone = {
						focusManager.clearFocus()
						button(ButtonID.GoForward, seedName)
					}
				),
				modifier = Modifier.focusRequester(focusRequester = focusRequester)
			)
		Text("Display name visible only to you")
			BigButton(
				text = "Generate seed phrase",
				action = {
					focusManager.clearFocus()
					signerDataModel.pushButton(ButtonID.GoForward, details = seedName)}
			)
	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		onDispose { focusManager.clearFocus() }
	}
}
