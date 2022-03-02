package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Text600

@Composable
fun NewAddressScreen(signerDataModel: SignerDataModel, increment: Boolean) {
	val derivationPath = remember { mutableStateOf("") }
	var derivationState by remember { mutableStateOf(DerivationState()) }
	val seedName = signerDataModel.screenData.value?.optString("seed_name") ?: ""
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Top,
		modifier = Modifier
			.padding(20.dp)
			.fillMaxSize()
	) {
		Row {
			HeadingOverline("Create new key")
			Spacer(Modifier.weight(1f))
		}
		SingleTextInput(
			content = derivationPath,
			update = {
				derivationPath.value = it
				derivationState = signerDataModel.checkAsDerivation(it)
				signerDataModel.clearError()
			},
			prefix = {
				Text(
					seedName,
					style = MaterialTheme.typography.body2,
					color = MaterialTheme.colors.Text600
				)
			},
			isCrypto = true,
			isCryptoColor = true,
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
		Spacer(Modifier.height(20.dp))
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
