package io.parity.signer.screens.settings.logs

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.navigation


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


fun NavGraphBuilder.logsNavigationSubgraph(routePath: String) {
	navigation(
		route = routePath,
		startDestination = LogsSubgraph.home,
	) {
		composable(route = LogsSubgraph.home) {
//Box(Modifier.statusBarsPadding()) {
//	LogsScreen(
//		model = screenData.f.toLogsScreenModel(),
//		navigator = rootNavigator,
//	)
//}
		}
		composable(route = LogsSubgraph.logs_details) {
			InterestsRoute(onTopicClick)
		}
	}
}

private object LogsSubgraph {
	const val home = "logs_home"
	const val logs_menu = "logs_menu"
	const val logs_menu_delete_confirm = "logs_menu_delete_confirm"
	const val logs_details = "logs_details"
	const val logs_add_comment = "logs_add_comment"
}
