package io.parity.signer.screens.keysetdetails

import android.util.Log
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
			navArgument(CoreUnlockedNavSubgraph.KeySetDetails.seedName) {
				type = NavType.StringType
				nullable = true
			}
		)
	) {
		val seedName =
			it.arguments?.getString(CoreUnlockedNavSubgraph.KeySetDetails.seedName)

		Log.e("TAGG", seedName.toString() + (seedName==null).toString())//todo dmitry remove
		KeySetDetailsScreenSubgraph(
			seedName = seedName,
			navController = navController,
			onBack = { navController.popBackStack() },
		)
	}
}
