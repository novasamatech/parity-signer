package io.parity.signer.screens.error

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


fun NavGraphBuilder.errorStateDestination(
	navController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.ErrorState.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.ErrorState.argHeader) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorState.argDescription) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorState.argVerbose) {
				type = NavType.StringType
			},
		),
	) {
		val argHeader =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorState.argHeader)!!
		val argDescr =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorState.argDescription)!!
		val argVerbose =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorState.argVerbose)!!

		ErrorStateScreen(
			header = argHeader,
			description = argDescr,
			verbose = argVerbose,
			onBack = { navController.popBackStack() },
		)
	}
}
