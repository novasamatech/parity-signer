package io.parity.signer.screens.settings.verifiercert

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.VerifierDetailsModel
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.usecases.ResetUseCase


class VerifierCertViewModel: ViewModel() {
	private val resetUseCase = ResetUseCase()
	private val uniffiInteractor = ServiceLocator.uniffiInteractor

	suspend fun getVerifierCertModel(): UniffiResult<VerifierDetailsModel> {
		return uniffiInteractor.getVerifierDetails()
	}

	fun wipeWithGeneralCertificate(onAfterAction: Callback) {
		resetUseCase.wipeNoGeneralCertWithAuth(onAfterAction)
	}
}
