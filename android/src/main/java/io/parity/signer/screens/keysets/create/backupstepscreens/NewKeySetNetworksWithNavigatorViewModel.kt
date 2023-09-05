package io.parity.signer.screens.keysets.create.backupstepscreens

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.domain.usecases.CreateKeySetUseCase
import io.parity.signer.domain.usecases.CreateKeySetViaStateMachineUseCase
import kotlinx.coroutines.launch


class NewKeySetNetworksWithNavigatorViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)
	private val createKeySetUseCase = CreateKeySetUseCase()

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	fun getDefaultPreselectedNetworks(): List<NetworkModel> =
		allNetworksUseCase.getDefaultPreselectedNetworks()

	fun createKeySetWithNetworks(
		seedName: String, seedPhrase: String,
		networksForKeys: Set<NetworkModel>,
		onPostReaction: (Boolean) -> Unit,
	): Unit {
		viewModelScope.launch {
			val result = createKeySetUseCase.createKeySetWithNetworks(
				seedName, seedPhrase,
				networksForKeys.map { it.key },
			)
			onPostReaction(result)
		}
	}
}
