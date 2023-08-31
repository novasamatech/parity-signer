package io.parity.signer.screens.keydetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun KeyDetailsScreenSubgraph(
	navController: NavHostController,
) {

	val vm = KeyDetailsScreenViewModel()
	val model = remember { vm.fetchModel() }

	//todo dmitry implement
	Box(modifier = Modifier.statusBarsPadding()) {
		KeyDetailsPublicKeyScreen(
			model = model,
			onBack = { navController.popBackStack() },
			onMenu = {},//menu show
		)
	}

	val menuNavController = rememberNavController()

	NavHost(
		navController = menuNavController,
		startDestination = KeyPublicDetailsMenuSubgraph.empty,
	) {
		composable(KeyPublicDetailsMenuSubgraph.empty) {}//no menu
		composable(KeyPublicDetailsMenuSubgraph.key_menu) {
			BottomSheetWrapperRoot(onClosedAction = {
//				todo dmitry as in different subgraphs
//				navigator.backAction()
			}) {
				KeyDetailsMenuAction(
					navigator = navigator,
					keyDetails = model
				)
			}
		}
	}
}


private object KeyPublicDetailsMenuSubgraph {
	const val empty = "key_menu_empty"
	const val key_menu = "key_menu"
}
