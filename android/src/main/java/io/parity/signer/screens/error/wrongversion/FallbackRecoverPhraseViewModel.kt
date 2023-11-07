package io.parity.signer.screens.error.wrongversion

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator


class FallbackRecoverPhraseViewModel: ViewModel() {
	private val seedStorage = ServiceLocator.seedStorage

	fun getSeedsList(): List<String> {
		return seedStorage.lastKnownSeedNames.value.toList()
	}


}
