package io.parity.signer.domain

import android.annotation.SuppressLint
import android.content.*
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.StateFlow


@SuppressLint("StaticFieldLeak")
class MainFlowViewModel() : ViewModel() {

	private val resetUseCase = ResetUseCase()

	val activity: FragmentActivity
		get() = ServiceLocator.activityScope!!.activity

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

	val authenticated: StateFlow<Boolean> = ServiceLocator.authentication.auth
}

