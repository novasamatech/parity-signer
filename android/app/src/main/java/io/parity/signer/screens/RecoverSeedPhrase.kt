package io.parity.signer.screens

import android.util.Log
import androidx.activity.viewModels
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
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
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.TextFieldValue
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.components.BigButton
import io.parity.signer.components.RestoreSeedPhraseBox
import io.parity.signer.components.RestoreSeedPhraseSuggest
import io.parity.signer.models.*
import io.parity.signer.models.guessWord
import io.parity.signer.models.validatePhrase

@Composable
fun RecoverSeedPhrase(
	button: (button: ButtonID, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {

	val seedPhrase = remember { mutableStateOf(listOf<String>()) }
	val guessWord = remember { mutableStateOf(listOf<String>()) }
	val seedValid = remember { mutableStateOf(false) }
	val seedWord = remember {
		mutableStateOf(
			TextFieldValue(
				" ",
				selection = TextRange(1)
			)
		)
	}
	val restoreSeedModel = remember(seedPhrase, seedWord, guessWord, seedValid) {
		RestoreSeedModel(
			seedPhrase,
			seedWord,
			guessWord,
			seedValid,
			signerDataModel::guessWord,
			signerDataModel::validatePhrase
		)
	}
	val createRoots = remember { mutableStateOf(true) }

	Column {
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			Text(
				signerDataModel.screenData.value?.optString("seed_name")
					?: "Error: no seed name"
			)
		}
		Text("SEED PHRASE", style = MaterialTheme.typography.overline)

		RestoreSeedPhraseBox(
			seedPhrase = restoreSeedModel.seedPhrase,
			seedWord = restoreSeedModel.seedWord,
			update = restoreSeedModel::update,
			keyboard = signerDataModel.screenData.value?.optBoolean("keyboard")
				?: false
		)

		RestoreSeedPhraseSuggest(
			restoreSeedModel.guessWord,
			push = restoreSeedModel::select
		)
		Spacer(Modifier.weight(0.8f))
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier.toggleable(
				value = createRoots.value,
				role = Role.Checkbox,
				onValueChange = { createRoots.value = it }
			)) {
			Checkbox(
				checked = createRoots.value,
				onCheckedChange = { createRoots.value = it })
			Text("Create root keys")
		}
		Spacer(Modifier.weight(0.1f))
		if (true) { //TODO: hide when keyboard is shown
			BigButton(
				text = "Next",
				action = {
					signerDataModel.screenData.value?.let { screenData ->
						screenData.optString("seed_name").let { seedName ->
							signerDataModel.addSeed(
								seedName = seedName,
								seedPhrase = restoreSeedModel.seedPhrase.value.joinToString(" "),
								createRoots = createRoots.value
							)
						}
					}
				},
				isDisabled = !restoreSeedModel.seedValid.value
			)
		}
		Spacer(Modifier.weight(0.1f))
	}

}
