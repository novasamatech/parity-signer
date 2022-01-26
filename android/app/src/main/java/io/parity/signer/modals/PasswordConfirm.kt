package io.parity.signer.modals

import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.models.SignerDataModel

@Composable
fun PasswordConfirm(signerDataModel: SignerDataModel) {
	var password by remember { mutableStateOf("") }
	var passwordRepeat by remember { mutableStateOf("") }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current

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
}
