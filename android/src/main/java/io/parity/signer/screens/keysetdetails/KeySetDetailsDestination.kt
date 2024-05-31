package io.parity.signer.screens.keysetdetails

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.keySetDetailsDestination(
	coreNavController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.KeySet.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.KeySet.seedName) {
				type = NavType.StringType
				nullable = true
			}
		)
	) {
		val seedName =
			it.arguments?.getString(CoreUnlockedNavSubgraph.KeySet.seedName)

		KeySetDetailsScreenSubgraph(
			originalSeedName = seedName,
			coreNavController = coreNavController,
		)
	}
}
