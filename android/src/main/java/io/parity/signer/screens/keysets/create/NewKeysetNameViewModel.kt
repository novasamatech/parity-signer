package io.parity.signer.screens.keysets.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.UniffiResult
import kotlinx.coroutines.runBlocking


class NewKeysetNameViewModel : ViewModel() {

	private val uniffi = ServiceLocator.uniffiInteractor

	val seedNames =
		ServiceLocator.seedStorage.lastKnownSeedNames

	fun createNewSeedPhrase(): UniffiResult<String> {
		return runBlocking {
			uniffi.createNewSeedPhrase()
		}
	}
}
