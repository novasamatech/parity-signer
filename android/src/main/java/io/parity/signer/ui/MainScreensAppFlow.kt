package io.parity.signer.ui

import android.util.Log
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.*
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.TopBar
import io.parity.signer.domain.NavigationMigrations
import io.parity.signer.domain.SignerMainViewModel
import io.parity.signer.screens.onboarding.UnlockAppAuthScreen
import io.parity.signer.screens.onboarding.WaitingScreen
import io.parity.signer.ui.rustnavigationselectors.*


fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		val signerMainViewModel: SignerMainViewModel = viewModel()

		val authenticated = signerMainViewModel.authenticated.collectAsState()

		BackHandler {
			signerMainViewModel.navigator.backAction()
		}

		if (authenticated.value) {
			SignerMainSubgraph(signerMainViewModel)
		} else {
			UnlockAppAuthScreen { signerMainViewModel.totalRefresh() }
		}
		LaunchedEffect(Unit) {
			Log.d(NAVIGATION_TAG, "main rust-handled screen navigation subgraph opened")
		}
	}
}


@Composable
fun SignerMainSubgraph(signerMainViewModel: SignerMainViewModel) {

	val actionResultState = signerMainViewModel.actionResult.collectAsState()
	val shieldAlert = signerMainViewModel.networkState.collectAsState()
	val localNavAction = signerMainViewModel.localNavAction.collectAsState()

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
							signerMainViewModel = signerMainViewModel,
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
						BottomBar(signerMainViewModel = signerMainViewModel)
					}
				},
			) { innerPadding ->
				Box(modifier = Modifier.padding(innerPadding)) {
					ScreenSelector(
						screenData = actionResult.screenData,
						networkState = shieldAlert,
						navigate = signerMainViewModel.navigator::navigate,
						signerMainViewModel = signerMainViewModel
					)
					ModalSelector(
						modalData = actionResult.modalData,
						localNavAction = localNavAction.value,
						networkState = shieldAlert,
						navigate = signerMainViewModel.navigator::navigate,
						signerMainViewModel = signerMainViewModel,
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
					signerMainViewModel = signerMainViewModel
				)
				BottomSheetSelector(
					modalData = actionResult.modalData,
					localNavAction = localNavAction.value,
					networkState = shieldAlert,
					signerMainViewModel = signerMainViewModel,
					navigator = signerMainViewModel.navigator,
				)
				AlertSelector(
					alert = actionResult.alertData,
					networkState = shieldAlert,
					navigate = signerMainViewModel.navigator::navigate,
					acknowledgeWarning = signerMainViewModel::acknowledgeWarning
				)
			}
		}
	}
}


