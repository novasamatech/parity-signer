package io.parity.signer.ui

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import io.parity.signer.screens.onboarding.airgap.enableAirgapAppFlow
import io.parity.signer.screens.onboarding.initialUnlockAppScreenFlow
import io.parity.signer.screens.onboarding.splash.splashScreen
import io.parity.signer.screens.onboarding.termsconsent.termsConsentAppFlow


@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerMainNavigationGraph(
	navController: NavHostController,
	startDestination: String = MainGraphRoutes.splashRoute,
) {
	NavHost(navController = navController, startDestination = startDestination) {
		splashScreen(navController)
		termsConsentAppFlow(navController)
		enableAirgapAppFlow(navController)
		initialUnlockAppScreenFlow(navController)
		mainSignerAppFlow(navController)
	}
}

object MainGraphRoutes {
	const val splashRoute = "navigation_point_splash"
	const val onboardingRoute = "navigation_point_terms_consent"
	const val enableAirgapRoute = "navigation_point_enable_airgap"
	const val initialUnlockRoute = "navigation_point_initial_unlock"
	const val mainScreenRoute = "navigation_main_screen"
}
