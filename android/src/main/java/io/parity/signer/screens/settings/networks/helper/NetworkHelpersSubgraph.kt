package io.parity.signer.screens.settings.networks.helper

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.navigation
import io.parity.signer.domain.Callback
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.networkHelpersCoreSubgraph(
	navController: NavController,
) {
	networkHelpersSubgraph(
		routePath = CoreUnlockedNavSubgraph.networkHelpers,
		onScanClicked = { navController.navigate(CoreUnlockedNavSubgraph.Camera.destination(null)) },
		navController = navController,
	)
}

fun NavGraphBuilder.networkHelpersSubgraph(
	routePath: String,
	onScanClicked: Callback,
	navController: NavController,
) {
	navigation(
		route = routePath,
		startDestination = NetworkHelpersSubgraph.add_networks,
	) {
		composable(route = NetworkHelpersSubgraph.add_networks) {
			Box(modifier = Modifier.statusBarsPadding()) {
				HowAddNetworks(
					onClose = { navController.popBackStack() },
					onNext = { navController.navigate(NetworkHelpersSubgraph.update_metadata) },
					onScanClicked = onScanClicked,
				)
			}
		}
		composable(route = NetworkHelpersSubgraph.update_metadata) {
			Box(modifier = Modifier.statusBarsPadding()) {
				HowUpdateMetadata(
					onClose = { navController.popBackStack() },
					onDone = {
						navController.popBackStack(
							NetworkHelpersSubgraph.add_networks,
							true
						)
					},
					onScanClicked = onScanClicked,
				)
			}
		}
	}
}

private object NetworkHelpersSubgraph {
	const val add_networks = "networks_add_network"
	const val update_metadata = "networks_update_metadata"
}
