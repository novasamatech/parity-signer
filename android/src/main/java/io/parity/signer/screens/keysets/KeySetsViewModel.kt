package io.parity.signer.screens.keysets

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeySetsSelectModel
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.backend.mapError
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow


class KeySetsViewModel : ViewModel() {
	private val seedStorage = ServiceLocator.seedStorage
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	private val _keySetModel = MutableStateFlow<KeySetsSelectModel?>(
		KeySetsSelectModel(emptyList())
	)
	val keySetModel: StateFlow<KeySetsSelectModel?> = _keySetModel.asStateFlow()
	val networkState: StateFlow<NetworkState> =
		networkExposedStateKeeper.airGapModeState

	suspend fun updateKeySetModel() {
		val seedNames = seedStorage.lastKnownSeedNames.value.toList()
		val keysets = uniffiInteractor.getKeySets(seedNames)
		_keySetModel.value = keysets.mapError()
	}
}
