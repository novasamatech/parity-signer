package io.parity.signer.screens.keysets.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator


class NewKeysetNameViewModel: ViewModel() {
	val seedNames =
		ServiceLocator.seedStorage.lastKnownSeedNames
}
