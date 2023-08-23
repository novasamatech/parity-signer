package io.parity.signer.ui

import android.util.Log
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.components.panels.TopBar
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NavigationMigrations
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.findActivity
import io.parity.signer.screens.initial.UnlockAppAuthScreen
import io.parity.signer.screens.initial.WaitingScreen
import io.parity.signer.ui.rustnavigationselectors.AlertSelector
import io.parity.signer.ui.rustnavigationselectors.BottomSheetSelector
import io.parity.signer.ui.rustnavigationselectors.CombinedScreensSelector
import io.parity.signer.ui.rustnavigationselectors.ModalSelector
import io.parity.signer.ui.rustnavigationselectors.ScreenSelector


fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		val sharedViewModel: SharedViewModel = viewModel()

		val authenticated = sharedViewModel.authenticated.collectAsState()

		BackHandler {
			sharedViewModel.navigator.backAction()
		}

		if (authenticated.value) {
			MainUnlockedSubgraphVault(sharedViewModel)
		} else {
			val currentActivity =
				LocalContext.current.findActivity() as FragmentActivity
			UnlockAppAuthScreen {
				val authentication = ServiceLocator.authentication
				authentication.authenticate(currentActivity) {
					sharedViewModel.totalRefresh()
				}
			}
		}
		LaunchedEffect(Unit) {
			Log.d(
				NAVIGATION_TAG,
				"main rust-handled screen navigation subgraph opened"
			)
		}
	}
}


@Composable
fun MainUnlockedSubgraphVault(sharedViewModel: SharedViewModel) {

	val actionResultState = sharedViewModel.actionResult.collectAsState()
	val shieldNetworkState = sharedViewModel.networkState.collectAsState()
	val localNavAction = sharedViewModel.localNavAction.collectAsState()

	val actionResult = actionResultState.value

	if (actionResult == null) {
		WaitingScreen()
	} else {
		// Structure to contain all app
		Box {
			//screens before redesign
			val navigator: Navigator = sharedViewModel.navigator
			Scaffold(
				modifier = Modifier
					.navigationBarsPadding()
					.captionBarPadding()
					.statusBarsPadding(),
				topBar = {
					if (NavigationMigrations.shouldShowBar(
							localNavAction = localNavAction.value,
							globalNavAction = actionResult,
						)
					) {
						TopBar(
							sharedViewModel = sharedViewModel,
							actionResult = actionResult,
							networkState = shieldNetworkState,
						)
					}
				},
			) { innerPadding ->
				Box(modifier = Modifier.padding(innerPadding)) {
					ScreenSelector(
						screenData = actionResult.screenData,
						navigator = navigator,
						sharedViewModel = sharedViewModel,
					)
					ModalSelector(
						modalData = actionResult.modalData,
						localNavAction = localNavAction.value,
						sharedViewModel = sharedViewModel,
					)
				}
			}
			//new screens selectors
			Box(
				modifier = Modifier
					.navigationBarsPadding()
					.captionBarPadding(),
			) {
				CombinedScreensSelector(
					screenData = actionResult.screenData,
					localNavAction = localNavAction.value,
					networkState = shieldNetworkState,
					sharedViewModel = sharedViewModel
				)
				BottomSheetSelector(
					modalData = actionResult.modalData,
					localNavAction = localNavAction.value,
					networkState = shieldNetworkState,
					sharedViewModel = sharedViewModel,
					navigator = navigator,
				)
				AlertSelector(
					alert = actionResult.alertData,
					navigator = navigator,
					acknowledgeWarning = sharedViewModel::acknowledgeWarning
				)
			}
		}
	}
}


