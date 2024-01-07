package io.parity.signer.screens.initial.termsconsent

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed


class OnBoardingViewModel : ViewModel() {


	companion object {
		fun shouldShowSingleRunChecks(context: Context): Boolean {
			return !context.isDbCreatedAndOnboardingPassed()
		}
	}
}
