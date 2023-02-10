package io.parity.signer.ui

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import io.parity.signer.domain.findActivity
import io.parity.signer.screens.onboarding.eachstartchecks.airgap.enableAirgapAppFlow
import io.parity.signer.screens.onboarding.initialUnlockAppScreenFlow
import io.parity.signer.screens.onboarding.splash.splashScreen
import io.parity.signer.screens.onboarding.termsconsent.termsConsentAppFlow
import kotlinx.coroutines.delay


@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerMainNavigationGraph(
	navController: NavHostController,
	startDestination: String = MainGraphRoutes.splashRoute,
) {
	val context = LocalContext.current
	LaunchedEffect(key1 = Unit, block = {
		val window = context.findActivity()!!.window
		/**
		 * Due to this bug https://issuetracker.google.com/issues/227926002 when using
		 * splash screen API on some devices with Compose Jetpack Navigation leads to a blank
		 * screen. However wrapping the [NavHost] in a [Scaffold] or setting background solves this issue,
		 * so here it is. And setting backgroudn back below
		 */
		delay(50)
		window.setBackgroundDrawableResource(android.R.color.transparent)
	})
	Box(
		modifier = Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
	) {
		NavHost(
			navController = navController,
			startDestination = startDestination
		) {
			splashScreen(navController)
			termsConsentAppFlow(navController)
			enableAirgapAppFlow(navController)
			initialUnlockAppScreenFlow(navController)
			mainSignerAppFlow(navController)
		}
	}
}

const val NAVIGATION_TAG = "navigation"

object MainGraphRoutes {
	const val splashRoute = "navigation_point_splash"
	const val onboardingRoute = "navigation_point_terms_consent"
	const val enableAirgapRoute = "navigation_point_enable_airgap"
	const val initialUnlockRoute = "navigation_point_initial_unlock"
	const val mainScreenRoute = "navigation_main_screen"
}
