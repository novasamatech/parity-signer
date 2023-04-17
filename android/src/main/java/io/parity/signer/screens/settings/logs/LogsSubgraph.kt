package io.parity.signer.screens.settings.logs

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.ui.Modifier
import androidx.navigation.*
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.createderivation.HomeDerivationSheetsSubGraph
import io.parity.signer.screens.createderivation.derivationsubscreens.NetworkSelectionBottomSheet
import io.parity.signer.screens.settings.logs.logslist.LogsScreen
import io.parity.signer.ui.BottomSheetWrapperRoot


//is ScreenData.Log
//Box(Modifier.statusBarsPadding()) {
//	LogsScreen(
//		model = screenData.f.toLogsScreenModel(),
//		navigator = rootNavigator,
//	)
//}


//				is ModalData.LogRight ->
//					BottomSheetWrapperRoot(onClosedAction = {
//						navigator.backAction()
//					}) {
//						LogsMenu(
//							navigator = sharedViewModel.navigator,
//						)
//					}


//		is ScreenData.LogDetails -> LogDetails(screenData.f)


fun NavGraphBuilder.logsNavigationSubgraph(
	routePath: String,
	rootNavigator: Navigator,
	navController: NavController,
) {
	navigation(
		route = routePath,
		startDestination = LogsSubgraph.waiting,
	) {
		composable(route = LogsSubgraph.waiting) {

		}
		composable(route = LogsSubgraph.home) {
			val subNavController = rememberNavController()
			Box(Modifier.statusBarsPadding()) {
				LogsScreen(
					model = LogsScreenModel(),
					rootNavigator = rootNavigator,
					onMenu = { subNavController.navigate(LogsMenuSubgraph.logs_menu) },
					onLogClicked = { logId -> navController.navigate(LogsSubgraph.logs_details + "/" + logId) },
				)
			}
			//bottom sheets
			NavHost(
				navController = subNavController,
				startDestination = LogsMenuSubgraph.logs_empty,
			) {
				composable(LogsMenuSubgraph.logs_empty) {}
				composable(LogsMenuSubgraph.logs_menu) {
//					BottomSheetWrapperRoot(onClosedAction = closeAction) {
//						NetworkSelectionBottomSheet(
//							networks = deriveViewModel.allNetworks,
//							currentlySelectedNetwork = selectedNetwork.value,
//							onClose = closeAction,
//							onSelect = { newNetwork ->
//								deriveViewModel.updateSelectedNetwork(newNetwork)
//								closeAction()
//							},
//						)
//					}
				}
				composable(LogsMenuSubgraph.logs_menu_delete_confirm) {

				}
			}

		}
		composable(
			route = LogsSubgraph.logs_details + "/{${LogsSubgraph.PARAM_LOG_DETAILS}}",
			arguments = listOf(navArgument(LogsSubgraph.PARAM_LOG_DETAILS) {
				type = NavType.LongType
			})
		) { backStackEntry ->
			val logElement =
				backStackEntry.arguments?.getLong(LogsSubgraph.PARAM_LOG_DETAILS)
					?.toUInt()
			if (logElement == null) {
				navController.popBackStack()
			} else {

			}

		}
		composable(route = LogsSubgraph.logs_add_comment) {
//todo dmitry implement
		}
	}
}

private object LogsSubgraph {
	const val waiting = "logs_waiting"
	const val home = "logs_home"

	const val logs_details = "logs_details"
	const val logs_add_comment = "logs_add_comment"
	const val PARAM_LOG_DETAILS = "logID"
}

private object LogsMenuSubgraph {
	const val logs_empty = "logs_menu_empty"
	const val logs_menu = "logs_menu"
	const val logs_menu_delete_confirm = "logs_menu_delete_confirm"
}
