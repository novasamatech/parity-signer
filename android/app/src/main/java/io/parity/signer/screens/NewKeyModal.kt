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
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addKey
import io.parity.signer.models.proposeDerivePath
import io.parity.signer.models.proposeIncrement

@Composable
fun NewKeyModal(signerDataModel: SignerDataModel, increment: Boolean) {
	var derivationPath by remember {
		mutableStateOf(
			if (increment)
				signerDataModel.proposeIncrement()
			else
				signerDataModel.proposeDerivePath()
		)
	}

	var password by remember { mutableStateOf("") }
	var passwordRepeat by remember { mutableStateOf("") }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier.fillMaxSize()
	) {
		Text("Create new key")
		Text(lastError.value.toString())
		TextField(
			value = derivationPath,
			onValueChange = {
				derivationPath = it
				signerDataModel.clearError()
			},
			label = { Text("Derivation path") },
			singleLine = true,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = KeyboardCapitalization.None,
				keyboardType = KeyboardType.Text,
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = { focusManager.clearFocus() }
			)
		)
		TextField(
			value = password,
			onValueChange = {
				password = it
				signerDataModel.clearError()
			},
			label = { Text("Password (optional)") },
			singleLine = true,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = KeyboardCapitalization.None,
				keyboardType = KeyboardType.Password,
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = { focusManager.clearFocus() }
			)
		)
		if (!password.isEmpty()) {
			TextField(
				value = passwordRepeat,
				onValueChange = {
					passwordRepeat = it
					signerDataModel.clearError()
				},
				label = { Text("Repeat password") },
				singleLine = true,
				keyboardOptions = KeyboardOptions(
					autoCorrect = false,
					capitalization = KeyboardCapitalization.None,
					keyboardType = KeyboardType.Password,
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(
					onDone = { focusManager.clearFocus() }
				)
			)
		}
		Row {
			TextButton(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = {
					password = ""
					passwordRepeat = ""
					derivationPath = signerDataModel.proposeIncrement()
				},
				enabled = true
			) {
				Text("Suggest N+1")
			}
			TextButton(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = {
					signerDataModel.addKey(derivationPath, password)
				},
				enabled = (password == passwordRepeat || password.isEmpty()) && !derivationPath.isEmpty() && lastError.value?.isEmpty() as Boolean
			) {
				Text("Create")
			}
		}
	}
}
