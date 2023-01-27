package io.parity.signer

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.*
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.components.BigButton
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.TopBar
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.NavigationMigrations
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.navigationselectors.*


const val mainScreenRoute = "navigation_main_screen"

fun NavGraphBuilder.mainSignerSubgraph() {
	composable(route = mainScreenRoute) {
		SignerMainSubgraph()
	}
}

@Composable
fun SignerMainSubgraph() {
	val signerDataModel: SignerDataModel = viewModel()
	val authenticated = signerDataModel.authenticated.collectAsState()
	val actionResult = signerDataModel.actionResult.collectAsState()
	val shieldAlert = signerDataModel.alertState.collectAsState()
	val localNavAction = signerDataModel.localNavAction.collectAsState()

	if (authenticated.value) {
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
							alertState = shieldAlert,
						)
					}
				},
				bottomBar = {
					if (NavigationMigrations.shouldShowBar(
							localNavAction = localNavAction.value,
							globalNavAction = actionResult.value,)
						&& actionResult.value.footer
					) {
						BottomBar(signerDataModel = signerDataModel)
					}
				},
			) { innerPadding ->
				Box(modifier = Modifier.padding(innerPadding)) {
					ScreenSelector(
						screenData = actionResult.value.screenData,
						alertState = shieldAlert,
						navigate = signerDataModel.navigator::navigate,
						signerDataModel = signerDataModel
					)
					ModalSelector(
						modalData = actionResult.value.modalData,
						localNavAction = localNavAction.value,
						alertState = shieldAlert,
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
					alertState = shieldAlert,
					signerDataModel = signerDataModel
				)
				BottomSheetSelector(
					modalData = actionResult.value.modalData,
					localNavAction = localNavAction.value,
					alertState = shieldAlert,
					signerDataModel = signerDataModel,
					navigator = signerDataModel.navigator,
				)
				AlertSelector(
					alert = actionResult.value.alertData,
					alertState = shieldAlert,
					navigate = signerDataModel.navigator::navigate,
					acknowledgeWarning = signerDataModel::acknowledgeWarning
				)
			}
		}
	} else {
		Column(verticalArrangement = Arrangement.Center) {
			Spacer(Modifier.weight(0.5f))
			BigButton(
				text = stringResource(R.string.unlock_app_button),
				action = {
					ServiceLocator.authentication.authenticate(signerDataModel.activity) {
						signerDataModel.totalRefresh()
					}
				}
			)
			Spacer(Modifier.weight(0.5f))
		}
	}
}
