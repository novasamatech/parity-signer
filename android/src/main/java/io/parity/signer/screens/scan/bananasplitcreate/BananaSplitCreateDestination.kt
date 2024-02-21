package io.parity.signer.screens.scan.bananasplitcreate

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import androidx.navigation.navigation
import io.parity.signer.screens.scan.bananasplitcreate.create.CreateBananaSplitScreen
import io.parity.signer.screens.scan.bananasplitcreate.show.BananaSplitShowFull
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.bananaSplitCreateDestination(
	navController: NavController,
) {
	navigation(
		route = CoreUnlockedNavSubgraph.createBananaSplit,
		startDestination = BananaSplitCreateDestination.ShowBS,
	) {
		composable(
			route = BananaSplitCreateDestination.ShowBS,
		) {
			BananaSplitShowFull(navController)
		}
		composable(
			route = BananaSplitCreateDestination.CreateBsCreateScreen,
		) {
			CreateBananaSplitScreen(navController)
		}
	}
}


internal object BananaSplitCreateDestination {
	const val CreateBsCreateScreen = "create_banana_split"
	const val ShowBS = "show_banana_split"
}
