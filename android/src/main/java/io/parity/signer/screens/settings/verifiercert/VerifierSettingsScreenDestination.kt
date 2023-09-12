package io.parity.signer.screens.settings.verifiercert

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.domain.backend.mapError
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
				vm.getVerifierCertModel().mapError()!!
				//todo dmitry post error
			}
		}
		VerifierScreenFull(
			verifierDetails = model,
			wipe = {
				vm.wipeWithGeneralCertificate {
					navController.navigate(
						CoreUnlockedNavSubgraph.keySetList
					)
				}
			},
			onBack = navController::popBackStack,
		)
	}
}
