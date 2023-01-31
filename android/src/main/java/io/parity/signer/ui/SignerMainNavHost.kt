package io.parity.signer.ui

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import io.parity.signer.screens.onboarding.enableAirgapAppFlow
import io.parity.signer.screens.onboarding.termsconsent.termsConsentAppFlow
import io.parity.signer.screens.onboarding.unlockAppScreenRoute
import io.parity.signer.screens.onboarding.unlockAppScreenFlow


@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerNavHost(
	navController: NavHostController,
	startDestination: String = unlockAppScreenRoute,
) {
	NavHost(navController = navController, startDestination = startDestination) {
//		onboardingAppFlow() todo onboarding
		termsConsentAppFlow(navController)
		enableAirgapAppFlow(navController)
		unlockAppScreenFlow(navController)
		mainSignerAppFlow(navController)
	}
}
