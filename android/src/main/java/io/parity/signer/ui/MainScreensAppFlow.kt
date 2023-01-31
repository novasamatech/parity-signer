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
import io.parity.signer.domain.SignerDataModel
import io.parity.signer.screens.onboarding.unlockAppScreenRoute
import io.parity.signer.ui.rustnavigationselectors.*


const val mainScreenRoute = "navigation_main_screen"

fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = mainScreenRoute) {
		SignerMainSubgraph()
		LaunchedEffect(Unit) {
			ServiceLocator.authentication.auth.collect {authenticated ->
				if (!authenticated) {
					globalNavController.navigate(unlockAppScreenRoute)
				}
			}
		}
	}
}


@Composable
fun SignerMainSubgraph() {
	val signerDataModel: SignerDataModel = viewModel()
	val actionResult = signerDataModel.actionResult.collectAsState()
	val shieldAlert = signerDataModel.networkState.collectAsState()
	val localNavAction = signerDataModel.localNavAction.collectAsState()

	BackHandler {
		signerDataModel.navigator.backAction()
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
						signerDataModel = signerDataModel,
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
					BottomBar(signerDataModel = signerDataModel)
				}
			},
		) { innerPadding ->
			Box(modifier = Modifier.padding(innerPadding)) {
				ScreenSelector(
					screenData = actionResult.value.screenData,
					networkState = shieldAlert,
					navigate = signerDataModel.navigator::navigate,
					signerDataModel = signerDataModel
				)
				ModalSelector(
					modalData = actionResult.value.modalData,
					localNavAction = localNavAction.value,
					networkState = shieldAlert,
					navigate = signerDataModel.navigator::navigate,
					signerDataModel = signerDataModel,
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
				signerDataModel = signerDataModel
			)
			BottomSheetSelector(
				modalData = actionResult.value.modalData,
				localNavAction = localNavAction.value,
				networkState = shieldAlert,
				signerDataModel = signerDataModel,
				navigator = signerDataModel.navigator,
			)
			AlertSelector(
				alert = actionResult.value.alertData,
				networkState = shieldAlert,
				navigate = signerDataModel.navigator::navigate,
				acknowledgeWarning = signerDataModel::acknowledgeWarning
			)
		}
	}
}
