package io.parity.signer.screens.settings.networks.list

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.UniffiResult


class NetworkListViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	suspend fun getNetworkList(): UniffiResult<NetworksListModel> {
			return uniffiInteractor.getAllNetworks()
	}
}
