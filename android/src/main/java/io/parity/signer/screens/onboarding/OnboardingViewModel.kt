package io.parity.signer.screens.onboarding

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow


class OnboardingViewModel: ViewModel() {
	// Internal model values
	private val _onBoardingDone = MutableStateFlow(OnboardingWasShown.Unknown)
	val onBoardingDone: StateFlow<OnboardingWasShown> = _onBoardingDone
}


enum class OnboardingWasShown {
	Unknown,
	No,
	Yes;
}
