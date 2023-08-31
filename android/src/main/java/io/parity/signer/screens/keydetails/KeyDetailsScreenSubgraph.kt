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
import io.parity.signer.domain.backend.mapError
import io.parity.signer.domain.toKeyDetailsModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import kotlinx.coroutines.runBlocking


@Composable
fun KeyDetailsScreenSubgraph(
	navController: NavHostController,
	keyAddr: String,
	keySpec: String
) {

	val vm = KeyDetailsScreenViewModel()
	//todo dmitry fix
	val model = remember {
		runBlocking {
			vm.fetchModel(keyAddr, keySpec)
		}
	}.mapError()!!.toKeyDetailsModel()

	//todo dmitry implement
	Box(modifier = Modifier.statusBarsPadding()) {
		KeyDetailsPublicKeyScreen(
			model = model,
			onBack = { navController.popBackStack() },
			onMenu = { KeyPublicDetailsMenuSubgraph.key_menu },
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
				//todo dmitry
//				KeyDetailsMenuAction(
//					navigator = navigator,
//					keyDetails = model
//				)
			}
		}
	}
}


private object KeyPublicDetailsMenuSubgraph {
	const val empty = "key_menu_empty"
	const val key_menu = "key_menu"
}
