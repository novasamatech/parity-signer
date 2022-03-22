package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.RestoreSeedPhraseBox
import io.parity.signer.components.RestoreSeedPhraseSuggest
import io.parity.signer.models.*

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

	Column(
		verticalArrangement = Arrangement.Top,
		modifier = Modifier.padding(horizontal = 12.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			Text(
				signerDataModel.screenData.value?.optString("seed_name")?.decode64()
					?: "Error: no seed name",
				style = MaterialTheme.typography.subtitle1
			)
		}
		HeadingOverline("SEED PHRASE")

		Spacer(Modifier.height(12.dp))
		RestoreSeedPhraseBox(
			seedPhrase = restoreSeedModel.seedPhrase,
			seedWord = restoreSeedModel.seedWord,
			update = restoreSeedModel::update,
			keyboard = signerDataModel.screenData.value?.optBoolean("keyboard")
				?: false
		)

		Spacer(Modifier.height(12.dp))
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
		//if (true) { //TODO: hide when keyboard is shown
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
		//}
		Spacer(Modifier.weight(0.1f))
	}

}
