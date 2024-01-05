package io.parity.signer.domain

import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.wifi.WifiManager
import android.provider.Settings
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.uniffi.historyAcknowledgeWarnings
import io.parity.signer.uniffi.historyGetWarnings
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow


class NetworkExposedStateKeeper(
	private val appContext: Context,
	private val rustInteractor: UniffiInteractor
) {

	private val _airplaneModeEnabled: MutableStateFlow<Boolean?> =
		MutableStateFlow(null)
	val airPlaneModeEnabled: StateFlow<Boolean?> = _airplaneModeEnabled

	private val _wifiDisabledState: MutableStateFlow<Boolean?> =
		MutableStateFlow(null)
	val wifiDisabledState: StateFlow<Boolean?> = _wifiDisabledState

	private val _bluetoothDisabledState: MutableStateFlow<Boolean?> =
		MutableStateFlow(null)
	val bluetoothDisabledState: StateFlow<Boolean?> = _bluetoothDisabledState

	//todo dmitry implement
	private val _usbDisconnected: MutableStateFlow<Boolean?> =
		MutableStateFlow(null)
	val usbDisconnected: StateFlow<Boolean?> = _usbDisconnected

	private val _airGapModeState: MutableStateFlow<NetworkState> =
		MutableStateFlow(NetworkState.None)
	val airGapModeState: StateFlow<NetworkState> = _airGapModeState

	private val isCurentlyBreached: Boolean
		get() = airPlaneModeEnabled.value == true && wifiDisabledState.value == true
			&& bluetoothDisabledState.value == true && usbDisconnected.value == true

	init {
		registerAirplaneBroadcastReceiver()
		registerWifiBroadcastReceiver()
		registerBluetoothBroadcastReceiver()
		reactOnAirplaneMode()
		reactOnWifiAwareState()
		reactOnBluetooth()
	}

	/**
	 * Expects that rust nav machine is initialized that should always be the case
	 * as it's required to show UI calling this function
	 */
	fun acknowledgeWarning() {
		if (airGapModeState.value == NetworkState.Past) {
			historyAcknowledgeWarnings()
			_airGapModeState.value = NetworkState.None
		}
	}

	private fun registerAirplaneBroadcastReceiver() {
		val intentFilter = IntentFilter(Intent.ACTION_AIRPLANE_MODE_CHANGED)
		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				reactOnAirplaneMode()
			}
		}
		appContext.registerReceiver(receiver, intentFilter)
	}

	private fun registerBluetoothBroadcastReceiver() {
		val intentFilter = IntentFilter(BluetoothAdapter.ACTION_STATE_CHANGED)
		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				reactOnBluetooth()
			}
		}
		appContext.registerReceiver(receiver, intentFilter)
	}

	private fun updateGeneralAirgapState() {
		if (isCurentlyBreached) {
			if (airGapModeState.value != NetworkState.Active) {
				_airGapModeState.value = NetworkState.Active
				if (appContext.isDbCreatedAndOnboardingPassed()) {
					rustInteractor.historyDeviceWasOnline()
				}
			}
		} else {
			if (airGapModeState.value == NetworkState.Active) {
				_airGapModeState.value =
					if (appContext.isDbCreatedAndOnboardingPassed())
						NetworkState.Past else NetworkState.None
			}
		}
	}

	private fun reactOnAirplaneMode() {
		val airplaneModeOff = Settings.Global.getInt(
			appContext.contentResolver,
			Settings.Global.AIRPLANE_MODE_ON,
			0
		) == 0
		_airplaneModeEnabled.value = !airplaneModeOff
		updateGeneralAirgapState()
	}

	private fun reactOnBluetooth() {
		val bluetooth =
			appContext.applicationContext.getSystemService(BluetoothManager::class.java)?.adapter
		val btEnabled = bluetooth?.isEnabled == true
		_bluetoothDisabledState.value = !btEnabled
		updateGeneralAirgapState()
	}

	private fun registerWifiBroadcastReceiver() {
		val intentFilter = IntentFilter(WifiManager.NETWORK_STATE_CHANGED_ACTION)
		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				reactOnWifiAwareState()
			}
		}
		appContext.registerReceiver(receiver, intentFilter)
	}

	private fun reactOnWifiAwareState() {
		val wifi =
			appContext.applicationContext.getSystemService(Context.WIFI_SERVICE) as WifiManager?
		val wifiEnabled = wifi?.isWifiEnabled == true
		_wifiDisabledState.value = !wifiEnabled
		updateGeneralAirgapState()
	}

	/**
	 * Can't do initially as navigation should be initialized before we check rust.
	 */
	fun updateAlertStateFromHistory() {
		_airGapModeState.value = if (historyGetWarnings()) {
			if (airGapModeState.value == NetworkState.Active) NetworkState.Active else NetworkState.Past
		} else {
			NetworkState.None
		}
	}
}

/**
 * Describes current state of network detection alertness
 */
enum class NetworkState {
	None,
	Active,
	Past
}
