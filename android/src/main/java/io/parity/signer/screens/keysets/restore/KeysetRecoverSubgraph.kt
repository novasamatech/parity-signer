package io.parity.signer.screens.keysets.restore

import androidx.activity.compose.BackHandler
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
import io.parity.signer.domain.popUpToTop
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.keysets.restore.keysetname.KeysetRecoverNameScreen
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

	val localNavController = rememberNavController()
	NavHost(
		navController = localNavController,
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
					localNavController.navigate(KeysetRecoverSubgraph.seedPhrase)
				},
				seedNames = existingSeedNames.value,
				modifier = Modifier
					.statusBarsPadding()
					.imePadding()
			)
		}
		//todo dmitry test suggested words
		composable(KeysetRecoverSubgraph.seedPhrase) {
			KeysetRecoverPhraseScreen(
				model = model.value,
				backAction = localNavController::popBackStack,
				onNewInput = { newInput -> viewModel.onUserInput(newInput) },
				onAddSuggestedWord = { suggestedWord ->
					viewModel.onAddword(suggestedWord)
				},
				onDone = {
					if (model.value.validSeed) {
						localNavController.navigate(
							KeysetRecoverSubgraph.NetworksSelection.destination(
								model.value.enteredWords.joinToString { " " })
						)
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
				onBack = localNavController::popBackStack,
				navigateOnSuccess = {
					coreNavController.navigate(
						CoreUnlockedNavSubgraph.KeySetDetails.destination(
							keysetName
						)
					) {
						popUpToTop(coreNavController)
					}
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
