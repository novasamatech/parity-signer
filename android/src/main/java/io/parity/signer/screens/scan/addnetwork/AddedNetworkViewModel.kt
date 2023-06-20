package io.parity.signer.screens.scan.addnetwork

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.tryCreateAddress


class AddedNetworkViewModel : ViewModel() {

	private val repository: SeedRepository
		get() = ServiceLocator.activityScope!!.seedRepository

	fun getSeedList(): List<String> {
		return repository.getLastKnownSeedNames().toList()
	}

	suspend fun processAddNetworkToSeeds(network: NetworkModel, seeds: List<String>): Boolean {
		var result = true
		val seedPairs = repository.fillSeedToPhrasesAuth(seeds)
		when (seedPairs) {
			is RepoResult.Failure -> TODO() //todo dmitry implement
			is RepoResult.Success -> seedPairs.result.forEach {seedPair ->
				try {
					tryCreateAddress(
						seedName = seedPair.first, seedPhrase = seedPair.second,
						path = network.pathId, network = network.key,
					)
				} catch (e: ErrorDisplayed) {
					result = false
					submitErrorState("can't create network key for added network, ${e.message}")
				}
			}
		}
		return result
	}
}
