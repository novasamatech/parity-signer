package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.RestoreSeedPhraseBox
import io.parity.signer.components.RestoreSeedPhraseSuggest
import io.parity.signer.models.*
import org.json.JSONArray
import io.parity.signer.uniffi.Action

@Composable
fun RecoverSeedPhrase(
	button: (action: Action, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {
  val screenData = signerDataModel.screenData.observeAsState()
	val seedPhrase = screenData.value?.optJSONArray("draft")?.toListOfJSONObjects()?: listOf() //remember { mutableStateOf(listOf<String>()) }
	val guessWord = screenData.value?.optJSONArray("guess_set")?.toListOfStrings() ?: listOf() //remember { mutableStateOf(listOf<String>()) }
	val seedPhraseReady = screenData.value?.optString("ready_seed")
	val seedWordText = screenData.value?.optString("user_input")?: ""
	val seedWord = TextFieldValue(
				seedWordText,
				selection = TextRange(seedWordText.length)
	)
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
			seedPhrase = seedPhrase,
			seedWord = seedWord,
			button = button,
			keyboard = signerDataModel.screenData.value?.optBoolean("keyboard")
				?: false
		)

		Spacer(Modifier.height(12.dp))
		RestoreSeedPhraseSuggest(
			guessWord,
			button
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
							seedPhraseReady?.let {
								signerDataModel.addSeed(
									seedName = seedName,
									seedPhrase = it,
									createRoots = createRoots.value
								)
							}
						}
					}
				},
				isDisabled = seedPhraseReady == null
			)
		//}
		Spacer(Modifier.weight(0.1f))
	}

}
