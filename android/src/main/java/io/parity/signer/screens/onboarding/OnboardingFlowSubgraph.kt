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
import io.parity.signer.components.BigButton
import io.parity.signer.models.AlertState
import io.parity.signer.screens.TermsConsentScreen
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.theme.Text600


@Composable
internal fun OnboardingFlowSubgraph() {
	//todo onboarding this is old reference implementation, break it in a few
	val onboardingModel: OnboardingViewModel = viewModel()

	val onBoardingDone = onboardingModel.onBoardingDone.collectAsState()

	when (onBoardingDone.value) {
		OnboardingWasShown.No -> {
			if (shieldAlert.value == AlertState.None) {
				Scaffold(
					modifier = Modifier
						.navigationBarsPadding()
						.captionBarPadding()
						.statusBarsPadding(),
				) { padding ->
					TermsConsentScreen(
						signerDataModel::onBoard,
						modifier = Modifier.padding(padding)
					)
				}
			} else {
				EnableAirgapScreen()
			}
		}
		OnboardingWasShown.Unknown -> {
			if (authenticated.value) {
				WaitingScreen()
			} else {
				Column(verticalArrangement = Arrangement.Center) {
					Spacer(Modifier.weight(0.5f))
					BigButton(
						text = stringResource(R.string.unlock_app_button),
						action = {
							signerDataModel.lateInit()
						}
					)
					Spacer(Modifier.weight(0.5f))
				}
			}
		}
		OnboardingWasShown.Yes -> TODO()
	}
}

@Composable
fun EnableAirgapScreen() {
	Box(
		contentAlignment = Alignment.Center,
		modifier = Modifier
			.padding(12.dp)
			.fillMaxSize(1f),
	) {
		Text(
			text = stringResource(R.string.enable_airplane_mode_error),
			color = MaterialTheme.colors.Text600
		)
	}
}
