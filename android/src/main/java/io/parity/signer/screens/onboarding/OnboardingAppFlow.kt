package io.parity.signer.screens.onboarding

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.components.BigButton
import io.parity.signer.models.AlertState
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.theme.Text600



const val onboardingRoute = "navigation_point_onboarding" //todo onboarding remove this part

fun NavGraphBuilder.onboardingAppFlow() {
	composable(route = onboardingRoute) {
//		OnboardingAppFlow()
	}
}

//@Composable
//private fun OnboardingAppFlow() {
//	//todo onboarding this is old reference implementation, break it in a few screens and remove this file
//	val onboardingModel: OnboardingViewModel = viewModel()
//
//	val onBoardingDone = onboardingModel.onBoardingDone.collectAsState()
//
//	when (onBoardingDone.value) {
//		OnboardingWasShown.No -> {
//			if (shieldAlert.value == AlertState.None) {
//				Scaffold(
//					modifier = Modifier
//						.navigationBarsPadding()
//						.captionBarPadding()
//						.statusBarsPadding(),
//				) { padding ->
//					TermsConsentScreen(
//						signerDataModel::onBoard,
//						modifier = Modifier.padding(padding)
//					)
//				}
//			} else {
//				EnableAirgapScreen()
//			}
//		}
//		OnboardingWasShown.Unknown -> {
//			if (authenticated.value) {
//				WaitingScreen()
//			} else {
//				Column(verticalArrangement = Arrangement.Center) {
//					Spacer(Modifier.weight(0.5f))
//					BigButton(
//						text = stringResource(R.string.unlock_app_button),
//						action = {
//							signerDataModel.lateInit()
//						}
//					)
//					Spacer(Modifier.weight(0.5f))
//				}
//			}
//		}
//		OnboardingWasShown.Yes -> TODO()
//	}
//}


