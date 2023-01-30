package io.parity.signer.screens.onboarding

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable


const val onboardingRoute = "navigation_point_onboarding" //todo onboarding remove this part

fun NavGraphBuilder.onboardingSubgraph() {
	composable(route = unlockAppScreenRoute) {
		OnboardingFlowSubgraph()
	}
}



