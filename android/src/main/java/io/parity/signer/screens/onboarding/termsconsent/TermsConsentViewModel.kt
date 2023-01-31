package io.parity.signer.screens.onboarding.termsconsent

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed
import io.parity.signer.uniffi.historyInitHistoryWithCert


class TermsConsentViewModel : ViewModel() {

	fun shouldProceedRightAway(context: Context): Boolean {
		return context.isDbCreatedAndOnboardingPassed()
	}

	fun onBoard() {
		wipe()
		copyAsset("")
		totalRefresh()
		historyInitHistoryWithCert()
	}
}
