package io.parity.signer.ui

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import io.parity.signer.screens.onboarding.airgap.enableAirgapAppFlow
import io.parity.signer.screens.onboarding.termsconsent.termsConsentAppFlow
import io.parity.signer.screens.onboarding.unlockAppScreenFlow


@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerMainNavigationGraph(
	navController: NavHostController,
	startDestination: String = MainGraphRoutes.unlockAppScreenRoute,
) {
	NavHost(navController = navController, startDestination = startDestination) {
		termsConsentAppFlow(navController)
		enableAirgapAppFlow(navController)
		unlockAppScreenFlow(navController)
		mainSignerAppFlow(navController)
	}
}

object MainGraphRoutes {
	const val termsConsentRoute = "navigation_point_terms_consent"
	const val enableAirgapRoute = "navigation_point_enable_airgap"
	const val unlockAppScreenRoute = "navigation_point_unlock_app"
	const val mainScreenRoute = "navigation_main_screen"
}
