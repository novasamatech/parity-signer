package io.parity.signer.screens.settings.networks.list

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.domain.backend.mapError
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.runBlocking


fun NavGraphBuilder.networkListDestination(
	navController: NavController,
) {
	composable(SettingsNavSubgraph.networkList) {
		val vm: NetworkListViewModel = viewModel()

		val model = remember {
			runBlocking {
				vm.getNetworkList()
			}.handleErrorAppState(navController)
		} ?: return@composable

		Box(modifier = Modifier.statusBarsPadding()) {
			NetworksListScreen(
				model = model,
				onBack = navController::popBackStack,
				onOpenNetwork = { networkKey ->
					navController.navigate(
						SettingsNavSubgraph.NetworkDetails.destination(networkKey)
					)
				},
				onNetworkHelp = { navController.navigate(CoreUnlockedNavSubgraph.networkHelpers) },
				onAddNetwork = { navController.navigate(CoreUnlockedNavSubgraph.Camera.destination(null)) },
			)
		}
	}
}
