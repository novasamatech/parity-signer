package io.parity.signer.screens.onboarding.eachstartchecks

import android.content.Context
import android.util.Log
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.screens.onboarding.eachstartchecks.airgap.EnableAirgapScreen
import io.parity.signer.screens.onboarding.eachstartchecks.screenlock.SetScreenLockScreen
import io.parity.signer.ui.MainGraphRoutes
import io.parity.signer.ui.NAVIGATION_TAG


fun NavGraphBuilder.enableEachStartAppFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.enableAirgapRoute) {
		val viewModel: EachStartViewModel = viewModel()
		val context: Context = LocalContext.current

		if (viewModel.checkIsAuthPossible(context)) {
			// next screen - airgap
			LaunchedEffect(viewModel) {
				Log.d(NAVIGATION_TAG, "airgap screen opened")
				viewModel.isFinished.collect {
					if (it) globalNavController.navigate(MainGraphRoutes.initialUnlockRoute) {
						popUpTo(0)
					}
				}
			}
			EnableAirgapScreen()
		} else {
			//first show enable screen lock if needed
			SetScreenLockScreen {
				//todo dmitry callback
			}
		}
	}
}
