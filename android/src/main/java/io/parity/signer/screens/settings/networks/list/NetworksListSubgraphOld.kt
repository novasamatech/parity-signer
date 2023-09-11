package io.parity.signer.screens.settings.networks.list

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.panels.CameraParentScreen
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.settings.networks.helper.networkHelpersSubgraph
import io.parity.signer.uniffi.Action

//todo dmitry not used?
@Composable
fun NetworksListSubgraph(model: NetworksListModel, rootNavigator: Navigator) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = NetworkListSubgraph.home,
	) {
		composable(NetworkListSubgraph.home) {
			NetworksListScreen(
				model = model,
				coreNavController= rememberNavController(),
				onBack = { rootNavigator.backAction() },
				onOpenNetwork = { networkKey ->
					rootNavigator.navigate(Action.GO_FORWARD, networkKey)
					CameraParentSingleton.lastPossibleParent =
						CameraParentScreen.NetworkDetailsScreen(networkKey)
				},
				onNetworkHelp = { navController.navigate(NetworkListSubgraph.network_help) },
				onAddNetwork = {
					rootNavigator.backAction()
					rootNavigator.navigate(Action.NAVBAR_SCAN)
				},
			)
		}
		networkHelpersSubgraph(
			routePath = NetworkListSubgraph.network_help,
			onScanClicked = {
				rootNavigator.backAction()
				rootNavigator.navigate(Action.NAVBAR_SCAN)
			},
			navController = navController,
		)
	}
}


private object NetworkListSubgraph {
	const val home = "network_list_main"
	const val network_help = "network_help"
}
