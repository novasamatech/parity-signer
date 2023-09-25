package io.parity.signer.screens.keysetdetails

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.keysBySeedName

fun NavGraphBuilder.keySetDetailsDestination(
	navController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.KeySetDetails.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.KeySetDetails.seedNameArg) {
				type = NavType.StringType
			}
		)
	) {
		val seedName =
			it.arguments?.getString(CoreUnlockedNavSubgraph.KeySetDetails.seedNameArg)

		val model = try {
			//todo export this to vm and handle errors - open default for example
			keysBySeedName(seedName!!).toKeySetDetailsModel()
		} catch (e: ErrorDisplayed) {
			navController.navigate(
				CoreUnlockedNavSubgraph.ErrorScreen.destination(
					argHeader = "Unexpected error in keysBySeedName",
					argDescription = e.toString(),
					argVerbose = e.getDebugDetailedDescriptionString(),
				)
			)
			null
		}
		model?.let {
			KeySetDetailsScreenSubgraph(
				fullModel = model,
				navController = navController,
				onBack = { navController.popBackStack() },
			)
		}
	}
}
