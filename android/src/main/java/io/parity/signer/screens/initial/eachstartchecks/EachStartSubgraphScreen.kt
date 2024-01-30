package io.parity.signer.screens.initial.eachstartchecks

import android.content.Context
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed
import io.parity.signer.screens.initial.eachstartchecks.airgap.AirgapScreen
import io.parity.signer.screens.initial.eachstartchecks.rootcheck.RootExposedScreen
import io.parity.signer.screens.initial.eachstartchecks.screenlock.SetScreenLockScreen
import io.parity.signer.ui.rootnavigation.MainGraphRoutes


fun NavGraphBuilder.enableEachStartAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.eachTimeOnboardingRoute) {
		val viewModel: EachStartViewModel = viewModel()
		val context: Context = LocalContext.current

		val goToNextFlow: Callback = {
			globalNavController.navigate(MainGraphRoutes.mainScreenRoute) {
				popUpTo(0)
			}
		}

		val state = remember {
			mutableStateOf(
				if (viewModel.isDeviceRooted()) {
					EachStartSubgraphScreenSteps.ROOT_EXPOSED
				} else if (!viewModel.isAuthPossible(context)) {
					EachStartSubgraphScreenSteps.SET_SCREEN_LOCK_BLOCKER
				} else if (viewModel.networkState.value == NetworkState.Active || !context.isDbCreatedAndOnboardingPassed()){
					EachStartSubgraphScreenSteps.AIR_GAP
				} else {
					goToNextFlow()
				}
			)
		}

		Box(modifier = Modifier
				.navigationBarsPadding()
				.captionBarPadding()
				.statusBarsPadding()
		) {
			when (state.value) {
				EachStartSubgraphScreenSteps.ROOT_EXPOSED -> {
					RootExposedScreen()
				}
				EachStartSubgraphScreenSteps.SET_SCREEN_LOCK_BLOCKER -> {
					//first show enable screen lock if needed
					val lifecycleOwner: LifecycleOwner = LocalLifecycleOwner.current
					DisposableEffect(this) {
						val observer = LifecycleEventObserver { _, event ->
							if (event.targetState == Lifecycle.State.RESUMED) {
								if (viewModel.isAuthPossible(context)) {
									state.value = EachStartSubgraphScreenSteps.AIR_GAP
								}
							}
						}
						lifecycleOwner.lifecycle.addObserver(observer)
						onDispose {
							lifecycleOwner.lifecycle.removeObserver(observer)
						}
					}
					SetScreenLockScreen()
				}
				EachStartSubgraphScreenSteps.AIR_GAP -> {
					AirgapScreen(isInitialOnboarding = true) {
						//go to next screen
						goToNextFlow()
					}
				}
			}
		}
	}
}

private enum class EachStartSubgraphScreenSteps { ROOT_EXPOSED, AIR_GAP, SET_SCREEN_LOCK_BLOCKER }
