package io.parity.signer.ui.rootnavigation

import android.os.Build
import timber.log.Timber
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.MainFlowViewModel
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.addVaultLogger
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.initial.UnlockAppAuthScreen
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch


fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		val mainFlowViewModel: MainFlowViewModel = viewModel()

		val authenticated =
			mainFlowViewModel.authenticated.collectAsStateWithLifecycle()

		val unlockedNavController =
			rememberNavController().apply { addVaultLogger() }

		val networkState =
			mainFlowViewModel.networkState.collectAsStateWithLifecycle()

		if (authenticated.value) {
			// Structure to contain all app
			Box(
				modifier = Modifier
					.navigationBarsPadding()
					.captionBarPadding(),
			) {
				CoreUnlockedNavSubgraph(unlockedNavController)
			}

			//check for network and navigate to blocker screen if needed
			when (networkState.value) {
				NetworkState.Active -> {
					if (unlockedNavController.currentDestination
							?.route != CoreUnlockedNavSubgraph.airgapBreached
					) {
						unlockedNavController.navigate(CoreUnlockedNavSubgraph.airgapBreached)
					}
				}
				else -> {}
			}
		} else {
			UnlockAppAuthScreen(onUnlockClicked = {
				mainFlowViewModel.viewModelScope.launch {
					mainFlowViewModel.onUnlockClicked()
						.handleErrorAppState(globalNavController)
				}
			})
		}
	}
}



