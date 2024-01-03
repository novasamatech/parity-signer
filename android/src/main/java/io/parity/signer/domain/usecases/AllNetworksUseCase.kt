package io.parity.signer.domain.usecases

import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.mapError
import io.parity.signer.domain.backend.mapErrorForce
import kotlinx.coroutines.runBlocking


class AllNetworksUseCase(val uniffiInteractor: UniffiInteractor) {
	private var allNetworks: List<NetworkModel> = runBlocking { getNetworks() }

	fun updateCache(): Unit {
		allNetworks = runBlocking { getNetworks() }
	}

	fun getAllNetworks(): List<NetworkModel> = allNetworks

	private val preselectedkeys = listOf<String>("Polkadot", "Kusama", "Westend")

	fun getDefaultPreselectedNetworks(): List<NetworkModel> = allNetworks
		.filter { preselectedkeys.contains(it.title) }

	private suspend fun getNetworks(): List<NetworkModel> {
		return uniffiInteractor.getAllNetworks().mapErrorForce()
	}
}
