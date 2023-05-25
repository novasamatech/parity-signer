package io.parity.signer.screens.keysets.restore

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreenView
import kotlinx.coroutines.Dispatchers


@Composable
fun KeysetRecoverPhraseScreenFull(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
	val viewModel: KeysetRecoverViewModel = viewModel()
	//Dispatchers.Main.immediate because it used in TextField to workaround bug
	//https://issuetracker.google.com/issues/160257648
	val state = viewModel.recoverState.collectAsState(Dispatchers.Main.immediate)

	LaunchedEffect(key1 = Unit) {
		viewModel.initValue(initialRecoverSeedPhrase)
	}

	state.value?.let { state ->
		KeysetRecoverPhraseScreenView(
			model = state,
			backAction = {
				viewModel.resetState()
				rootNavigator.backAction() },
			onNewInput = { newInput -> viewModel.onTextEntry(newInput) },
			onAddSuggestedWord = { suggestedWord -> viewModel.addWord(suggestedWord)  },
			onDone = {
				state.readySeed?.let { seedFinal ->
					viewModel.resetState()
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




