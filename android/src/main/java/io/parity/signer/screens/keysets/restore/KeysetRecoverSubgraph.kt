package io.parity.signer.screens.keysets.restore

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.domain.submitErrorState
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
	val model = viewModel.recoverSeed.collectAsStateWithLifecycle()
	var keysetName by rememberSaveable() {
		mutableStateOf("")
	}

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeysetRecoverSubgraph.keysetName,
	) {

		composable(KeysetRecoverSubgraph.keysetName) {
			val existingSeedNames =
				viewModel.existingSeeds.collectAsStateWithLifecycle()
			KeysetRecoverNameScreen(
				initialKeySetName = keysetName,
				onBack = { coreNavController.popBackStack() },
				onNext = { restoredName ->
					keysetName = restoredName
					navController.navigate(KeysetRecoverSubgraph.seedPhrase)
				},
				seedNames = existingSeedNames.value,
				modifier = Modifier
					.statusBarsPadding()
					.imePadding()
			)
		}
		composable(KeysetRecoverSubgraph.seedPhrase) {
				KeysetRecoverPhraseScreen(
					model = model.value,
					backAction = navController::popBackStack,
					onNewInput = { newInput -> viewModel.onTextEntry(newInput) },
					onAddSuggestedWord = { suggestedWord ->
						viewModel.addWord(suggestedWord)
					},
					onDone = {
						if (model.value.validSeed) {
							navController.navigate(KeysetRecoverSubgraph.NetworksSelection.destination(
								model.value.enteredWords.joinToString { " " }))
						} else {
							submitErrorState("navigation to finish called, but seed is not valid")
						}
					},
					modifier = Modifier
						.statusBarsPadding()
						.imePadding()
				)
		}
		composable(
			route = KeysetRecoverSubgraph.NetworksSelection.route,
			arguments = listOf(
				navArgument(KeysetRecoverSubgraph.NetworksSelection.seedNameArg) {
					type = NavType.StringType
				}
			)
		) {
			val seedName =
				it.arguments?.getString(KeysetRecoverSubgraph.NetworksSelection.seedNameArg)!!

			RecoverKeysetSelectNetworkRestoreFlowFullScreen(
				seedName = keysetName,
				seedPhrase = seedName,
				onBack = navController::popBackStack,
				navigateOnSuccess = {
					coreNavController.navigate(
						CoreUnlockedNavSubgraph.KeySetDetails.destination(
							keysetName
						)
					)
				},
			)
		}
	}
}

private object KeysetRecoverSubgraph {
	const val keysetName = "recover_keyset_name"
	const val seedPhrase = "recover_keyset_phrase"

	object NetworksSelection {
		internal const val seedNameArg = "seed_name_arg"
		private const val baseRoute = "recover_keyset_networks_confirmation"
		const val route = "$baseRoute/{${seedNameArg}}"
		fun destination(seedName: String) = "$baseRoute/${seedName}"
	}
}
