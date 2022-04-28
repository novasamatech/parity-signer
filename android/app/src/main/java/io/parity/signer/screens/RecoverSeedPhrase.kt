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
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.RestoreSeedPhraseBox
import io.parity.signer.components.RestoreSeedPhraseSuggest
import io.parity.signer.models.*
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun RecoverSeedPhrase(
	screenData: JSONObject,
	button: (button: ButtonID, details: String) -> Unit,
	addSeed: (String, String, Boolean) -> Unit
) {
	val seedPhrase = screenData.optJSONArray("draft")?.toListOfJSONObjects()
		?: listOf() //remember { mutableStateOf(listOf<String>()) }
	val guessWord = screenData.optJSONArray("guess_set")?.toListOfStrings()
		?: listOf() //remember { mutableStateOf(listOf<String>()) }
	val seedPhraseReady = screenData.optString("ready_seed")
	val seedWordText = screenData.optString("user_input") ?: ""
	val seedWord = TextFieldValue(
		seedWordText,
		selection = TextRange(seedWordText.length)
	)
	val createSeedKeys = remember { mutableStateOf(true) }

	Column(
		verticalArrangement = Arrangement.Top,
		modifier = Modifier.padding(horizontal = 12.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			Text(
				screenData.optString("seed_name").decode64(),
				style = MaterialTheme.typography.subtitle1
			)
		}
		HeadingOverline("SEED PHRASE")

		Spacer(Modifier.height(12.dp))
		RestoreSeedPhraseBox(
			seedPhrase = seedPhrase,
			seedWord = seedWord,
			button = button,
			keyboard = screenData.optBoolean("keyboard")
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
				value = createSeedKeys.value,
				role = Role.Checkbox,
				onValueChange = { createSeedKeys.value = it }
			)) {
			Checkbox(
				checked = createSeedKeys.value,
				onCheckedChange = { createSeedKeys.value = it })
			Text("Create seed keys")
		}
		Spacer(Modifier.weight(0.1f))
		//TODO: hide when keyboard is shown
		BigButton(
			text = "Next",
			action = {
				screenData.optString("seed_name").let { seedName ->
					addSeed(
						seedName,
						seedPhraseReady,
						createSeedKeys.value
					)
				}
			},
			isDisabled = seedPhraseReady.isBlank()
		)
		Spacer(Modifier.weight(0.1f))
	}

}
