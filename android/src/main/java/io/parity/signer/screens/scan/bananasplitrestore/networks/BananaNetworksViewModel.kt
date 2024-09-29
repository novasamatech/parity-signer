package io.parity.signer.screens.scan.bananasplitrestore.networks

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase


class BananaNetworksViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	fun getDefaultPreselectedNetworks(): List<NetworkModel> =
		allNetworksUseCase.getDefaultPreselectedNetworks()
}
