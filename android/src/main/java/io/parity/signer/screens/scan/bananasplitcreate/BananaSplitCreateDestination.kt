package io.parity.signer.screens.scan.bananasplitcreate

import androidx.compose.runtime.remember
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.compose.navigation
import androidx.navigation.navArgument
import io.parity.signer.screens.scan.bananasplitcreate.create.CreateBananaSplitScreen
import io.parity.signer.screens.scan.bananasplitcreate.show.BananaSplitShowFull
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.bananaSplitCreateDestination(
	navController: NavController,
) {
	navigation(
		route = CoreUnlockedNavSubgraph.CreateBananaSplit.route,
		startDestination = BananaSplitCreateDestination.ShowBS,
		arguments = listOf(navArgument(CoreUnlockedNavSubgraph.CreateBananaSplit.seedNameArg) {
			type = NavType.StringType
		}),
	) {
		composable(
			route = BananaSplitCreateDestination.ShowBS,
		) { entry ->
			val parentEntry = remember(entry) {
				navController.getBackStackEntry(CoreUnlockedNavSubgraph.CreateBananaSplit.route)
			}
			val seedName =
				parentEntry.arguments?.getString(CoreUnlockedNavSubgraph.CreateBananaSplit.seedNameArg)!!

			BananaSplitShowFull(navController, seedName)
		}
		composable(
			route = BananaSplitCreateDestination.CreateBsCreateScreen,
		) { entry ->
			val parentEntry = remember(entry) {
				navController.getBackStackEntry(CoreUnlockedNavSubgraph.CreateBananaSplit.route)
			}
			val seedName =
				parentEntry.arguments?.getString(CoreUnlockedNavSubgraph.CreateBananaSplit.seedNameArg)!!

			CreateBananaSplitScreen(navController, seedName)
		}
	}
}


internal object BananaSplitCreateDestination {
	const val CreateBsCreateScreen = "create_banana_split"
	const val ShowBS = "show_banana_split"
}
