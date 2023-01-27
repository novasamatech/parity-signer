package io.parity.signer.screens.onboarding

import androidx.lifecycle.ViewModel
import io.parity.signer.ui.navigationselectors.OnboardingWasShown
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow


class OnboardingViewModel: ViewModel() {
	// Internal model values
	private val _onBoardingDone = MutableStateFlow(OnboardingWasShown.InProgress)
	val onBoardingDone: StateFlow<OnboardingWasShown> = _onBoardingDone
}
