package io.parity.signer.domain.usecases

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.AuthOperationResult

/**
 * Creates key set
 */
class CreateKeySetUseCase() {

	suspend fun createKeySetWithNetworks(
		seedName: String,
		seedPhrase: String,
		networksKeys: List<String>,
	): AuthOperationResult {
		val repository = ServiceLocator.activityScope!!.seedRepository
		return repository.addSeed(
			seedName = seedName,
			seedPhrase = seedPhrase,
			networksKeys = networksKeys,
		)
	}
}
