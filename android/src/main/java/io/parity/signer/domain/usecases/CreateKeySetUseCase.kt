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
	): Unit {
		val repository = ServiceLocator.activityScope!!.seedRepository
		repository.addSeed(
			seedName = seedName,
			seedPhrase = seedPhrase,
			networksKeys = networksKeys,
		)
	}
}
