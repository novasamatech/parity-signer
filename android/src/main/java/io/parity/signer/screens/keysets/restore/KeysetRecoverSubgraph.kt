package io.parity.signer.screens.keysets.restore

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreen
import io.parity.signer.screens.keysets.restore.restorephrase.RecoverKeysetSelectNetworkRestoreFlowFullScreen
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


@Composable
fun KeysetRecoverSubgraph(
	coreNavController: NavController,
) {
//background
	Box(
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	val viewModel: KeysetRecoverViewModel = viewModel()
	val model = viewModel.recoverState.collectAsStateWithLifecycle()

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeysetRecoverSubgraph.keysetName,
	) {

		composable(KeysetRecoverSubgraph.keysetName) {
			val seedNames =
				viewModel.existingSeeds.collectAsStateWithLifecycle()
			KeysetRecoverNameScreen(
				onBack = { coreNavController.popBackStack() },
				onNext = {restoredName ->
					//todo dmitry remember restored name
					navController.navigate(KeysetRecoverSubgraph.keysetRecoverSeed) },
				seedNames = seedNames.value,
				modifier = Modifier
					.statusBarsPadding()
					.imePadding()
			)
		}
		composable(KeysetRecoverSubgraph.keysetRecoverSeed) {
			model.value?.let { stateModel ->
				KeysetRecoverPhraseScreen(
					model = stateModel,
					backAction = navController::popBackStack,
					onNewInput = { newInput -> viewModel.onTextEntry(newInput) },
					onAddSuggestedWord = { suggestedWord ->
						viewModel.addWord(suggestedWord)
					},
					onDone = {
						stateModel.readySeed?.let { seedFinal ->
							navController.navigate(KeysetRecoverSubgraph.keysetRecoverNetworks)
						}
					},
					modifier = Modifier
						.statusBarsPadding()
						.imePadding()
				)
			}
		}
		composable(KeysetRecoverSubgraph.keysetRecoverNetworks) {
			RecoverKeysetSelectNetworkRestoreFlowFullScreen(
				seedName = model.value!!.seedName,
				seedPhrase = model.value!!.readySeed!!,
				onBack = navController::popBackStack,
				navigateOnSuccess = {
					coreNavController.navigate(
						CoreUnlockedNavSubgraph.KeySetDetails.destination(
							model.value!!.seedName
						)
					)
				},
			)
		}
	}
}

private object KeysetRecoverSubgraph {
	const val keysetName = "recover_keyset_name"
	const val keysetRecoverSeed = "recover_keyset_phrase"
	const val keysetRecoverNetworks = "recover_keyset_backup_confirmation"
}
