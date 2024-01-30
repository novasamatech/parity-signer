package io.parity.signer.screens.initial.eachstartchecks

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.RootUtils
import kotlinx.coroutines.flow.StateFlow


class EachStartViewModel : ViewModel() {

	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	fun isAuthPossible(context: Context): Boolean = Authentication.canAuthenticate(context)

	fun isDeviceRooted(): Boolean {
		return RootUtils.isDeviceRooted()
	}

	val networkState: StateFlow<NetworkState> = networkExposedStateKeeper.airGapModeState

}
