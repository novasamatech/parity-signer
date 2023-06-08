package io.parity.signer.screens.keysets.create.backupstepscreens

import android.content.Context
import android.widget.Toast
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.domain.usecases.CreateKeySetViaStateMachineUseCase
import kotlinx.coroutines.launch


class NewKeySetNetworksViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)
	private val createKeySetUseCase = CreateKeySetViaStateMachineUseCase()

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	fun getDefaultPreselectedNetworks(): List<NetworkModel> =
		allNetworksUseCase.getDefaultPreselectedNetworks()

	fun createKeySetWithNetworks(
		seedName: String, seedPhrase: String,
		networksForKeys: Set<NetworkModel>,
		navigator: Navigator,
		onAfterFinishCleanup: Callback = {},
	): Unit {
		viewModelScope.launch {
			createKeySetUseCase.createKeySetWithNetworks(
				seedName, seedPhrase,
				networksForKeys, navigator
			)
			onAfterFinishCleanup()
		}
	}
}
