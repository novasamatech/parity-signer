package io.parity.signer.ui

import android.util.Log
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.*
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.TopBar
import io.parity.signer.domain.MainFlowViewModel
import io.parity.signer.domain.MainFlowViewModelFactory
import io.parity.signer.domain.NavigationMigrations
import io.parity.signer.domain.findActivity
import io.parity.signer.screens.onboarding.UnlockAppAuthScreen
import io.parity.signer.screens.onboarding.WaitingScreen
import io.parity.signer.ui.rustnavigationselectors.*


fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		val mainFlowViewModel: MainFlowViewModel = viewModel(
			factory = MainFlowViewModelFactory(
				appContext = LocalContext.current.applicationContext,
				activity = LocalContext.current.findActivity() as FragmentActivity
			)
		)

		val authenticated = mainFlowViewModel.authenticated.collectAsState()

		BackHandler {
			mainFlowViewModel.navigator.backAction()
		}

		if (authenticated.value) {
			SignerMainSubgraph(mainFlowViewModel)
		} else {
			UnlockAppAuthScreen { mainFlowViewModel.totalRefresh() }
		}
		LaunchedEffect(Unit) {
			Log.d(NAVIGATION_TAG, "main rust-handled screen navigation subgraph opened")
		}
	}
}


@Composable
fun SignerMainSubgraph(mainFlowViewModel: MainFlowViewModel) {

	val actionResultState = mainFlowViewModel.actionResult.collectAsState()
	val shieldAlert = mainFlowViewModel.networkState.collectAsState()
	val localNavAction = mainFlowViewModel.localNavAction.collectAsState()

	val actionResult = actionResultState.value

	if (actionResult == null) {
		WaitingScreen()
	} else {
		// Structure to contain all app
		Box {
			//screens before redesign
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
							mainFlowViewModel = mainFlowViewModel,
							actionResult = actionResult,
							networkState = shieldAlert,
						)
					}
				},
				bottomBar = {
					if (NavigationMigrations.shouldShowBar(
							localNavAction = localNavAction.value,
							globalNavAction = actionResult,
						)
						&& actionResult.footer
					) {
						BottomBar(mainFlowViewModel = mainFlowViewModel)
					}
				},
			) { innerPadding ->
				Box(modifier = Modifier.padding(innerPadding)) {
					ScreenSelector(
						screenData = actionResult.screenData,
						networkState = shieldAlert,
						navigate = mainFlowViewModel.navigator::navigate,
						mainFlowViewModel = mainFlowViewModel
					)
					ModalSelector(
						modalData = actionResult.modalData,
						localNavAction = localNavAction.value,
						networkState = shieldAlert,
						navigate = mainFlowViewModel.navigator::navigate,
						mainFlowViewModel = mainFlowViewModel,
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
					networkState = shieldAlert,
					mainFlowViewModel = mainFlowViewModel
				)
				BottomSheetSelector(
					modalData = actionResult.modalData,
					localNavAction = localNavAction.value,
					networkState = shieldAlert,
					mainFlowViewModel = mainFlowViewModel,
					navigator = mainFlowViewModel.navigator,
				)
				AlertSelector(
					alert = actionResult.alertData,
					networkState = shieldAlert,
					navigate = mainFlowViewModel.navigator::navigate,
					acknowledgeWarning = mainFlowViewModel::acknowledgeWarning
				)
			}
		}
	}
}


