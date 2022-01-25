package io.parity.signer.screens

import android.util.Log
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Checkbox
import androidx.compose.material.Divider
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.TextFieldValue
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.components.BigButton
import io.parity.signer.models.*
import io.parity.signer.models.guessWord
import io.parity.signer.models.validatePhrase

@Composable
fun RecoverSeedPhrase(
	button: (button: ButtonID, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {
	var seedPhrase by remember { mutableStateOf(listOf<String>()) }
	var seedWord by remember {
		mutableStateOf(
			TextFieldValue(
				" ",
				selection = TextRange(1)
			)
		)
	}
	var guessWord by remember { mutableStateOf(listOf<String>()) }
	var errorMessage: String? by remember { mutableStateOf(null) }
	val createRoots = remember { mutableStateOf(true) }
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
				Log.d("word", "[" + word + "]")
				String
				signerDataModel.clearError()
				if (word.text == "") {
					Log.d("branch", "1")
					if (seedPhrase.count() > 0) {
						seedPhrase = seedPhrase.subList(0, seedPhrase.lastIndex)
					}
					seedWord = TextFieldValue(" ", selection = TextRange(1))
					guessWord = signerDataModel.guessWord(seedWord.text.substring(1))
				} else {
					if (word.text.first() != ' ') {
						if (seedPhrase.count() > 0) {
							seedPhrase = seedPhrase.subList(0, seedPhrase.lastIndex)
						}
						seedWord = TextFieldValue(" ", selection = TextRange(1))
						guessWord = signerDataModel.guessWord(seedWord.text.substring(1))
					} else {
						if (word.text.last() == ' ') {
							seedWord = TextFieldValue(word.text.dropLast(1))
							if (guessWord.count() == 1) {
								Log.d("branch", "2")
								seedPhrase += guessWord.last()
								seedWord = TextFieldValue(" ", selection = TextRange(1))
								guessWord =
									signerDataModel.guessWord(seedWord.text.substring(1))
							} else {
								Log.d("branch", "3-incomplete")
								if (guessWord.contains(seedWord.text.drop(1))) {
									Log.d("branch", "3")
									seedPhrase += seedWord.text.drop(1)
									seedWord = TextFieldValue(" ", selection = TextRange(1))
									guessWord =
										signerDataModel.guessWord(seedWord.text.substring(1))
								}
							}
						} else {
							Log.d("branch", "4")
							seedWord = word
							guessWord = signerDataModel.guessWord(word.text.substring(1))
						}
					}
				}
				Log.d("branch", "end")
				errorMessage =
					signerDataModel.validatePhrase(seedPhrase.joinToString(" "))
			},
			singleLine = true,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = KeyboardCapitalization.None,
				keyboardType = KeyboardType.Ascii,
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
		Row(Modifier.toggleable(
			value = createRoots.value,
			role = Role.Checkbox,
			onValueChange = { createRoots.value = it }
		)) {
			Checkbox(
				checked = createRoots.value,
				onCheckedChange = { createRoots.value = it })
			Text("Create root keys")
		}
		BigButton(
			text = "Next",
			action = {
				signerDataModel.screenData.value?.let { screenData ->
					screenData.optString("seed_name").let { seedName ->
						signerDataModel.addSeed(
							seedName = seedName,
							seedPhrase = seedPhrase.joinToString(" "),
							createRoots = createRoots.value
						)
					}
				}
			}
		)
	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		guessWord = signerDataModel.guessWord(seedWord.text.drop(1))
		onDispose { focusManager.clearFocus() }
	}
}
