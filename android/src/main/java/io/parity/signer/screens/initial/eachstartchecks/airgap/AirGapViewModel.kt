package io.parity.signer.screens.initial.eachstartchecks.airgap

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch


class AirGapViewModel : ViewModel() {

	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	private val _state = MutableStateFlow<AirGapScreenState>(
		AirGapScreenState(
			airplaneModeEnabled = false,
			wifiDisabled = false,
			bluetoothDisabled = false
		)
	)
	val state: StateFlow<AirGapScreenState> = _state.asStateFlow()

	var scope: CoroutineScope? = null

	fun onCableCheckboxClicked() {
		_state.value = _state.value.copy(cablesDisconnected = !_state.value.cablesDisconnected)
	}

	fun init() {
		val scope = CoroutineScope(viewModelScope.coroutineContext + Job())
		scope.launch {
			networkExposedStateKeeper.airPlaneModeEnabled.collect {
				_state.value = _state.value.copy(airplaneModeEnabled = (it != false))
			}
		}
		scope.launch {
			networkExposedStateKeeper.bluetoothDisabledState.collect {
				_state.value = _state.value.copy(bluetoothDisabled = (it != false))
			}
		}
		scope.launch {
			networkExposedStateKeeper.wifiDisabledState.collect {
				_state.value = _state.value.copy(wifiDisabled = (it != false))
			}
		}
		this.scope = scope
	}


	fun unInit() {
		scope?.cancel()
		_state.value =_state.value.copy(cablesDisconnected = false)
	}
}
