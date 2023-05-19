package io.parity.signer.screens.keysets.restore

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreenView


@Composable
fun KeysetRecoverPhraseScreenFull(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
	var keySetName by remember { mutableStateOf("") }

	val canProceed = keySetName.isNotEmpty() //&& !seedNames.contains(keySetName)
	val viewModel = viewModel<KeysetRecoverViewModel>()

	KeysetRecoverPhraseScreenView(
		model = initialRecoverSeedPhrase,
		backAction = {rootNavigator.backAction()},
		onNewInput = {newInput -> },
		onAddSuggestedWord = {suggestedWord -> },

//		addSeed = { seedName, //todo dmitry remove
//								seedPhrase ->
//			viewModel.addSeed(
//				seedName = seedName,
//				seedPhrase = seedPhrase,
//				navigator = rootNavigator
//			)
//		}
	)
}




