package io.parity.signer.domain.usecases

import io.parity.signer.dependencygraph.ServiceLocator

/**
 * Creates key set
 */
class CreateKeySetUseCase() {

	suspend fun createKeySetWithNetworks(
		seedName: String,
		seedPhrase: String,
		networksKeys: List<String>,
	): Boolean {
		val repository = ServiceLocator.activityScope!!.seedRepository
		return repository.addSeed( //todo dmitry validate check this method.
			seedName = seedName,
			seedPhrase = seedPhrase,
			networksKeys = networksKeys,
		)
	}
}
