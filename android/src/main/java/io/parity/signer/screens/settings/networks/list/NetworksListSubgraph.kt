package io.parity.signer.screens.settings.networks.list

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.settings.networks.helper.networkHelpersSubgraph


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
				rootNavigator = rootNavigator,
				onNetworkHelp = { navController.navigate(NetworkListSubgraph.network_help) },
				onAddNetwork = {}, //todo dmitry finish same as below
			)
		}
		networkHelpersSubgraph(
			routePath = NetworkListSubgraph.network_help,
			onScanClicked = {}, ////			todo dmitry finish
			navController = navController,
		)
	}
}


private object NetworkListSubgraph {
	const val home = "network_list_main"
	const val network_help = "network_help"
}
