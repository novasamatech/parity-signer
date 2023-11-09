package io.parity.signer.domain

import android.annotation.SuppressLint
import android.content.*
import android.util.Log
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch


@SuppressLint("StaticFieldLeak")
class MainFlowViewModel() : ViewModel() {

	private val resetUseCase = ResetUseCase()
	private val authentication = ServiceLocator.authentication
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	val networkState: StateFlow<NetworkState> =
		networkExposedStateKeeper.airGapModeState

	val activity: FragmentActivity
		get() = ServiceLocator.activityScope!!.activity

	fun onUnlockClicked() {
		viewModelScope.launch {
			when (authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> resetUseCase.totalRefresh()
				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Log.e("Signer", "Auth failed, not unlocked")
				}
			}
		}
	}

	val authenticated: StateFlow<Boolean> = authentication.auth
}

