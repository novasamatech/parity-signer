package io.parity.signer.domain

import android.annotation.SuppressLint
import android.content.*
import timber.log.Timber
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.screens.error.ErrorStateDestinationState
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

	suspend fun onUnlockClicked(): OperationResult<Unit, ErrorStateDestinationState> {
			return when (authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> resetUseCase.totalRefresh()
				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Timber.e("Signer", "Auth failed, not unlocked")
					OperationResult.Ok(Unit)
				}
			}
	}

	val authenticated: StateFlow<Boolean> = authentication.auth
}

