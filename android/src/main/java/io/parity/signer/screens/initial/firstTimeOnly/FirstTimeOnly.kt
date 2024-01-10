package io.parity.signer.screens.initial.firstTimeOnly

import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import androidx.navigation.navigation
import io.parity.signer.screens.initial.eachstartchecks.osversion.OutdatedOsVersionScreen
import io.parity.signer.screens.initial.eachstartchecks.osversion.OutdatedOsVersionViewModel
import io.parity.signer.screens.initial.termsconsent.TermsConsentScreenFull
import io.parity.signer.ui.rootnavigation.MainGraphRoutes


fun NavGraphBuilder.firstTimeOnlyOnboarding(
	routePath: String,
	navController: NavHostController,
) {
	navigation(
		route = routePath,
		startDestination = FirstTimeOnboarding.osVersionNotification,
	) {
		composable(route = FirstTimeOnboarding.osVersionNotification) {
			val osVersionVM: OutdatedOsVersionViewModel = viewModel()
			if (osVersionVM.isShouldShow()) {
				OutdatedOsVersionScreen(
					exposedVulnarabilities = osVersionVM.getVulnerabilities(),
					onProceed = {
						navController.navigate(FirstTimeOnboarding.termsConsentRoute) {
							popUpTo(0)
						}
					}
				)
			} else {
				navController.navigate(FirstTimeOnboarding.termsConsentRoute) {
					popUpTo(0)
				}
			}
		}
		composable(route = FirstTimeOnboarding.termsConsentRoute) {
			TermsConsentScreenFull(
				navigateNextScreen = {
					navController.navigate(MainGraphRoutes.eachTimeOnboardingRoute) {
						popUpTo(0)
					}
				},
			)
		}
	}
}

private object FirstTimeOnboarding {
	const val osVersionNotification = "navigation_point_"

	//	const val onboardingExplanationRoute = "navigation_onboarding_explanation"
	const val termsConsentRoute = "navigation_point_terms_consent"
}
