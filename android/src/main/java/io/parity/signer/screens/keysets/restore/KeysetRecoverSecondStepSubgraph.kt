package io.parity.signer.screens.keysets.restore

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreen
import io.parity.signer.screens.keysets.restore.restorephrase.RecoverKeysetSelectNetworkRestoreFlowScreen
import kotlinx.coroutines.Dispatchers


@Composable
fun NewKeysetRecoverSecondStepSubgraph(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
//background
	Box(
		modifier = Modifier
            .fillMaxSize(1f)
            .statusBarsPadding()
            .background(MaterialTheme.colors.background)
	)

	val viewModel: KeysetRecoverViewModel = viewModel()
	//Dispatchers.Main.immediate because it used in TextField to workaround bug
	//https://issuetracker.google.com/issues/160257648
	val state = viewModel.recoverState.collectAsState(Dispatchers.Main.immediate)

	DisposableEffect(key1 = Unit) {
		viewModel.initValue(initialRecoverSeedPhrase)
		onDispose { viewModel.resetState() }
	}

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeysetRecoverSubgraph.KeysetRecoverSeed,
	) {
		composable(KeysetRecoverSubgraph.KeysetRecoverSeed) {
			state.value?.let { stateModel ->
				KeysetRecoverPhraseScreen(
					model = stateModel,
					backAction = rootNavigator::backAction,
					onNewInput = { newInput -> viewModel.onTextEntry(newInput) },
					onAddSuggestedWord = { suggestedWord ->
						viewModel.addWord(suggestedWord)
					},
					onDone = {
						stateModel.readySeed?.let { seedFinal ->
							navController.navigate(KeysetRecoverSubgraph.KeysetRecoverNetworks)
						}
					},
				)
			}
			BackHandler(onBack = rootNavigator::backAction)
		}
		composable(KeysetRecoverSubgraph.KeysetRecoverNetworks) {
			RecoverKeysetSelectNetworkRestoreFlowScreen(
				seedName = state.value!!.seedName,
				seedPhrase = state.value!!.readySeed!!,
				rootNavigator = rootNavigator,
				onBack = navController::popBackStack,
			)
		}
	}
}

internal object KeysetRecoverSubgraph {
	const val KeysetRecoverSeed = "new_keyset_backup_main"
	const val KeysetRecoverNetworks = "new_keyset_backup_confirmation"
}
