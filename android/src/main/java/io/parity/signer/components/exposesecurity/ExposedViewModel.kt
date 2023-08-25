package io.parity.signer.components.exposesecurity

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NetworkState
import kotlinx.coroutines.flow.StateFlow


class ExposedViewModel: ViewModel() {
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	val networkState: StateFlow<NetworkState> =
		networkExposedStateKeeper.airGapModeState

	fun acknowledgeWarning() {
		networkExposedStateKeeper.acknowledgeWarning()
	}
}
