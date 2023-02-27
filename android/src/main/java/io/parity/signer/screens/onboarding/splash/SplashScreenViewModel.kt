package io.parity.signer.screens.onboarding.splash

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.onboarding.termsconsent.OnBoardingViewModel


class SplashScreenViewModel : ViewModel() {

	fun shouldShowOnboarding(context: Context): Boolean {
		return OnBoardingViewModel.shouldShowOnboarding(context)
	}

	fun isShouldShowAirgap(): Boolean {
		return ServiceLocator.networkExposedStateKeeper.airGapModeState.value != NetworkState.Active
	}

}
