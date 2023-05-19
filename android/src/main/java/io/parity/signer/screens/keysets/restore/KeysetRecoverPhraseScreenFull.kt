package io.parity.signer.screens.keysets.restore

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreenView


@Composable
fun KeysetRecoverPhraseScreenFull(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
	val viewModel: KeysetRecoverViewModel = viewModel()
	val state = viewModel.recoverState.collectAsState()

	LaunchedEffect(key1 = Unit) {
		viewModel.initValue(initialRecoverSeedPhrase)
	}

	state.value?.let { state ->
		KeysetRecoverPhraseScreenView(
			model = initialRecoverSeedPhrase,
			backAction = { rootNavigator.backAction() },
			onNewInput = { newInput -> viewModel.onTextEntry(newInput) },
			onAddSuggestedWord = { suggestedWord -> viewModel.onTextEntry(suggestedWord)  },
			onDone = {
				state.readySeed?.let { seedFinal ->
					viewModel.addSeed(
						seedName = state.seedName,
						seedPhrase = seedFinal,
						navigator = rootNavigator
					)
				}
			}
		)
	}
}




