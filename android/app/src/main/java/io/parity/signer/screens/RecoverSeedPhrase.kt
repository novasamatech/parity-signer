package io.parity.signer.screens

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Divider
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.guessWord
import io.parity.signer.models.validatePhrase

@Composable
fun RecoverSeedPhrase(
	button: (button: ButtonID, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {
	var seedPhrase by remember { mutableStateOf(mutableListOf<String>()) }
	var seedWord by remember { mutableStateOf(" ") }
	var guessWord by remember { mutableStateOf(mutableListOf<String>()) }
	var errorMessage: String?  by remember { mutableStateOf(null) }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column {
		Text(
			signerDataModel.screenData.value?.optString("seed_name")
				?: "Error: no seed name"
		)
		Text("SEED PHRASE")
		Text(seedPhrase.joinToString(" "))
		Divider()
		TextField(
			value = seedWord,
			onValueChange = { word ->
				Log.d("word", word)
				String
				signerDataModel.clearError()
				if (word != " ") {
					if (word == "") {
						if (seedPhrase.count() > 0) {
							seedPhrase.removeLast()
						}
						seedWord = " "
						guessWord = signerDataModel.guessWord(seedWord.substring(0))
					} else {
						if (word.last() == ' ') {
							seedWord = word.dropLast(1)
							if (guessWord.count() == 1) {
								seedPhrase += guessWord.last()
								seedWord = " "
								guessWord = signerDataModel.guessWord(seedWord.substring(0))
							} else {
								if (guessWord.contains(seedWord.drop(1))) {
									seedPhrase += seedWord.drop(1)
									seedWord = " "
									guessWord = signerDataModel.guessWord(seedWord.substring(0))
								}
							}
						} else {
							seedWord = word
							guessWord = signerDataModel.guessWord(word.substring(0))
						}
					}
				}
				errorMessage = signerDataModel.validatePhrase(seedPhrase.joinToString(" "))
			},
			singleLine = true,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = KeyboardCapitalization.None,
				keyboardType = KeyboardType.Text,
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = {
					focusManager.clearFocus()
				}
			),
			modifier = Modifier.focusRequester(focusRequester = focusRequester)
		)
		Text(guessWord.toString())
	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		guessWord = signerDataModel.guessWord(seedWord.drop(1))
		onDispose { focusManager.clearFocus() }
	}
}
