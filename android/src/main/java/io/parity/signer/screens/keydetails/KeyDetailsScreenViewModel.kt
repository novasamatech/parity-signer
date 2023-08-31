package io.parity.signer.screens.keydetails

import io.parity.signer.domain.usecases.KeyPublicModelUseCase


class KeyDetailsScreenViewModel {
	private val useCase = KeyPublicModelUseCase()

	suspend fun fetchModel() = useCase.getPublicKeyModel()
}
