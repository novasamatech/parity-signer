package io.parity.signer.ui

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
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NavigationMigrations
import io.parity.signer.domain.MainFlowViewModel
import io.parity.signer.ui.rustnavigationselectors.*



fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		SignerMainSubgraph()

		LaunchedEffect(Unit) {
			ServiceLocator.authentication.auth.collect {authenticated ->
				if (!authenticated) {
					globalNavController.navigate(MainGraphRoutes.unlockAppScreenRoute)
				}
			}
		}
	}
}


@Composable
fun SignerMainSubgraph() {
	val mainFlowViewModel: MainFlowViewModel = viewModel()
	val actionResult = mainFlowViewModel.actionResult.collectAsState()
	val shieldAlert = mainFlowViewModel.networkState.collectAsState()
	val localNavAction = mainFlowViewModel.localNavAction.collectAsState()

	BackHandler {
		mainFlowViewModel.navigator.backAction()
	}
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
						globalNavAction = actionResult.value,
					)
				) {
					TopBar(
						mainFlowViewModel = mainFlowViewModel,
						networkState = shieldAlert,
					)
				}
			},
			bottomBar = {
				if (NavigationMigrations.shouldShowBar(
						localNavAction = localNavAction.value,
						globalNavAction = actionResult.value,
					)
					&& actionResult.value.footer
				) {
					BottomBar(mainFlowViewModel = mainFlowViewModel)
				}
			},
		) { innerPadding ->
			Box(modifier = Modifier.padding(innerPadding)) {
				ScreenSelector(
					screenData = actionResult.value.screenData,
					networkState = shieldAlert,
					navigate = mainFlowViewModel.navigator::navigate,
					mainFlowViewModel = mainFlowViewModel
				)
				ModalSelector(
					modalData = actionResult.value.modalData,
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
				screenData = actionResult.value.screenData,
				localNavAction = localNavAction.value,
				networkState = shieldAlert,
				mainFlowViewModel = mainFlowViewModel
			)
			BottomSheetSelector(
				modalData = actionResult.value.modalData,
				localNavAction = localNavAction.value,
				networkState = shieldAlert,
				mainFlowViewModel = mainFlowViewModel,
				navigator = mainFlowViewModel.navigator,
			)
			AlertSelector(
				alert = actionResult.value.alertData,
				networkState = shieldAlert,
				navigate = mainFlowViewModel.navigator::navigate,
				acknowledgeWarning = mainFlowViewModel::acknowledgeWarning
			)
		}
	}
}
