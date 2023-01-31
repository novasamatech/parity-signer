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
	startDestination: String = MainGraphRoutes.termsConsentRoute,
) {
	NavHost(navController = navController, startDestination = startDestination) {
		termsConsentAppFlow(navController)
		enableAirgapAppFlow(navController)
		mainSignerAppFlow(navController)
	}
}

object MainGraphRoutes {
	const val termsConsentRoute = "navigation_point_terms_consent"
	const val enableAirgapRoute = "navigation_point_enable_airgap"
	const val mainScreenRoute = "navigation_main_screen"
}
