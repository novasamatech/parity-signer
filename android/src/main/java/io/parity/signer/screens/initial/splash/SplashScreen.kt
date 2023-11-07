package io.parity.signer.screens.initial.splash

import android.content.res.Configuration
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.screens.initial.WaitingScreen
import io.parity.signer.ui.rootnavigation.MainGraphRoutes


/**
 * To avoid glitches there is a screen that decides what is the initial screen should be
 */
fun NavGraphBuilder.splashScreen(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.splashRoute) {
		AnimatedVisibility(visible = true) {
			WaitingScreen()
		}

		val viewModel: SplashScreenViewModel = viewModel()
		val context = LocalContext.current
		LaunchedEffect(Unit) {
			if (viewModel.shouldShowOnboarding(context)) {
				globalNavController.navigate(MainGraphRoutes.firstTimeOnboarding) {
					popUpTo(0)
				}
			} else if (viewModel.isShouldShowAirgap()) {
				globalNavController.navigate(MainGraphRoutes.eachTimeOnboardingRoute) {
					popUpTo(0)
				}
			} else {
				globalNavController.navigate(MainGraphRoutes.mainScreenRoute) {
					popUpTo(0)
				}
			}
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewSplash() {
	AnimatedVisibility(visible = true) {
		WaitingScreen()
	}
}
