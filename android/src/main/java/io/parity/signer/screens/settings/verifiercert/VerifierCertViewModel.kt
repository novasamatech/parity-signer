package io.parity.signer.screens.settings.verifiercert

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.VerifierDetailsModel
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.screens.settings.networks.list.NetworksListModel


class VerifierCertViewModel: ViewModel() {
	private val resetUseCase = ResetUseCase()
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	suspend fun getVerifierCertModel(): UniffiResult<VerifierDetailsModel> {
		return when (val result = uniffiInteractor.getAllNetworks()) {
			is UniffiResult.Error -> UniffiResult.Error(result.error)
			is UniffiResult.Success -> UniffiResult.Success(NetworksListModel(result.result))
		}
	}

	fun wipeWithGeneralCertificate(onAfterAction: Callback) {
		resetUseCase.wipeNoGeneralCertWithAuth()
	}
}
