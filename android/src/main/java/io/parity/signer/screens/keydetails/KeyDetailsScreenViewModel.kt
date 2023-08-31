package io.parity.signer.screens.keydetails

import io.parity.signer.dependencygraph.ServiceLocator


class KeyDetailsScreenViewModel {
	private val uniFfi = ServiceLocator.uniffiInteractor

	suspend fun fetchModel(keyAddr: String, networkSpecs: String) =
		uniFfi.getKeyPublicKey(keyAddr, networkSpecs)
}
