package io.parity.signer.screens.keysetdetails

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.keySetDetailsDestination(
	navController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.KeySetDetails.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.KeySetDetails.seedNameOptionalArg) {
				type = NavType.StringType
				defaultValue = null
			}
		)
	) {
		val seedName =
			it.arguments?.getString(CoreUnlockedNavSubgraph.KeySetDetails.seedNameOptionalArg)

		KeySetDetailsScreenSubgraph(
			seedName = seedName,
			navController = navController,
			onBack = { navController.popBackStack() },
		)
	}
}
