package io.parity.signer.screens.keysets.restore.old

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.RestoreSeedPhraseBox
import io.parity.signer.components.RestoreSeedPhraseSuggest
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MRecoverSeedPhrase

//todo dmitry remove
@Composable
fun RecoverSeedPhrase(
	recoverSeedPhrase: MRecoverSeedPhrase,
	button: (action: Action, details: String) -> Unit,
	addSeed: (
		seedName: String,
		seedPhrase: String,
	) -> Unit
) {
	val seedPhrase =
		recoverSeedPhrase.draft // remember { mutableStateOf(listOf<String>()) }
	val guessWord =
		recoverSeedPhrase.guessSet // remember { mutableStateOf(listOf<String>()) }
	val seedPhraseReady = recoverSeedPhrase.readySeed
	val seedWordText = " " + recoverSeedPhrase.userInput // TODO: `" " +` in rust
	val seedWord = TextFieldValue(
		seedWordText,
		selection = TextRange(seedWordText.length)
	)

	Column(
		verticalArrangement = Arrangement.Top,
		modifier = Modifier.padding(horizontal = 12.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			Text(
				recoverSeedPhrase.seedName,
				style = MaterialTheme.typography.subtitle1
			)
		}
		HeadingOverline("Secret Recovery Phrase")

		Spacer(Modifier.height(12.dp))
		RestoreSeedPhraseBox(
			seedPhrase = seedPhrase,
			seedWord = seedWord,
			button = button,
			keyboard = recoverSeedPhrase.keyboard
		)

		Spacer(Modifier.height(12.dp))
		RestoreSeedPhraseSuggest(
			guessWord,
			button
		)
		Spacer(Modifier.weight(0.1f))
		if (recoverSeedPhrase.keyboard) {
			BigButton(
				text = "Recover Key Set",
				action = {
					recoverSeedPhrase.seedName.let { seedName ->
						seedPhraseReady?.let {
							addSeed(
								seedName,
								it,
							)
						}
					}
				},
				isDisabled = seedPhraseReady == null
			)
		}
		Spacer(Modifier.weight(0.1f))
	}
}
