package io.parity.signer.screens.initial.eachstartchecks.airgap

import android.content.Context
import android.provider.Settings
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.FeatureFlags
import io.parity.signer.domain.FeatureOption
import io.parity.signer.domain.NetworkExposedStateKeeper
import io.parity.signer.domain.NetworkState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class AirGapViewModel : ViewModel() {

	private val appContext = ServiceLocator.appContext
	private val networkExposedStateKeeper: NetworkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	private val _state = MutableStateFlow<AirGapScreenState>(
		AirGapScreenState(
			airplaneModeEnabled = false,
			wifiDisabled = false,
			bluetoothDisabled = false,
			isAdbDisabled = false,
			isUsbDisconnected = false,
		)
	)
	val state: StateFlow<AirGapScreenState> = _state.asStateFlow()

	var scope: CoroutineScope? = null

	private fun isAdbEnabled(context: Context): Boolean {
		if (FeatureFlags.isEnabled(FeatureOption.SKIP_USB_CHECK)) return false

		return Settings.Global.getInt(context.contentResolver,
			Settings.Global.ADB_ENABLED, 0
		) == 1;
	}

	fun init() {
		val scope = CoroutineScope(viewModelScope.coroutineContext + Job())
		scope.launch {
			networkExposedStateKeeper.airPlaneModeEnabled.collect { newState ->
				_state.update {it.copy(airplaneModeEnabled = (newState != false)) }
			}
		}
		scope.launch {
			networkExposedStateKeeper.bluetoothDisabledState.collect { newState ->
				_state.update { it.copy(bluetoothDisabled = (newState != false)) }
			}
		}
		scope.launch {
			networkExposedStateKeeper.wifiDisabledState.collect { newState ->
				_state.update { it.copy(wifiDisabled = (newState != false)) }
			}
		}
		scope.launch {
			networkExposedStateKeeper.usbDisconnected.collect { newState ->
				_state.update { it.copy(isUsbDisconnected = (newState != false)) }
			}
		}
		scope.launch {
			while (true) {
				val adbEnabled = isAdbEnabled(appContext)
				if (_state.value.isAdbDisabled == !adbEnabled) {
					//skip it's the same
				} else {
					_state.update { it.copy(isAdbDisabled = (!adbEnabled)) }
				}
				delay(1000) //1s
			}
		}
		this.scope = scope
	}

	fun unInit() {
		scope?.cancel()
	}

	fun onConfirmedAirgap() {
		if (networkExposedStateKeeper.airGapModeState.value == NetworkState.Past) {
			networkExposedStateKeeper.acknowledgeWarning()
		}
	}
}
