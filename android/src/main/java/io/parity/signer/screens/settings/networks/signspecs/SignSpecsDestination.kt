package io.parity.signer.screens.settings.networks.signspecs

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.SettingsNavSubgraph
import kotlinx.coroutines.runBlocking


/**
 * aka SignSufficientCrypto was before. Actually the same thing
 */
fun NavGraphBuilder.signSpecsDestination(
	navController: NavController,
) {
	composable(
		route = SettingsNavSubgraph.SignNetworkSpecs.route,
		arguments = listOf(
			navArgument(SettingsNavSubgraph.SignNetworkSpecs.networkKey) {
				type = NavType.StringType
			}
		)
	) {
		val networkKey =
			it.arguments?.getString(SettingsNavSubgraph.SignNetworkSpecs.networkKey)!!

		val vm: SignSpecsViewModel = viewModel()
		val model = remember() {
			runBlocking {
				vm.getKeysListModel().handleErrorAppState(navController)
			}
		} ?: return@composable
		SignSpecsFull(
			model = model,
			inputData = SignSpecsInput.NetworkSpecs(
				networkKey = networkKey,
			),
			onBack = navController::popBackStack,
		)
	}
	composable(
		route = SettingsNavSubgraph.SignMetadataSpecs.route,
		arguments = listOf(
			navArgument(SettingsNavSubgraph.SignMetadataSpecs.networkKey) {
				type = NavType.StringType
			},
			navArgument(SettingsNavSubgraph.SignMetadataSpecs.metadataSpecVer) {
				type = NavType.StringType
			},
		)
	) {
		val networkKey =
			it.arguments?.getString(SettingsNavSubgraph.SignMetadataSpecs.networkKey)!!
		val metadataSpecVer =
			it.arguments?.getString(SettingsNavSubgraph.SignMetadataSpecs.metadataSpecVer)!!

		val vm: SignSpecsViewModel = viewModel()
		val model = remember() {
			runBlocking {
				vm.getKeysListModel().handleErrorAppState(navController)
			}
		} ?: return@composable
		SignSpecsFull(
			model = model,
			inputData = SignSpecsInput.NetworkMetadataSpecs(
				networkKey = networkKey,
				versionSpec = metadataSpecVer
			),
			onBack = navController::popBackStack,
		)
	}
}
