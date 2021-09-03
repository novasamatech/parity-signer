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
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.ImeOptions
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.models.SignerDataModel

@Composable
fun NewSeedModal(signerDataModel: SignerDataModel) {
	var seedName by remember { mutableStateOf("") }
	var seedPhrase by remember { mutableStateOf("") }
	var recover by remember { mutableStateOf(false) }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier.fillMaxSize()
	) {
			Text("Create new seed")
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
					keyboardType = KeyboardType.Ascii,
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(
					onDone = { focusManager.clearFocus() }
				)
			)
			Row {
				Text("Custom seed")
				Switch(
					checked = recover,
					onCheckedChange = {
						recover = it
						signerDataModel.clearError()
					}
				)
			}
			if (recover) {
				TextField(
					value = seedPhrase,
					onValueChange = {
						seedPhrase = it
						signerDataModel.clearError()
					},
					label = { Text("Seed phrase") },
					singleLine = true,
					keyboardOptions = KeyboardOptions(
						autoCorrect = false,
						keyboardType = KeyboardType.Ascii,
						capitalization = KeyboardCapitalization.None,
						imeAction = ImeAction.Done
					),
					keyboardActions = KeyboardActions(
						onDone = { focusManager.clearFocus() }
					)
				)
			}
			TextButton(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = {
					signerDataModel.addSeed(seedName, seedPhrase)
				},
				enabled = !seedName.isEmpty() && lastError.value?.isEmpty() as Boolean
			) {
				Text("Create")
			}


	}
}
