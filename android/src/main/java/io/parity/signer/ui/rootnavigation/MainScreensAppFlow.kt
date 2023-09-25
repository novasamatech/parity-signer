package io.parity.signer.ui.rootnavigation

import android.util.Log
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.domain.MainFlowViewModel
import io.parity.signer.screens.initial.UnlockAppAuthScreen
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


fun NavGraphBuilder.mainSignerAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.mainScreenRoute) {
		val mainFlowViewModel: MainFlowViewModel = viewModel()

		val authenticated =
			mainFlowViewModel.authenticated.collectAsStateWithLifecycle()

		if (authenticated.value) {
			// Structure to contain all app
			Box(
				modifier = Modifier
					.navigationBarsPadding()
					.captionBarPadding(),
			) {
				CoreUnlockedNavSubgraph()
			}
		} else {
			UnlockAppAuthScreen(onUnlockClicked = mainFlowViewModel::onUnlockClicked)
		}
		LaunchedEffect(Unit) {
			Log.d(
				NAVIGATION_TAG,
				"main rust-handled screen navigation subgraph opened"
			)
		}
	}
}



