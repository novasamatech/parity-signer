package io.parity.signer.screens.scan.bananasplit.networks

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.domain.usecases.CreateKeySetUseCase
import kotlinx.coroutines.launch


class BananaNetworksViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	fun getDefaultPreselectedNetworks(): List<NetworkModel> =
		allNetworksUseCase.getDefaultPreselectedNetworks()
}
