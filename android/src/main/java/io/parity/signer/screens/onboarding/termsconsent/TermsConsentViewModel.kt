package io.parity.signer.screens.onboarding.termsconsent

import androidx.lifecycle.ViewModel
import io.parity.signer.uniffi.historyInitHistoryWithCert


class TermsConsentViewModel: ViewModel() {
	fun onBoard() {
		wipe()
		copyAsset("")
		totalRefresh()
		historyInitHistoryWithCert()
	}
}
