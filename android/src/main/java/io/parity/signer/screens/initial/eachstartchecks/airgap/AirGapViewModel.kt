package io.parity.signer.screens.initial.eachstartchecks.airgap

import android.content.Context
import android.provider.Settings
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.FeatureFlags
import io.parity.signer.domain.FeatureOption
import io.parity.signer.domain.NetworkState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class AirGapViewModel : ViewModel() {

	private val networkExposedStateKeeper =
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

	//todo dmitry update adb and usb checks
	var scope: CoroutineScope? = null

	fun isAdbEnabled(context: Context): Boolean {
		//todo dmitry check usb checks
		if (FeatureFlags.isEnabled(FeatureOption.SKIP_USB_CHECK)) return false

		return Settings.Global.getInt(context.contentResolver,
			Settings.Global.ADB_ENABLED, 0
		) == 1;
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
	}

	fun onConfirmedAirgap() {
		if (networkExposedStateKeeper.airGapModeState.value == NetworkState.Past) {
			networkExposedStateKeeper.acknowledgeWarning()
		}
	}
}
