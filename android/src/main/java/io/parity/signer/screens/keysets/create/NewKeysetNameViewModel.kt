package io.parity.signer.screens.keysets.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.mapError
import kotlinx.coroutines.runBlocking


class NewKeysetNameViewModel : ViewModel() {

	private val uniffi = ServiceLocator.uniffiInteractor

	val seedNames =
		ServiceLocator.seedStorage.lastKnownSeedNames

	fun createNewSeedPhrase(): String? {
		return runBlocking {
			uniffi.createNewSeedPhrase().mapError()
		}
	}
}
