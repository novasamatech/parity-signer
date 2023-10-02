package io.parity.signer.screens.keysets.create.backupstepscreens

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.domain.usecases.CreateKeySetUseCase
import kotlinx.coroutines.launch


class NewKeySetNetworksViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)
	private val createKeySetUseCase = CreateKeySetUseCase()

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	fun getDefaultPreselectedNetworks(): List<NetworkModel> =
		allNetworksUseCase.getDefaultPreselectedNetworks()

	fun createKeySetWithNetworks(
		seedName: String, seedPhrase: String,
		networkForKeys: Set<NetworkModel>,
		onAfterCreate: (Boolean) -> Unit = {},
	): Unit {
		viewModelScope.launch {
			val success = createKeySetUseCase.createKeySetWithNetworks(
				seedName, seedPhrase,
				networkForKeys.map { it.key },
			)
			onAfterCreate(success)
		}
	}
}
