package io.parity.signer.screens.settings.logs.logslist

import android.util.Log
import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.backend.CompletableResult
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.onboarding.WaitingScreen
import io.parity.signer.screens.settings.logs.LogsSubgraph
import io.parity.signer.screens.settings.logs.LogsViewModel
import io.parity.signer.screens.settings.logs.toLogsScreenModel
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun LogsScreenFull(
	rootNavigator: Navigator,
	navController: NavController,
) {
	val menuNavController = rememberNavController()
	val viewModel: LogsViewModel = viewModel<LogsViewModel>()
	val context = LocalContext.current

	val logsState = viewModel.logsState.collectAsState()
	val logsCurrentValue = logsState.value

	Box(Modifier.statusBarsPadding()) {
		when (logsCurrentValue) {
			is CompletableResult.Err -> {
				Log.e(TAG, "error in getting logs ${logsCurrentValue.error}")
				Toast.makeText(context, logsCurrentValue.error, Toast.LENGTH_LONG)
					.show()
				viewModel.resetValues()
				navController.popBackStack()
			}
			CompletableResult.InProgress -> {
				WaitingScreen()
			}
			is CompletableResult.Ok -> {
				LogsScreen(
					model = logsCurrentValue.result.toLogsScreenModel(context),
					rootNavigator = rootNavigator,
					onMenu = { menuNavController.navigate(LogsMenuSubgraph.logs_menu) },
					onBack = { navController.popBackStack() },
					onLogClicked = { logId -> navController.navigate(LogsSubgraph.logs_details + "/" + logId) },
				)
			}
		}
		LaunchedEffect(Unit) {
			viewModel.updateLogsData()
		}
	}
	//bottom sheets
	NavHost(
		navController = menuNavController,
		startDestination = LogsMenuSubgraph.logs_empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(LogsMenuSubgraph.logs_empty) {}//no menu
		composable(LogsMenuSubgraph.logs_menu) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				LogsMenuGeneral(
					onCreateComment = {
						navController.navigate(LogsSubgraph.logs_add_comment)
						menuNavController.popBackStack()
					},
					onDeleteClicked = {
						menuNavController.navigate(LogsMenuSubgraph.logs_menu_delete_confirm) {
							popUpTo(LogsMenuSubgraph.logs_empty)
						}
					},
					onCancel = closeAction,
				)
			}
		}
		composable(LogsMenuSubgraph.logs_menu_delete_confirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				LogeMenuDeleteConfirm(
					onCancel = closeAction,
					doRemoveKeyAndNavigateOut = {
						viewModel.actionClearLogsHistory(context)
						closeAction()
					}
				)
			}
		}
	}
}

private const val TAG = "logs_full"

private object LogsMenuSubgraph {
	const val logs_empty = "logs_menu_empty"
	const val logs_menu = "logs_menu"
	const val logs_menu_delete_confirm = "logs_menu_delete_confirm"
}
