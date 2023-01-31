package io.parity.signer.screens.onboarding.termsconsent

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed
import io.parity.signer.uniffi.historyInitHistoryWithCert
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow


class OnBoardingViewModel : ViewModel() {

	private val _isFinishedOnboarding: MutableStateFlow<Boolean> =
		MutableStateFlow(false)
	val isFinishedOnboarding: StateFlow<Boolean> =
		_isFinishedOnboarding.asStateFlow()

	fun checkShouldProceed(context: Context) {
		_isFinishedOnboarding.value = context.isDbCreatedAndOnboardingPassed()
	}
}
