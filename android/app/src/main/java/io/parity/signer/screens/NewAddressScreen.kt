package io.parity.signer.modals

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
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.*

@Composable
fun NewAddressScreen(signerDataModel: SignerDataModel, increment: Boolean) {
	var derivationPath = remember { mutableStateOf("") }
	var derivationState by remember { mutableStateOf(DerivationState()) }
	val seedName = signerDataModel.screenData.value?.optString("seed_name") ?: ""
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier.fillMaxSize()
	) {
		Text("Create new key")
		Text(lastError.value.toString())

		SingleTextInput(
			content = derivationPath,
			update = {
				derivationPath.value = it
				derivationState = signerDataModel.checkAsDerivation(it)
				signerDataModel.clearError()
			},
			capitalize = false,
			onDone = {
				focusManager.clearFocus()
				if (derivationState.isValid) {
					if (derivationState.hasPassword) {
						signerDataModel.pushButton(
							ButtonID.CheckPassword,
							details = derivationPath.value
						)
					} else {
						signerDataModel.addKey(
							path = derivationPath.value,
							seedName = seedName
						)
					}
				}
			},
			focusManager = focusManager,
			focusRequester = focusRequester
		)

		Row {
			BigButton(
				text = "Next",
				action = {
					if (derivationState.hasPassword) {
						signerDataModel.pushButton(
							ButtonID.CheckPassword,
							details = derivationPath.value
						)
					} else {
						signerDataModel.addKey(
							path = derivationPath.value,
							seedName = seedName
						)
					}
				},
				isDisabled = !derivationState.isValid
			)
		}
	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		derivationPath.value =
			signerDataModel.screenData.value?.optString("suggested_derivation")
				?: ""
		derivationState = signerDataModel.checkAsDerivation(derivationPath.value)
		onDispose { focusManager.clearFocus() }
	}
}
