package io.parity.signer.ui.rootnavigation

import androidx.compose.animation.AnimatedContentTransitionScope
import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import io.parity.signer.domain.findActivity
import io.parity.signer.screens.error.errorStateDestination
import io.parity.signer.screens.initial.eachstartchecks.enableEachStartAppFlow
import io.parity.signer.screens.initial.firstTimeOnly.firstTimeOnlyOnboarding
import io.parity.signer.screens.initial.splash.splashScreen
import kotlinx.coroutines.delay


@Composable
fun RootNavigationGraph(
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
			startDestination = startDestination,
			enterTransition = {
				slideIntoContainer(
					AnimatedContentTransitionScope.SlideDirection.Start,
					animationSpec = tween()
				)
			},
			exitTransition = {
				ExitTransition.None
			},
			popEnterTransition = {
				EnterTransition.None
			},
			popExitTransition = {
				slideOutOfContainer(
					AnimatedContentTransitionScope.SlideDirection.End,
					animationSpec = tween()
				)
			}
		) {
			splashScreen(navController)
			firstTimeOnlyOnboarding(
				routePath = MainGraphRoutes.firstTimeOnboarding,
				navController = navController,
			)
			enableEachStartAppFlow(navController)
			mainSignerAppFlow(navController)
			errorStateDestination( //same destination as for core graph - so we can show error outside of unlock screen
				navController = navController,
			)
		}
	}
}

object MainGraphRoutes {
	const val splashRoute = "navigation_point_splash"
	const val firstTimeOnboarding =
		"navigation_point_once_onboarding"
	const val eachTimeOnboardingRoute = "navigation_point_enable_airgap"
	const val mainScreenRoute = "navigation_main_screen"
}



