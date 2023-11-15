package io.parity.signer.screens.settings.verifiercert

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.runBlocking


fun NavGraphBuilder.verifierSettingsDestination(
	navController: NavController,
) {
	composable(SettingsNavSubgraph.generalVerifier) {
		val vm: VerifierCertViewModel = viewModel()

		val model = remember {
			runBlocking {
				vm.getVerifierCertModel()
			}.handleErrorAppState(navController)
		} ?: return@composable

		VerifierScreenFull(
			verifierDetails = model,
			wipe = {
				vm.wipeWithGeneralCertificate {
					navController.navigate(
						CoreUnlockedNavSubgraph.KeySet.destination(null)
					)
				}
			},
			onBack = navController::popBackStack,
		)
	}
}
