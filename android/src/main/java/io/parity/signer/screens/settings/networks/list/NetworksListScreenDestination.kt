package io.parity.signer.screens.settings.networks.list

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.domain.backend.mapError
import io.parity.signer.screens.settings.SettingsScreenSubgraph
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.runBlocking


fun NavGraphBuilder.networkListDestination(
	navController: NavController,
) {
	composable(SettingsScreenSubgraph.networkList) {
		val vm: NetworkListViewModel = viewModel()

		val model = remember {
			runBlocking {
				vm.getNetworkList().mapError()!!
				//todo dmitry post error
			}
		}
		NetworksListScreen(
			model = model,
			coreNavController = navController,
			onBack = navController::popBackStack,
			onOpenNetwork = { networkKey ->
				navController.navigate(
					SettingsScreenSubgraph.NetworkDetails.destination(networkKey)
				)
			},
			onNetworkHelp = { navController.navigate(CoreUnlockedNavSubgraph.networkHelpers) },
			onAddNetwork = { navController.navigate(CoreUnlockedNavSubgraph.camera) },
		)
	}
}
