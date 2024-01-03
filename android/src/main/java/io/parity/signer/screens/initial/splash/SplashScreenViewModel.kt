package io.parity.signer.screens.initial.splash

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.initial.termsconsent.OnBoardingViewModel


class SplashScreenViewModel : ViewModel() {

	fun shouldShowSingleRunChecks(context: Context): Boolean {
		return OnBoardingViewModel.shouldShowSingleRunChecks(context)
	}

	fun isShouldShowAirgap(): Boolean {
		return ServiceLocator.networkExposedStateKeeper.airGapModeState.value != NetworkState.Active
	}

}
