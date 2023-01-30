package io.parity.signer.screens.onboarding

import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable


const val onboardingRoute = "navigation_point_onboarding"

fun NavGraphBuilder.onboardingSubgraph() {
	composable(route = onboardingRoute) {
		OnboardingFlowSubgraph()
	}
}



