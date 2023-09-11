package io.parity.signer.domain

import android.annotation.SuppressLint
import android.content.*
import android.security.keystore.UserNotAuthenticatedException
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.domain.storage.SeedStorage
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.json.JSONObject


@SuppressLint("StaticFieldLeak")
class SharedViewModel() : ViewModel() {

	private val resetUseCase = ResetUseCase()
	val context: Context
		get() = ServiceLocator.appContext.applicationContext
	val activity: FragmentActivity
		get() = ServiceLocator.activityScope!!.activity
	val networkState: StateFlow<NetworkState> =
		ServiceLocator.networkExposedStateKeeper.airGapModeState

	init {
		// Imitate ios behavior
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			resetUseCase.totalRefresh()
		}
	}

	fun onUnlockClicked() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			resetUseCase.totalRefresh()
		}
	}

	val navigator = ServiceLocator.navigator

	val authenticated: StateFlow<Boolean> = ServiceLocator.authentication.auth
	val actionResult: StateFlow<ActionResult?> = navigator.actionResult
}

