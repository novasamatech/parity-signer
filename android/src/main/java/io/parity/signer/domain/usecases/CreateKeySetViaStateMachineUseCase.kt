package io.parity.signer.domain.usecases

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.tryCreateAddress

/**
 * Creates key set
 */
class CreateKeySetViaStateMachineUseCase() {

	suspend fun createKeySetWithNetworks(
		seedName: String,
		seedPhrase: String,
		networksKeys: Set<NetworkModel>,
		navigator: Navigator
	): Unit {
		val repository = ServiceLocator.activityScope!!.seedRepository
		repository.addSeed(
			seedName = seedName,
			seedPhrase = seedPhrase,
			navigator = navigator,
			isOptionalAuth = false
		)
		networksKeys.forEach { networkKey ->
			try {
				tryCreateAddress(
					seedName = seedName, seedPhrase = seedPhrase,
					path = networkKey.pathId, network = networkKey.key,
				)
			} catch (e: Exception) {
				submitErrorState("can't create network key for new keyset, ${e.message}")
			}
		}
	}
}
