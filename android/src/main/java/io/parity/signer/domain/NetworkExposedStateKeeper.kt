package io.parity.signer.domain

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.provider.Settings
import io.parity.signer.uniffi.historyAcknowledgeWarnings
import io.parity.signer.uniffi.historyDeviceWasOnline
import io.parity.signer.uniffi.historyGetWarnings
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow


class NetworkExposedStateKeeper(private val appContext: Context) {

	init {
		reactOnAirplaneMode()
		registerAirplaneBroadcastReceiver()
	}

	private val _networkState: MutableStateFlow<NetworkState> =
		MutableStateFlow(NetworkState.None)
	val networkState: StateFlow<NetworkState> = _networkState

	fun acknowledgeWarning() {
		if (networkState.value == NetworkState.Past) {
			historyAcknowledgeWarnings()
			_networkState.value = NetworkState.None
		}
	}

	private fun registerAirplaneBroadcastReceiver() {
		val intentFilter = IntentFilter("android.intent.action.AIRPLANE_MODE")
		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				reactOnAirplaneMode()
			}
		}
		appContext.registerReceiver(receiver, intentFilter)
	}

	/**
	 * Can't do initially as navigation should be init before we check rust.
	 */
	fun updateAlertState() {
		_networkState.value = if (historyGetWarnings()) {
			if (networkState.value == NetworkState.Active) NetworkState.Active else NetworkState.Past
		} else {
			NetworkState.None
		}
	}

	/**
	 * Checks if airplane mode was off
	 */
	private fun reactOnAirplaneMode() {
		if (Settings.Global.getInt(
				appContext.contentResolver,
				Settings.Global.AIRPLANE_MODE_ON,
				0
			) == 0
		) {
			if (networkState.value != NetworkState.Active) {
				_networkState.value = NetworkState.Active
				if (appContext.isDbCreatedAndOnboardingPassed()) {
					historyDeviceWasOnline()
				}
			}
		} else {
			if (networkState.value == NetworkState.Active) {
				_networkState.value = if (appContext.isDbCreatedAndOnboardingPassed())
					NetworkState.Past else NetworkState.None
			}
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
