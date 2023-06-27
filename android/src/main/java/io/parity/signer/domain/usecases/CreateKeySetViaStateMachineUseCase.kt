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
	): Boolean {
		val repository = ServiceLocator.activityScope!!.seedRepository
		var success = repository.addSeed(
			seedName = seedName,
			seedPhrase = seedPhrase,
			navigator = navigator,
			isOptionalAuth = false
		)
		if (success) {
			networksKeys.forEach { networkKey ->
				try {
					tryCreateAddress(
						seedName = seedName, seedPhrase = seedPhrase,
						path = networkKey.pathId, network = networkKey.key,
					)
				} catch (e: Exception) {
					success = false
					submitErrorState("can't create network key for new keyset, ${e.message}")
				}
			}
		}
		return success
	}

}
