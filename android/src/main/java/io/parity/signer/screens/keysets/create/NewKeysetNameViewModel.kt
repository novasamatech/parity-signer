package io.parity.signer.screens.keysets.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator


class NewKeysetNameViewModel : ViewModel() {

	private val uniffi = ServiceLocator.uniffiInteractor

	val seedNames =
		ServiceLocator.seedStorage.lastKnownSeedNames

	fun getSeedPhrase(): String {
		return "ababab"//todo dmitry get it from uniffi
	}
}
