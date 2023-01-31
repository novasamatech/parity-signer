package io.parity.signer.screens.onboarding.termsconsent

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed


class OnBoardingViewModel : ViewModel() {

//	private val _isFinishedOnboarding: MutableStateFlow<Boolean> =
//		MutableStateFlow(false)
//	val isFinishedOnboarding: StateFlow<Boolean> =
//		_isFinishedOnboarding.asStateFlow()


	companion object {
		fun shouldShowOnboarding(context: Context): Boolean {
			return !context.isDbCreatedAndOnboardingPassed()
		}
	}
}
