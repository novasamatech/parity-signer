package io.parity.signer.screens.keysetdetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyAndNetworkModel
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.NetworkInfoModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.backend.BackupInteractor
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.keysBySeedName
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch


class KeySetDetailsViewModel : ViewModel() {
	private val preferencesRepository = ServiceLocator.preferencesRepository
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val backupInteractor = BackupInteractor()
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper
	private val seedRepository = ServiceLocator.activityScope!!.seedRepository

	val filters = preferencesRepository.networksFilter.stateIn(
		viewModelScope,
		SharingStarted.WhileSubscribed(5_000),
		initialValue = emptySet(),
	)
	val networkState: StateFlow<NetworkState> =
		networkExposedStateKeeper.airGapModeState

	private suspend fun getKeySetDetails(requestseedName: String?): OperationResult<KeySetDetailsScreenState, ErrorDisplayed> {
		val seedName = requestseedName ?: preferencesRepository.getLastSelectedSeed()
		?: seedRepository.getLastKnownSeedNames().firstOrNull()

		val fullModel = try {
			//todo export this to vm and handle errors - open default for example
			keysBySeedName(seedName!!).toKeySetDetailsModel()
		} catch (e: ErrorDisplayed) {
			//todo dmitry
//			navController.navigate(
//				CoreUnlockedNavSubgraph.ErrorScreen.destination(
//					argHeader = "Unexpected error in keysBySeedName",
//					argDescription = e.toString(),
//					argVerbose = e.getDebugDetailedDescriptionString(),
//				)
			)
			null
		}
	}

	suspend fun feedModelForSeed(seedName: String?): StateFlow<OperationResult<KeySetDetailsScreenState, ErrorDisplayed>> {
		val result = getKeySetDetails(requestseedName = seedName)
		return filters.map { filterInstance ->
			when (result) {
				is OperationResult.Err -> result
				is OperationResult.Ok -> {
					if (filterInstance.isEmpty()) result else {
						val value = result.result
						OperationResult.Ok(
							value.copy(keysAndNetwork = value.keysAndNetwork
								.filter { filterInstance.contains(it.network.networkSpecsKey) })
						)
					}
				}
			}
		}.stateIn(
			viewModelScope,
			SharingStarted.WhileSubscribed(1_000),
			initialValue = result,
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

	suspend fun removeSeed(root: KeyModel): OperationResult<Unit, Exception> {
		return seedRepository.removeKeySet(root.seedName)
	}

	suspend fun getSeedPhrase(seedName: String): String? {
		return when (val result = seedRepository.getSeedPhraseForceAuth(seedName)) {
			is RepoResult.Failure -> {
				null
			}

			is RepoResult.Success -> {
				backupInteractor.notifyRustSeedWasShown(seedName)
				result.result
			}
		}
	}
}

sealed class KeySetDetailsScreenState {

	object LoadingState: KeySetDetailsScreenState()

	object EmptyState: KeySetDetailsScreenState()

	/**
	 * Local copy of shared [MKeys] class
	 */
	data class KeySetDetailsState(
		val keysAndNetwork: List<KeyAndNetworkModel>,
		val root: KeyModel?,
	): KeySetDetailsScreenState() {
		companion object {
			fun createStub(): KeySetDetailsState = KeySetDetailsState(
				keysAndNetwork = listOf(
					KeyAndNetworkModel(
						key = KeyModel.createStub(addressKey = "address key"),
						network = NetworkInfoModel.createStub()
					),
					KeyAndNetworkModel(
						key = KeyModel.createStub(addressKey = "address key2"),
						network = NetworkInfoModel.createStub(networkName = "Some")
					),
					KeyAndNetworkModel(
						key = KeyModel.createStub(addressKey = "address key3")
							.copy(path = "//polkadot//path3"),
						network = NetworkInfoModel.createStub()
					),
				),
				root = KeyModel.createStub()
					.copy(path = "//polkadot"),
			)
		}
	}
}
