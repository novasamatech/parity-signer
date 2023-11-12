package io.parity.signer.screens.settings.logs

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.compose.navigation
import androidx.navigation.navArgument
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.screens.settings.logs.comment.AddLogCommentScreen
import io.parity.signer.screens.settings.logs.logdetails.LogDetailsScreen
import io.parity.signer.screens.settings.logs.logslist.LogsScreenFull

fun NavGraphBuilder.logsNavigationSubgraph(
	navController: NavController,
) {
	navigation(
		route = SettingsNavSubgraph.logs,
		startDestination = LogsSubgraph.home,
	) {
		composable(route = LogsSubgraph.home) {
			LogsScreenFull(navController)
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
				LogDetailsScreen(navController, logElement)
			}
		}
		composable(route = LogsSubgraph.logs_add_comment) {
			AddLogCommentScreen(
				onBack = { navController.popBackStack() }
			)
		}
	}
}

internal object LogsSubgraph {
	const val home = "logs_home"
	const val logs_details = "logs_details"
	const val logs_add_comment = "logs_add_comment"
	const val PARAM_LOG_DETAILS = "logID"
}
