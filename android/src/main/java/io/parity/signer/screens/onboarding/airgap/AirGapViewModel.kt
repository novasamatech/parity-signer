package io.parity.signer.screens.onboarding.airgap

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.mapState
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map


class AirGapViewModel : ViewModel() {

	private val networkExposedStateKeeper = ServiceLocator.networkExposedStateKeeper

	val isFlightModeEnabled: StateFlow<Boolean> =
		networkExposedStateKeeper.airplaneModeState.mapState(viewModelScope) { value: NetworkState -> value == NetworkState.Active }

	val isFinished: Flow<Boolean> = isFlightModeEnabled.map { value: Boolean -> !value }

}
