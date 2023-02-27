package io.parity.signer.screens.onboarding.eachstartchecks

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.RootUtils
import io.parity.signer.domain.mapState
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map


class EachStartViewModel : ViewModel() {

	private val networkExposedStateKeeper = ServiceLocator.networkExposedStateKeeper

	val isFlightModeEnabled: StateFlow<Boolean> =
		networkExposedStateKeeper.airGapModeState.mapState(viewModelScope) { value: NetworkState -> value == NetworkState.Active }

	val isFinished: Flow<Boolean> = isFlightModeEnabled.map { value: Boolean -> !value }

	fun checkIsAuthPossible(context: Context): Boolean = Authentication.canAuthenticate(context)

	fun isDeviceRooted(): Boolean {
		return RootUtils.isDeviceRooted()
	}

}
