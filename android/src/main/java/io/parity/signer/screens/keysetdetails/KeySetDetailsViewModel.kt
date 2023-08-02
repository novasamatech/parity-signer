package io.parity.signer.screens.keysetdetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch


class KeySetDetailsViewModel : ViewModel() {
	private val preferencesRepository = ServiceLocator.preferencesRepository
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)

	val filters = preferencesRepository.networksFilter.stateIn(
		viewModelScope,
		SharingStarted.WhileSubscribed(5_000),
		initialValue = emptySet(),
	)

	fun makeFilteredFlow(original : KeySetDetailsModel): StateFlow<KeySetDetailsModel> {
		return filters.map { filterInstance ->
			if (filterInstance.isEmpty()) original else {
				original.copy(keysAndNetwork = original.keysAndNetwork
					.filter { filterInstance.contains(it.network.networkSpecsKey) })
			}
		}.stateIn(
			viewModelScope,
			SharingStarted.WhileSubscribed(1_000),
			initialValue = original,
		)
	}

	fun getAllNetworks(): List<NetworkModel> {
		return allNetworksUseCase.getAllNetworks()
	}

	fun setFilters(networksToFilter: Set<NetworkModel>) {
		viewModelScope.launch {
			preferencesRepository.setNetworksFilter(networksToFilter.map { it.key }
				.toSet())
		}
	}
}
